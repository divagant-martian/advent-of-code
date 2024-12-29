const std = @import("std");
const libgrid = @import("grid");
const Sorted = @import("sorted_array").Sorted;
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const Str: type = []const u8;
const Int: type = i32;

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

const Node = struct {
    pos: libgrid.Position,
    dir: libgrid.Direction,

    pub fn eq(this: *const @This(), other: *const @This()) bool {
        return this.dir == other.dir and this.pos.eq(&other.pos);
    }
};

const NodeInfo = struct { dist: usize, prev: Node, other_prevs: std.AutoArrayHashMap(Node, void) };

const Path: type = std.ArrayList(Node);
const PartialPath = struct { expanded: Path, next_nodes: std.ArrayList(Node) };

const CompCtx = struct {
    info: std.AutoHashMap(Node, NodeInfo),

    const MaxDist: usize = std.math.maxInt(usize);

    pub fn compare(self: *const @This(), a: *const Node, b: *const Node) std.math.Order {
        return std.math.order(self.distance(a), self.distance(b));
    }

    pub fn distance(self: *const @This(), item: *const Node) usize {
        if (self.info.get(item.*)) |info| {
            return info.dist;
        }
        return @This().MaxDist;
    }

    pub fn put(self: *@This(), item: Node, dist: usize, prev: Node) !void {
        try self.info.put(item, .{ .dist = dist, .prev = prev, .other_prevs = std.AutoArrayHashMap(Node, void).init(self.info.allocator) });
    }

    /// Expands a path one node.
    ///
    /// Returns whether this path is now fully expanded
    pub fn expand_path_once(self: *const @This(), partial_path: *const Path, next: Node) !PartialPath {
        var expanded_path = try partial_path.clone();
        try expanded_path.append(next);
        const next_info = self.info.get(next).?;
        var next_nodes = std.ArrayList(Node).init(self.info.allocator);
        if (next.eq(&next_info.prev)) {
            return .{ .expanded = expanded_path, .next_nodes = next_nodes };
        } else {
            try next_nodes.appendSlice(next_info.other_prevs.keys());
            try next_nodes.append(next_info.prev);
            return .{ .expanded = expanded_path, .next_nodes = next_nodes };
        }
    }

    pub fn shortest_distance(self: *const CompCtx, pos: libgrid.Position) usize {
        var best_dist = CompCtx.MaxDist;
        for ([_]libgrid.Direction{ .up, .down, .left, .right }) |dir| {
            const node = Node{ .pos = pos, .dir = dir };
            const node_dist = self.distance(&node);
            if (node_dist < best_dist) {
                best_dist = node_dist;
            }
        }
        return best_dist;
    }

    pub fn shortest_paths(self: *const CompCtx, end_pos: libgrid.Position) !std.ArrayList(std.ArrayList(Node)) {
        const allocator = self.info.allocator;
        var paths = std.ArrayList(Path).init(allocator);

        const best_dist = self.shortest_distance(end_pos);
        var queue = std.ArrayList(PartialPath).init(allocator);

        // start by adding the terminal nodes with the best distance
        for ([_]libgrid.Direction{ .up, .down, .left, .right }) |dir| {
            const end_node = Node{ .pos = end_pos, .dir = dir };
            const node_dist = self.distance(&end_node);
            if (node_dist == best_dist) {
                var next_nodes = std.ArrayList(Node).init(allocator);
                try next_nodes.append(end_node);
                const partial_path = PartialPath{ .expanded = Path.init(allocator), .next_nodes = next_nodes };
                try queue.append(partial_path);
            }
        }

        while (queue.popOrNull()) |partial_path| {
            for (partial_path.next_nodes.items) |next| {
                const expanded_partial_path = try self.expand_path_once(&partial_path.expanded, next);
                if (expanded_partial_path.next_nodes.items.len == 0) {
                    try paths.append(expanded_partial_path.expanded);
                } else {
                    try queue.append(expanded_partial_path);
                }
            }

            partial_path.expanded.deinit();
            partial_path.next_nodes.deinit();
        }

        return paths;
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

    fn shortest_path_info(self: *const Map) !CompCtx {
        const allocator = self.grid.get_allocator();

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
                } else if (dist_via_me == neighbor_dist) {
                    const info = comp_ctx.info.getPtr(node).?;
                    if (!info.*.prev.eq(&curr)) {
                        try info.*.other_prevs.put(curr, {});
                    }
                }
            }
        }

        return comp_ctx;
    }

    pub fn printable_path(self: *const Map, path: []const Node) !libgrid.Grid([]const u8) {
        var sp_grid = try libgrid.Grid([]const u8).repeat(" ", self.grid.get_lines(), self.grid.cols, self.grid.get_allocator());

        var grid_iter = self.grid.iterator();

        while (grid_iter.next()) |item| {
            // turn each tile into a drawable string, ignoring start
            sp_grid.get_mut(item.position).?.* = switch (item.value.*) {
                .wall => "#",
                .start => continue,
                .end => continue,
                .empty => continue,
            };
        }

        for (path) |curr| {
            // add each node in the path as an arrow
            sp_grid.get_mut(curr.pos).?.* = switch (curr.dir) {
                .up => "\x1b[1;33m^\x1b[0m",
                .down => "\x1b[1;33mv\x1b[0m",
                .left => "\x1b[1;33m<\x1b[0m",
                .right => "\x1b[1;33m>\x1b[0m",
            };
        }

        sp_grid.get_mut(self.start).?.* = "\x1b[1;33mS\x1b[0m";
        sp_grid.get_mut(self.end).?.* = "\x1b[1;33mE\x1b[0m";

        return sp_grid;
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
    std.debug.print("{c}", .{map});
    const info = try map.shortest_path_info();

    const dist = info.shortest_distance(map.end);
    const all_shortests = try info.shortest_paths(map.end);

    switch (args.part) {
        .a => {
            const printable = try map.printable_path(all_shortests.items[0].items);
            std.log.info("shortest path: {s}", .{printable.formatter(std.fmt.formatText)});
            std.log.info("{d}", .{dist});
        },
        .b => {
            var positions = std.AutoHashMap(libgrid.Position, void).init(allocator);
            for (all_shortests.items) |path| {
                for (path.items) |node| {
                    try positions.put(node.pos, {});
                }
            }

            std.log.info("{d}", .{positions.count()});
        },
    }
}
