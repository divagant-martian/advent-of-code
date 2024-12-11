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

    line_processing: while (lines.next()) |line| {
        const report = try parse_report(line, allocator);
        const len = report.items.len + 1;
        for (0..len) |skip| {
            switch (report_safety(report.items, skip)) {
                .safe => {
                    count += 1;
                    continue :line_processing;
                },
                .unsafe => {},
            }
        }
    }

    std.debug.print("{d}\n", .{count});
}

fn parse_report(line: []const u8, allocator: std.mem.Allocator) !std.ArrayList(isize) {
    var str_levels = std.mem.tokenizeScalar(u8, line, ' ');
    var levels = std.ArrayList(isize).init(allocator);
    while (str_levels.next()) |str_level| {
        const level = try std.fmt.parseInt(isize, str_level, 10);
        try levels.append(level);
    }
    return levels;
}

fn report_safety(report: []const isize, skip: usize) Safety {
    var was_increasing: ?bool = null;
    var maybe_prev_level: ?isize = null;
    for (report, 0..) |level, idx| {
        // check if this level counts
        if (idx == skip) {
            continue;
        }

        // check if there is a previous level to compare to. If there isn't,
        // this is the first level
        if (maybe_prev_level) |prev_level| {
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
        }
        maybe_prev_level = level;
    }

    return Safety.safe;
}
