const std = @import("std");
const kernels = @import("root.zig");

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 20;

fn checksum(out: []const f32) f64 {
    var sum: f64 = 0.0;

    for (out) |value| {
        sum += value;
    }

    return sum;
}

fn benchmarkScalar(
    io: std.Io,
    a: []const f32,
    b: []const f32,
    out: []f32,
) i96 {
    var warmup: usize = 0;

    while (warmup < WARMUP) : (warmup += 1) {
        kernels.vectorAdd(a, b, out);
    }

    const start = std.Io.Clock.awake.now(io);

    var run: usize = 0;
    while (run < RUNS) : (run += 1) {
        kernels.vectorAdd(a, b, out);
    }

    const end = std.Io.Clock.awake.now(io);

    return start.durationTo(end).toNanoseconds();
}

fn benchmarkSimd(
    io: std.Io,
    a: []const f32,
    b: []const f32,
    out: []f32,
) i96 {
    var warmup: usize = 0;

    while (warmup < WARMUP) : (warmup += 1) {
        kernels.vectorAddSimd(a, b, out);
    }

    const start = std.Io.Clock.awake.now(io);

    var run: usize = 0;
    while (run < RUNS) : (run += 1) {
        kernels.vectorAddSimd(a, b, out);
    }

    const end = std.Io.Clock.awake.now(io);

    return start.durationTo(end).toNanoseconds();
}

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    const allocator = std.heap.page_allocator;

    const a = try allocator.alloc(f32, N);
    defer allocator.free(a);

    const b = try allocator.alloc(f32, N);
    defer allocator.free(b);

    const out = try allocator.alloc(f32, N);
    defer allocator.free(out);

    for (a, 0..) |*value, i| {
        value.* = @floatFromInt(i % 1000);
    }

    for (b, 0..) |*value, i| {
        value.* = @floatFromInt((i * 3) % 1000);
    }

    @memset(out, 0);

    const scalar_elapsed = benchmarkScalar(io, a, b, out);
    const scalar_checksum = checksum(out);

    @memset(out, 0);

    const simd_elapsed = benchmarkSimd(io, a, b, out);
    const simd_checksum = checksum(out);

    const runs_i96: i96 = @intCast(RUNS);

    std.debug.print(
        "=== Rust-Zig SIMD Lab / vector_add ===\n" ++
        "elements:             {d}\n" ++
        "runs:                 {d}\n" ++
        "\n" ++
        "Zig scalar:\n" ++
        "  total ns:           {d}\n" ++
        "  per run ns:         {d}\n" ++
        "  checksum:           {d:.3}\n" ++
        "\n" ++
        "Zig explicit SIMD:\n" ++
        "  total ns:           {d}\n" ++
        "  per run ns:         {d}\n" ++
        "  checksum:           {d:.3}\n",
        .{
            N,
            RUNS,

            scalar_elapsed,
            @divTrunc(scalar_elapsed, runs_i96),
            scalar_checksum,

            simd_elapsed,
            @divTrunc(simd_elapsed, runs_i96),
            simd_checksum,
        },
    );
}
