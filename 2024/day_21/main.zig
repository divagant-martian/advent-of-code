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

const Generator = struct {
    from: numkey.NumKey = .a,
    to: numkey.NumKey = .a,
    c_parts_generators: bool = false,

    fn dostuff(from: numkey.NumKey, to: numkey.NumKey, allocator: std.mem.Allocator) !void {
        // translate 1
        const a_trad = try dirkey.DirStr.from_num_keys(from, to, allocator);
        var a_perms = Permutator{ .visited = a_trad };
        // permutate 1
        while (try a_perms.next()) |a_perm| {
            var a_pairs = std.mem.window(dirkey.DirKey, a_perm.dir_keys, 2, 1);
            // windows 1
            while (a_pairs.next()) |a_pair| {
                // translate 2
                const b_trad = try dirkey.DirStr.from_dir_keys(a_pair[0], a_pair[1], allocator);
                var b_perms = Permutator{ .visited = b_trad };
                // permutate 2
                while (try b_perms.next()) |b_perm| {
                    var b_pairs = std.mem.window(dirkey.DirKey, b_perm.dir_keys, 2, 1);
                    // windows 2
                    while (b_pairs.next()) |b_pair| {
                        // translate 3 (final)
                        const c_trad = try dirkey.DirStr.from_dir_keys(b_pair[0], b_pair[1], allocator);
                        // permutate 3 (final)
                        var c_perms = Permutator{ .visited = c_trad };
                        while (try c_perms.next()) |c_perm| {
                            std.debug.print("({c}) A:{c}  B:{c}  C:{c}\n", .{ a_perm, a_pair, b_pair, c_perm });
                        }
                    }
                }
            }
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    try Generator.dostuff(.a, .n0, allocator);
}
