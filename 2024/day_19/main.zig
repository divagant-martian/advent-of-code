const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const Str: type = []const u8;

const Language = struct {
    alphabet: std.ArrayList(Str),

    fn accept_info(self: *const Language, str: Str) !std.AutoHashMap(usize, std.StringHashMap(void)) {
        const max_len = str.len;

        var visited_distances = std.AutoHashMap(usize, std.StringHashMap(void)).init(self.alphabet.allocator);

        // keeps track of whats the partially accepted index of the string.
        var expansions = std.ArrayList(usize).init(self.alphabet.allocator);
        try expansions.append(0);
        defer expansions.deinit();

        while (expansions.popOrNull()) |idx| {
            if (idx == max_len) {
                continue;
            }
            for (self.alphabet.items) |letter| {
                if (std.mem.startsWith(u8, str[idx..], letter)) {
                    const advanced_dist = idx + letter.len;
                    var entry = try visited_distances.getOrPut(advanced_dist);
                    if (entry.found_existing) {} else {
                        entry.value_ptr.* = std.StringHashMap(void).init(self.alphabet.allocator);
                        try expansions.append(advanced_dist);
                    }
                    try entry.value_ptr.put(letter, {});
                }
            }
        }

        return visited_distances;
    }

    fn accepts(self: *const Language, str: Str) !bool {
        const distances = try self.accept_info(str);
        return distances.contains(str.len);
    }

    fn count_accepts(self: *const Language, str: Str) !usize {
        var distances = try self.accept_info(str);
        if (!distances.contains(str.len)) {
            return 0;
        }
        var acc_counts = try self.alphabet.allocator.alloc(usize, str.len + 1);
        defer self.alphabet.allocator.free(acc_counts);
        acc_counts[0] = 1;
        defer distances.deinit();

        for (1..str.len + 1) |dist| {
            if (distances.get(dist)) |letter_set| {
                var letters = letter_set.keyIterator();
                var sum: usize = 0;
                while (letters.next()) |letter| {
                    const query = dist - letter.len;
                    sum += acc_counts[query];
                }
                acc_counts[dist] = sum;
            }
        }

        return acc_counts[str.len];
    }
};

const _ = error{ missingAlphabet, missingStrings };

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var args_iter = try std.process.argsWithAllocator(allocator);
    defer args_iter.deinit();

    const args = try Args.from_args_iter(&args_iter);

    var file = try std.fs.cwd().openFile(args.file_name, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    var sections = std.mem.tokenizeScalar(u8, data, '\n');

    const alphabet_section = sections.next() orelse return error.missingAlphabet;
    var letter_iter = std.mem.tokenizeSequence(u8, alphabet_section, ", ");
    var alphabet = std.ArrayList(Str).init(allocator);
    while (letter_iter.next()) |letter| {
        try alphabet.append(letter);
    }

    var strings = std.ArrayList(Str).init(allocator);
    while (sections.next()) |str| {
        try strings.append(str);
    }

    const lang = Language{ .alphabet = alphabet };
    // std.log.debug("alphabet is {s}", .{lang.alphabet.items});

    switch (args.part) {
        .a => {
            var accepts: usize = 0;
            for (strings.items) |str| {
                if (try lang.accepts(str)) {
                    std.log.debug("language accepts {s}", .{str});
                    accepts += 1;
                } else {
                    std.log.debug("language does NOT accept {s}", .{str});
                }
            }

            std.log.info("accepted strings: {d}", .{accepts});
        },
        .b => {
            var count: usize = 0;
            for (strings.items) |str| {
                count += try lang.count_accepts(str);
                std.debug.print("count {d}\n", .{count});
            }
            std.log.info("all options: {d}", .{count});
        },
    }
}
