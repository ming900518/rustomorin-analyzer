#![expect(clippy::cast_possible_truncation)]
use proc_macro2::Span;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::LSP_NAME;

#[non_exhaustive]
pub enum Lints {
    Unwrap(Span),
    Unsafe(Span),
    Panic(Span),
    TryExpr(Span),
    Static(Span),
}

impl From<Lints> for Option<Diagnostic> {
    fn from(value: Lints) -> Self {
        match value {
            Lints::Unwrap(span) => Some(Diagnostic {
                range: {
                    Range {
                        start: Position {
                            line: span.start().line as u32 - 1,
                            character: span.start().column as u32,
                        },
                        end: Position {
                            line: span.end().line as u32 - 1,
                            character: span.end().column as u32,
                        },
                    }
                },
                severity: Some(DiagnosticSeverity::WARNING),
                message: "「那傢伙竟敢無視燈？」\n立希不希望各位使用 `unwrap` method 來無視錯誤，請嘗試改用 `expect` method".to_string(),
                source: Some(LSP_NAME.to_owned()),
                ..Default::default()
            }),
            Lints::Unsafe(span) => Some(Diagnostic {
                range: {
                    Range {
                        start: Position {
                            line: span.start().line as u32 - 1,
                            character: span.start().column as u32,
                        },
                        end: Position {
                            line: span.end().line as u32 - 1,
                            character: span.end().column as u32,
                        },
                    }
                },
                severity: Some(DiagnosticSeverity::INFORMATION),
                message: "「妳是抱著多大的覺悟說出這種話的？」\n祥子提醒你再次思考一下使用 `unsafe` 的必要性".to_string(),
                source: Some(LSP_NAME.to_owned()),
                ..Default::default()
            }),
            Lints::Panic(span) => Some(Diagnostic {
                range: {
                    Range {
                        start: Position {
                            line: span.start().line as u32 - 1,
                            character: span.start().column as u32,
                        },
                        end: Position {
                            line: span.end().line as u32 - 1,
                            character: span.end().column as u32,
                        },
                    }
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: "「為什麼要演奏春日影！」\n超級不爽世不允許任何人使用 `panic!` macro".to_string(),
                source: Some(LSP_NAME.to_owned()),
                ..Default::default()
            }),
            Lints::TryExpr(span) => Some(Diagnostic {
                range: {
                    Range {
                        start: Position {
                            line: span.start().line as u32 - 1,
                            character: span.start().column as u32,
                        },
                        end: Position {
                            line: span.end().line as u32 - 1,
                            character: span.end().column as u32,
                        },
                    }
                },
                severity: Some(DiagnosticSeverity::INFORMATION),
                message: "「蛤？」\n愛音發現此處使用了 try expression".to_string(),
                source: Some(LSP_NAME.to_owned()),
                ..Default::default()
            }),
            Lints::Static(span) => Some(Diagnostic {
                range: {
                    Range {
                        start: Position {
                            line: span.start().line as u32 - 1,
                            character: span.start().column as u32,
                        },
                        end: Position {
                            line: span.end().line as u32 - 1,
                            character: span.end().column as u32,
                        },
                    }
                },
                severity: Some(DiagnosticSeverity::INFORMATION),
                message: "「一輩子跟我一起組樂團嗎」\n燈指出這個 reference 被指定了 `static` lifetime".to_string(),
                source: Some(LSP_NAME.to_owned()),
                ..Default::default()
            }),
        }
    }
}
