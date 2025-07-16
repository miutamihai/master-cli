const std = @import("std");
const profile = @import("../profile/types.zig");

pub const Config = struct {
    version: []const u8,
    current_profile: usize,
    profiles: []const profile.Profile,
};

pub const WithHandle = struct { file: std.fs.File, config: Config };
