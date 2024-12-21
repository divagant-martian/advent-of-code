const std = @import("std");

pub const Args = struct {
    part: Part,
    file_name: []const u8,

    pub const Part = enum { a, b };
    pub const Err = error{ missingProgramPart, unkownPart, missingFile };

    pub fn from_args_iter(args: *std.process.ArgIterator) Err!Args {

        // process program name
        _ = args.next().?;
        if (args.next()) |part_arg| {
            const part: Args.Part = switch (std.mem.eql(u8, part_arg, "a")) {
                true => .a,
                false => switch (std.mem.eql(u8, part_arg, "b")) {
                    true => .b,
                    false => return Args.Err.unkownPart,
                },
            };

            if (args.next()) |name| {
                return Args{ .part = part, .file_name = name };
            } else {
                return Args.Err.missingFile;
            }
        } else {
            return Args.Err.missingProgramPart;
        }
    }
};
