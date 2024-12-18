const std = @import("std");

// pub const std_options = .{
// .log_level = .debug,
// };

const Position = struct {
    i: i16,
    j: i16,

    fn sub(self: *const Position, other: *const Position) Position {
        const i = self.i - other.i;
        const j = self.j - other.j;
        return Position{ .i = i, .j = j };
    }

    fn add(self: *const Position, other: *const Position) Position {
        const i = self.i + other.i;
        const j = self.j + other.j;
        return Position{ .i = i, .j = j };
    }

    fn less_than(self: *const Position, other: *const Position) bool {
        return self.i < other.i and self.j < other.j;
    }
};

const Antenna = union(enum) {
    lowercase: u8,
    uppercase: u8,
    digit: u8,
};

const Map = struct {
    antennas: std.AutoHashMap(Antenna, std.AutoArrayHashMap(Position, void)),
    cols: i16,
    lines: i16,

    const newErr = error{ invalidChar, notRectangular };

    fn new(data: []const u8, allocator: std.mem.Allocator) !Map {
        var lines = std.mem.tokenizeScalar(u8, data, '\n');

        var antennas = std.AutoHashMap(Antenna, std.AutoArrayHashMap(Position, void)).init(allocator);

        var line_number: i16 = 0;
        var cols: i16 = 0;
        while (lines.next()) |line| : (line_number += 1) {
            var col_number: i16 = 0;
            for (line) |char| {
                const antenna = switch (char) {
                    '.' => {
                        col_number += 1;
                        continue;
                    },
                    'a'...'z' => Antenna{ .lowercase = char },
                    'A'...'Z' => Antenna{ .uppercase = char },
                    '0'...'9' => Antenna{ .digit = char },
                    else => return Map.newErr.invalidChar,
                };

                const pos = Position{ .i = line_number, .j = col_number };
                var entry = try antennas.getOrPut(antenna);
                if (!entry.found_existing) {
                    entry.value_ptr.* = std.AutoArrayHashMap(Position, void).init(allocator);
                }
                try entry.value_ptr.put(pos, {});
                col_number += 1;
            }
            if (cols == 0) {
                cols = col_number;
            } else if (cols != line.len) {
                return Map.newErr.notRectangular;
            }
        }
        return Map{ .antennas = antennas, .lines = line_number, .cols = cols };
    }

    fn get_antinodes(self: *const Map) !std.AutoArrayHashMap(Position, void) {
        var antinodes = std.AutoArrayHashMap(Position, void).init(self.antennas.allocator);
        var iter = self.antennas.iterator();
        const kinda_zero = Position{ .i = -1, .j = -1 };
        const max = Position{ .i = self.lines, .j = self.cols };

        while (iter.next()) |entry| {
            const positions = entry.value_ptr.keys();

            for (positions, 1..) |first, i| {
                if (i == positions.len) {
                    break;
                }
                for (positions[i..]) |second| {
                    const direction = second.sub(&first);
                    const fst_antinode = first.sub(&direction);
                    const snd_antinode = second.add(&direction);
                    if (fst_antinode.less_than(&max) and kinda_zero.less_than(&fst_antinode)) {
                        try antinodes.put(fst_antinode, {});
                    }
                    if (snd_antinode.less_than(&max) and kinda_zero.less_than(&snd_antinode)) {
                        try antinodes.put(snd_antinode, {});
                    }
                }
            }
        }

        return antinodes;
    }
};

const Part = enum {
    a,
    b,

    const Err = error{ missingProgramPart, unkownPart };

    fn from_args(args: *std.process.ArgIterator) Err!Part {
        if (args.next()) |part| {
            if (std.mem.eql(u8, part, "a")) {
                return Part.a;
            } else if (std.mem.eql(u8, part, "b")) {
                return Part.b;
            } else {
                return Part.Err.unkownPart;
            }
        } else {
            return Part.Err.missingProgramPart;
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    // process program name
    _ = args.next().?;

    const part = try Part.from_args(&args);

    var file_name: []const u8 = undefined;
    if (args.next()) |name| {
        file_name = name;
    } else {
        std.log.err("missing file name", .{});
        std.process.exit(1);
    }

    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    const map = try Map.new(data, allocator);

    switch (part) {
        .a => {
            const antinodes = try map.get_antinodes();
            std.log.info("antinode count: {d}", .{antinodes.count()});
        },
        .b => {
            std.log.err("unimplemented", .{});
            std.process.exit(1);
        },
    }
}
