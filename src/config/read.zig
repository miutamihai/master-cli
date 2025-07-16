const std = @import("std");

const constants = @import("./constants.zig");
const types = @import("./types.zig");
const config = @import("binary_config");
const profile = @import("../profile/types.zig");

fn generate() types.Config {
    const git_credentials =
        profile.GitCredentials{ .name = "miutamihai", .email = "mihaimiuta@proton.me", .ssh_key = "~/.ssh/personal" };

    const personal_profile = profile.Profile{ .name = "personal", .git_credentials = git_credentials };

    return types.Config{ .version = config.version, .current_profile = 0, .profiles = &[_]profile.Profile{personal_profile} };
}

pub fn read(allocator: std.mem.Allocator) !types.WithHandle {
    const app_data_dir = try std.fs.getAppDataDir(allocator, constants.APP_NAME);
    const config_path = try std.fs.path.join(allocator, &[_][]const u8{ app_data_dir, constants.CONFIG_FILE_NAME });

    const file = std.fs.openFileAbsolute(config_path, .{ .mode = .read_write }) catch blk: {
        _ = std.fs.openDirAbsolute(app_data_dir, .{}) catch {
            _ = try std.fs.makeDirAbsolute(app_data_dir);
        };

        const handle = try std.fs.cwd().createFile(config_path, .{ .read = true });

        break :blk handle;
    };

    const bytes = try file.readToEndAlloc(allocator, 1_000_000);

    if (bytes.len == 0) {
        const data = generate();

        _ = try file.write(try std.json.stringifyAlloc(allocator, data, .{}));

        return .{ .config = data, .file = file };
    }

    const parsed = try std.json.parseFromSlice(types.Config, allocator, bytes, .{});

    return .{ .config = parsed.value, .file = file };
}
