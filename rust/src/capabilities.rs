use crate::backend::BackendKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Architecture {
    X86,
    X86_64,
    Aarch64,
    Wasm32,
    Other,
}

impl Architecture {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::X86 => "x86",
            Self::X86_64 => "x86_64",
            Self::Aarch64 => "aarch64",
            Self::Wasm32 => "wasm32",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum VectorModel {
    Scalar,
    FixedWidth,
    Scalable,
}

impl VectorModel {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::FixedWidth => "fixed-width",
            Self::Scalable => "scalable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities {
    architecture: Architecture,
    avx2: bool,
    fma: bool,
    neon: bool,
    wasm_simd128: bool,
}

impl Capabilities {
    #[must_use]
    pub fn detect() -> Self {
        Self {
            architecture: detect_architecture(),
            avx2: detect_avx2(),
            fma: detect_fma(),
            neon: detect_neon(),
            wasm_simd128: detect_wasm_simd128(),
        }
    }

    #[must_use]
    pub const fn architecture(self) -> Architecture {
        self.architecture
    }

    #[must_use]
    pub const fn has_avx2(self) -> bool {
        self.avx2
    }

    #[must_use]
    pub const fn has_fma(self) -> bool {
        self.fma
    }

    #[must_use]
    pub const fn has_neon(self) -> bool {
        self.neon
    }

    #[must_use]
    pub const fn has_wasm_simd128(self) -> bool {
        self.wasm_simd128
    }

    #[must_use]
    pub const fn vector_model(self) -> VectorModel {
        if self.avx2 || self.neon || self.wasm_simd128 {
            VectorModel::FixedWidth
        } else {
            VectorModel::Scalar
        }
    }

    /// Returns the fastest rust-simd backend available in this process.
    ///
    /// This describes implemented rust-simd backends, not merely CPU features.
    #[must_use]
    pub const fn best_backend(self) -> BackendKind {
        if self.avx2 && self.fma {
            BackendKind::Avx2Fma
        } else if self.avx2 {
            BackendKind::Avx2
        } else if self.neon {
            BackendKind::Neon
        } else if self.wasm_simd128 {
            BackendKind::WasmSimd128
        } else {
            BackendKind::Scalar
        }
    }
}

const fn detect_architecture() -> Architecture {
    #[cfg(target_arch = "x86")]
    {
        return Architecture::X86;
    }

    #[cfg(target_arch = "x86_64")]
    {
        return Architecture::X86_64;
    }

    #[cfg(target_arch = "aarch64")]
    {
        return Architecture::Aarch64;
    }

    #[cfg(target_arch = "wasm32")]
    {
        return Architecture::Wasm32;
    }

    #[allow(unreachable_code)]
    Architecture::Other
}

fn detect_avx2() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        return std::is_x86_feature_detected!("avx2");
    }

    #[allow(unreachable_code)]
    false
}

fn detect_fma() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        return std::is_x86_feature_detected!("fma");
    }

    #[allow(unreachable_code)]
    false
}

fn detect_neon() -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        return std::arch::is_aarch64_feature_detected!("neon");
    }

    #[allow(unreachable_code)]
    false
}

fn detect_wasm_simd128() -> bool {
    cfg!(all(target_arch = "wasm32", target_feature = "simd128"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_current_architecture() {
        let capabilities = Capabilities::detect();

        #[cfg(target_arch = "x86")]
        assert_eq!(capabilities.architecture(), Architecture::X86);

        #[cfg(target_arch = "x86_64")]
        assert_eq!(capabilities.architecture(), Architecture::X86_64);

        #[cfg(target_arch = "aarch64")]
        assert_eq!(capabilities.architecture(), Architecture::Aarch64);

        #[cfg(target_arch = "wasm32")]
        assert_eq!(capabilities.architecture(), Architecture::Wasm32);
    }

    #[test]
    fn best_backend_matches_detected_capabilities() {
        let capabilities = Capabilities::detect();

        let expected = if capabilities.has_avx2() && capabilities.has_fma() {
            BackendKind::Avx2Fma
        } else if capabilities.has_avx2() {
            BackendKind::Avx2
        } else if capabilities.has_neon() {
            BackendKind::Neon
        } else if capabilities.has_wasm_simd128() {
            BackendKind::WasmSimd128
        } else {
            BackendKind::Scalar
        };

        assert_eq!(capabilities.best_backend(), expected);
    }

    #[test]
    fn fma_backend_requires_avx2_and_fma() {
        let capabilities = Capabilities::detect();

        if capabilities.best_backend() == BackendKind::Avx2Fma {
            assert!(capabilities.has_avx2());
            assert!(capabilities.has_fma());
        }
    }

    #[test]
    fn neon_backend_requires_neon() {
        let capabilities = Capabilities::detect();

        if capabilities.best_backend() == BackendKind::Neon {
            assert!(capabilities.has_neon());
        }
    }

    #[test]
    fn wasm_backend_requires_simd128() {
        let capabilities = Capabilities::detect();

        if capabilities.best_backend() == BackendKind::WasmSimd128 {
            assert!(capabilities.has_wasm_simd128());
        }
    }
}
