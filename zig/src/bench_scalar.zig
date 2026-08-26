const std = @import("std");
const kernels = @import("root.zig");

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

fn checksum(data: []const f32) f64 {
    var sum: f64 = 0.0;
    for (data) |x| sum += x;
    return sum;
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

    for (a, 0..) |*x, i| {
        x.* = @floatFromInt(i % 1000);
    }

    for (b, 0..) |*x, i| {
        x.* = @floatFromInt((i * 3) % 1000);
    }

    @memset(out, 0);

    var warmup: usize = 0;
    while (warmup < WARMUP) : (warmup += 1) {
        kernels.vectorAdd(a, b, out);
    }

    var timings: [RUNS]i96 = undefined;

    var run: usize = 0;
    while (run < RUNS) : (run += 1) {
        const start = std.Io.Clock.awake.now(io);

        kernels.vectorAdd(a, b, out);

        const end = std.Io.Clock.awake.now(io);
        timings[run] = start.durationTo(end).toNanoseconds();
    }

    var sorted = timings;
    std.mem.sort(i96, &sorted, {}, std.sort.asc(i96));

    var total: i96 = 0;
    for (timings) |t| total += t;

    std.debug.print(
        "kernel=vector_add\n" ++
        "language=zig\n" ++
        "implementation=scalar\n" ++
        "elements={d}\n" ++
        "warmup={d}\n" ++
        "runs={d}\n" ++
        "total_ns={d}\n" ++
        "min_ns={d}\n" ++
        "median_ns={d}\n" ++
        "checksum={d:.3}\n",
        .{
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
