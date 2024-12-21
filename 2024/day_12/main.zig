const std = @import("std");
const lib_grid = @import("grid");
const Args = @import("args").Args;

const Num: type = u8;

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
        pub fn call(c: u8) !Num {
            return c;
        }
    }.call;

    const grid = try lib_grid.Grid(Num).new(data, parse_fn, allocator);

    const formatter = grid.formatter(std.fmt.formatIntValue);

    std.log.info("{c: <3}", .{formatter});
}
