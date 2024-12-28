const std = @import("std");
const libgrid = @import("grid");
const Sorted = @import("sorted_array").Sorted;
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const Str: type = []const u8;
const Int: type = i32;

const VerticalDir = enum { up, down };

const Tile = enum {
    wall,
    start,
    end,
    empty,

    pub const ParseErr = error{invalidChar};

    pub fn parse(c: u8) !Tile {
        const tile: Tile = switch (c) {
            '#' => .wall,
            'S' => .start,
            'E' => .end,
            '.' => .empty,
            else => return Tile.ParseErr.invalidChar,
        };

        return tile;
    }
};

const Map = struct {
    grid: libgrid.Grid(Tile),
    start: libgrid.Position,
    end: libgrid.Position,

    const ParseErr = error{ missingStart, duplicatedStart, missingEnd, duplicatedEnd };

    fn parse(data: []const u8, allocator: std.mem.Allocator) !Map {
        const grid = try libgrid.Grid(Tile).new(data, Tile.parse, allocator);
        var start_pos: ?libgrid.Position = null;
        var end_pos: ?libgrid.Position = null;

        var iter = grid.iterator();
        while (iter.next()) |entry| {
            if (entry.value.* == .start) {
                if (start_pos == null) {
                    start_pos = entry.position;
                } else {
                    return Map.ParseErr.duplicatedStart;
                }
            } else if (entry.value.* == .end) {
                if (end_pos == null) {
                    end_pos = entry.position;
                } else {
                    return Map.ParseErr.duplicatedEnd;
                }
            }
        }

        if (start_pos) |start| {
            if (end_pos) |end| {
                return Map{ .grid = grid, .start = start, .end = end };
            } else {
                return Map.ParseErr.missingEnd;
            }
        } else {
            return Map.ParseErr.missingStart;
        }
    }

    fn shortest_path(self: *const Map) !usize {
        const Node = struct {
            pos: libgrid.Position,
            dir: libgrid.Direction,

            pub fn eq(this: *const @This(), other: *const @This()) bool {
                return this.dir == other.dir and this.pos.eq(&other.pos);
            }
        };

        const NodeInfo = struct { dist: usize, prev: Node };

        const allocator = self.grid.get_allocator();

        const CompCtx = struct {
            info: std.AutoHashMap(Node, NodeInfo),

            const MaxDist: usize = std.math.maxInt(usize);

            pub fn compare(me: *const @This(), a: *const Node, b: *const Node) std.math.Order {
                return std.math.order(me.distance(a), me.distance(b));
            }

            pub fn distance(me: *const @This(), item: *const Node) usize {
                if (me.info.get(item.*)) |info| {
                    return info.dist;
                }
                return @This().MaxDist;
            }

            pub fn put(me: *@This(), item: Node, dist: usize, prev: Node) !void {
                try me.info.put(item, .{ .dist = dist, .prev = prev });
            }
        };

        var comp_ctx = CompCtx{ .info = std.AutoHashMap(Node, NodeInfo).init(allocator) };

        // initialization: the start node has distance 0 and goes in the queue
        const start_node = Node{ .pos = self.start, .dir = .right };
        try comp_ctx.put(start_node, 0, start_node);

        var sorted = Sorted(Node).init(allocator);
        try sorted.add(start_node, &comp_ctx);

        // process the queue
        while (sorted.pop_back()) |curr| {
            var neighbors = self.grid.neighbors(curr.pos);
            while (neighbors.next()) |neighbor| {

                // check if it's worth expanding
                if (neighbor.item == .wall) {
                    continue;
                }

                // this tile is worth expanding, we need to figure out what
                // node it is based on what direction we end up if we go there
                const node = Node{ .pos = neighbor.pos, .dir = curr.pos.direction(&neighbor.pos) };

                const neighbor_dist = comp_ctx.distance(&node);
                const relative_dist: usize = 1 + 1000 * curr.dir.count_rotations(node.dir);
                const dist_via_me = comp_ctx.distance(&curr) + relative_dist;

                if (dist_via_me < neighbor_dist) {
                    try comp_ctx.put(node, dist_via_me, curr);
                    if (sorted.find_index(&node, Node.eq)) |remove_idx| {
                        _ = sorted.remove(remove_idx);
                    }
                    try sorted.add(node, &comp_ctx);
                }
            }
        }

        var best_end: ?Node = null;
        var best_dist: usize = std.math.maxInt(usize);

        var iter = comp_ctx.info.iterator();

        while (iter.next()) |entry| {
            if (entry.key_ptr.pos.eq(&self.end)) {
                std.log.info("{any}: {d}", .{ entry.key_ptr.*, entry.value_ptr.dist });
                if (entry.value_ptr.dist < best_dist) {
                    best_end = entry.key_ptr.*;
                    best_dist = entry.value_ptr.dist;
                }
            }
        }

        var sp_grid = try libgrid.Grid([]const u8).repeat(" ", self.grid.get_lines(), self.grid.cols, self.grid.get_allocator());

        var grid_iter = self.grid.iterator();

        while (grid_iter.next()) |item| {
            sp_grid.get_mut(item.position).?.* = switch (item.value.*) {
                .wall => "#",
                .start => continue,
                .end => "\x1b[1;33mE\x1b[0m",
                .empty => continue,
            };
        }

        var curr = best_end orelse unreachable;

        while (!curr.pos.eq(&self.start)) {
            const prev = comp_ctx.info.get(curr).?.prev;
            sp_grid.get_mut(prev.pos).?.* = switch (curr.dir) {
                .up => "\x1b[1;33m^\x1b[0m",
                .down => "\x1b[1;33mv\x1b[0m",
                .left => "\x1b[1;33m<\x1b[0m",
                .right => "\x1b[1;33m>\x1b[0m",
            };
            // std.log.info("prev of {d}, {d} is {d}, {d}", .{ curr.i, curr.j, prev.i, prev.j });
            curr = prev;
        }

        sp_grid.get_mut(self.start).?.* = "\x1b[1;33mS\x1b[0m";

        std.log.info("shortest path: {s}", .{sp_grid.formatter(std.fmt.formatText)});

        return best_dist;
    }

    pub fn format(self: *const Map, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        const formatter = self.grid.formatter(grid_fmt);
        try formatter.format(fmt, options, writer);
    }
};

const grid_fmt = struct {
    fn foo(
        value: Tile,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        const char: u8 = switch (value) {
            .empty => '.',
            .start => 'S',
            .end => 'E',
            .wall => '#',
        };
        try std.fmt.formatAsciiChar(char, options, writer);
    }
}.foo;

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

    const map = try Map.parse(data, allocator);

    switch (args.part) {
        .a => {
            std.debug.print("{c}", .{map});
            const dist = try map.shortest_path();
            std.log.info("{d}", .{dist});
        },
        .b => {
            unreachable;
        },
    }
}
