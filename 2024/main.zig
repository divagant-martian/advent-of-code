const std = @import("std");

pub const std_options = .{
    .log_level = .debug,
};

const Fragment = struct {
    /// It is null if the fragment is empty space.
    file_id: ?u16,
    size: u16,
};

const FragmentList = std.DoublyLinkedList(Fragment);

const FragmentFmt = struct {
    node: *const FragmentList.Node,

    pub fn format(
        self: FragmentFmt,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        const n = self.node.data;

        if (n.file_id) |file_id| {
            try writer.print("{d}({d})", .{ file_id, n.size });
        } else {
            try writer.print("..{d}.", .{n.size});
        }
    }
};

fn next_empty(node: *FragmentList.Node, check_current: bool, stop_at: ?*FragmentList.Node) ?*FragmentList.Node {
    var maybe_candidate: ?*FragmentList.Node = switch (check_current) {
        true => node,
        false => node.next,
    };

    while (maybe_candidate) |candidate| {
        if (@intFromPtr(candidate) == @intFromPtr(stop_at)) {
            return null;
        }
        if (candidate.data.file_id == null) {
            return candidate;
        }

        maybe_candidate = candidate.next;
    }

    return null;
}

fn prev_file(node: *FragmentList.Node, check_current: bool) ?*FragmentList.Node {
    var candidate = switch (check_current) {
        true => node,
        false => node.prev orelse return null,
    };

    while (candidate.data.file_id == null) {
        candidate = candidate.prev orelse return null;
    }
    std.log.debug("prev file is {?}({d})", .{ candidate.data.file_id, candidate.data.size });

    return candidate;
}

const DiskMap = struct {
    fragments: FragmentList,
    allocator: std.mem.Allocator,

    fn new(data: []const u8, allocator: std.mem.Allocator) !DiskMap {
        var index: u16 = 0;
        var fragments = FragmentList{};

        for (data) |char| {
            if (char == '\n') {
                break;
            }

            const size = try std.fmt.parseInt(u4, &[_]u8{char}, 10);

            const file_id = switch (index % 2 == 0) {
                true => index / 2,
                false => null,
            };

            if (size > 0) {
                const fragment = Fragment{ .file_id = file_id, .size = size };
                const node_ptr: *FragmentList.Node = try allocator.create(FragmentList.Node);
                node_ptr.* = FragmentList.Node{ .data = fragment };
                fragments.append(node_ptr);
            }

            index += 1;
        }

        return DiskMap{ .fragments = fragments, .allocator = allocator };
    }

    fn compactify(self: *DiskMap) void {
        var empty = self.fragments.first.?;
        var file = self.fragments.last.?;

        out: while (true) {
            while (empty.data.file_id != null) {
                empty = empty.next.?;
                if (@intFromPtr(empty) == @intFromPtr(file)) {
                    break :out;
                }
            }

            while (file.data.file_id == null) {
                file = file.prev.?;
                if (@intFromPtr(empty) == @intFromPtr(file)) {
                    break :out;
                }
            }

            if (empty.data.size >= file.data.size) {
                const new_file = file.prev;
                // file fits, so it needs to go into the empty space.
                self.fragments.remove(file);

                const rem_empty = empty.data.size - file.data.size;

                if (rem_empty == 0) {
                    // If the empty space has the same size as the file, we replace the data.
                    empty.data = file.data;
                } else {
                    // Otherwise, we reduce the empty space size and insert the file before it.
                    empty.data.size = rem_empty;
                    self.fragments.insertBefore(empty, file);
                }
                file = new_file.?;
            } else {
                // file won't fit, so fit whatever we can in empty.
                const rem_file = file.data.size - empty.data.size;

                empty.data.file_id = file.data.file_id;
                file.data.size = rem_file;
            }

            std.log.debug("step:   {s}", .{self});
        }
    }

    fn find_place_for_file(self: *DiskMap, file: *FragmentList.Node) !enum { moved, doesNotFit } {
        // search the empty space to the left that fits the file
        var empty = next_empty(self.fragments.first.?, true, file) orelse return .doesNotFit;

        while (empty.data.size < file.data.size) {
            empty = next_empty(empty, false, file) orelse return .doesNotFit;
        }

        const rem_empty = empty.data.size - file.data.size;
        if (rem_empty == 0) {
            // put file in empty
            empty.data = file.data;
        } else {
            // create a new node with the contents of file to be inserted before empty.
            const new_file: *FragmentList.Node = try self.allocator.create(FragmentList.Node);
            new_file.* = FragmentList.Node{ .data = file.data };
            self.fragments.insertBefore(empty, new_file);
            // reduce empty's size to what remains.
            empty.data.size = rem_empty;
        }

        file.data.file_id = null;
        return .moved;
    }

    const MergeResult = enum { nothingToMerge, mergedLeft, mergedRight, mergedBoth };

    fn merge_empty(self: *DiskMap, empty: *FragmentList.Node) MergeResult {
        if (empty.data.file_id != null) {
            @panic("trying to merge non empty fragment");
        }

        var merged: MergeResult = .nothingToMerge;

        if (empty.next) |next| {
            if (next.data.file_id == null) {
                empty.data.size += next.data.size;
                self.fragments.remove(next);
                self.allocator.destroy(next);
                merged = .mergedLeft;
            }
        }

        if (empty.prev) |prev| {
            if (prev.data.file_id == null) {
                empty.data.size += prev.data.size;
                self.fragments.remove(prev);
                self.allocator.destroy(prev);
                merged = switch (merged) {
                    .nothingToMerge => .mergedRight,
                    .mergedLeft => .mergedBoth,
                    else => unreachable,
                };
            }
        }

        return merged;
    }

    fn smart_compactify(self: *DiskMap) !void {
        var file = prev_file(self.fragments.last.?, true);

        while (file) |curr_file| {
            const move_result = try self.find_place_for_file(curr_file);
            std.log.debug("{s} {any}", .{ FragmentFmt{ .node = curr_file }, move_result });

            if (move_result == .moved) {
                const merge_result = self.merge_empty(curr_file);
                std.log.debug("{s} {any}", .{ FragmentFmt{ .node = curr_file }, merge_result });
            }

            file = prev_file(curr_file, false);

            std.log.debug("{s}", .{self});
        }
    }

    fn checksum(self: *const DiskMap) usize {
        var n: usize = 0;
        var total: usize = 0;
        var node = self.fragments.first;

        while (node) |noud| {
            const s: usize = noud.data.size;

            if (noud.data.file_id) |file_id| {
                total += file_id * ((2 * n + s - 1) * s) / 2;
                std.log.debug("n = {d}, s = {d}", .{ n, s });
                std.log.debug("total after file {d}: {d}", .{ file_id, total });
            }

            node = noud.next;
            n += s;
        }

        return total;
    }

    pub fn format(
        self: *const DiskMap,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        var node = self.fragments.first;

        try writer.writeAll("[ ");

        while (node) |n| {
            try writer.print("{s} ", .{FragmentFmt{ .node = n }});
            node = n.next;
        }

        try writer.writeAll("]");
    }
};

const Part = enum {
    a,
    b,

    const Err = error{ missingProgramPart, unkownPart };

    fn from_args(args: *std.process.ArgIterator) Err!Part {
        if (args.next()) |part| {
            if (std.mem.eql(u8, part, "a")) {
                return Part.a;
            } else if (std.mem.eql(u8, part, "b")) {
                return Part.b;
            } else {
                return Part.Err.unkownPart;
            }
        } else {
            return Part.Err.missingProgramPart;
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    // process program name
    _ = args.next().?;

    const part = try Part.from_args(&args);

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

    var disk_map = try DiskMap.new(data, allocator);

    std.log.info("start: {s}", .{disk_map});

    switch (part) {
        .a => {
            disk_map.compactify();
        },
        .b => {
            try disk_map.smart_compactify();
        },
    }

    std.log.info("end:   {s}", .{disk_map});
    std.log.info("chksum: {d}", .{disk_map.checksum()});
}
