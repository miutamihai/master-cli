const std = @import("std");
const Profile = @import("./profile/types.zig").ProfileCommandKind;
const Git = @import("./git/types.zig").GitCommandKind;

const CommandKind = enum { profile, git, version };

pub const Commands = union(CommandKind) { profile: ?Profile, git: ?Git, version };

const helpHeader =
    \\Master Maker's CLI
;

fn getCommandDescription(field: []const u8) []const u8 {
    return switch (std.meta.stringToEnum(CommandKind, field).?) {
        .profile => "Set the current profile or add a new one",
        .git => "Manipulate git branches or repositories",
        .version => "Print the current version and exit",
    };
}

fn getGitDescription(field: []const u8) []const u8 {
    return switch (std.meta.stringToEnum(Git, field).?) {
        .init => "Initializes a repository with the current profile's git credentials",
        .restart => "Restarts the target branch from the base branch",
        .submit => "Commits & submits the code changes to the remote",
    };
}

fn getProfileDescription(field: []const u8) []const u8 {
    return switch (std.meta.stringToEnum(Profile, field).?) {
        .add => "Adds a new profile",
        .set => "Sets the input profile as the current one",
    };
}

fn makeHelpString(comptime T: type, top: []const u8, comptime getFieldHelp: (fn (field: []const u8) []const u8)) []const u8 {
    var result = top;

    result = helpHeader ++ "\n\n" ++ result ++ "\n";

    const fieldNames = std.meta.fieldNames(T);

    var charCount: u8 = 0;

    for (fieldNames) |field| {
        if (field.len > charCount) {
            charCount = field.len;
        }
    }

    for (fieldNames) |field| {
        const description = getFieldHelp(field);

        var space: []const u8 = "    ";
        var index = 0;

        while (index <= charCount - field.len) : (index += 1) {
            space = space ++ " ";
        }

        result = std.fmt.comptimePrint("{s}\n  {s}{s}{s}", .{ result, field, space, description });
    }

    return result ++ "\n";
}

pub fn get(command: ?Commands) []const u8 {
    if (command == null) {
        const top: []const u8 =
            \\Usage: mm [command] [options]
            \\Commands:
        ;

        return comptime makeHelpString(CommandKind, top, getCommandDescription);
    }

    return switch (command.?) {
        .git => |git| {
            if (git == null) {
                const top: []const u8 =
                    \\Usage: mm git [command] [options]
                    \\Commands:
                ;

                return comptime makeHelpString(Git, top, getGitDescription);
            }

            return switch (git.?) {
                .init => "Initializes a repository with the current profile's git credentials",
                .restart => "Restarts the target branch from the base branch",
                .submit => "Commits & submits the code changes to the remote",
            };
        },
        .profile => |profile| {
            if (profile == null) {
                const top: []const u8 =
                    \\Usage: mm profile [command] [options]
                    \\Commands:
                ;

                return comptime makeHelpString(Profile, top, getProfileDescription);
            }

            return switch (profile.?) {
                .add => "Adds a new profile",
                .set => "Sets the input profile as the current one",
            };
        },
        .version => {
            const top: []const u8 = "Usage: mm version";

            return helpHeader ++ "\n\n" ++ top ++ "\n";
        },
    };
}
