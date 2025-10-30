const std = @import("std");
const Config = @import("../config/types.zig").WithHandle;
const Cache = @import("../cache.zig").WithHandle;
const CommandRunner = @import("../common/run_command.zig").CommandInterface;

pub const Environment = struct { allocator: std.mem.Allocator, config: Config, cache: Cache, command_runner: *CommandRunner };
