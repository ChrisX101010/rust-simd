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
