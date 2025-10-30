const std = @import("std");

const MAX_OUTPUT_SIZE = 10_000_000;

const ExecutionError = error{ExitCodeNot0} || error{OutOfMemory} || std.process.Child.SpawnError || std.process.Child.RunError;

const Config = struct { verbose: ?bool, allow_error: ?bool };

pub const CommandInterface = struct {
    const Self = @This();

    interface: *const fn (*Self, allocator: std.mem.Allocator, program: []const u8, args: []const []const u8) ExecutionError![]const u8,

    pub fn execute(interface: *Self, allocator: std.mem.Allocator, program: []const u8, args: []const []const u8) ExecutionError![]const u8 {
        return interface.*.interface(interface, allocator, program, args);
    }
};

pub const Command = struct {
    const Self = @This();

    interface: CommandInterface,

    pub fn init() Self {
        return Self{ .interface = CommandInterface{ .interface = execute } };
    }

    pub fn execute(interface: *CommandInterface, allocator: std.mem.Allocator, program: []const u8, args: []const []const u8) ExecutionError![]const u8 {
        const command: *Command = @fieldParentPtr("interface", interface);

        return command.*.runCommand(allocator, program, args);
    }

    fn runCommand(_: Self, allocator: std.mem.Allocator, program: []const u8, args: []const []const u8) ExecutionError![]const u8 {
        var argv = std.ArrayList([]const u8).empty;
        defer argv.deinit(allocator);

        try argv.append(allocator, program);
        try argv.appendSlice(allocator, args);

        var child = std.process.Child.init(argv.items, allocator);

        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;

        try child.spawn();

        var stdout: std.ArrayListUnmanaged(u8) = .empty;
        var stderr: std.ArrayListUnmanaged(u8) = .empty;

        defer stdout.deinit(allocator);
        defer stderr.deinit(allocator);

        _ = try child.collectOutput(allocator, &stdout, &stderr, MAX_OUTPUT_SIZE);

        const exit_code = try child.wait();

        if (exit_code.Exited != 0) {
            return ExecutionError.ExitCodeNot0;
        }

        return stdout.items[0..];
    }
};
