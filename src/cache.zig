const std = @import("std");
const builtin = @import("builtin");

//FIXME combine constants
const config_constants = @import("./config/constants.zig");

const CACHE_FILE_NAME = "cache.json";

pub const RepositoryData = struct {
    remote_url: []const u8,
    default_branch: []const u8,
};

pub const Cache = struct {
    repository_data: []RepositoryData,

    pub fn getByRemoteUrl(self: @This(), remote_url: []const u8) ?RepositoryData {
        for (self.repository_data) |entry| {
            if (std.mem.eql(u8, remote_url, entry.remote_url)) {
                return entry;
            }
        }

        return null;
    }
};

pub const WithHandle = struct { file: std.fs.File, cache: Cache };

fn generate() Cache {
    return Cache{ .repository_data = &[_]RepositoryData{} };
}

pub fn read(allocator: std.mem.Allocator) !WithHandle {
    const cache_dir = try std.fs.getAppDataDir(allocator, config_constants.APP_NAME);
    const cache_path = try std.fs.path.join(allocator, &[_][]const u8{ cache_dir, CACHE_FILE_NAME });

    const file = std.fs.openFileAbsolute(cache_path, .{ .mode = .read_write }) catch blk: {
        _ = std.fs.openDirAbsolute(cache_dir, .{}) catch {
            _ = try std.fs.makeDirAbsolute(cache_dir);
        };

        const handle = try std.fs.cwd().createFile(cache_path, .{ .read = true });

        break :blk handle;
    };

    const bytes = try file.readToEndAlloc(allocator, 1_000_000);

    if (bytes.len == 0) {
        const data = generate();

        var buffer: [1024]u8 = undefined;
        var file_writer: std.fs.File.Writer = file.writer(&buffer);
        const writer_pointer: *std.Io.Writer = &file_writer.interface;

        try std.json.Stringify.value(data, .{}, writer_pointer);

        try file_writer.interface.flush();

        return .{ .cache = data, .file = file };
    }

    const parsed = try std.json.parseFromSlice(Cache, allocator, bytes, .{});

    return .{ .cache = parsed.value, .file = file };
}

pub fn write(cache_file: std.fs.File, new_cache: Cache) !void {
    var buffer: [1024]u8 = undefined;
    var file_writer: std.fs.File.Writer = cache_file.writer(&buffer);
    const writer_pointer: *std.Io.Writer = &file_writer.interface;
    try std.json.Stringify.value(new_cache, .{}, writer_pointer);
    try file_writer.interface.flush();
}
