const std = @import("std");

pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{ .name = "main", .root_source_file = b.path("./main.zig"), .target = b.standardTargetOptions(.{}), .optimize = b.standardOptimizeOption(.{}) });
    const grid = b.addModule("grid", .{ .root_source_file = b.path("../grid.zig") });
    const args = b.addModule("args", .{ .root_source_file = b.path("../args.zig") });
    const sorted_array = b.addModule("sorted_array", .{ .root_source_file = b.path("./sorted_array.zig") });

    exe.root_module.addImport("grid", grid);
    exe.root_module.addImport("args", args);
    exe.root_module.addImport("sorted_array", sorted_array);

    b.installArtifact(exe);

    const run_exe = b.addRunArtifact(exe);
    if (b.args) |run_args| {
        run_exe.addArgs(run_args);
    }

    const run_step = b.step("run", "run bin");
    run_step.dependOn(&run_exe.step);
}
