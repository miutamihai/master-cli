const std = @import("std");
const log = @import("../common/log.zig");
const run_command = @import("../common/run_command.zig").run_command;
const types = @import("./types.zig");
const ConfigWithHandle = @import("../config/types.zig").WithHandle;
const Config = @import("../config/types.zig").Config;
const APP_NAME = @import("../config/constants.zig").APP_NAME;
const Profile = @import("../profile/types.zig").Profile;

const Sha256 = std.crypto.hash.sha2.Sha256;
const ExecutionError = error{ ExitCodeNot0, RepositoryAlreadyInitialized };

fn make_ssh_config(allocator: std.mem.Allocator, current_profile: Profile) ![]const u8 {
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

pub fn handle(allocator: std.mem.Allocator, config_with_handle: ConfigWithHandle, command: types.GitCommand, verbose: bool) !void {
    switch (command) {
        .init => |init_input| {
            const exists = if (std.fs.cwd().access(".git", .{})) true else |_| false;

            if (exists) {
                return ExecutionError.RepositoryAlreadyInitialized;
            }

            const logger = log.scoped(allocator, .git_init, .{});

            const current_profile = config_with_handle.config.profiles[config_with_handle.config.current_profile];
            const ssh_config_path = try make_ssh_config(allocator, current_profile);
            try logger.log("Running git init", .{});
            try run_command(allocator, "git", &[_][]const u8{"init"}, .{ .verbose = verbose, .allow_error = null });
            try run_command(allocator, "git", &[_][]const u8{ "config", "user.name", current_profile.git_credentials.name }, .{ .verbose = false, .allow_error = false });
            try run_command(allocator, "git", &[_][]const u8{ "config", "user.email", current_profile.git_credentials.email }, .{ .verbose = false, .allow_error = false });
            try run_command(allocator, "git", &[_][]const u8{ "config", "core.sshCommand", try std.fmt.allocPrint(allocator, "ssh -F \"{s}\"", .{ssh_config_path}) }, .{ .verbose = false, .allow_error = false });
            try run_command(allocator, "git", &[_][]const u8{ "remote", "add", "origin", init_input.remote }, .{ .verbose = false, .allow_error = false });
        },
        .restart => |restart_input| {
            const logger = log.scoped(allocator, .git_restart, .{ .color_maps = &[_]log.ColorMap{
                log.ColorMap{ .color = log.Colors.magenta, .word = restart_input.origin },
                log.ColorMap{ .color = log.Colors.magenta, .word = restart_input.destination },
            } });

            try logger.log("Checking out to origin {s}", .{restart_input.origin});
            try run_command(allocator, "git", &[_][]const u8{ "checkout", restart_input.origin }, .{ .verbose = verbose, .allow_error = null });

            try logger.log("Deleting old destination branch {s}", .{restart_input.destination});
            try run_command(allocator, "git", &[_][]const u8{ "branch", "-D", restart_input.destination }, .{ .verbose = verbose, .allow_error = true });

            try logger.log("Pulling latest origin branch {s}", .{restart_input.origin});
            try run_command(allocator, "git", &[_][]const u8{ "pull", "origin", restart_input.origin }, .{ .verbose = verbose, .allow_error = null });

            try logger.log("Recreating destination branch {s}", .{restart_input.destination});
            try run_command(allocator, "git", &[_][]const u8{ "checkout", "-b", restart_input.destination }, .{ .verbose = verbose, .allow_error = null });
        },
    }
}
