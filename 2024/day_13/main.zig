const std = @import("std");
const Args = @import("args").Args;

const Int: type = i64;

const Str: type = []const u8;

const Equation: type = struct {
    ax: Int,
    ay: Int,
    bx: Int,
    by: Int,
    cx: Int,
    cy: Int,

    const Self = @This();

    const ParseErr = error{
        MissingLine,
        InvalidLine,
    };

    fn parse(input: Str, part: Args.Part) !Self {
        const Iter = struct {
            iter: std.mem.TokenIterator(u8, .any),
            fn next(self: *@This()) !Int {
                _ = self.iter.next() orelse {
                    return Self.ParseErr.InvalidLine;
                };

                const int_str = self.iter.next() orelse {
                    return Self.ParseErr.InvalidLine;
                };

                return try std.fmt.parseInt(Int, int_str, 10);
            }
        };

        var iter = Iter{
            .iter = std.mem.tokenizeAny(u8, input, "\n,+="),
        };

        const add: Int = switch (part) {
            .a => 0,
            .b => 10000000000000,
        };

        return Self{
            .ax = try iter.next(),
            .ay = try iter.next(),
            .bx = try iter.next(),
            .by = try iter.next(),
            .cx = try iter.next() + add,
            .cy = try iter.next() + add,
        };
    }

    fn solve(self: *const Self) ?struct { a: Int, b: Int } {
        const det = (self.ax * self.by) - (self.bx * self.ay);
        if (det == 0) {
            std.log.debug("determinant is zero", .{});
            return null;
        }

        const a_times_det = self.by * self.cx - self.bx * self.cy;
        if (@mod(a_times_det, det) != 0) {
            std.log.debug("cannot compute a: determinant ({d}) does not divide {d}", .{ det, a_times_det });
            return null;
        }

        const b_times_det = self.ax * self.cy - self.ay * self.cx;
        if (@mod(b_times_det, det) != 0) {
            std.log.debug("cannot compute b: determinant ({d}) does not divide {d}", .{ det, b_times_det });
            return null;
        }

        return .{ .a = @divExact(a_times_det, det), .b = @divExact(b_times_det, det) };
    }

    pub fn format(
        self: *const Self,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        // "x: {ax}a + {bx}b = {cx}"
        try writer.writeAll("x: ");
        try std.fmt.formatIntValue(self.ax, fmt, options, writer);
        try writer.writeAll("a + ");
        try std.fmt.formatIntValue(self.bx, fmt, options, writer);
        try writer.writeAll("b = ");
        try std.fmt.formatIntValue(self.cx, fmt, options, writer);

        // "y: {ay}a + {by}b = {cy}"
        try writer.writeAll("\ny: ");
        try std.fmt.formatIntValue(self.ay, fmt, options, writer);
        try writer.writeAll("a + ");
        try std.fmt.formatIntValue(self.by, fmt, options, writer);
        try writer.writeAll("b = ");
        try std.fmt.formatIntValue(self.cy, fmt, options, writer);
    }
};

fn parse_equations(data: Str, part: Args.Part, allocator: std.mem.Allocator) !std.ArrayList(Equation) {
    var equations = std.ArrayList(Equation).init(allocator);

    var iter = std.mem.tokenizeSequence(u8, data, "\n\n");
    while (iter.next()) |input| {
        const equation = try Equation.parse(input, part);
        try equations.append(equation);
    }

    return equations;
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

    const equations = try parse_equations(data, args.part, allocator);

    var total_cost: Int = 0;
    for (equations.items, 1..) |equation, idx| {
        std.log.debug("equation {d}\n{d}", .{ idx, equation });

        const solution = equation.solve() orelse continue;
        const cost = 3 * solution.a + solution.b;
        total_cost += cost;
    }

    std.log.info("total: {d}", .{total_cost});
}
