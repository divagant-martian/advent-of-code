const std = @import("std");
const dirkey = @import("dir_key");

pub fn Product(comptime GenT: type) type {
    return struct {
        generators: std.ArrayList(GenT),
        currents: std.ArrayList(dirkey.DirStr),
        advancing_permutator: usize,
        still_going: ?void = {},

        pub fn new_from_generators(generators: std.ArrayList(GenT)) !@This() {
            var currents = try std.ArrayList(dirkey.DirStr).initCapacity(generators.allocator, generators.items.len);
            for (generators.items) |*gen| {
                const perm = (try gen.next()).?;
                currents.appendAssumeCapacity(perm);
            }
            const advancing_permutator = generators.items.len - 1;
            return .{ .generators = generators, .currents = currents, .advancing_permutator = advancing_permutator };
        }

        pub fn reset(self: *@This()) !void {
            self.currents.clearRetainingCapacity();
            for (self.generators.items) |*gen| {
                try gen.reset();
                const perm = (try gen.next()).?;
                self.currents.appendAssumeCapacity(perm);
            }
            self.advancing_permutator = self.generators.items.len - 1;
            self.still_going = {};
        }

        pub fn join_currents(self: *const @This()) !dirkey.DirStr {
            var joined = std.ArrayList(dirkey.DirKey).init(self.generators.allocator);
            errdefer joined.deinit();
            try joined.append(.enter);
            for (self.currents.items) |dir_str| {
                try joined.appendSlice(dir_str.dir_keys[1..]);
            }

            return dirkey.DirStr.from_arraylist(joined);
        }

        pub fn next(self: *@This()) !?dirkey.DirStr {
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
