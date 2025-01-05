const std = @import("std");
const dirkey = @import("dir_key");

pub const Permutator = struct {
    og: dirkey.DirStr,
    visited: dirkey.DirStr,
    still_going: ?void = {},

    pub fn new(dis_str: dirkey.DirStr) !Permutator {
        return Permutator{
            .og = try dis_str.clone(),
            .visited = dis_str,
        };
    }

    pub fn reset(self: *Permutator) !void {
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
