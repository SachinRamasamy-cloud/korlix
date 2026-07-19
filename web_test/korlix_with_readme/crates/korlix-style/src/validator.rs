use crate::registry::{lookup, suggest};
use crate::variants::parse_variant;
use korlix_core::DiagnosticSet;

/// Validate a set of classes used in a module, emit warnings for unknowns.
pub fn validate_classes(classes: &std::collections::HashSet<String>, diag: &mut DiagnosticSet) {
    for class in classes {
        if !is_valid(class) {
            let suggestions = suggest(class);
            let hint = if suggestions.is_empty() {
                "Check the Korlix utility class docs.".to_string()
            } else {
                format!("Did you mean: {}?", suggestions.join(", "))
            };
            diag.push(
                korlix_core::Diagnostic::warning(
                    "KX-E201",
                    format!("Unknown utility class `.{}`", class),
                )
                .with_hint(hint),
            );
        }
    }
}

fn is_valid(class: &str) -> bool {
    // direct lookup
    if lookup(class).is_some() {
        return true;
    }

    // variant: sm:flex, hover:bg-red-500, dark:text-white …
    if let Some((_prefix, base)) = parse_variant(class) {
        return is_valid(base);
    }

    // arbitrary values: w-[320px], bg-[#ff0000]
    if class.contains('[') && class.ends_with(']') {
        return true;
    }

    false
}
