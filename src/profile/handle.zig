const std = @import("std");
const types = @import("./types.zig");
const log = @import("../common/log.zig");
const write_config = @import("../config/write.zig").write;
const ConfigWithHandle = @import("../config/types.zig").WithHandle;
const Config = @import("../config/types.zig").Config;
const Profile = @import("../profile/types.zig").Profile;

// TODO: Merge this with the other placeholder
const PLACEHOLDER_TEXT = "REPLACE_ME";

const ProfileErrors = error{ UnfulfilledPlaceholder, ProfileNotFound };

fn make_profile(allocator: std.mem.Allocator, name: []const u8) !types.Profile {
    const placeholder = types.Profile{ .name = name, .git_credentials = .{ .email = PLACEHOLDER_TEXT, .name = PLACEHOLDER_TEXT, .ssh_key = PLACEHOLDER_TEXT } };

    const file_name = "temp";

    const json_placeholder = try std.json.stringifyAlloc(allocator, placeholder, .{});

    var file_handle = try std.fs.cwd().createFile(file_name, .{ .read = true });

    _ = try file_handle.write(json_placeholder);

    const editor = std.posix.getenv("EDITOR") orelse "vi";

    var editor_process = std.process.Child.init(&[_][]const u8{ editor, file_name }, allocator);

    try editor_process.spawn();

    _ = try editor_process.wait();

    file_handle = try std.fs.cwd().openFile(file_name, .{ .mode = .read_write });

    const editted_json = try file_handle.readToEndAlloc(allocator, 1_000_000);

    _ = try std.fs.cwd().deleteFile(file_name);

    if (std.mem.indexOf(u8, editted_json, PLACEHOLDER_TEXT) != null) {
        return ProfileErrors.UnfulfilledPlaceholder;
    }

    const parsed =
        try std.json.parseFromSlice(types.Profile, allocator, editted_json, .{});

    return parsed.value;
}

pub fn handle(allocator: std.mem.Allocator, config_with_handle: ConfigWithHandle, command: types.ProfileCommand) !void {
    switch (command) {
        .add => |profile_input| {
            const logger = log.scoped(allocator, .profile_add, .{ .color_maps = &[_]log.ColorMap{.{ .color = log.Colors.magenta, .word = profile_input.name }} });

            try logger.log("Adding profile {s}", .{profile_input.name});

            var new_profiles = std.ArrayList(types.Profile).init(allocator);

            try new_profiles.appendSlice(config_with_handle.config.profiles);
            const new_profile = try make_profile(allocator, profile_input.name);
            try new_profiles.append(new_profile);

            const new_config = Config{ .version = config_with_handle.config.version, .profiles = new_profiles.items, .current_profile = new_profiles.items.len - 1 };
            try write_config(allocator, config_with_handle.file, new_config);

            try logger.log("Profile {s} set as the current profile", .{profile_input.name});
        },
        .set => |set_input| {
            const logger = log.scoped(allocator, .profile_set, .{ .color_maps = &[_]log.ColorMap{.{ .color = log.Colors.magenta, .word = set_input.name }} });

            try logger.log("Setting profile {s} as current profile", .{set_input.name});

            const profiles = config_with_handle.config.profiles;

            var i: usize = 0;

            const new_index = while (i < profiles.len) : (i += 1) {
                if (std.mem.eql(u8, profiles[i].name, set_input.name)) {
                    break i;
                }
            } else null;

            if (new_index == null) {
                return ProfileErrors.ProfileNotFound;
            }

            var new_config = config_with_handle.config;

            new_config.current_profile = new_index.?;

            try write_config(allocator, config_with_handle.file, new_config);

            try logger.log("Profile {s} set as the current profile", .{set_input.name});
        },
    }
}
