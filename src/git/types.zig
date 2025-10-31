const common = @import("../common/types.zig");

pub const GitCommandKind = enum { init, restart, main };

pub const GitCommand = union(GitCommandKind) {
    init: struct { remote: []const u8 },
    restart: common.OriginDestination,
    main,
};
