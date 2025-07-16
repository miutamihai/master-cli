const std = @import("std");

const ExecutionError = error{ExitCodeNot0};

const Config = struct { verbose: ?bool, allow_error: ?bool };

pub fn run_command(allocator: std.mem.Allocator, program: []const u8, args: []const []const u8, config: ?Config) !void {
    var full_args: std.ArrayList([]const u8) = std.ArrayList([]const u8).init(allocator);

    try full_args.append(program);
    try full_args.appendSlice(args);

    var child = std.process.Child.init(full_args.items, allocator);

    const verbose = config.?.verbose == true;

    if (!verbose) {
        child.stderr_behavior = .Ignore;
        child.stdout_behavior = .Ignore;
    }

    try child.spawn();

    const exit_code = try child.wait();

    const allow_error = config.?.allow_error == true;

    if (exit_code.Exited != 0 and !allow_error) {
        return ExecutionError.ExitCodeNot0;
    }
}
