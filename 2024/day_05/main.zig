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

    if (second_part) {
        @panic("we haven't done part 2");
    } else {
        const rules = try Rules.new(&lines, allocator);
        const updates = try parse_updates(&lines, allocator);

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

            const fst = tokens.next() orelse return InvalidRules.MissingNum;
            const snd = tokens.next() orelse return InvalidRules.MissingNum;

            if (tokens.next()) |_| {
                return InvalidRules.TrailingChars;
            }

            const fst_num = try std.fmt.parseInt(u8, fst, 10);
            const snd_num = try std.fmt.parseInt(u8, snd, 10);

            const result = try inner.getOrPut(fst_num);

            if (!result.found_existing) {
                result.value_ptr.* = std.ArrayList(u8).init(allocator);
            }

            try result.value_ptr.append(snd_num);
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
