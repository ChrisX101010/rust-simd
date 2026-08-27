const std = @import("std");
const kernels = @import("ffi/vector_add.zig");

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

fn bench(
    io: std.Io,
    data: []const f32,
    comptime implementation: u8,
) void {
    var result: f32 = 0.0;

    var warmup: usize = 0;
    while (warmup < WARMUP) : (warmup += 1) {
        result = switch (implementation) {
            0 => kernels.reduce_sum_f32(data.ptr, data.len),
            1 => kernels.reduce_sum_f32_simd(data.ptr, data.len),
            2 => kernels.reduce_sum_f32_simd_4acc(data.ptr, data.len),
            else => unreachable,
        };
    }

    var timings: [RUNS]i96 = undefined;

    var run: usize = 0;
    while (run < RUNS) : (run += 1) {
        const start = std.Io.Clock.awake.now(io);

        result = switch (implementation) {
            0 => kernels.reduce_sum_f32(data.ptr, data.len),
            1 => kernels.reduce_sum_f32_simd(data.ptr, data.len),
            2 => kernels.reduce_sum_f32_simd_4acc(data.ptr, data.len),
            else => unreachable,
        };

        const end = std.Io.Clock.awake.now(io);
        timings[run] = start.durationTo(end).toNanoseconds();
    }

    var sorted = timings;
    std.mem.sort(i96, &sorted, {}, std.sort.asc(i96));

    const name = switch (implementation) {
        0 => "scalar",
        1 => "explicit_simd_1acc",
        2 => "explicit_simd_4acc",
        else => "unknown",
    };

    std.debug.print(
        "kernel=reduce_sum\n" ++
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

    const data = try allocator.alloc(f32, N);
    defer allocator.free(data);

    for (data, 0..) |*x, i| {
        x.* = @as(f32, @floatFromInt(i % 997)) * 0.001;
    }

    bench(io, data, 0);
    bench(io, data, 1);
    bench(io, data, 2);
}
