const std = @import("std");
const types = @import("../types.zig");
const git = @import("../git/types.zig");
const profile = @import("../profile/types.zig");
const common = @import("../common/types.zig");

const ParsingError = error{ UnknownCommand, MalformedInputs };

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

pub fn parse(args: [][:0]u8) !types.Input {
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

    const command = blk: switch (command_enum) {
        .git => {
            const command_kind = std.meta.stringToEnum(git.GitCommandKind, args[2]) orelse {
                return ParsingError.UnknownCommand;
            };

            const git_command: git.GitCommand = inner_blk: switch (command_kind) {
                .restart => {
                    const pair = get_origin_destination_pair(args) orelse {
                        return ParsingError.MalformedInputs;
                    };

                    break :inner_blk git.GitCommand{ .restart = pair };
                },
                .init => {
                    if (args.len < 4) {
                        return ParsingError.MalformedInputs;
                    }

                    const remote = args[3];

                    break :inner_blk git.GitCommand{ .init = .{ .remote = remote } };
                },
            };

            break :blk types.Command{ .git = git_command };
        },
        .profile => {
            const command_kind = std.meta.stringToEnum(profile.ProfileCommandKind, args[2]) orelse {
                return ParsingError.UnknownCommand;
            };

            const profile_command: profile.ProfileCommand = inner_blk: switch (command_kind) {
                .add => {
                    if (args.len < 4) {
                        return ParsingError.MalformedInputs;
                    }

                    const name = args[3];

                    break :inner_blk profile.ProfileCommand{ .add = .{ .name = name } };
                },
                .set => {
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
            break :blk types.Command{ .version = undefined };
        },
    };

    return .{ .command = command, .verbose = verbose };
}
