#ifndef RUST_ZIG_SIMD_H
#define RUST_ZIG_SIMD_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

void vector_add_f32(
    const float *a,
    const float *b,
    float *out,
    size_t len
);

void vector_add_f32_simd(
    const float *a,
    const float *b,
    float *out,
    size_t len
);

#ifdef __cplusplus
}
#endif

#endif
