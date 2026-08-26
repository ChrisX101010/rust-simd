pub fn vectorAdd(
    a: []const f32,
    b: []const f32,
    out: []f32,
) void {
    if (a.len != b.len or a.len != out.len) {
        @panic("slice lengths must match");
    }

    for (a, b, out) |av, bv, *ov| {
        ov.* = av + bv;
    }
}

test "vectorAdd is correct" {
    var a = [_]f32{ 1.0, 2.0, 3.0, 4.0 };
    var b = [_]f32{ 10.0, 20.0, 30.0, 40.0 };
    var out = [_]f32{ 0.0, 0.0, 0.0, 0.0 };

    vectorAdd(&a, &b, &out);

    try std.testing.expectEqual(@as(f32, 11.0), out[0]);
    try std.testing.expectEqual(@as(f32, 22.0), out[1]);
    try std.testing.expectEqual(@as(f32, 33.0), out[2]);
    try std.testing.expectEqual(@as(f32, 44.0), out[3]);
}

const std = @import("std");

pub fn vectorAddSimd(
    a: []const f32,
    b: []const f32,
    out: []f32,
) void {
    if (a.len != b.len or a.len != out.len) {
        @panic("slice lengths must match");
    }

    const V = @Vector(8, f32);

    var i: usize = 0;

    while (i + 8 <= a.len) : (i += 8) {
        const av: V = a[i..][0..8].*;
        const bv: V = b[i..][0..8].*;

        const cv = av + bv;

        out[i..][0..8].* = cv;
    }

    while (i < a.len) : (i += 1) {
        out[i] = a[i] + b[i];
    }
}

test "vectorAddSimd is correct" {
    var a = [_]f32{ 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0 };
    var b = [_]f32{ 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0 };
    var out = [_]f32{ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 };

    vectorAddSimd(&a, &b, &out);

    try std.testing.expectEqual(
        [_]f32{ 11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0, 88.0 },
        out,
    );
}

test "vectorAdd handles SIMD boundary lengths" {
    const allocator = std.testing.allocator;

    for ([_]usize{ 0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33 }) |n| {
        const a = try allocator.alloc(f32, n);
        defer allocator.free(a);

        const b = try allocator.alloc(f32, n);
        defer allocator.free(b);

        const out = try allocator.alloc(f32, n);
        defer allocator.free(out);

        for (a, 0..) |*value, i| {
            value.* = @floatFromInt(i);
        }

        for (b, 0..) |*value, i| {
            value.* = @floatFromInt(i * 3);
        }

        vectorAdd(a, b, out);

        for (out, 0..) |value, i| {
            try std.testing.expectEqual(
                a[i] + b[i],
                value,
            );
        }
    }
}
