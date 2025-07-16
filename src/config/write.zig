const std = @import("std");
const types = @import("./types.zig");
const constants = @import("./constants.zig");

pub fn write(allocator: std.mem.Allocator, config_file: std.fs.File, new_config: types.Config) !void {
    const json_string = try std.json.stringifyAlloc(allocator, new_config, .{});
    _ = try config_file.seekTo(0);
    _ = try config_file.write(json_string);
}
