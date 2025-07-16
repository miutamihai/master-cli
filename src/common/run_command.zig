const std = @import("std");

const ExecutionError = error{ExitCodeNot0};

const Config = struct { verbose: ?bool, allow_error: ?bool };

fn init_process(allocator: std.mem.Allocator, program: []const u8, args: []const []const u8) !std.process.Child {
    var full_args: std.ArrayList([]const u8) = std.ArrayList([]const u8).init(allocator);

    try full_args.append(program);
    try full_args.appendSlice(args);

    return std.process.Child.init(full_args.items, allocator);
}

pub fn run_command(allocator: std.mem.Allocator, program: []const u8, args: []const []const u8, config: ?Config) !void {
    var child = try init_process(allocator, program, args);

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

pub fn command_output(allocator: std.mem.Allocator, program: []const u8, args: []const []const u8) ![]const u8 {
    var child = try init_process(allocator, program, args);

    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;

    try child.spawn();

    const maxSize = 1024;
    var stdout: std.ArrayListUnmanaged(u8) = .empty;
    var stderr: std.ArrayListUnmanaged(u8) = .empty;

    _ = try child.collectOutput(allocator, &stdout, &stderr, maxSize);

    const exit_code = try child.wait();

    if (exit_code.Exited != 0) {
        return ExecutionError.ExitCodeNot0;
    }

    return try stdout.toOwnedSlice(allocator);
}
