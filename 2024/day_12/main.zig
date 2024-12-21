const std = @import("std");
const lib_grid = @import("grid");
const Args = @import("args").Args;
const Position = lib_grid.Position;

const Char: type = u8;

const Grid: type = lib_grid.Grid(Char);

const Id: type = u16;
const Info = struct { char: Char, area: usize = 0, perimeter: usize = 0 };

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

fn get_regions_price(regions: []const Info) usize {
    var price: usize = 0;
    for (regions) |info| {
        price += info.perimeter * info.area;
    }
    return price;
}

fn traverse_region(grid: *const Grid, pos: Position, id: Id, known_ids: *lib_grid.Grid(?Id)) !Info {
    const letter = grid.get(pos) orelse @panic("position outside of grid");
    var queue = std.ArrayList(Position).init(grid.get_allocator());
    try queue.append(pos);

    var info = Info{
        .char = letter,
    };

    while (queue.popOrNull()) |check_pos| {
        // std.log.debug("checking pos {any}", .{check_pos});
        const maybe_id = known_ids.get_mut(check_pos).?;
        if (maybe_id.* != null) {
            continue;
        } else {
            maybe_id.* = id;
        }

        info.area += 1;
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

    const formatter = grid.formatter(std.fmt.formatIntValue);
    std.log.debug("{c: <3}", .{formatter});

    switch (args.part) {
        .a => {
            const regions = try get_regions(&grid);
            for (regions.values()) |info| {
                std.log.debug("[{c}] area: {d} perimeter: {d}", info);
            }
            const price = get_regions_price(regions.values());
            std.log.info("price {d}", .{price});
        },
        .b => {
            @panic("unimplemented");
        },
    }
}
