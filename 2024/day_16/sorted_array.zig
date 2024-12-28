const std = @import("std");

pub fn Sorted(comptime T: type) type {
    return struct {
        inner: std.ArrayList(T),

        const Self = @This();

        pub fn init(allocator: std.mem.Allocator) Self {
            return Self{ .inner = std.ArrayList(T).init(allocator) };
        }

        /// Finds by linear search the position at which `item` is according to
        /// the `std.math.Order` returned by `comp_ctx.compare(item, other)`.
        /// This will be sorted with "smaller" elements to the end.
        pub fn add(self: *Self, item: T, comp_ctx: anytype) !void {
            for (self.inner.items, 0..) |at_idx, idx| {
                const order: std.math.Order = comp_ctx.compare(&item, &at_idx);
                switch (order) {
                    .eq => return try self.inner.insert(idx, item),
                    .gt => return try self.inner.insert(idx, item),
                    .lt => {},
                }
            }

            try self.inner.append(item);
        }

        pub fn remove(self: *Self, idx: usize) T {
            return self.inner.orderedRemove(idx);
        }

        pub fn find_index(self: *const Self, item: *const T, eq_fn: fn (*const T, *const T) bool) ?usize {
            for (self.inner.items, 0..) |at_idx, idx| {
                if (eq_fn(&at_idx, item)) {
                    return idx;
                }
            }
            return null;
        }

        /// Will return the highest element.
        pub fn pop_front(self: *Self) ?T {
            if (self.inner.items.len == 0) {
                return null;
            }

            return self.inner.orderedRemove(0);
        }

        /// Will return the lowest element.
        pub fn pop_back(self: *Self) ?T {
            return self.inner.popOrNull();
        }
    };
}

test "sorted add" {
    var sorted = Sorted(usize).init(std.testing.allocator);
    defer sorted.inner.deinit();
    const CompCtx = struct {
        fn compare(_: *const @This(), a: *const usize, b: *const usize) std.math.Order {
            return std.math.order(a.*, b.*);
        }
    };
    const ctx = CompCtx{};
    try sorted.add(3, ctx);
    std.debug.print("{d}", .{sorted.inner.items});
    try sorted.add(4, ctx);
    std.debug.print("{d}", .{sorted.inner.items});
    try sorted.add(2, ctx);
    std.debug.print("{d}", .{sorted.inner.items});
    try sorted.add(1, ctx);
    std.debug.print("{d}", .{sorted.inner.items});
    try sorted.add(5, ctx);
    std.debug.print("{d}", .{sorted.inner.items});

    try std.testing.expectEqual(5, sorted.inner.items[0]);
    try std.testing.expectEqual(4, sorted.inner.items[1]);
    try std.testing.expectEqual(3, sorted.inner.items[2]);
    try std.testing.expectEqual(2, sorted.inner.items[3]);
    try std.testing.expectEqual(1, sorted.inner.items[4]);
}
