const std = @import("std");
const log = @import("../common/log.zig");
const runCommand = @import("../common/run_command.zig").runCommand;
const commandOutput = @import("../common/run_command.zig").commandOutput;
const types = @import("./types.zig");
const ConfigWithHandle = @import("../config/types.zig").WithHandle;
const Config = @import("../config/types.zig").Config;
const APP_NAME = @import("../config/constants.zig").APP_NAME;
const Profile = @import("../profile/types.zig").Profile;
const help = @import("../help.zig");

const Sha256 = std.crypto.hash.sha2.Sha256;
const ExecutionError = error{ ExitCodeNot0, RepositoryAlreadyInitialized, UnstashedChanges };

fn makeSshConfig(allocator: std.mem.Allocator, current_profile: Profile) ![]const u8 {
    const abs_ssh_config_path = try std.fs.path.resolvePosix(allocator, &[_][]const u8{current_profile.git_credentials.ssh_key});

    const app_data_dir = try std.fs.getAppDataDir(allocator, APP_NAME);

    const path = try std.fs.cwd().realpathAlloc(allocator, ".");

    var hasher = Sha256.init(.{});

    hasher.update(path);
    const digest = hasher.finalResult();

    const file_name = try std.fmt.allocPrint(allocator, "{s}", .{std.fmt.fmtSliceHexLower(&digest)});

    const ssh_dir_path = try std.fs.path.resolvePosix(allocator, &[_][]const u8{ app_data_dir, "ssh_configs" });

    _ = std.fs.openDirAbsolute(ssh_dir_path, .{}) catch {
        _ = try std.fs.makeDirAbsolute(ssh_dir_path);
    };

    const file_path = try std.fs.path.resolvePosix(allocator, &[_][]const u8{ ssh_dir_path, file_name });
    const file_handle = try std.fs.cwd().createFile(file_path, .{ .read = true });

    try file_handle.writeAll(try std.fmt.allocPrint(allocator, "Host *\n\tUser git\n\tIdentityFile {s}\n\tIdentitiesOnly yes\n", .{abs_ssh_config_path}));

    return file_path;
}

fn makeCommitFile(allocator: std.mem.Allocator) ![]const u8 {
    const path = try std.fs.cwd().realpathAlloc(allocator, ".");

    var hasher = Sha256.init(.{});

    hasher.update(path);
    const digest = hasher.finalResult();

    const file_name = try std.fmt.allocPrint(allocator, "{s}", .{std.fmt.fmtSliceHexLower(&digest)});

    const editor = std.posix.getenv("EDITOR") orelse "vi";

    var editor_process = std.process.Child.init(&[_][]const u8{ editor, file_name }, allocator);

    try editor_process.spawn();

    _ = try editor_process.wait();

    return file_name;
}

fn getDefaultBranch(allocator: std.mem.Allocator) ![]const u8 {
    const raw = try commandOutput(allocator, "git", &[_][]const u8{ "remote", "show", "origin" });

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

fn hasUnstashedChanges(allocator: std.mem.Allocator) !bool {
    const output = try commandOutput(allocator, "git", &[_][]const u8{ "status", "-s" });

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

pub fn handle(allocator: std.mem.Allocator, config_with_handle: ConfigWithHandle, command: types.GitCommand, verbose: bool) !void {
    switch (command) {
        .init => |init_input| {
            const exists = if (std.fs.cwd().access(".git", .{})) true else |_| false;

            if (exists) {
                return ExecutionError.RepositoryAlreadyInitialized;
            }

            const logger = log.scoped(allocator, .git_init, .{});

            const current_profile = config_with_handle.config.profiles[config_with_handle.config.current_profile];
            const ssh_config_path = try makeSshConfig(allocator, current_profile);
            try logger.log("Running git init", .{});
            try runCommand(allocator, "git", &[_][]const u8{"init"}, .{ .verbose = verbose, .allow_error = null });
            try runCommand(allocator, "git", &[_][]const u8{ "config", "user.name", current_profile.git_credentials.name }, .{ .verbose = false, .allow_error = false });
            try runCommand(allocator, "git", &[_][]const u8{ "config", "user.email", current_profile.git_credentials.email }, .{ .verbose = false, .allow_error = false });
            try runCommand(allocator, "git", &[_][]const u8{ "config", "core.sshCommand", try std.fmt.allocPrint(allocator, "ssh -F \"{s}\"", .{ssh_config_path}) }, .{ .verbose = false, .allow_error = false });
            try runCommand(allocator, "git", &[_][]const u8{ "remote", "add", "origin", init_input.remote }, .{ .verbose = false, .allow_error = false });
        },
        .restart => |restart_input| {
            const maybe_unstashed = try hasUnstashedChanges(allocator);

            if (maybe_unstashed) {
                return ExecutionError.UnstashedChanges;
            }

            const logger = log.scoped(allocator, .git_restart, .{ .color_maps = &[_]log.ColorMap{
                log.ColorMap{ .color = log.Colors.magenta, .word = restart_input.origin },
                log.ColorMap{ .color = log.Colors.magenta, .word = restart_input.destination },
            } });

            try logger.log("Checking out to origin {s}", .{restart_input.origin});
            try runCommand(allocator, "git", &[_][]const u8{ "checkout", restart_input.origin }, .{ .verbose = verbose, .allow_error = null });

            try logger.log("Deleting old destination branch {s}", .{restart_input.destination});
            try runCommand(allocator, "git", &[_][]const u8{ "branch", "-D", restart_input.destination }, .{ .verbose = verbose, .allow_error = true });

            try logger.log("Pulling latest origin branch {s}", .{restart_input.origin});
            try runCommand(allocator, "git", &[_][]const u8{ "pull", "origin", restart_input.origin }, .{ .verbose = verbose, .allow_error = null });

            try logger.log("Recreating destination branch {s}", .{restart_input.destination});
            try runCommand(allocator, "git", &[_][]const u8{ "checkout", "-b", restart_input.destination }, .{ .verbose = verbose, .allow_error = null });
        },
        .submit => {
            const current_branch = try commandOutput(allocator, "git", &[_][]const u8{ "branch", "--show-current" });

            const logger = log.scoped(allocator, .git_submit, .{ .color_maps = &[_]log.ColorMap{log.ColorMap{ .color = log.Colors.magenta, .word = current_branch }} });

            try logger.log("Asking for commit message", .{});
            const commit_file = try makeCommitFile(allocator);

            try logger.log("Commiting changes for branch {s}", .{current_branch});
            try runCommand(allocator, "git", &[_][]const u8{ "commit", "-F", commit_file, "-a" }, .{ .verbose = verbose, .allow_error = false });

            _ = try std.fs.cwd().deleteFile(commit_file);

            try logger.log("Pushing changes to origin", .{});
            try runCommand(allocator, "git", &[_][]const u8{ "push", "origin" }, .{ .verbose = verbose, .allow_error = false });

            const remote = try commandOutput(allocator, "git", &[_][]const u8{ "remote", "-v" });

            if (std.mem.indexOf(u8, remote, "github") != null) {
                // Look for the gh executable & try to open PR if present

                const gh_exists = (try commandOutput(allocator, "which", &[_][]const u8{"gh"})).len > 0;

                if (!gh_exists) {
                    return;
                }

                // Ensure that the correct editor is used
                const editor = std.posix.getenv("EDITOR") orelse "vi";

                try runCommand(allocator, "gh", &[_][]const u8{ "config", "set", "editor", editor }, .{ .verbose = verbose, .allow_error = false });

                const default_branch = try getDefaultBranch(allocator);

                try runCommand(allocator, "gh", &[_][]const u8{ "pr", "create", "-B", default_branch, "-e" }, .{ .verbose = verbose, .allow_error = false });
            }
        },
        .main => {
            var logger = log.scoped(allocator, .git_main, .{});
            try logger.log("Checking for unstashed changes", .{});
            const maybe_unstashed = try hasUnstashedChanges(allocator);

            if (maybe_unstashed) {
                return ExecutionError.UnstashedChanges;
            }

            try logger.log("Determining the default branch", .{});
            const default_branch = try getDefaultBranch(allocator);

            logger.setConfig(.{ .color_maps = &[_]log.ColorMap{log.ColorMap{ .color = log.Colors.magenta, .word = default_branch }} });

            try logger.log("Moving to branch {s}", .{default_branch});

            try runCommand(allocator, "git", &[_][]const u8{ "checkout", default_branch }, .{ .verbose = verbose, .allow_error = false });

            try logger.log("Pulling latest changes for branch {s}", .{default_branch});

            try runCommand(allocator, "git", &[_][]const u8{ "pull", "origin", default_branch }, .{ .verbose = verbose, .allow_error = false });
        },
    }
}
