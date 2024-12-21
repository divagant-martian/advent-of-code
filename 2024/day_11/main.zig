const std = @import("std");

pub const std_options = .{
    .log_level = .info,
};

const Num = u64;
const List = std.DoublyLinkedList(Num);

const Stones = struct {
    list: std.AutoArrayHashMap(Num, usize),

    const Self = @This();

    fn new(data: []const u8, allocator: std.mem.Allocator) !Self {
        var list = std.AutoArrayHashMap(Num, usize).init(allocator);

        var tokens = std.mem.tokenizeAny(u8, data, " \n");

        while (tokens.next()) |token| {
            const stone = try std.fmt.parseInt(Num, token, 10);
            const result = try list.getOrPutValue(stone, 0);
            result.value_ptr.* += 1;
        }

        return Stones{ .list = list };
    }

    fn blink(self: *Self) !void {
        var new_list = std.AutoArrayHashMap(Num, usize).init(self.list.allocator);
        defer new_list.deinit();

        var iter = self.list.iterator();

        while (iter.next()) |entry| {
            const stone = entry.key_ptr.*;
            const appearances = entry.value_ptr.*;

            switch (try blink_stone(stone)) {
                .one => |new_stone| {
                    const result = try new_list.getOrPutValue(new_stone, 0);
                    result.value_ptr.* += appearances;
                },
                .two => |new_stones| {
                    for (new_stones) |new_stone| {
                        const result = try new_list.getOrPutValue(new_stone, 0);
                        result.value_ptr.* += appearances;
                    }
                },
            }
        }

        std.mem.swap(std.AutoArrayHashMap(Num, usize), &new_list, &self.list);
    }

    pub fn format(
        self: *const Self,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        var maybe_node = self.list.first;

        try writer.writeAll("[ ");

        while (maybe_node) |node| {
            try writer.print("{d} ", .{node.data});
            maybe_node = node.next;
        }

        try writer.writeAll("]");
    }
};

const BlinkResult = union(enum) { one: Num, two: [2]Num };

fn blink_stone(stone: Num) !BlinkResult {
    if (stone == 0) {
        return BlinkResult{ .one = 1 };
    } else {
        const digits = std.math.log10_int(stone) + 1;
        if (digits % 2 == 0) {
            const tens = try std.math.powi(Num, 10, digits / 2);

            const right_stone = stone % tens;
            const left_stone = stone / tens;

            return BlinkResult{ .two = [2]Num{ left_stone, right_stone } };
        } else {
            return BlinkResult{ .one = stone * 2024 };
        }
    }
}

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

    var stones = try Stones.new(data, allocator);

    std.log.debug("start: {s}", .{stones});

    const blinks: u8 = switch (part) {
        .a => 25,
        .b => 75,
    };

    for (0..blinks) |i| {
        std.log.debug("blink = {d}", .{i});
        try stones.blink();
    }

    var total: usize = 0;
    for (stones.list.values()) |appearances| {
        total += appearances;
    }

    std.log.info("total: {d}", .{total});
}
