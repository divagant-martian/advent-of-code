const std = @import("std");
const lib_grid = @import("grid");
const Args = @import("args").Args;
const Position = lib_grid.Position;

const Char: type = u8;

const Grid: type = lib_grid.Grid(Char);

const Id: type = u16;
const Info = struct { char: Char, area: usize = 0, perimeter: usize = 0, concave_corners: usize = 0, convex_corners: usize = 0 };

fn get_regions(grid: *const Grid) !std.AutoArrayHashMap(Id, Info) {
    var regions = std.AutoArrayHashMap(Id, Info).init(grid.get_allocator());

    // for each position we keep whether an id is already known.
    var id_grid = try std.ArrayList(?Id).initCapacity(grid.get_allocator(), grid.items().len);
    id_grid.appendNTimesAssumeCapacity(null, grid.items().len);

    var known_ids = lib_grid.Grid(?Id){ .grid = id_grid, .cols = grid.cols };
    var next_id: Id = 0;

    for (0..known_ids.get_lines()) |i| {
        for (0..known_ids.cols) |j| {
            const pos = Position{ .i = i, .j = j };
            if (known_ids.get(pos).? == null) {
                const info = try traverse_region(grid, pos, next_id, &known_ids);
                try regions.put(next_id, info);
                next_id += 1;
            }
        }
    }

    return regions;
}

fn get_regions_price(regions: []const Info, part: Args.Part) usize {
    var price: usize = 0;
    for (regions) |info| {
        price += switch (part) {
            .a => info.area * info.perimeter,
            .b => info.area * (info.convex_corners + info.concave_corners),
        };
    }
    return price;
}

fn traverse_region(grid: *const Grid, pos: Position, id: Id, known_ids: *lib_grid.Grid(?Id)) !Info {
    const letter = grid.get(pos) orelse @panic("position outside of grid");
    var queue = std.ArrayList(Position).init(grid.get_allocator());
    try queue.append(pos);

    var info = Info{ .char = letter };

    while (queue.popOrNull()) |check_pos| {
        const maybe_id = known_ids.get_mut(check_pos).?;
        if (maybe_id.* != null) {
            continue;
        } else {
            maybe_id.* = id;
        }

        info.area += 1;
        info.concave_corners += count_concave_corners(grid, check_pos);
        info.convex_corners += count_convex_corners(grid, check_pos);
        var neighbors = grid.neighbors(check_pos);

        // assume this position is the whole region. We substract perimeters
        // per neighbors on the same region
        info.perimeter += 4;
        while (neighbors.next()) |neighbor| {
            if (neighbor.item == letter) {
                info.perimeter -= 1;
                if (known_ids.get(neighbor.pos).? == null) {
                    try queue.append(neighbor.pos);
                }
            }
        }
    }

    return info;
}

fn count_concave_corners(grid: *const Grid, pos: Position) usize {
    const letter = grid.get(pos) orelse @panic("trying to find corner with position outside of grid");
    var dir: ?lib_grid.Direction = lib_grid.Direction.up;

    var corners: usize = 0;
    while (dir) |first_dir| {
        const second_dir = first_dir.rotate();
        const maybe_first_pos = pos.move(first_dir);
        const maybe_second_pos = pos.move(second_dir);

        var is_first_different: bool = true;
        var is_second_different: bool = true;

        if (maybe_first_pos) |first_pos| {
            if (grid.get(first_pos)) |first_neighbor| {
                if (first_neighbor == letter) {
                    is_first_different = false;
                }
            }
        }

        if (maybe_second_pos) |second_pos| {
            if (grid.get(second_pos)) |second_neighbor| {
                if (second_neighbor == letter) {
                    is_second_different = false;
                }
            }
        }

        if (is_first_different and is_second_different) {
            corners += 1;
        }

        dir = first_dir.rotate_no_repeat();
    }

    return corners;
}

fn count_convex_corners(grid: *const Grid, pos: Position) usize {
    const letter = grid.get(pos) orelse @panic("trying to find corner with position outside of grid");
    var dir: ?lib_grid.Direction = lib_grid.Direction.up;

    var corners: usize = 0;
    while (dir) |first_dir| : (dir = first_dir.rotate_no_repeat()) {
        const second_dir = first_dir.rotate();

        // std.log.debug("checking [{c}] at ({d},{d}) direction {any}", .{ letter, pos.i, pos.j, first_dir });

        const first_pos = pos.move(first_dir) orelse continue;
        const second_pos = pos.move(second_dir) orelse continue;
        const diagonal_pos = first_pos.move(second_dir) orelse continue;

        const first_neighbor = grid.get(first_pos) orelse continue;
        const second_neighbor = grid.get(second_pos) orelse continue;
        const diag_neighbor = grid.get(diagonal_pos) orelse continue;

        if ((letter == first_neighbor) and (letter == second_neighbor) and (letter != diag_neighbor)) {
            corners += 1;
        }
    }

    return corners;
}

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

    const parse_fn = struct {
        pub fn call(c: u8) !Char {
            return c;
        }
    }.call;

    const grid = try Grid.new(data, parse_fn, allocator);
    defer grid.deinit();

    const formatter = grid.formatter(std.fmt.formatIntValue);
    std.log.debug("{c: <6}", .{formatter});

    const regions = try get_regions(&grid);
    for (regions.values()) |info| {
        std.log.debug("[{c}] area: {d} perimeter: {d: <3} concave: {d: <3} convex: {d: <3}", info);
    }
    const price = get_regions_price(regions.values(), args.part);
    std.log.info("price {d}", .{price});
}
