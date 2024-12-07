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

    while (lines.next()) |line| {
        var str_ints = std.mem.tokenizeScalar(u8, line, ' ');

        const fst = str_ints.next() orelse unreachable;
        const snd = str_ints.next() orelse unreachable;

        const fst_int = try std.fmt.parseInt(u32, fst, 10);
        const snd_int = try std.fmt.parseInt(u32, snd, 10);

        const fst_idx = std.sort.upperBound(
            u32,
            fst_int,
            fst_arr.items,
            {},
            comptime std.sort.asc(u32),
        );
        try fst_arr.insert(fst_idx, fst_int);

        const snd_idx = std.sort.upperBound(
            u32,
            snd_int,
            snd_arr.items,
            {},
            comptime std.sort.asc(u32),
        );
        try snd_arr.insert(snd_idx, snd_int);
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
