const std = @import("std");
const kernels = @import("ffi/vector_add.zig");

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

fn bench(
    io: std.Io,
    a: []const f32,
    b: []const f32,
    comptime implementation: u8,
) void {
    var result: f32 = 0.0;

    var warmup: usize = 0;
    while (warmup < WARMUP) : (warmup += 1) {
        result = switch (implementation) {
            0 => kernels.dot_f32(a.ptr, b.ptr, a.len),
            1 => kernels.dot_f32_simd(a.ptr, b.ptr, a.len),
            else => unreachable,
        };
    }

    var timings: [RUNS]i96 = undefined;

    var run: usize = 0;
    while (run < RUNS) : (run += 1) {
        const start = std.Io.Clock.awake.now(io);

        result = switch (implementation) {
            0 => kernels.dot_f32(a.ptr, b.ptr, a.len),
            1 => kernels.dot_f32_simd(a.ptr, b.ptr, a.len),
            else => unreachable,
        };

        const end = std.Io.Clock.awake.now(io);
        timings[run] = start.durationTo(end).toNanoseconds();
    }

    var sorted = timings;
    std.mem.sort(i96, &sorted, {}, std.sort.asc(i96));

    const name = switch (implementation) {
        0 => "scalar",
        1 => "explicit_simd",
        else => "unknown",
    };

    std.debug.print(
        "kernel=dot\n" ++
        "language=zig\n" ++
        "implementation={s}\n" ++
        "elements={d}\n" ++
        "warmup={d}\n" ++
        "runs={d}\n" ++
        "median_ns={d}\n" ++
        "result={d:.6}\n",
        .{
            name,
            N,
            WARMUP,
            RUNS,
            sorted[RUNS / 2],
            result,
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

    for (a, 0..) |*x, i| {
        x.* = @as(f32, @floatFromInt(i % 997)) * 0.001;
    }

    for (b, 0..) |*x, i| {
        x.* = @as(f32, @floatFromInt(i % 991)) * 0.002;
    }

    bench(io, a, b, 0);
    bench(io, a, b, 1);
}
