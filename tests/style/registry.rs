//! Style registry tests

#[cfg(test)]
mod tests {
    use korlix_style::registry::{all_class_names, lookup, suggest};

    #[test]
    fn test_lookup_flex() {
        let rule = lookup("flex").expect("flex should exist");
        assert!(rule
            .declarations
            .iter()
            .any(|(k, v)| k == "display" && v == "flex"));
    }

    #[test]
    fn test_lookup_bg_primary() {
        let rule = lookup("bg-primary").expect("bg-primary should exist");
        assert!(rule
            .declarations
            .iter()
            .any(|(k, _)| k == "background-color"));
    }

    #[test]
    fn test_lookup_text_blue_500() {
        let rule = lookup("text-blue-500").expect("text-blue-500 should exist");
        assert!(rule
            .declarations
            .iter()
            .any(|(k, v)| k == "color" && v.contains('#')));
    }

    #[test]
    fn test_spacing_classes() {
        for cls in ["p-4", "px-6", "py-3", "mt-8", "mb-4", "gap-6"] {
            assert!(lookup(cls).is_some(), "Missing class: {}", cls);
        }
    }

    #[test]
    fn test_typography_classes() {
        for cls in [
            "text-xl",
            "text-2xl",
            "font-bold",
            "font-semibold",
            "text-center",
            "uppercase",
        ] {
            assert!(lookup(cls).is_some(), "Missing class: {}", cls);
        }
    }

    #[test]
    fn test_border_radius() {
        for cls in [
            "rounded",
            "rounded-sm",
            "rounded-lg",
            "rounded-xl",
            "rounded-full",
        ] {
            assert!(lookup(cls).is_some(), "Missing class: {}", cls);
        }
    }

    #[test]
    fn test_color_palette_exists() {
        for color in [
            "slate-500",
            "blue-500",
            "red-500",
            "green-500",
            "purple-500",
            "pink-500",
        ] {
            assert!(
                lookup(&format!("bg-{}", color)).is_some(),
                "Missing bg-{}",
                color
            );
            assert!(
                lookup(&format!("text-{}", color)).is_some(),
                "Missing text-{}",
                color
            );
        }
    }

    #[test]
    fn test_suggest_typo() {
        let suggestions = suggest("bg-blu");
        assert!(
            !suggestions.is_empty(),
            "Should suggest something for bg-blu"
        );
    }

    #[test]
    fn test_grid_cols() {
        for n in 1..=12 {
            let cls = format!("grid-cols-{}", n);
            assert!(lookup(&cls).is_some(), "Missing {}", cls);
        }
    }

    #[test]
    fn test_sr_only() {
        let rule = lookup("sr-only").expect("sr-only should exist");
        assert!(rule
            .declarations
            .iter()
            .any(|(k, v)| k == "position" && v == "absolute"));
    }

    #[test]
    fn test_total_class_count() {
        let names = all_class_names();
        assert!(
            names.len() > 1000,
            "Registry should have 1000+ classes, got {}",
            names.len()
        );
    }
}
