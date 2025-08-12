const std = @import("std");
const Profile = @import("./profile/types.zig").ProfileCommandKind;
const Git = @import("./git/types.zig").GitCommandKind;

const CommandKind = enum { profile, git, version };

pub const Commands = union(CommandKind) { profile: ?Profile, git: ?Git, version };

const help_header =
    \\Master Maker's CLI
;

const verbose_flag_help =
    \\Flags:
    \\
    \\    -v, --verbose    Enable verbose logging
;

fn makeFinalCommandHelpStringNoVerbose(message: []const u8) []const u8 {
    return help_header ++ "\n\n" ++ message ++ "\n";
}

fn makeFinalCommandHelpString(message: []const u8) []const u8 {
    return comptime makeFinalCommandHelpStringNoVerbose(message) ++ verbose_flag_help ++ "\n";
}

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

fn makeHelpString(comptime T: type, message: []const u8, comptime getFieldHelp: (fn (field: []const u8) []const u8)) []const u8 {
    var result: []const u8 = help_header ++ "\n\n" ++ message ++ "\n";

    const field_names = std.meta.fieldNames(T);

    var char_count: u8 = 0;

    for (field_names) |field| {
        if (field.len > char_count) {
            char_count = field.len;
        }
    }

    for (field_names) |field| {
        const description = getFieldHelp(field);

        var space: []const u8 = "    ";
        var index = 0;

        while (index <= char_count - field.len) : (index += 1) {
            space = space ++ " ";
        }

        result = std.fmt.comptimePrint("{s}\n  {s}{s}{s}", .{ result, field, space, description });
    }

    return result ++ "\n";
}

pub fn get(command: ?Commands) []const u8 {
    if (command == null) {
        const message: []const u8 =
            \\Usage: mm [command] [options]
            \\Commands:
        ;

        return comptime makeHelpString(CommandKind, message, getCommandDescription);
    }

    return switch (command.?) {
        .git => |git| {
            if (git == null) {
                const message: []const u8 =
                    \\Commands related to the git integration
                    \\
                    \\Usage: mm git [command] [options]
                    \\Commands:
                ;

                return comptime makeHelpString(Git, message, getGitDescription);
            }

            return switch (git.?) {
                .init => {
                    const message: []const u8 =
                        \\Initializes a new repository with the current profile's git credentials
                        \\
                        \\Usage: mm git init <remote_url>
                    ;

                    return comptime makeFinalCommandHelpString(message);
                },
                .restart => {
                    const message: []const u8 =
                        \\Restarts the target branch from the base branch
                        \\
                        \\Usage: mm git restart -o <origin_branch> -d <destination_branch>
                    ;

                    return comptime makeFinalCommandHelpString(message);
                },
                .submit => {
                    const message: []const u8 =
                        \\Commits & submits the code changes to the remote
                        \\
                        \\Usage: mm git submit
                    ;

                    return comptime makeFinalCommandHelpString(message);
                },
            };
        },
        .profile => |profile| {
            if (profile == null) {
                const message: []const u8 =
                    \\Usage: mm profile [command] [options]
                    \\Commands:
                ;

                return comptime makeHelpString(Profile, message, getProfileDescription);
            }

            return switch (profile.?) {
                .add => {
                    const message: []const u8 =
                        \\Adds a new profile
                        \\
                        \\Usage: mm profile add <profile_name>
                    ;

                    return comptime makeFinalCommandHelpString(message);
                },
                .set => {
                    const message: []const u8 =
                        \\Sets the input profile as the current one
                        \\
                        \\Usage: mm profile set <profile_name>
                    ;

                    return comptime makeFinalCommandHelpString(message);
                },
            };
        },
        .version => {
            const message: []const u8 =
                \\Prints the current version and exits
                \\
                \\Usage: mm version
            ;

            return comptime makeFinalCommandHelpStringNoVerbose(message);
        },
    };
}
