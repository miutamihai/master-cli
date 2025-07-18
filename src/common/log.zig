const std = @import("std");

const AnsiCodes = enum {
    cyan,
    red,
    yellow,
    magenta,
    green,

    escape,
    reset,

    const Self = @This();

    pub fn to_string(self: Self) []const u8 {
        return switch (self) {
            .cyan => "\x1B[96m",
            .red => "\x1B[91m",
            .yellow => "\x1B[93m",
            .magenta => "\x1B[95m",
            .green => "\x1B[92m",
            .escape => "\x33",
            .reset => "\x1B[0m",
        };
    }
};

pub const Colors = enum {
    cyan,
    red,
    yellow,
    magenta,
    green,
};

fn color_to_code(color: Colors) AnsiCodes {
    return switch (color) {
        .cyan => AnsiCodes.cyan,
        .red => AnsiCodes.red,
        .yellow => AnsiCodes.yellow,
        .magenta => AnsiCodes.magenta,
        .green => AnsiCodes.green,
    };
}

pub const LogLevel = enum {
    info,
    warn,
    err,
};

pub const ColorMap = struct { word: []const u8, color: Colors };
pub const Config = struct { level: LogLevel = .info, color_maps: []const ColorMap = &[_]ColorMap{} };

const Logger = @This();

allocator: std.mem.Allocator,
std_out: std.fs.File,
config: Config,
scope_name: ?[]const u8,

pub fn init(allocator: std.mem.Allocator, config: Config) Logger {
    return .{ .std_out = std.io.getStdOut(), .allocator = allocator, .config = config, .scope_name = null };
}

pub fn scoped(allocator: std.mem.Allocator, comptime scope: @Type(.enum_literal), config: Config) Logger {
    var instance = init(allocator, config);

    if (scope != .default) {
        instance.scope_name = @tagName(scope);
    }

    return instance;
}

pub fn log(self: Logger, comptime fmt: []const u8, args: anytype) !void {
    const message = try std.fmt.allocPrint(self.allocator, fmt, args);
    var iterator = std.mem.splitAny(u8, message, " ");
    var message_parts = std.ArrayList(u8).init(self.allocator);
    defer message_parts.deinit();

    try message_parts.appendSlice(try self.display_level(self.config.level));

    while (iterator.next()) |word| {
        var part = word;

        const maybe_index = for (self.config.color_maps, 0..self.config.color_maps.len) |color_entry, index| {
            if (std.mem.eql(u8, color_entry.word, word)) {
                break index;
            }
        } else null;

        if (maybe_index) |index| {
            part = try self.colored(self.config.color_maps[index].color, word);
        }

        try message_parts.append(' ');
        try message_parts.appendSlice(part);
    }
    try message_parts.append('\n');

    try self.std_out.writeAll(message_parts.items);
}

fn display_level(self: Logger, level: LogLevel) ![]const u8 {
    var parts = std.ArrayList(u8).init(self.allocator);

    try parts.appendSlice(try switch (level) {
        .info => self.colored(Colors.cyan, "info"),
        .warn => self.colored(Colors.yellow, "warn"),
        .err => self.colored(Colors.red, "error"),
    });

    if (self.scope_name) |scope| {
        try parts.appendSlice(try self.colored(Colors.green, "("));
        try parts.appendSlice(try self.colored(Colors.green, scope));
        try parts.appendSlice(try self.colored(Colors.green, ")"));
    }

    try parts.appendSlice(":");

    return parts.items;
}

fn colored(self: Logger, color: Colors, text: []const u8) ![]const u8 {
    return try std.mem.concat(self.allocator, u8, &[_][]const u8{ color_to_code(color).to_string(), text, AnsiCodes.reset.to_string() });
}
