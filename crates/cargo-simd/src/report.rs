use std::fmt::Write as _;

pub const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Fail,
    NotRun,
}

impl CheckStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::NotRun => "not-run",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FailureRecord {
    pub backend: String,
    pub operation: String,
    pub case_name: Option<String>,
    pub length: usize,
    pub index: Option<usize>,
    pub actual: f32,
    pub expected: f64,
    pub error: f64,
    pub tolerance: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VerificationReport {
    pub tool_version: &'static str,
    pub backend_filter: Option<String>,
    pub backends: Vec<String>,
    pub lengths: Vec<usize>,
    pub lengths_completed: usize,
    pub numerical_cases: CheckStatus,
    pub vector_add: CheckStatus,
    pub fma: CheckStatus,
    pub reduce_sum: CheckStatus,
    pub dot: CheckStatus,
    pub failures: Vec<FailureRecord>,
}

impl VerificationReport {
    pub fn new(backend_filter: Option<String>, backends: Vec<String>, lengths: Vec<usize>) -> Self {
        Self {
            tool_version: env!("CARGO_PKG_VERSION"),
            backend_filter,
            backends,
            lengths,
            lengths_completed: 0,
            numerical_cases: CheckStatus::NotRun,
            vector_add: CheckStatus::NotRun,
            fma: CheckStatus::NotRun,
            reduce_sum: CheckStatus::NotRun,
            dot: CheckStatus::NotRun,
            failures: Vec::new(),
        }
    }

    pub fn passed(&self) -> bool {
        self.failures.is_empty()
            && self.lengths_completed == self.lengths.len()
            && self.numerical_cases == CheckStatus::Pass
            && self.vector_add == CheckStatus::Pass
            && self.fma == CheckStatus::Pass
            && self.reduce_sum == CheckStatus::Pass
            && self.dot == CheckStatus::Pass
    }

    pub fn status(&self) -> &'static str {
        if self.passed() { "pass" } else { "fail" }
    }

    pub fn to_text(&self) -> String {
        let mut out = String::new();

        writeln!(&mut out, "cargo-simd verify").expect("writing to String cannot fail");
        writeln!(&mut out).expect("writing to String cannot fail");

        if self.backend_filter.is_some() {
            writeln!(&mut out, "selected backend:").expect("writing to String cannot fail");
        } else {
            writeln!(&mut out, "available backends:").expect("writing to String cannot fail");
        }

        for backend in &self.backends {
            writeln!(&mut out, "  - {backend}").expect("writing to String cannot fail");
        }

        writeln!(&mut out).expect("writing to String cannot fail");
        writeln!(&mut out, "running differential verification...")
            .expect("writing to String cannot fail");

        if self.lengths_completed == self.lengths.len() {
            writeln!(&mut out, "running structured numerical cases...")
                .expect("writing to String cannot fail");
        }

        if self.passed() {
            writeln!(&mut out).expect("writing to String cannot fail");
            writeln!(&mut out, "verification summary").expect("writing to String cannot fail");
            writeln!(&mut out, "  lengths tested     {}", self.lengths_completed)
                .expect("writing to String cannot fail");
            writeln!(&mut out, "  backends tested    {}", self.backends.len())
                .expect("writing to String cannot fail");
            writeln!(
                &mut out,
                "  numerical cases    {}",
                display_status(self.numerical_cases)
            )
            .expect("writing to String cannot fail");
            writeln!(
                &mut out,
                "  vector_add         {}",
                display_status(self.vector_add)
            )
            .expect("writing to String cannot fail");
            writeln!(
                &mut out,
                "  fma                {}",
                display_status(self.fma)
            )
            .expect("writing to String cannot fail");
            writeln!(
                &mut out,
                "  reduce_sum         {}",
                display_status(self.reduce_sum)
            )
            .expect("writing to String cannot fail");
            writeln!(
                &mut out,
                "  dot                {}",
                display_status(self.dot)
            )
            .expect("writing to String cannot fail");
            writeln!(&mut out).expect("writing to String cannot fail");
            writeln!(&mut out, "SIMD verification: PASS").expect("writing to String cannot fail");
        } else {
            for failure in &self.failures {
                writeln!(&mut out).expect("writing to String cannot fail");
                writeln!(&mut out, "SIMD verification: FAILED")
                    .expect("writing to String cannot fail");
                writeln!(&mut out).expect("writing to String cannot fail");
                writeln!(&mut out, "  backend            {}", failure.backend)
                    .expect("writing to String cannot fail");
                writeln!(&mut out, "  operation          {}", failure.operation)
                    .expect("writing to String cannot fail");

                if let Some(case_name) = &failure.case_name {
                    writeln!(&mut out, "  case               {case_name}")
                        .expect("writing to String cannot fail");
                }

                writeln!(&mut out, "  length             {}", failure.length)
                    .expect("writing to String cannot fail");

                if let Some(index) = failure.index {
                    writeln!(&mut out, "  index              {index}")
                        .expect("writing to String cannot fail");
                }

                writeln!(&mut out, "  actual             {}", failure.actual)
                    .expect("writing to String cannot fail");
                writeln!(&mut out, "  expected           {}", failure.expected)
                    .expect("writing to String cannot fail");
                writeln!(&mut out, "  absolute error     {}", failure.error)
                    .expect("writing to String cannot fail");
                writeln!(&mut out, "  tolerance          {}", failure.tolerance)
                    .expect("writing to String cannot fail");
            }
        }

        out
    }

    pub fn to_json(&self) -> String {
        let mut out = String::new();

        out.push('{');

        write!(&mut out, "\"schema_version\":{},", REPORT_SCHEMA_VERSION)
            .expect("writing to String cannot fail");

        out.push_str("\"tool\":\"cargo-simd\",");

        out.push_str("\"tool_version\":");
        push_json_string(&mut out, self.tool_version);
        out.push(',');

        out.push_str("\"status\":");
        push_json_string(&mut out, self.status());
        out.push(',');

        out.push_str("\"backend_filter\":");
        match &self.backend_filter {
            Some(backend) => push_json_string(&mut out, backend),
            None => out.push_str("null"),
        }
        out.push(',');

        out.push_str("\"backends\":[");
        for (index, backend) in self.backends.iter().enumerate() {
            if index != 0 {
                out.push(',');
            }
            push_json_string(&mut out, backend);
        }
        out.push_str("],");

        out.push_str("\"lengths\":{");

        write!(&mut out, "\"total\":{},", self.lengths.len())
            .expect("writing to String cannot fail");

        write!(&mut out, "\"completed\":{},", self.lengths_completed)
            .expect("writing to String cannot fail");

        out.push_str("\"cases\":[");
        for (index, length) in self.lengths.iter().enumerate() {
            if index != 0 {
                out.push(',');
            }

            write!(&mut out, "{length}").expect("writing to String cannot fail");
        }
        out.push_str("]},");

        out.push_str("\"checks\":{");

        out.push_str("\"numerical_cases\":");
        push_json_string(&mut out, self.numerical_cases.as_str());
        out.push(',');

        out.push_str("\"vector_add\":");
        push_json_string(&mut out, self.vector_add.as_str());
        out.push(',');

        out.push_str("\"fma\":");
        push_json_string(&mut out, self.fma.as_str());
        out.push(',');

        out.push_str("\"reduce_sum\":");
        push_json_string(&mut out, self.reduce_sum.as_str());
        out.push(',');

        out.push_str("\"dot\":");
        push_json_string(&mut out, self.dot.as_str());

        out.push_str("},");

        out.push_str("\"failures\":[");

        for (index, failure) in self.failures.iter().enumerate() {
            if index != 0 {
                out.push(',');
            }

            out.push('{');

            out.push_str("\"backend\":");
            push_json_string(&mut out, &failure.backend);
            out.push(',');

            out.push_str("\"operation\":");
            push_json_string(&mut out, &failure.operation);
            out.push(',');

            out.push_str("\"case\":");
            match &failure.case_name {
                Some(case_name) => push_json_string(&mut out, case_name),
                None => out.push_str("null"),
            }
            out.push(',');

            write!(&mut out, "\"length\":{},", failure.length)
                .expect("writing to String cannot fail");

            out.push_str("\"index\":");
            match failure.index {
                Some(index) => {
                    write!(&mut out, "{index}").expect("writing to String cannot fail");
                }
                None => out.push_str("null"),
            }
            out.push(',');

            out.push_str("\"actual\":");
            push_json_f32(&mut out, failure.actual);
            out.push(',');

            out.push_str("\"expected\":");
            push_json_f64(&mut out, failure.expected);
            out.push(',');

            out.push_str("\"error\":");
            push_json_f64(&mut out, failure.error);
            out.push(',');

            out.push_str("\"tolerance\":");
            push_json_f64(&mut out, failure.tolerance);

            out.push('}');
        }

        out.push_str("]}");

        out
    }
}

fn display_status(status: CheckStatus) -> &'static str {
    match status {
        CheckStatus::Pass => "PASS",
        CheckStatus::Fail => "FAIL",
        CheckStatus::NotRun => "NOT RUN",
    }
}

fn push_json_f32(out: &mut String, value: f32) {
    push_json_f64(out, value as f64);
}

fn push_json_f64(out: &mut String, value: f64) {
    if value.is_finite() {
        write!(out, "{value}").expect("writing to String cannot fail");
    } else if value.is_nan() {
        push_json_string(out, "NaN");
    } else if value.is_sign_positive() {
        push_json_string(out, "Infinity");
    } else {
        push_json_string(out, "-Infinity");
    }
}

fn push_json_string(out: &mut String, value: &str) {
    out.push('"');

    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            character if character <= '\u{1f}' => {
                write!(out, "\\u{:04x}", character as u32).expect("writing to String cannot fail");
            }
            character => out.push(character),
        }
    }

    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    fn passing_report() -> VerificationReport {
        VerificationReport {
            tool_version: "0.2.0-test",
            backend_filter: None,
            backends: vec![
                "scalar".to_owned(),
                "avx2".to_owned(),
                "avx2+fma".to_owned(),
            ],
            lengths: vec![0, 1, 7, 8],
            lengths_completed: 4,
            numerical_cases: CheckStatus::Pass,
            vector_add: CheckStatus::Pass,
            fma: CheckStatus::Pass,
            reduce_sum: CheckStatus::Pass,
            dot: CheckStatus::Pass,
            failures: Vec::new(),
        }
    }

    #[test]
    fn passing_report_has_pass_status() {
        let report = passing_report();

        assert!(report.passed());
        assert_eq!(report.status(), "pass");
    }

    #[test]
    fn failure_changes_status() {
        let mut report = passing_report();

        report.dot = CheckStatus::Fail;
        report.failures.push(FailureRecord {
            backend: "avx2".to_owned(),
            operation: "dot".to_owned(),
            case_name: None,
            length: 1025,
            index: None,
            actual: 10.0,
            expected: 11.0,
            error: 1.0,
            tolerance: 0.01,
        });

        assert!(!report.passed());
        assert_eq!(report.status(), "fail");
    }

    #[test]
    fn json_contains_schema_and_status() {
        let json = passing_report().to_json();

        assert!(json.contains("\"schema_version\":1"));
        assert!(json.contains("\"status\":\"pass\""));
        assert!(json.contains("\"failures\":[]"));
    }

    #[test]
    fn json_contains_length_cases() {
        let json = passing_report().to_json();

        assert!(json.contains("\"total\":4"));
        assert!(json.contains("\"completed\":4"));
        assert!(json.contains("\"cases\":[0,1,7,8]"));
    }

    #[test]
    fn json_escapes_strings() {
        let mut report = passing_report();

        report.backends = vec!["quote\"slash\\newline\n".to_owned()];

        let json = report.to_json();

        assert!(json.contains("quote\\\"slash\\\\newline\\n"));
    }

    #[test]
    fn non_finite_values_are_valid_json_strings() {
        let mut report = passing_report();

        report.dot = CheckStatus::Fail;
        report.failures.push(FailureRecord {
            backend: "scalar".to_owned(),
            operation: "dot".to_owned(),
            case_name: Some("non-finite".to_owned()),
            length: 1,
            index: None,
            actual: f32::NAN,
            expected: f64::INFINITY,
            error: f64::NEG_INFINITY,
            tolerance: 0.0,
        });

        let json = report.to_json();

        assert!(json.contains("\"actual\":\"NaN\""));
        assert!(json.contains("\"expected\":\"Infinity\""));
        assert!(json.contains("\"error\":\"-Infinity\""));
    }
}
