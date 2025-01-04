const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;
const numkey = @import("num_key");
const dirkey = @import("dir_key");

pub const std_options = .{
    .log_level = .debug,
};

const Permutator = struct {
    visited: dirkey.DirStr,
    still_going: ?void = {},

    pub fn next(self: *Permutator) !?dirkey.DirStr {
        _ = self.still_going orelse return null;

        var a: []dirkey.DirKey = self.visited.dir_keys;

        const yield = try self.visited.with_enter();
        var j = a.len - 2;

        while (a[j].ge(a[j + 1])) : (j -= 1) {
            if (j == 0) {
                self.still_going = null;
                return yield;
            }
        }

        var l = a.len - 1;
        while (a[j].ge(a[l])) : (l -= 1) {}
        std.mem.swap(dirkey.DirKey, &a[j], &a[l]);

        var k = j + 1;
        l = a.len - 1;
        while (k < l) {
            std.mem.swap(dirkey.DirKey, &a[k], &a[l]);
            k += 1;
            l -= 1;
        }

        return yield;
    }
};

const CPad = struct {
    n: numkey.NumKey = .a,
    a: dirkey.DirKey = .enter,
    b: dirkey.DirKey = .enter,

    fn execute(self: *CPad, dir_str: *const dirkey.DirStr) !numkey.NumStr {
        var num_str = std.ArrayList(numkey.NumKey).init(dir_str.allocator);
        errdefer num_str.deinit();

        for (dir_str.dir_keys) |key| {
            const pressed_num = try self.press_once(key) orelse continue;
            try num_str.append(pressed_num);
        }

        return numkey.NumStr.from_arraylist(&num_str);
    }

    fn press_once(self: *CPad, key: dirkey.DirKey) !?numkey.NumKey {
        // a c key can be enter or a move:
        // - if the c key is a move, simply move b
        // - if the c key is enter, the current b is pressed (which changes a)
        if (key.into_move()) |dir| {
            self.b = self.b.move_once(dir) orelse return error.movedToInvalidBPos;
        } else {
            // b is a press, which makes it a move in a
            if (self.b.into_move()) |dir| {
                self.a = self.a.move_once(dir) orelse return error.movedToInvalidAPos;
            } else {
                // pressing b is a press in a
                if (self.a.into_move()) |dir| {
                    self.n = self.n.move_once(dir) orelse return error.movedToInvalidNPos;
                } else {
                    return self.n;
                }
            }
        }

        return null;
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    // var args_iter = try std.process.argsWithAllocator(allocator);
    // defer args_iter.deinit();

    // const args = try Args.from_args_iter(&args_iter);

    // var file = try std.fs.cwd().openFile(args.file_name, .{});
    // defer file.close();

    // var buf_reader = std.io.bufferedReader(file.reader());
    // const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    // var lines = std.mem.tokenizeScalar(u8, data, '\n');

    // while (lines.next()) |line| {
    // var numstring = try numkey.NumStr.parse(line, allocator);
    //
    //     std.debug.print("numstring: {}", .{numstring});
    //
    //     var dirstring0 = try numstring_to_dirstring(numstring.items, allocator);
    //     try dirstring0.insert(0, .enter);
    //
    //     std.debug.print("dirstring0: ", .{});
    //     print_dirstring(dirstring0.items);
    //
    //     var dirstring1 = try dirstring_to_dirstring(dirstring0.items, allocator);
    //     try dirstring1.insert(0, .enter);
    //
    //     std.debug.print("dirstring1: ", .{});
    //     print_dirstring(dirstring1.items);
    //
    //     const dirstring2 = try dirstring_to_dirstring(dirstring1.items, allocator);
    //
    //     std.debug.print("dirstring2: ", .{});
    //     print_dirstring(dirstring2.items);
    //
    //     std.debug.print("final len: {d}\n\n", .{dirstring2.items.len});
    // }
    // const start = try dirkey.DirStr.parse("^vv>", allocator);
    // var permutator = Permutator{ .visited = start };
    // while (try permutator.next()) |shuffled| {
    // std.debug.print("{}\n", .{shuffled});
    // }
    //
    const c_dir_str = try dirkey.DirStr.parse("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A", allocator);

    var c_pad = CPad{};
    const num_str = try c_pad.execute(&c_dir_str);
    std.debug.print("{}\n", .{num_str});

    std.debug.print("final state N:{}, A:{}, B:{}\n", .{ c_pad.n, c_pad.a, c_pad.b });
}
