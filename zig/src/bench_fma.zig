const std = @import("std");
const kernels = @import("ffi/vector_add.zig");

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

fn checksum(data: []const f32) f64 {
    var sum: f64 = 0.0;

    for (data) |x| {
        sum += x;
    }

    return sum;
}

fn bench(
    io: std.Io,
    a: []const f32,
    b: []const f32,
    c: []const f32,
    out: []f32,
    comptime implementation: u8,
) void {
    var warmup: usize = 0;

    while (warmup < WARMUP) : (warmup += 1) {
        switch (implementation) {
            0 => kernels.fma_f32(
                a.ptr, b.ptr, c.ptr, out.ptr, out.len
            ),
            1 => kernels.fma_f32_simd(
                a.ptr, b.ptr, c.ptr, out.ptr, out.len
            ),
            2 => kernels.fma_f32_muladd(
                a.ptr, b.ptr, c.ptr, out.ptr, out.len
            ),
            3 => kernels.fma_f32_muladd_simd(
                a.ptr, b.ptr, c.ptr, out.ptr, out.len
            ),
            else => unreachable,
        }
    }

    var timings: [RUNS]i96 = undefined;

    var run: usize = 0;

    while (run < RUNS) : (run += 1) {
        const start = std.Io.Clock.awake.now(io);

        switch (implementation) {
            0 => kernels.fma_f32(
                a.ptr, b.ptr, c.ptr, out.ptr, out.len
            ),
            1 => kernels.fma_f32_simd(
                a.ptr, b.ptr, c.ptr, out.ptr, out.len
            ),
            2 => kernels.fma_f32_muladd(
                a.ptr, b.ptr, c.ptr, out.ptr, out.len
            ),
            3 => kernels.fma_f32_muladd_simd(
                a.ptr, b.ptr, c.ptr, out.ptr, out.len
            ),
            else => unreachable,
        }

        const end = std.Io.Clock.awake.now(io);
        timings[run] = start.durationTo(end).toNanoseconds();
    }

    var sorted = timings;
    std.mem.sort(i96, &sorted, {}, std.sort.asc(i96));

    var total: i96 = 0;

    for (timings) |t| {
        total += t;
    }

    std.debug.print(
        "kernel=fma\n" ++
        "language=zig\n" ++
        "implementation={s}\n" ++
        "elements={d}\n" ++
        "warmup={d}\n" ++
        "runs={d}\n" ++
        "total_ns={d}\n" ++
        "min_ns={d}\n" ++
        "median_ns={d}\n" ++
        "checksum={d:.6}\n",
        .{
            switch (implementation) { 0 => "scalar", 1 => "explicit_simd", 2 => "muladd", 3 => "muladd_simd", else => "unknown" },
            N,
            WARMUP,
            RUNS,
            total,
            sorted[0],
            sorted[RUNS / 2],
            checksum(out),
        },
    );
}

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    const allocator = std.heap.page_allocator;

    const a = try allocator.alloc(f32, N);
    defer allocator.free(a);

    const b = try allocator.alloc(f32, N);
    defer allocator.free(b);

    const c = try allocator.alloc(f32, N);
    defer allocator.free(c);

    const out = try allocator.alloc(f32, N);
    defer allocator.free(out);

    for (a, 0..) |*x, i| {
        x.* = @as(f32, @floatFromInt(i % 997)) * 0.001;
    }

    for (b, 0..) |*x, i| {
        x.* = @as(f32, @floatFromInt(i % 991)) * 0.002;
    }

    for (c, 0..) |*x, i| {
        x.* = @as(f32, @floatFromInt(i % 983)) * 0.003;
    }

    @memset(out, 0);

    bench(io, a, b, c, out, 0);
    bench(io, a, b, c, out, 1);
    bench(io, a, b, c, out, 2);
    bench(io, a, b, c, out, 3);
}
