const std = @import("std");
const log = @import("../common/log.zig");
const types = @import("./types.zig");
const ConfigWithHandle = @import("../config/types.zig").WithHandle;
const Config = @import("../config/types.zig").Config;
const APP_NAME = @import("../config/constants.zig").APP_NAME;
const Profile = @import("../profile/types.zig").Profile;
const help = @import("../help.zig");
const CacheWithHandle = @import("../cache.zig").WithHandle;
const writeCache = @import("../cache.zig").write;
const RepositoryData = @import("../cache.zig").RepositoryData;
const Environment = @import("../common/environment.zig").Environment;

const Sha256 = std.crypto.hash.sha2.Sha256;
const ExecutionError = error{ ExitCodeNot0, RepositoryAlreadyInitialized, UnstashedChanges };

fn makeSshConfig(allocator: std.mem.Allocator, current_profile: Profile) ![]const u8 {
    const abs_ssh_config_path = try std.fs.path.resolvePosix(allocator, &[_][]const u8{current_profile.git_credentials.ssh_key});

    const app_data_dir = try std.fs.getAppDataDir(allocator, APP_NAME);

    const path = try std.fs.cwd().realpathAlloc(allocator, ".");

    var res: [0x100]u8 = undefined;
    const file_name = std.base64.standard.Encoder.encode(&res, path);

    const ssh_dir_path = try std.fs.path.resolvePosix(allocator, &[_][]const u8{ app_data_dir, "ssh_configs" });

    _ = std.fs.openDirAbsolute(ssh_dir_path, .{}) catch {
        _ = try std.fs.makeDirAbsolute(ssh_dir_path);
    };

    const file_path = try std.fs.path.resolvePosix(allocator, &[_][]const u8{ ssh_dir_path, file_name });
    const file_handle = try std.fs.cwd().createFile(file_path, .{ .read = true });

    try file_handle.writeAll(try std.fmt.allocPrint(allocator, "Host *\n\tUser git\n\tIdentityFile {s}\n\tIdentitiesOnly yes\n", .{abs_ssh_config_path}));

    return file_path;
}

fn getDefaultBranchFromRemote(environment: Environment) ![]const u8 {
    const allocator = environment.allocator;
    const runner = environment.command_runner;

    const raw = try runner.execute(allocator, "git", &[_][]const u8{ "remote", "show", "origin" });

    var iterator = std.mem.splitAny(u8, raw, "\n");

    while (iterator.next()) |line| {
        if (std.mem.indexOf(u8, line, "HEAD branch:") != null) {
            const len = "HEAD branch:".len;

            return std.mem.trim(u8, line[(len + 2)..line.len], " ");
        }
    }

    // Should never be hit
    return "";
}

fn getDefaultBranch(environment: Environment) ![]const u8 {
    const allocator = environment.allocator;
    const runner = environment.command_runner;
    const cache_with_handle = environment.cache;

    const remote_url = try runner.execute(allocator, "git", &[_][]const u8{ "config", "--get", "remote.origin.url" });

    var default_branch: []const u8 = "";

    const maybe_cached_default_branch = cache_with_handle.cache.getByRemoteUrl(remote_url);

    if (maybe_cached_default_branch) |cached_repository_data| {
        default_branch = cached_repository_data.default_branch;
    } else {
        default_branch = try getDefaultBranchFromRemote(environment);

        var new_cache = cache_with_handle.cache;
        var new_data = std.ArrayList(RepositoryData).empty;
        try new_data.appendSlice(allocator, new_cache.repository_data);
        try new_data.append(allocator, RepositoryData{ .remote_url = remote_url, .default_branch = default_branch });

        new_cache.repository_data = new_data.items;

        try writeCache(cache_with_handle.file, new_cache);
    }

    return default_branch;
}

fn hasUnstashedChanges(environment: Environment) !bool {
    const allocator = environment.allocator;
    const runner = environment.command_runner;
    const output = try runner.execute(allocator, "git", &[_][]const u8{ "status", "-s" });

    if (output.len == 0) {
        return false;
    }

    var lines = std.mem.splitAny(u8, output, "\n");

    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t");

        if (std.mem.startsWith(u8, trimmed, "M")) {
            return true;
        }
    }

    return false;
}

pub fn handle(environment: Environment, command: types.GitCommand) !void {
    switch (command) {
        .init => |init_input| {
            const exists = if (std.fs.cwd().access(".git", .{})) true else |_| false;

            if (exists) {
                return ExecutionError.RepositoryAlreadyInitialized;
            }

            const allocator = environment.allocator;
            const config = environment.config.config;
            const logger = log.scoped(allocator, .git_init, .{});

            const current_profile = config.profiles[config.current_profile];
            const ssh_config_path = try makeSshConfig(allocator, current_profile);
            try logger.log("Running git init", .{});
            const runner = environment.command_runner;

            _ = try runner.execute(allocator, "git", &[_][]const u8{"init"});
            _ = try runner.execute(allocator, "git", &[_][]const u8{ "config", "user.name", current_profile.git_credentials.name });
            _ = try runner.execute(allocator, "git", &[_][]const u8{ "config", "user.email", current_profile.git_credentials.email });
            _ = try runner.execute(allocator, "git", &[_][]const u8{ "config", "core.sshCommand", try std.fmt.allocPrint(allocator, "ssh -F \"{s}\"", .{ssh_config_path}) });
            _ = try runner.execute(allocator, "git", &[_][]const u8{ "remote", "add", "origin", init_input.remote });
        },
        .restart => |restart_input| {
            const allocator = environment.allocator;

            const maybe_unstashed = try hasUnstashedChanges(environment);

            if (maybe_unstashed) {
                return ExecutionError.UnstashedChanges;
            }

            const destination = restart_input.destination;
            const origin = restart_input.origin orelse (try getDefaultBranch(environment));

            const logger = log.scoped(allocator, .git_restart, .{ .color_maps = &[_]log.ColorMap{
                log.ColorMap{ .color = log.Colors.magenta, .word = origin },
                log.ColorMap{ .color = log.Colors.magenta, .word = destination },
            } });

            try logger.log("Checking out to origin {s}", .{origin});
            const runner = environment.command_runner;

            _ = try runner.execute(allocator, "git", &[_][]const u8{ "checkout", origin });

            try logger.log("Deleting old destination branch {s}", .{destination});
            _ = try runner.execute(allocator, "git", &[_][]const u8{ "branch", "-D", destination });

            try logger.log("Pulling latest origin branch {s}", .{origin});
            _ = try runner.execute(allocator, "git", &[_][]const u8{ "pull", "origin", origin });

            try logger.log("Recreating destination branch {s}", .{destination});
            _ = try runner.execute(allocator, "git", &[_][]const u8{ "checkout", "-b", destination });
        },
        .submit => {
            const allocator = environment.allocator;
            const runner = environment.command_runner;

            const default_branch = try getDefaultBranch(environment);
            var current_branch = try runner.execute(allocator, "git", &[_][]const u8{ "branch", "--show-current" });

            current_branch = std.mem.trim(u8, current_branch, " \n");

            const logger = log.scoped(allocator, .git_submit, .{ .color_maps = &[_]log.ColorMap{
                log.ColorMap{ .color = log.Colors.magenta, .word = current_branch },
                log.ColorMap{ .color = log.Colors.magenta, .word = default_branch },
            } });

            try logger.log("Pushing changes to origin", .{});
            _ = try runner.execute(allocator, "git", &[_][]const u8{ "push", "origin" });

            const remote = try runner.execute(allocator, "git", &[_][]const u8{ "remote", "-v" });

            if (std.mem.indexOf(u8, remote, "github") != null) {
                // Look for the gh executable & try to open PR if present

                const gh_exists = (try runner.execute(allocator, "which", &[_][]const u8{"gh"})).len > 0;

                if (!gh_exists) {
                    return;
                }

                try logger.log("Opening github PR from {s} to {s}", .{ current_branch, default_branch });

                _ = try runner.execute(allocator, "gh", &[_][]const u8{ "pr", "create", "-B", default_branch, "-e" });
            }
        },
        .main => {
            const allocator = environment.allocator;
            const runner = environment.command_runner;

            var logger = log.scoped(allocator, .git_main, .{});
            try logger.log("Checking for unstashed changes", .{});
            const maybe_unstashed = try hasUnstashedChanges(environment);

            if (maybe_unstashed) {
                return ExecutionError.UnstashedChanges;
            }

            const default_branch = try getDefaultBranch(environment);
            logger.setConfig(.{ .color_maps = &[_]log.ColorMap{log.ColorMap{ .color = log.Colors.magenta, .word = default_branch }} });

            try logger.log("Moving to branch {s}", .{default_branch});

            _ = try runner.execute(allocator, "git", &[_][]const u8{ "checkout", default_branch });

            try logger.log("Pulling latest changes for branch {s}", .{default_branch});

            _ = try runner.execute(allocator, "git", &[_][]const u8{ "pull", "origin", default_branch });
        },
    }
}
