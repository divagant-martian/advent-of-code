const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    // process program name
    _ = args.next().?;

    var second_part = false;
    if (args.next()) |part| {
        if (std.mem.eql(u8, part, "a")) {
            second_part = false;
        } else if (std.mem.eql(u8, part, "b")) {
            second_part = true;
        } else {
            std.log.err("parts a or b, you on drugs: {s}", .{part});
            std.process.exit(1);
        }
    } else {
        std.log.err("missing program part", .{});
        std.process.exit(1);
    }

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
    var lines = std.mem.splitScalar(u8, data, '\n');

    const rules = try Rules.new(&lines, allocator);
    const updates = try parse_updates(&lines, allocator);

    if (second_part) {
        var total: usize = 0;

        for (updates.items) |*update| {
            if (!rules.is_sorted(update.items)) {
                std.log.debug("{d} will be sorted", .{update.items});
                const sorted_update = rules.sort(update.items) catch unreachable;

                if (!rules.is_sorted(sorted_update.items)) {
                    std.log.err("{d} is not sorted", .{sorted_update.items});
                    @panic("BUG");
                }

                const mid_idx = sorted_update.items.len / 2;
                const mid_page = sorted_update.items[mid_idx];
                std.log.info("{d} is now sorted, mid page is {d}", .{ sorted_update.items, mid_page });
                total += mid_page;
            }
        }

        std.log.info("part 2: {d}", .{total});
    } else {
        var total: usize = 0;

        for (updates.items) |update| {
            if (rules.is_sorted(update.items)) {
                const mid_idx = update.items.len / 2;
                const mid_page = update.items[mid_idx];
                std.log.info("{d} is sorted, mid page is {d}", .{ update.items, mid_page });
                total += mid_page;
            } else {
                std.log.warn("{d} ain't sorted", .{update.items});
            }
        }

        std.log.info("part 1: {d}", .{total});
    }
}

const Rules = struct {
    inner: std.AutoArrayHashMap(u8, std.ArrayList(u8)),

    fn new(lines: *std.mem.SplitIterator(u8, .scalar), allocator: std.mem.Allocator) !Rules {
        var inner = std.AutoArrayHashMap(u8, std.ArrayList(u8)).init(allocator);

        while (lines.next()) |line| {
            if (line.len == 0) {
                break;
            }

            var tokens = std.mem.tokenizeScalar(u8, line, '|');

            const lhs = tokens.next() orelse return InvalidRules.MissingNum;
            const rhs = tokens.next() orelse return InvalidRules.MissingNum;

            if (tokens.next()) |_| {
                return InvalidRules.TrailingChars;
            }

            const lhs_num = try std.fmt.parseInt(u8, lhs, 10);
            const rhs_num = try std.fmt.parseInt(u8, rhs, 10);

            const result = try inner.getOrPut(lhs_num);

            if (!result.found_existing) {
                result.value_ptr.* = std.ArrayList(u8).init(allocator);
            }

            try result.value_ptr.append(rhs_num);
        }

        return Rules{ .inner = inner };
    }

    fn is_sorted(self: *const Rules, update: []const u8) bool {
        for (update, 0..) |page, i| {
            const page_rules = self.inner.get(page) orelse continue;

            for (0..i) |prev_i| {
                const prev_page = update[prev_i];

                if (std.mem.containsAtLeast(u8, page_rules.items, 1, &[_]u8{prev_page})) {
                    return false;
                }
            }
        }

        return true;
    }

    fn sort(self: *const Rules, update: []const u8) !std.ArrayList(u8) {
        var sorted_update = try std.ArrayList(u8).initCapacity(self.inner.allocator, update.len);
        var counts = std.AutoArrayHashMap(u8, usize).init(self.inner.allocator);
        var sources = std.AutoArrayHashMap(u8, void).init(self.inner.allocator);

        var rules_iter = self.inner.iterator();
        while (rules_iter.next()) |entry| {
            const lhs = entry.key_ptr.*;

            if (contains(u8, update, lhs)) {
                for (entry.value_ptr.items) |rhs| {
                    if (contains(u8, update, rhs)) {
                        (try counts.getOrPutValue(rhs, 0)).value_ptr.* += 1;
                        _ = sources.swapRemove(rhs);
                    }
                }
                if (!counts.contains(lhs)) {
                    try sources.put(lhs, {});
                }
            }
        }

        var queue = sources;

        std.log.debug("queue: {d}", .{queue.keys()});

        while (queue.popOrNull()) |node| {
            try sorted_update.append(node.key);

            const neighbors = self.inner.get(node.key) orelse continue;
            for (neighbors.items) |neighbor| {
                if (contains(u8, update, neighbor)) {
                    const count = counts.getPtr(neighbor).?;
                    count.* -= 1;

                    if (count.* == 0) {
                        try queue.put(neighbor, {});
                        _ = counts.swapRemove(neighbor);
                    }
                }
            }
        }

        if (counts.keys().len != 0) {
            @panic("cycle detected, you bitch!");
        }

        return sorted_update;
    }
};

const InvalidRules = error{
    MissingNum,
    TrailingChars,
};

fn parse_updates(lines: *std.mem.SplitIterator(u8, .scalar), allocator: std.mem.Allocator) !std.ArrayList(std.ArrayList(u8)) {
    var updates = std.ArrayList(std.ArrayList(u8)).init(allocator);

    while (lines.next()) |line| {
        if (line.len == 0) {
            break;
        }

        var tokens = std.mem.tokenizeScalar(u8, line, ',');
        var update = std.ArrayList(u8).init(allocator);

        while (tokens.next()) |token| {
            const page = try std.fmt.parseInt(u8, token, 10);
            try update.append(page);
        }

        try updates.append(update);
    }

    return updates;
}

fn contains(comptime T: type, haystack: []const T, needle: T) bool {
    return std.mem.containsAtLeast(T, haystack, 1, &[_]T{needle});
}
