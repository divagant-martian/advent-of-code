const std = @import("std");
const libgrid = @import("grid");
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
        const Node = struct { pos: libgrid.Position, dir: libgrid.Direction };
        const NodeInfo = struct { dist: usize, prev: Node, visited: bool };
        const QueuedNode = struct { node: Node, dist: usize };

        var node_infos = std.AutoHashMap(Node, NodeInfo).init(self.grid.get_allocator());
        defer node_infos.deinit();

        const cmp_fn = struct {
            fn cmp(_: void, a: QueuedNode, b: QueuedNode) std.math.Order {
                return std.math.order(a.dist, b.dist);
            }
        }.cmp;
        var queue = std.PriorityQueue(QueuedNode, void, cmp_fn).init(self.grid.get_allocator(), {});
        defer queue.deinit();

        const start_node = Node{ .pos = self.start, .dir = .right };
        try node_infos.put(start_node, NodeInfo{ .dist = 0, .prev = start_node, .visited = false });
        try queue.add(.{ .node = start_node, .dist = 0 });

        while (queue.count() > 0) {
            const curr = queue.remove().node;

            const curr_info = node_infos.getPtr(curr).?;
            const curr_dist = curr_info.*.dist;

            curr_info.*.visited = true;

            var neighbor_iter = self.grid.neighbors(curr.pos);
            while (neighbor_iter.next()) |neighbor| {
                if (self.grid.get(neighbor.pos).? == .wall) {
                    continue;
                }

                const neighbor_dir = curr.pos.direction(&neighbor.pos);
                const neighbor_node = Node{ .pos = neighbor.pos, .dir = neighbor_dir };
                const neighbor_info_ptr = node_infos.getPtr(neighbor_node);

                if (neighbor_info_ptr) |neighbor_info| {
                    if (neighbor_info.visited) {
                        continue;
                    }
                }

                const relative_dist = 1 + 1000 * curr.dir.count_rotations(neighbor_dir);
                const alt_dist = curr_dist + relative_dist;

                if (neighbor_info_ptr) |neighbor_info| {
                    if (neighbor_info.dist > alt_dist) {
                        neighbor_info.* = .{ .dist = alt_dist, .prev = curr, .visited = neighbor_info.*.visited };
                        queue.update(.{ .node = neighbor_node, .dist = neighbor_info.dist }, .{ .node = neighbor_node, .dist = alt_dist }) catch {
                            try queue.add(.{ .node = neighbor_node, .dist = alt_dist });
                        };
                    } else {
                        queue.update(.{ .node = neighbor_node, .dist = neighbor_info.dist }, .{ .node = neighbor_node, .dist = std.math.maxInt(usize) }) catch {
                            try queue.add(.{ .node = neighbor_node, .dist = std.math.maxInt(usize) });
                        };
                    }
                } else {
                    try node_infos.put(neighbor_node, .{ .dist = alt_dist, .prev = curr, .visited = false });
                    try queue.add(.{ .node = neighbor_node, .dist = alt_dist });
                }
            }
        }

        var best_end: ?Node = null;
        var best_dist: usize = std.math.maxInt(usize);
        var iter = node_infos.iterator();

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
            const prev = node_infos.get(curr).?.prev;
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
