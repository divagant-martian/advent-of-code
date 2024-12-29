const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const Opcode = enum {
    a_div,
    b_div,
    c_div,
    b_xor,
    b_str,
    b_xor_c,
    jnz,
    out,
};

const Op = struct { opcode: Opcode, operand: u3 };

const Program = struct {
    a_reg: usize,
    b_reg: usize,
    c_reg: usize,

    instruction_pointer: usize,
    operations: std.ArrayList(Op),

    fn parse(data: []const u8, allocator: std.mem.Allocator) error{ missingRegisterDec, missingRegister, invalidRegister, invalidByte, outOfMemory, missingProgramDec, missingOperand }!Program {
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
        var operations = std.ArrayList(Op).initCapacity(allocator, sections.rest().len / 4) catch return error.outOfMemory;
        while (sections.next()) |opcode_str| {
            const opcode_byte = std.fmt.parseInt(u3, opcode_str, 10) catch return error.invalidByte;
            const opcode: Opcode = switch (opcode_byte) {
                0 => .a_div,
                1 => .b_xor,
                2 => .b_str,
                3 => .jnz,
                4 => .b_xor_c,
                5 => .out,
                6 => .b_div,
                7 => .c_div,
            };

            const operand_str = sections.next() orelse return error.missingOperand;
            const operand = std.fmt.parseInt(u3, operand_str, 10) catch return error.invalidByte;

            operations.appendAssumeCapacity(Op{ .opcode = opcode, .operand = operand });
        }

        return Program{ .a_reg = a_reg, .b_reg = b_reg, .c_reg = c_reg, .instruction_pointer = 0, .operations = operations };
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
    const program = try Program.parse(data, allocator);
    std.debug.print("{any}", .{program});
}
