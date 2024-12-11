const std = @import("std");

const Safety = enum {
    safe,
    unsafe,
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    var lines = std.mem.tokenizeScalar(u8, data, '\n');
    var count: usize = 0;

    while (lines.next()) |line| {
        switch (try read_report(line)) {
            .safe => count += 1,
            .unsafe => {},
        }
    }

    std.debug.print("{d}\n", .{count});
}

fn read_report(line: []const u8) !Safety {
    var str_levels = std.mem.tokenizeScalar(u8, line, ' ');
    var prev_level = try std.fmt.parseInt(isize, str_levels.next().?, 10);
    var was_increasing: ?bool = null;

    while (str_levels.next()) |str_level| {
        const level = try std.fmt.parseInt(isize, str_level, 10);
        const diff = level - prev_level;

        const is_increasing = switch (diff) {
            1...3 => true,
            -3...-1 => false,
            else => {
                return Safety.unsafe;
            },
        };

        if (was_increasing) |actually_was_increasing| {
            if (actually_was_increasing != is_increasing) {
                return Safety.unsafe;
            }
        }

        was_increasing = is_increasing;
        prev_level = level;
    }

    return Safety.safe;
}
