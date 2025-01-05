const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;
const numkey = @import("num_key");
const dirkey = @import("dir_key");

pub const std_options = .{
    .log_level = .debug,
};

const Permutator = struct {
    og: dirkey.DirStr,
    visited: dirkey.DirStr,
    still_going: ?void = {},

    fn new(dis_str: dirkey.DirStr) !Permutator {
        return Permutator{
            .og = try dis_str.clone(),
            .visited = dis_str,
        };
    }

    fn reset(self: *Permutator) !void {
        std.mem.copyForwards(dirkey.DirKey, self.visited.dir_keys, self.og.dir_keys);
        self.still_going = {};
    }

    pub fn next(self: *Permutator) !?dirkey.DirStr {
        _ = self.still_going orelse return null;

        var a: []dirkey.DirKey = self.visited.dir_keys;

        const yield = try self.visited.with_enter();

        if (a.len < 2) {
            self.still_going = null;
            return yield;
        }

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

fn PermProduct(comptime GenT: type) type {
    return struct {
        generators: std.ArrayList(GenT),
        currents: std.ArrayList(dirkey.DirStr),
        advancing_permutator: usize,
        still_going: ?void = {},

        fn new_b_level(b_str: dirkey.DirStr) !PermProduct(Permutator) {
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
            return PermProduct(Permutator).new_from_generators(generators);
        }

        fn new_from_generators(generators: std.ArrayList(GenT)) !@This() {
            var currents = try std.ArrayList(dirkey.DirStr).initCapacity(generators.allocator, generators.items.len);
            for (generators.items) |*gen| {
                const perm = (try gen.next()).?;
                currents.appendAssumeCapacity(perm);
            }
            const advancing_permutator = generators.items.len - 1;
            return .{ .generators = generators, .currents = currents, .advancing_permutator = advancing_permutator };
        }

        fn reset(self: *@This()) !void {
            self.currents.clearRetainingCapacity();
            for (self.generators.items) |*gen| {
                try gen.reset();
                const perm = (try gen.next()).?;
                self.currents.appendAssumeCapacity(perm);
            }
            self.advancing_permutator = self.generators.items.len - 1;
            self.still_going = {};
        }

        fn join_currents(self: *const @This()) !dirkey.DirStr {
            var joined = std.ArrayList(dirkey.DirKey).init(self.generators.allocator);
            errdefer joined.deinit();
            try joined.append(.enter);
            for (self.currents.items) |dir_str| {
                try joined.appendSlice(dir_str.dir_keys[1..]);
            }

            return dirkey.DirStr.from_arraylist(joined);
        }

        fn next(self: *@This()) !?dirkey.DirStr {
            _ = self.still_going orelse return null;
            const currents = try self.join_currents();
            const maybe_new = try self.generators.items[self.advancing_permutator].next();

            if (maybe_new) |new_perm| {
                self.currents.items[self.advancing_permutator] = new_perm;
            } else reset: {
                if (self.advancing_permutator == 0) {
                    self.still_going = null;
                } else {
                    for (self.advancing_permutator..self.generators.items.len) |j| {
                        try self.generators.items[j].reset();
                    }
                    self.advancing_permutator -= 1;
                    for (self.advancing_permutator..self.generators.items.len) |i| {
                        self.currents.items[i] = (try self.generators.items[i].next()) orelse break :reset;
                    }
                }
            }

            return currents;
        }
    };
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
                var b_perms = try Permutator.new(b_trad);
                // permutate 2
                while (try b_perms.next()) |b_perm| {
                    var perm_prod = try PermProduct(Permutator).new_b_level(b_perm);
                    while (try perm_prod.next()) |combined| {
                        std.debug.print("perprod: {}\n", .{combined});
                    }
                    // break;
                    // std.debug.print("\n", .{});
                    // std.debug.print("\n", .{});
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
