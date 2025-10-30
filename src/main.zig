const std = @import("std");
const parser = @import("./common/arg_parser.zig");
const log = @import("./common/log.zig");
const git = @import("./git/handle.zig");
const profile = @import("./profile/handle.zig");
const config = @import("./config/read.zig");
const cache = @import("./cache.zig");
const Environment = @import("./common/environment.zig").Environment;
const CommandRunner = @import("./common/run_command.zig").Command;
const CommandInterface = @import("./common/run_command.zig").CommandInterface;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const allocator = arena.allocator();

    const config_with_handle = try config.read(allocator);
    const cache_with_handle = try cache.read(allocator);

    var runner = CommandRunner.init();
    const environment = Environment{ .allocator = allocator, .config = config_with_handle, .cache = cache_with_handle, .command_runner = &runner.interface };

    const args = std.process.argsAlloc(allocator) catch |err| {
        std.debug.print("Failed to allocate arguments\n", .{});
        return err;
    };

    const maybe_input = try parser.parse(args);

    switch (maybe_input) {
        .help => |help| {
            const logger = log.scoped(allocator, .help, .{});

            try logger.plain("{s}", .{help});
        },
        .input => |input| {
            switch (input.command) {
                .git => |git_command| {
                    try git.handle(environment, git_command);
                },
                .profile => |profile_command| {
                    try profile.handle(allocator, config_with_handle, profile_command);
                },
                .version => {
                    const logger = log.scoped(allocator, .version, .{ .color_maps = &[_]log.ColorMap{log.ColorMap{ .color = log.Colors.magenta, .word = config_with_handle.config.version }} });

                    try logger.log("Current version: {s}", .{config_with_handle.config.version});
                },
            }
        },
    }
}
