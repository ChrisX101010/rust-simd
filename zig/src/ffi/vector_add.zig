export fn vector_add_f32(
    a: [*]const f32,
    b: [*]const f32,
    out: [*]f32,
    len: usize,
) void {
    var i: usize = 0;

    while (i < len) : (i += 1) {
        out[i] = a[i] + b[i];
    }
}

export fn vector_add_f32_simd(
    a: [*]const f32,
    b: [*]const f32,
    out: [*]f32,
    len: usize,
) void {
    const V = @Vector(8, f32);

    var i: usize = 0;

    while (i + 8 <= len) : (i += 8) {
        const av: V = a[i..][0..8].*;
        const bv: V = b[i..][0..8].*;

        out[i..][0..8].* = av + bv;
    }

    while (i < len) : (i += 1) {
        out[i] = a[i] + b[i];
    }
}

export fn simd_ffi_noop() void {}

export fn simd_ffi_add_one(x: u64) u64 {
    return x + 1;
}

pub export fn fma_f32(
    a: [*]const f32,
    b: [*]const f32,
    c: [*]const f32,
    out: [*]f32,
    len: usize,
) void {
    var i: usize = 0;

    while (i < len) : (i += 1) {
        out[i] = a[i] * b[i] + c[i];
    }
}

pub export fn fma_f32_simd(
    a: [*]const f32,
    b: [*]const f32,
    c: [*]const f32,
    out: [*]f32,
    len: usize,
) void {
    const V = @Vector(8, f32);

    var i: usize = 0;

    while (i + 8 <= len) : (i += 8) {
        const av: V = a[i..][0..8].*;
        const bv: V = b[i..][0..8].*;
        const cv: V = c[i..][0..8].*;

        out[i..][0..8].* = av * bv + cv;
    }

    while (i < len) : (i += 1) {
        out[i] = a[i] * b[i] + c[i];
    }
}

pub export fn fma_f32_muladd(
    a: [*]const f32,
    b: [*]const f32,
    c: [*]const f32,
    out: [*]f32,
    len: usize,
) void {
    var i: usize = 0;

    while (i < len) : (i += 1) {
        out[i] = @mulAdd(f32, a[i], b[i], c[i]);
    }
}

pub export fn fma_f32_muladd_simd(
    a: [*]const f32,
    b: [*]const f32,
    c: [*]const f32,
    out: [*]f32,
    len: usize,
) void {
    const V = @Vector(8, f32);

    var i: usize = 0;

    while (i + 8 <= len) : (i += 8) {
        const av: V = a[i..][0..8].*;
        const bv: V = b[i..][0..8].*;
        const cv: V = c[i..][0..8].*;

        out[i..][0..8].* = @mulAdd(V, av, bv, cv);
    }

    while (i < len) : (i += 1) {
        out[i] = @mulAdd(f32, a[i], b[i], c[i]);
    }
}

pub export fn reduce_sum_f32(
    data: [*]const f32,
    len: usize,
) f32 {
    var i: usize = 0;
    var total: f32 = 0.0;

    while (i < len) : (i += 1) {
        total += data[i];
    }

    return total;
}

pub export fn reduce_sum_f32_simd(
    data: [*]const f32,
    len: usize,
) f32 {
    const V = @Vector(8, f32);

    var i: usize = 0;
    var acc = @as(V, @splat(0.0));

    while (i + 8 <= len) : (i += 8) {
        const v: V = data[i..][0..8].*;
        acc += v;
    }

    var total: f32 = @reduce(.Add, acc);

    while (i < len) : (i += 1) {
        total += data[i];
    }

    return total;
}

pub export fn reduce_sum_f32_simd_4acc(
    data: [*]const f32,
    len: usize,
) f32 {
    const V = @Vector(8, f32);

    var i: usize = 0;

    var acc0 = @as(V, @splat(0.0));
    var acc1 = @as(V, @splat(0.0));
    var acc2 = @as(V, @splat(0.0));
    var acc3 = @as(V, @splat(0.0));

    while (i + 32 <= len) : (i += 32) {
        acc0 += data[i..][0..8].*;
        acc1 += data[i + 8 ..][0..8].*;
        acc2 += data[i + 16 ..][0..8].*;
        acc3 += data[i + 24 ..][0..8].*;
    }

    const combined = acc0 + acc1 + acc2 + acc3;
    var total: f32 = @reduce(.Add, combined);

    while (i < len) : (i += 1) {
        total += data[i];
    }

    return total;
}
