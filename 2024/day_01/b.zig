const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    var lines = std.mem.tokenizeScalar(u8, data, '\n');

    // numbers on the right with how often they appear
    var counts = std.AutoArrayHashMap(usize, usize).init(allocator);
    defer counts.deinit();

    // set of numbers on the left
    var members = std.AutoArrayHashMap(usize, void).init(allocator);
    defer members.deinit();

    while (lines.next()) |line| {
        var str_ints = std.mem.tokenizeScalar(u8, line, ' ');
        try count_item(&str_ints, &counts, &members);
    }

    var total: usize = 0;
    var counts_iter = counts.iterator();

    while (counts_iter.next()) |entry| {
        if (members.contains(entry.key_ptr.*)) {
            total += entry.key_ptr.* * entry.value_ptr.*;
        }
    }

    std.debug.print("{d}\n", .{total});
}

fn parseInt(comptime T: type, items: *std.mem.TokenIterator(u8, std.mem.DelimiterType.scalar)) !T {
    const item = items.next() orelse unreachable;
    return std.fmt.parseInt(T, item, 10);
}

fn count_item(items: *std.mem.TokenIterator(u8, std.mem.DelimiterType.scalar), counts: *std.AutoArrayHashMap(usize, usize), members: *std.AutoArrayHashMap(usize, void)) !void {
    const left = try parseInt(usize, items);
    const right = try parseInt(usize, items);

    try members.put(left, {});
    if (counts.getEntry(right)) |entry| {
        entry.value_ptr.* += 1;
    } else {
        try counts.put(right, 1);
    }
}
