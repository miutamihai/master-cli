const git = @import("./git/types.zig");
const profile = @import("./profile/types.zig");

pub const CommandKind = enum { git, profile, version };

pub const Command = union(CommandKind) { git: git.GitCommand, profile: profile.ProfileCommand, version };

pub const Input = struct {
    command: Command,
    verbose: bool = false,
};
