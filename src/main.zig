const std = @import("std");
const parser = @import("./common/arg_parser.zig");
const log = @import("./common/log.zig");
const git = @import("./git/handle.zig");
const profile = @import("./profile/handle.zig");
const config = @import("./config/read.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const config_with_handle = try config.read(arena.allocator());

    const args = std.process.argsAlloc(arena.allocator()) catch |err| {
        std.debug.print("Failed to allocate arguments\n", .{});
        return err;
    };

    const input = try parser.parse(args);

    switch (input.command) {
        .git => |git_command| {
            try git.handle(arena.allocator(), config_with_handle, git_command, input.verbose);
        },
        .profile => |profile_command| {
            try profile.handle(arena.allocator(), config_with_handle, profile_command);
        },
        .version => {
            const logger = log.scoped(arena.allocator(), .version, .{ .color_maps = &[_]log.ColorMap{log.ColorMap{ .color = log.Colors.magenta, .word = config_with_handle.config.version }} });

            try logger.log("Current version: {s}", .{config_with_handle.config.version});
        },
    }
}
