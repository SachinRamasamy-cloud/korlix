//! Component registry tests

#[cfg(test)]
mod tests {
    use korlix_components::registry::{get_component, is_component, all_component_names};

    #[test]
    fn test_btn_registered() {
        let c = get_component("btn").expect("btn should be registered");
        assert_eq!(c.html_tag, "button");
        assert_eq!(c.aria_role.as_deref(), Some("button"));
    }

    #[test]
    fn test_image_registered() {
        let c = get_component("image").expect("image should be registered");
        assert_eq!(c.html_tag, "img");
        assert!(c.self_closing);
    }

    #[test]
    fn test_avatar_registered() {
        let c = get_component("avatar").expect("avatar should be registered");
        assert!(c.props.iter().any(|p| p.name == "name"));
        assert!(c.props.iter().any(|p| p.name == "src"));
        assert!(c.props.iter().any(|p| p.name == "size"));
    }

    #[test]
    fn test_toast_has_runtime_feature() {
        use korlix_components::schema::RuntimeFeature;
        let c = get_component("toast").expect("toast should be registered");
        assert!(c.runtime_features.contains(&RuntimeFeature::Toast));
    }

    #[test]
    fn test_modal_has_overlay_feature() {
        use korlix_components::schema::RuntimeFeature;
        let c = get_component("modal").expect("modal should be registered");
        assert!(c.runtime_features.contains(&RuntimeFeature::Overlay));
    }

    #[test]
    fn test_pagination_registered() {
        let c = get_component("pagination").expect("pagination should be registered");
        assert!(c.props.iter().any(|p| p.name == "page"));
        assert!(c.props.iter().any(|p| p.name == "total"));
        assert!(c.props.iter().any(|p| p.name == "perPage"));
    }

    #[test]
    fn test_minimum_component_count() {
        let names = all_component_names();
        assert!(names.len() >= 30, "Should have 30+ components, got {}", names.len());
    }

    #[test]
    fn test_is_component() {
        assert!(is_component("btn"));
        assert!(is_component("modal"));
        assert!(is_component("avatar"));
        assert!(!is_component("div"));
        assert!(!is_component("span"));
        assert!(!is_component("h1"));
    }

    #[test]
    fn test_forms_registered() {
        for name in ["input", "select", "textarea", "checkbox", "switch"] {
            assert!(get_component(name).is_some(), "{} should be registered", name);
        }
    }
}
