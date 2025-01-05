const std = @import("std");

pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{ .name = "main", .root_source_file = b.path("./main.zig"), .target = b.standardTargetOptions(.{}), .optimize = b.standardOptimizeOption(.{}) });
    const grid = b.addModule("grid", .{ .root_source_file = b.path("../grid.zig") });
    const args = b.addModule("args", .{ .root_source_file = b.path("../args.zig") });
    const num_key = b.addModule("num_key", .{ .root_source_file = b.path("./num_key.zig") });
    const dir_key = b.addModule("dir_key", .{ .root_source_file = b.path("./dir_key.zig") });
    const permutator = b.addModule("permutator", .{ .root_source_file = b.path("./permutator.zig") });
    const product = b.addModule("product", .{ .root_source_file = b.path("./product.zig") });

    permutator.addImport("dir_key", dir_key);
    product.addImport("dir_key", dir_key);

    num_key.addImport("grid", grid);
    num_key.addImport("dir_key", dir_key);

    dir_key.addImport("grid", grid);
    dir_key.addImport("num_key", num_key);

    exe.root_module.addImport("grid", grid);
    exe.root_module.addImport("args", args);
    exe.root_module.addImport("num_key", num_key);
    exe.root_module.addImport("dir_key", dir_key);
    exe.root_module.addImport("permutator", permutator);
    exe.root_module.addImport("product", product);

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
    unit_tests.root_module.addImport("args", args);
    unit_tests.root_module.addImport("num_key", num_key);
    unit_tests.root_module.addImport("dir_key", dir_key);
    unit_tests.root_module.addImport("dir_key", dir_key);
    unit_tests.root_module.addImport("permutator", permutator);
    unit_tests.root_module.addImport("product", product);
    const run_unit_tests = b.addRunArtifact(unit_tests);
    test_step.dependOn(&run_unit_tests.step);
}
