const std = @import("std");

pub const std_options = .{
    .log_level = .debug,
};

const Num: type = u64;

const Op = enum(u2) {
    add,
    mul,
    pip,

    fn apply(self: Op, lhs: Num, rhs: Num) Num {
        return switch (self) {
            .add => lhs + rhs,
            .mul => lhs * rhs,
            .pip => {
                const exp = std.math.log10_int(rhs);
                const tens = std.math.powi(Num, 10, exp + 1) catch unreachable;
                return lhs * tens + rhs;
            },
        };
    }

    pub fn format(self: Op, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;

        const char: u8 = switch (self) {
            .add => '+',
            .mul => '*',
            .pip => '|',
        };

        try writer.print("{u}", .{char});
    }
};

const Equation = struct {
    result: Num,
    operands: std.ArrayList(Num),

    const newErr = error{ missingResult, notEnoughOperands };

    const WithOps = struct {
        this: *const Equation,
        ops: []const Op,

        pub fn format(self: WithOps, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
            _ = fmt;
            _ = options;

            try writer.print("{d}", .{self.this.operands.items[0]});
            for (self.ops, self.this.operands.items[1..]) |operator, operand| {
                try writer.print(" {s} {d}", .{ operator, operand });
            }
        }
    };

    fn new(line: []const u8, allocator: std.mem.Allocator) !Equation {
        var tokens = std.mem.tokenizeAny(u8, line, ": ");

        const result_str = tokens.next() orelse return Equation.newErr.missingResult;
        const result = try std.fmt.parseInt(Num, result_str, 10);

        var operands = std.ArrayList(Num).init(allocator);
        while (tokens.next()) |operand_str| {
            const operand = try std.fmt.parseInt(Num, operand_str, 10);
            try operands.append(operand);
        }

        if (operands.items.len < 2) {
            return Equation.newErr.notEnoughOperands;
        }

        return Equation{ .result = result, .operands = operands };
    }

    fn is_solved_by(self: *const Equation, ops: []const Op) bool {
        if (ops.len + 1 != self.operands.items.len) {
            @panic("mismatched number of operands and operations");
        }

        return self.apply(ops) == self.result;
    }

    fn apply(self: *const Equation, ops: []const Op) Num {
        var result = self.operands.items[0];
        const len = ops.len;
        for (ops, self.operands.items[1 .. len + 1]) |operator, operand| {
            result = operator.apply(result, operand);
        }

        return result;
    }

    fn is_solvable(self: *const Equation, with_pip: bool) !bool {
        var to_check = std.ArrayList(std.ArrayList(Op)).init(self.operands.allocator);
        defer to_check.deinit();

        const empty_ops = std.ArrayList(Op).init(self.operands.allocator);
        try to_check.append(empty_ops);

        while (to_check.popOrNull()) |current_ops| {
            if (current_ops.items.len + 1 != self.operands.items.len) {
                const result = self.apply(current_ops.items);

                // check if worth exploring
                if (result > self.result) {
                    continue;
                }

                var with_mul = try current_ops.clone();
                try with_mul.append(.mul);

                var with_add = try current_ops.clone();
                try with_add.append(.add);

                try to_check.append(with_mul);
                try to_check.append(with_add);

                if (with_pip) {
                    var with_pipee = try current_ops.clone();
                    try with_pipee.append(.pip);

                    try to_check.append(with_pipee);
                }
            } else if (self.is_solved_by(current_ops.items)) {
                // solution found
                const with_ops = Equation.WithOps{ .this = self, .ops = current_ops.items };
                std.log.debug("Solution: {s} = {d}", .{ with_ops, self.result });
                return true;
            }
            current_ops.deinit();
        }

        return false;
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    // process program name
    _ = args.next().?;

    var second_part = false;
    if (args.next()) |part| {
        if (std.mem.eql(u8, part, "a")) {
            second_part = false;
        } else if (std.mem.eql(u8, part, "b")) {
            second_part = true;
        } else {
            std.log.err("parts a or b, you on drugs: {s}", .{part});
            std.process.exit(1);
        }
    } else {
        std.log.err("missing program part", .{});
        std.process.exit(1);
    }

    var file_name: []const u8 = undefined;
    if (args.next()) |name| {
        file_name = name;
    } else {
        std.log.err("missing file name", .{});
        std.process.exit(1);
    }

    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    var lines = std.mem.tokenizeScalar(u8, data, '\n');

    var total: usize = 0;
    while (lines.next()) |line| {
        const eq = try Equation.new(line, allocator);
        if (try eq.is_solvable(second_part)) {
            total += eq.result;
        }
        eq.operands.deinit();
    }
    std.log.info("sol: {d}", .{total});
}

test "pip" {
    try std.testing.expectEqual(1234567, Op.pip.apply(123, 4567));
}
