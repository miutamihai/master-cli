const std = @import("std");
const types = @import("../types.zig");
const git = @import("../git/types.zig");
const profile = @import("../profile/types.zig");
const common = @import("../common/types.zig");
const help = @import("../help.zig");

const ParsingError = error{ UnknownCommand, MalformedInputs };

pub const ParsingResultKind = enum { input, help };

pub const ParsingResult = union(ParsingResultKind) { input: types.Input, help: []const u8 };

fn get_origin_destination_pair(args: [][:0]u8) ?common.OriginDestination {
    var origin_index: usize = 0;
    var destination_index: usize = 0;

    for (args[2..args.len], 2..args.len) |arg, index| {
        if (index < args.len - 1) {
            if (std.mem.eql(u8, arg, "-o")) {
                origin_index = index + 1;
            } else if (std.mem.eql(u8, arg, "-d")) {
                destination_index = index + 1;
            }
        }
    }

    if (origin_index == 0 or destination_index == 0) {
        return null;
    }

    return common.OriginDestination{ .destination = args[destination_index], .origin = args[origin_index] };
}

pub fn parse(args: [][:0]u8) !ParsingResult {
    if (args.len <= 1) {
        return .{ .help = help.get(null) };
    }

    const command_string = args[1];

    const valid_options = comptime &[_][]const u8{
        @tagName(types.CommandKind.git),
        @tagName(types.CommandKind.profile),
        @tagName(types.CommandKind.version),
    };

    var valid = false;

    for (valid_options) |option| {
        if (std.mem.eql(u8, option, command_string)) {
            valid = true;

            break;
        }
    }

    if (!valid) {
        return ParsingError.UnknownCommand;
    }

    const command_enum = std.meta.stringToEnum(types.CommandKind, command_string) orelse unreachable;

    const verbose = for (args[2..args.len]) |arg| {
        if (std.mem.eql(u8, arg, "-v") or std.mem.eql(u8, arg, "--verbose")) {
            break true;
        }
    } else false;

    const isHelp = for (args[2..args.len]) |arg| {
        if (std.mem.eql(u8, arg, "-h") or std.mem.eql(u8, arg, "--help")) {
            break true;
        }
    } else false;

    const command = blk: switch (command_enum) {
        .git => {
            if (args.len <= 2) {
                return .{ .help = help.get(.{ .git = null }) };
            }

            const command_kind = std.meta.stringToEnum(git.GitCommandKind, args[2]) orelse {
                return ParsingError.UnknownCommand;
            };

            const git_command: git.GitCommand = inner_blk: switch (command_kind) {
                .restart => {
                    if (isHelp) {
                        return .{ .help = help.get(.{ .git = git.GitCommandKind.restart }) };
                    }

                    const pair = get_origin_destination_pair(args) orelse {
                        return ParsingError.MalformedInputs;
                    };

                    break :inner_blk git.GitCommand{ .restart = pair };
                },
                .init => {
                    if (isHelp) {
                        return .{ .help = help.get(.{ .git = git.GitCommandKind.init }) };
                    }

                    if (args.len < 4) {
                        return ParsingError.MalformedInputs;
                    }

                    const remote = args[3];

                    break :inner_blk git.GitCommand{ .init = .{ .remote = remote } };
                },
                .submit => {
                    if (isHelp) {
                        return .{ .help = help.get(.{ .git = git.GitCommandKind.init }) };
                    }

                    break :inner_blk git.GitCommand{ .submit = undefined };
                },
            };

            break :blk types.Command{ .git = git_command };
        },
        .profile => {
            if (args.len <= 2) {
                return .{ .help = help.get(.{ .profile = null }) };
            }

            const command_kind = std.meta.stringToEnum(profile.ProfileCommandKind, args[2]) orelse {
                return ParsingError.UnknownCommand;
            };

            const profile_command: profile.ProfileCommand = inner_blk: switch (command_kind) {
                .add => {
                    if (isHelp) {
                        return .{ .help = help.get(.{ .profile = profile.ProfileCommandKind.add }) };
                    }

                    if (args.len < 4) {
                        return ParsingError.MalformedInputs;
                    }

                    const name = args[3];

                    break :inner_blk profile.ProfileCommand{ .add = .{ .name = name } };
                },
                .set => {
                    if (isHelp) {
                        return .{ .help = help.get(.{ .profile = profile.ProfileCommandKind.set }) };
                    }

                    if (args.len < 4) {
                        return ParsingError.MalformedInputs;
                    }

                    const name = args[3];

                    break :inner_blk profile.ProfileCommand{ .set = .{ .name = name } };
                },
            };

            break :blk types.Command{ .profile = profile_command };
        },
        .version => {
            if (isHelp) {
                return .{ .help = help.get(.{ .version = undefined }) };
            }

            break :blk types.Command{ .version = undefined };
        },
    };

    return .{ .input = .{ .command = command, .verbose = verbose } };
}
