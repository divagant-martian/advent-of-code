const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const Opcode = enum(u3) {
    a_div = 0,
    b_div = 6,
    c_div = 7,
    b_xor = 1,
    b_str = 2,
    b_xor_c = 4,
    jnz = 3,
    out = 5,
};

const Op = struct { opcode: Opcode, operand: u3 };

const Program = struct {
    a_reg: usize,
    b_reg: usize,
    c_reg: usize,

    instruction_pointer: usize,
    bytes: std.ArrayList(u3),

    fn parse(data: []const u8, allocator: std.mem.Allocator) error{ missingRegisterDec, missingRegister, invalidRegister, invalidByte, outOfMemory, missingProgramDec }!Program {
        var sections = std.mem.tokenizeAny(u8, data, " ,\n");

        _ = sections.next() orelse return error.missingRegisterDec;
        _ = sections.next() orelse return error.missingRegisterDec;
        const a_reg_str = sections.next() orelse return error.missingRegister;
        const a_reg = std.fmt.parseInt(usize, a_reg_str, 10) catch return error.invalidRegister;

        _ = sections.next() orelse return error.missingRegisterDec;
        _ = sections.next() orelse return error.missingRegisterDec;
        const b_reg_str = sections.next() orelse return error.missingRegister;
        const b_reg = std.fmt.parseInt(usize, b_reg_str, 10) catch return error.invalidRegister;

        _ = sections.next() orelse return error.missingRegisterDec;
        _ = sections.next() orelse return error.missingRegisterDec;
        const c_reg_str = sections.next() orelse return error.missingRegister;
        const c_reg = std.fmt.parseInt(usize, c_reg_str, 10) catch return error.invalidRegister;

        _ = sections.next() orelse return error.missingProgramDec;
        var bytes = std.ArrayList(u3).initCapacity(allocator, sections.rest().len / 2) catch return error.outOfMemory;
        while (sections.next()) |byte_str| {
            const byte = std.fmt.parseInt(u3, byte_str, 10) catch return error.invalidByte;
            bytes.appendAssumeCapacity(byte);
        }

        return Program{ .a_reg = a_reg, .b_reg = b_reg, .c_reg = c_reg, .instruction_pointer = 0, .bytes = bytes };
    }

    fn clone(self: *const Program) !Program {
        return Program{ .a_reg = self.a_reg, .b_reg = self.b_reg, .c_reg = self.c_reg, .instruction_pointer = self.instruction_pointer, .bytes = try self.bytes.clone() };
    }

    fn step(self: *Program) union(enum) { halt, out: ?u3 } {
        if (self.instruction_pointer + 1 >= self.bytes.items.len) {
            return .halt;
        }

        std.log.debug("step: {d: >7}", .{self});

        const opcode: Opcode = @enumFromInt(self.bytes.items[self.instruction_pointer]);
        self.instruction_pointer += 1;
        const operand = self.bytes.items[self.instruction_pointer];
        self.instruction_pointer += 1;
        var out: ?u3 = null;
        switch (opcode) {
            .a_div => {
                const numerator = self.a_reg;
                const denominator = std.math.powi(usize, 2, self.combo_operand(operand)) catch @panic("(over/under)flow");
                self.a_reg = numerator / denominator;
            },
            .b_div => {
                const numerator = self.a_reg;
                const denominator = std.math.powi(usize, 2, self.combo_operand(operand)) catch @panic("(over/under)flow");
                self.b_reg = numerator / denominator;
            },
            .c_div => {
                const numerator = self.a_reg;
                const denominator = std.math.powi(usize, 2, self.combo_operand(operand)) catch @panic("(over/under)flow");
                self.c_reg = numerator / denominator;
            },
            .b_xor => {
                self.b_reg ^= operand;
            },
            .b_str => {
                self.b_reg = self.combo_operand(operand) % 8;
            },
            .b_xor_c => {
                self.b_reg ^= self.c_reg;
            },
            .jnz => {
                if (self.a_reg != 0) {
                    self.instruction_pointer = operand;
                }
            },
            .out => {
                out = @intCast(self.combo_operand(operand) % 8);
            },
        }
        return .{ .out = out };
    }

    fn combo_operand(self: *const Program, value: u3) usize {
        return switch (value) {
            0...3 => value,
            4 => self.a_reg,
            5 => self.b_reg,
            6 => self.c_reg,
            7 => @panic("this is part b"),
        };
    }

    pub fn format(self: *const Program, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        try writer.writeAll("{{ A: ");
        try std.fmt.formatIntValue(self.a_reg, fmt, options, writer);

        try writer.writeAll(", B: ");
        try std.fmt.formatIntValue(self.b_reg, fmt, options, writer);

        try writer.writeAll(", C: ");
        try std.fmt.formatIntValue(self.c_reg, fmt, options, writer);

        try writer.writeAll(", pointer: ");
        try std.fmt.formatIntValue(self.instruction_pointer, fmt, options, writer);

        if (self.instruction_pointer + 1 < self.bytes.items.len) {
            const opcode: Opcode = @enumFromInt(self.bytes.items[self.instruction_pointer]);
            const operand = self.bytes.items[self.instruction_pointer + 1];
            try std.fmt.format(writer, ", section: {s}({d})", .{ std.enums.tagName(Opcode, opcode).?, operand });
        }

        try writer.writeAll(" }}");
    }

    fn run(self: *Program) !std.ArrayList(u3) {
        var outputs = std.ArrayList(u3).init(self.bytes.allocator);
        while (true) {
            switch (self.step()) {
                .halt => break,
                .out => |maybe_out| {
                    if (maybe_out) |out| {
                        try outputs.append(out);
                    }
                },
            }
        }

        return outputs;
    }

    fn reset(self: *Program, a_reg: usize, b_reg: usize, c_reg: usize) void {
        self.a_reg = a_reg;
        self.b_reg = b_reg;
        self.c_reg = c_reg;
        self.instruction_pointer = 0;
    }

    fn solve(self: *Program) !usize {
        const b_reg = self.b_reg;
        const c_reg = self.c_reg;
        var desc_i = self.bytes.items.len;
        var a_candidates = try std.ArrayList(usize).initCapacity(self.bytes.allocator, 8);
        defer a_candidates.deinit();
        for (0..8) |value| {
            a_candidates.appendAssumeCapacity(value);
        }
        while (desc_i > 0) {
            desc_i -= 1;
            const out = self.bytes.items[desc_i];
            try self.find_a(&a_candidates, b_reg, c_reg, out);
        }

        return a_candidates.items[0] / 8;
    }

    fn find_a(self: *Program, a_candidates: *std.ArrayList(usize), b_reg: usize, c_reg: usize, out: u3) !void {
        var i: usize = 0;
        const len = a_candidates.items.len;
        while (i < len) : (i += 1) {
            const a_reg = a_candidates.orderedRemove(0);
            self.reset(a_reg, b_reg, c_reg);
            switch (self.step_until_output()) {
                .halt => {},
                .out => |output| if (output == out) {
                    for (0..8) |value| {
                        try a_candidates.append(a_reg * 8 + value);
                    }
                },
            }
        }
    }

    fn step_until_output(self: *Program) union(enum) { halt, out: u3 } {
        while (true) {
            switch (self.step()) {
                .halt => return .halt,
                .out => |maybe_out| if (maybe_out) |out| {
                    return .{ .out = out };
                },
            }
        }

        unreachable;
    }

    fn matches(self: *Program) bool {
        var find_idx: usize = 0;
        while (find_idx < self.bytes.items.len) {
            switch (self.step()) {
                .halt => break,
                .out => |maybe_out| {
                    if (maybe_out) |out| {
                        if (out != self.bytes.items[find_idx]) {
                            return false;
                        } else {
                            find_idx += 1;
                        }
                    }
                },
            }
        }

        return true;
    }
};

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

    var program = try Program.parse(data, allocator);
    defer program.bytes.deinit();

    switch (args.part) {
        .a => {
            const outputs = try program.run();
            std.debug.print("{d}\n", .{outputs.items});
        },
        .b => {
            const best_a = try program.solve();
            std.debug.print("a is {}", .{best_a});
        },
    }
}
