pub const GitCredentials = struct { name: []const u8, email: []const u8, ssh_key: []const u8 };

pub const Profile = struct { name: []const u8, git_credentials: GitCredentials };

pub const ProfileCommandKind = enum { add, set };

pub const ProfileCommand = union(ProfileCommandKind) { add: struct { name: []const u8 }, set: struct { name: []const u8 } };
