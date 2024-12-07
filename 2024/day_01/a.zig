const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    var lines = std.mem.tokenizeScalar(u8, data, '\n');

    var fst_arr = std.ArrayList(u32).init(allocator);
    defer fst_arr.deinit();

    var snd_arr = std.ArrayList(u32).init(allocator);
    defer snd_arr.deinit();
    var x: usize = 1;

    while (lines.next()) |line| {
        var str_ints = std.mem.tokenizeScalar(u8, line, ' ');

        x += 1;
        try add_item(&str_ints, &fst_arr);
        try add_item(&str_ints, &snd_arr);
    }

    var total: u32 = 0;

    for (fst_arr.items, snd_arr.items) |a, b| {
        if (a > b) {
            total += a - b;
        } else {
            total += b - a;
        }
    }

    std.debug.print("{d}\n", .{total});
}

fn add_item(items: *std.mem.TokenIterator(u8, std.mem.DelimiterType.scalar), dst: *std.ArrayList(u32)) !void {
    const item = items.next() orelse unreachable;
    const number = try std.fmt.parseInt(u32, item, 10);
    const idx = std.sort.upperBound(
        u32,
        number,
        dst.items,
        {},
        comptime std.sort.asc(u32),
    );

    try dst.insert(idx, number);
}
