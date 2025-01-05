const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;
const numkey = @import("num_key");
const dirkey = @import("dir_key");
const Permutator = @import("permutator").Permutator;
const Product = @import("product").Product;

pub const std_options = .{
    .log_level = .debug,
};

const CPad = struct {
    n: numkey.NumKey = .a,
    a: dirkey.DirKey = .enter,
    b: dirkey.DirKey = .enter,

    fn execute(self: *CPad, dir_str: *const dirkey.DirStr) !numkey.NumStr {
        var num_str = std.ArrayList(numkey.NumKey).init(dir_str.allocator);
        errdefer num_str.deinit();

        for (dir_str.dir_keys) |key| {
            std.debug.print("{}", .{key});
            const pressed_num = try self.press_once(key) orelse continue;
            try num_str.append(pressed_num);
            break;
        }

        return numkey.NumStr.from_arraylist(num_str);
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

const FlatMap = struct {
    perms: Permutator,
    curr_prod: Product(Permutator),

    fn new(dir_str: dirkey.DirStr) !FlatMap {
        var perms = try Permutator.new(dir_str);
        const curr_perm = (try perms.next()).?;
        const curr_prod = try new_b_level(curr_perm);
        return .{ .perms = perms, .curr_prod = curr_prod };
    }

    fn next(self: *FlatMap) !?dirkey.DirStr {
        var maybe_yield = try self.curr_prod.next();
        while (maybe_yield == null) {
            const perm = try self.perms.next() orelse return null;
            self.curr_prod = try new_b_level(perm);
            maybe_yield = try self.curr_prod.next();
        }
        return maybe_yield;
    }

    fn reset(self: *FlatMap) !void {
        try self.perms.reset();
        const curr_perm = (try self.perms.next()).?;
        self.curr_prod = try new_b_level(curr_perm);
    }
};

fn new_b_level(b_str: dirkey.DirStr) !Product(Permutator) {
    const allocator = b_str.allocator;

    var generators = std.ArrayList(Permutator).init(allocator);
    errdefer generators.deinit();

    // windows
    var pairs = std.mem.window(dirkey.DirKey, b_str.dir_keys, 2, 1);
    while (pairs.next()) |b_pair| {
        // translate
        const c_trad = try dirkey.DirStr.from_dir_keys(b_pair[0], b_pair[1], allocator);
        // permutate
        const c_perms = try Permutator.new(c_trad);
        try generators.append(c_perms);
    }
    return Product(Permutator).new_from_generators(generators);
}

const Generator = struct {
    from: numkey.NumKey = .a,
    to: numkey.NumKey = .a,
    c_parts_generators: std.ArrayList(Permutator),
    advancing: usize = 0,

    fn new(from: numkey.NumKey, to: numkey.NumKey, allocator: std.mem.Allocator) !Generator {
        // translate 1
        const a_trad = try dirkey.DirStr.from_num_keys(from, to, allocator);
        var a_perms = try Permutator.new(a_trad);
        // permutate 1
        while (try a_perms.next()) |a_perm| {
            var a_pairs = std.mem.window(dirkey.DirKey, a_perm.dir_keys, 2, 1);
            // windows 1
            while (a_pairs.next()) |a_pair| {
                // translate 2
                const b_trad = try dirkey.DirStr.from_dir_keys(a_pair[0], a_pair[1], allocator);
                var b_thing = try FlatMap.new(b_trad);
                while (try b_thing.next()) |b_thingy| {
                    std.debug.print("{}\n", .{b_thingy});
                }
                // break;
            }
            std.debug.print("---\n", .{});
            // break;
        }

        return Generator{ .from = from, .to = to, .c_parts_generators = std.ArrayList(Permutator).init(allocator) };
    }

    // fn next(self: *Generator) ?dirkey.DirStr {
    // var joined = std.ArrayList(dirkey.DirKey).init(self.c_parts_generators.allocator);
    // for (self.c_parts_generators.items) |gen| {
    //
    // }
    // }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    _ = try Generator.new(.a, .n0, allocator);
    var c_pad = CPad{};
    const dirstr = try dirkey.DirStr.parse("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A", allocator);
    const numstr = try c_pad.execute(&dirstr);
    std.debug.print("\n got: {}", .{numstr});
}
