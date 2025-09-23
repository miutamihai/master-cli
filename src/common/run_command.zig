const std = @import("std");

const ExecutionError = error{ExitCodeNot0};

const Config = struct { verbose: ?bool, allow_error: ?bool };

fn initProcess(allocator: std.mem.Allocator, program: []const u8, args: []const []const u8) !std.process.Child {
    var full_args = std.ArrayList([]const u8).empty;

    try full_args.append(allocator, program);
    try full_args.appendSlice(allocator, args);

    return std.process.Child.init(full_args.items, allocator);
}

pub fn runCommand(allocator: std.mem.Allocator, program: []const u8, args: []const []const u8, config: ?Config) !void {
    var child = try initProcess(allocator, program, args);

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

pub fn commandOutput(allocator: std.mem.Allocator, program: []const u8, args: []const []const u8) ![]const u8 {
    var child = try initProcess(allocator, program, args);

    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;

    try child.spawn();

    const max_size = 10_000_000;
    var stdout: std.ArrayListUnmanaged(u8) = .empty;
    var stderr: std.ArrayListUnmanaged(u8) = .empty;

    _ = try child.collectOutput(allocator, &stdout, &stderr, max_size);

    const exit_code = try child.wait();

    if (exit_code.Exited != 0) {
        return ExecutionError.ExitCodeNot0;
    }

    return try stdout.toOwnedSlice(allocator);
}
