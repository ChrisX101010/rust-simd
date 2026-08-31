use core::fmt;

/// Errors produced by checked SIMD operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdError {
    /// Two input slices have different lengths.
    InputLengthMismatch { left: usize, right: usize },

    /// An output slice does not match the input length.
    OutputLengthMismatch { expected: usize, actual: usize },

    /// A requested backend is not supported by the current CPU.
    UnsupportedBackend { backend: &'static str },
}

impl fmt::Display for SimdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputLengthMismatch { left, right } => {
                write!(f, "input lengths must match: left={left}, right={right}")
            }
            Self::OutputLengthMismatch { expected, actual } => {
                write!(
                    f,
                    "output length must match input length: expected={expected}, actual={actual}"
                )
            }
            Self::UnsupportedBackend { backend } => {
                write!(f, "backend is not supported by this CPU: {backend}")
            }
        }
    }
}

impl std::error::Error for SimdError {}

pub type Result<T> = core::result::Result<T, SimdError>;

#[inline]
pub(crate) fn validate_binary_inputs(a: &[f32], b: &[f32], out: Option<&[f32]>) -> Result<()> {
    if a.len() != b.len() {
        return Err(SimdError::InputLengthMismatch {
            left: a.len(),
            right: b.len(),
        });
    }

    if let Some(out) = out
        && out.len() != a.len()
    {
        return Err(SimdError::OutputLengthMismatch {
            expected: a.len(),
            actual: out.len(),
        });
    }

    Ok(())
}

#[inline]
pub(crate) fn validate_fma_inputs(a: &[f32], b: &[f32], c: &[f32], out: &[f32]) -> Result<()> {
    validate_binary_inputs(a, b, Some(out))?;

    if c.len() != a.len() {
        return Err(SimdError::InputLengthMismatch {
            left: a.len(),
            right: c.len(),
        });
    }

    Ok(())
}
