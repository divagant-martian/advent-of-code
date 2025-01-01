const std = @import("std");

pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{ .name = "main", .root_source_file = b.path("./main.zig"), .target = b.standardTargetOptions(.{}), .optimize = b.standardOptimizeOption(.{}) });
    const grid = b.addModule("grid", .{ .root_source_file = b.path("../grid.zig") });
    const args = b.addModule("args", .{ .root_source_file = b.path("../args.zig") });

    exe.root_module.addImport("grid", grid);
    exe.root_module.addImport("args", args);

    b.installArtifact(exe);

    const run_exe = b.addRunArtifact(exe);
    if (b.args) |run_args| {
        run_exe.addArgs(run_args);
    }

    const run_step = b.step("run", "run bin");
    run_step.dependOn(&run_exe.step);

    const test_step = b.step("test", "test bin");

    const unit_tests = b.addTest(.{ .root_source_file = b.path("./main.zig"), .target = b.resolveTargetQuery(.{}) });
    unit_tests.root_module.addImport("grid", grid);
    const run_unit_tests = b.addRunArtifact(unit_tests);
    test_step.dependOn(&run_unit_tests.step);
}
