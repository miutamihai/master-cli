const std = @import("std");
const types = @import("./types.zig");
const constants = @import("./constants.zig");

pub fn write(config_file: std.fs.File, new_config: types.Config) !void {
    var buffer: [1024]u8 = undefined;
    var file_writer: std.fs.File.Writer = config_file.writer(&buffer);
    const writer_pointer: *std.Io.Writer = &file_writer.interface;
    try std.json.Stringify.value(new_config, .{}, writer_pointer);
    try file_writer.interface.flush();
}
