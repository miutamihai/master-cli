const common = @import("../common/types.zig");

pub const GitCommandKind = enum { init, restart, submit, main };

pub const GitCommand = union(GitCommandKind) {
    init: struct { remote: []const u8 },
    restart: common.OriginDestination,
    submit,
    main,
};
