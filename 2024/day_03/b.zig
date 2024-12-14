const std = @import("std");

const OpIterator = struct {
    data: std.unicode.Utf8Iterator,

    fn next(self: *OpIterator) ?Op {
        var state: State = .pending;

        while (self.data.nextCodepoint()) |char| {
            blk: {
                // std.debug.print("read: {u}, state: {any}\n", .{ char, state });

                switch (state) {
                    .pending => if (char == 'm') {
                        state = State{ .mul = 0 };
                    } else if (char == 'd') {
                        state = State{ .do = .{ .is_do = false, .idx = 0 } };
                    },
                    .do => |*do_state| switch (do_state.idx) {
                        // Did read 'd'
                        0 => if (char == 'o') {
                            do_state.idx += 1;
                        } else {
                            state = State.pending;
                            break :blk;
                        },
                        // Did read 'o'
                        1 => switch (char) {
                            '(' => {
                                do_state.is_do = true;
                                do_state.idx += 1;
                            },
                            'n' => {
                                do_state.idx += 1;
                            },
                            else => {
                                state = State.pending;
                                break :blk;
                            },
                        },
                        // Did read `(` or `n`
                        2 => if (do_state.is_do) {
                            if (char == ')') {
                                return .do;
                            } else {
                                state = State.pending;
                                break :blk;
                            }
                        } else {
                            if (char == '\'') {
                                do_state.idx += 1;
                            } else {
                                state = State.pending;
                                break :blk;
                            }
                        },
                        // Did read `'`
                        3 => if (char == 't') {
                            do_state.idx += 1;
                        } else {
                            state = State.pending;
                            break :blk;
                        },
                        // Did read `t`
                        4 => if (char == '(') {
                            do_state.idx += 1;
                        } else {
                            state = State.pending;
                            break :blk;
                        },
                        // Did read `(`
                        5 => if (char == ')') {
                            return .dont;
                        } else {
                            state = State.pending;
                            break :blk;
                        },
                        else => unreachable,
                    },
                    .mul => |*val| switch (val.*) {
                        // Did read 'm'
                        0 => if (char == 'u') {
                            val.* = 1;
                        } else {
                            state = State.pending;
                            break :blk;
                        },
                        // Did read 'u'
                        1 => if (char == 'l') {
                            val.* = 2;
                        } else {
                            state = State.pending;
                            break :blk;
                        },
                        // Did read 'l'
                        2 => if (char == '(') {
                            val.* = 3;
                        } else {
                            state = State.pending;
                            break :blk;
                        },
                        // Did read '('
                        3 => {
                            const digit = digit_from_char(char) orelse {
                                state = State.pending;
                                break :blk;
                            };
                            state = State{ .number_a = .{ .acc = digit, .idx = 0 } };
                        },
                    },
                    .number_a => |*number_state| {
                        if (char == ',') {
                            state = State{ .comma = number_state.acc };
                        } else {
                            if (number_state.idx == 2) {
                                state = State.pending;
                                break :blk;
                            } else {
                                const next_digit = digit_from_char(char) orelse {
                                    state = State.pending;
                                    break :blk;
                                };

                                number_state.acc *= 10;
                                number_state.acc += next_digit;

                                number_state.idx += 1;
                            }
                        }
                    },
                    .comma => |a| {
                        const digit = digit_from_char(char) orelse {
                            state = State.pending;
                            break :blk;
                        };
                        state = State{ .number_b = .{ .a = a, .acc = digit, .idx = 0 } };
                    },
                    .number_b => |*number_state| {
                        if (char == ')') {
                            return Op{ .mul = .{ number_state.a, number_state.acc } };
                        } else {
                            if (number_state.idx == 2) {
                                state = State.pending;
                                break :blk;
                            } else {
                                const next_digit = digit_from_char(char) orelse {
                                    state = State.pending;
                                    break :blk;
                                };

                                number_state.acc *= 10;
                                number_state.acc += next_digit;

                                number_state.idx += 1;
                            }
                        }
                    },
                }
            }
        }

        return null;
    }
};

fn digit_from_char(char: u21) ?u16 {
    const byte =
        std.math.cast(u8, char) orelse return null;
    return std.fmt.parseInt(u16, &[_]u8{byte}, 10) catch return null;
}

const State = union(enum) {
    pending,
    do: struct { is_do: bool, idx: u3 },
    mul: u2,
    number_a: struct { acc: u16, idx: u2 },
    comma: u16,
    number_b: struct { a: u16, acc: u16, idx: u2 },
};

const Op = union(enum) {
    mul: struct { u16, u16 },
    do,
    dont,
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    var iter = OpIterator{ .data = (try std.unicode.Utf8View.init(data)).iterator() };

    var total: usize = 0;
    var do = true;

    while (iter.next()) |op| switch (op) {
        .mul => |operands| {
            if (do) {
                total += @as(usize, operands[0]) * @as(usize, operands[1]);
            }
        },
        .do => {
            do = true;
        },
        .dont => {
            do = false;
        },
    };

    std.debug.print("{d}\n", .{total});
}
