use crate::span::Span;
use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub span: Option<Span>,
    pub hint: Option<String>,
    pub notes: Vec<String>,
}

impl Diagnostic {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: code.into(),
            message: message.into(),
            span: None,
            hint: None,
            notes: vec![],
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code: code.into(),
            message: message.into(),
            span: None,
            hint: None,
            notes: vec![],
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }

    pub fn print(&self, source: Option<&str>) {
        let prefix = match self.severity {
            Severity::Error   => "error".red().bold(),
            Severity::Warning => "warning".yellow().bold(),
            Severity::Info    => "info".cyan().bold(),
            Severity::Hint    => "hint".blue().bold(),
        };

        let code = format!("[{}]", self.code).dimmed();
        println!("{} {}: {}", prefix, code, self.message.white().bold());

        if let Some(span) = &self.span {
            println!("  {} {}:{}", "→".dimmed(), span.start.line, span.start.col);
        }

        if let Some(src_line) = source {
            println!("  {}", src_line.dimmed());
            if let Some(span) = &self.span {
                let col = span.start.col.saturating_sub(1);
                let len = (span.end.col.saturating_sub(span.start.col)).max(1);
                let arrows = "^".repeat(len).red().bold();
                println!("  {}{}", " ".repeat(col), arrows);
            }
        }

        if let Some(hint) = &self.hint {
            println!("  {} {}", "hint:".green().bold(), hint);
        }

        for note in &self.notes {
            println!("  {} {}", "note:".blue().bold(), note);
        }
    }
}

#[derive(Debug, Default)]
pub struct DiagnosticSet {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, d: Diagnostic) {
        self.diagnostics.push(d);
    }

    pub fn error(&mut self, code: impl Into<String>, msg: impl Into<String>) {
        self.push(Diagnostic::error(code, msg));
    }

    pub fn warning(&mut self, code: impl Into<String>, msg: impl Into<String>) {
        self.push(Diagnostic::warning(code, msg));
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.is_error())
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.is_error()).count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .count()
    }

    pub fn print_all(&self) {
        for d in &self.diagnostics {
            d.print(None);
            println!();
        }
    }
}
