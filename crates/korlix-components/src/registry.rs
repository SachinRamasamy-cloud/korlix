use crate::schema::{ComponentCategory, ComponentSchema, PropSchema, RuntimeFeature, SlotSchema};
use indexmap::IndexMap;
use once_cell::sync::Lazy;

pub static COMPONENT_REGISTRY: Lazy<IndexMap<String, ComponentSchema>> = Lazy::new(build_registry);

fn prop(
    name: &str,
    type_ann: &str,
    required: bool,
    default: Option<&str>,
    desc: &str,
) -> PropSchema {
    PropSchema {
        name: name.into(),
        type_ann: type_ann.into(),
        required,
        default: default.map(|s| s.into()),
        description: desc.into(),
    }
}
fn slot(name: Option<&str>, desc: &str) -> SlotSchema {
    SlotSchema {
        name: name.map(|s| s.into()),
        description: desc.into(),
    }
}
#[allow(dead_code)]
fn schema(name: &str, cat: ComponentCategory, tag: &str, desc: &str) -> ComponentSchema {
    ComponentSchema {
        name: name.into(),
        category: cat,
        html_tag: tag.into(),
        self_closing: false,
        props: vec![],
        slots: vec![],
        default_classes: vec![],
        runtime_features: vec![],
        aria_role: None,
        description: desc.into(),
    }
}

fn build_registry() -> IndexMap<String, ComponentSchema> {
    let mut m = IndexMap::new();

    macro_rules! reg {
        ($s:expr) => {
            m.insert($s.name.clone(), $s);
        };
    }

    // ── Primitives ──────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "btn".into(),
        category: ComponentCategory::Primitive,
        html_tag: "button".into(),
        self_closing: false,
        aria_role: Some("button".into()),
        props: vec![
            prop(
                "variant",
                "string",
                false,
                Some("default"),
                "Visual variant: primary|secondary|ghost|danger"
            ),
            prop("size", "string", false, Some("md"), "Size: sm|md|lg"),
            prop("disabled", "bool", false, Some("false"), "Disabled state"),
            prop("loading", "bool", false, Some("false"), "Loading spinner"),
            prop("type", "string", false, Some("button"), "HTML button type"),
        ],
        slots: vec![slot(None, "Button label / content")],
        default_classes: vec!["kx-btn".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Interactive button with variants and loading state".into(),
    });

    reg!(ComponentSchema {
        name: "button".into(),
        category: ComponentCategory::Primitive,
        html_tag: "button".into(),
        self_closing: false,
        aria_role: Some("button".into()),
        props: vec![
            prop(
                "variant",
                "string",
                false,
                Some("default"),
                "Visual variant"
            ),
            prop("disabled", "bool", false, Some("false"), "Disabled state"),
        ],
        slots: vec![slot(None, "Button label")],
        default_classes: vec!["kx-btn".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Button element alias".into(),
    });

    reg!(ComponentSchema {
        name: "link".into(),
        category: ComponentCategory::Primitive,
        html_tag: "a".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop("href", "string", true, None, "Destination URL"),
            prop("external", "bool", false, Some("false"), "Opens in new tab"),
            prop("active", "bool", false, Some("false"), "Active route state"),
        ],
        slots: vec![slot(None, "Link text")],
        default_classes: vec![],
        runtime_features: vec![RuntimeFeature::Router],
        description: "SPA-aware anchor link".into(),
    });

    reg!(ComponentSchema {
        name: "icon".into(),
        category: ComponentCategory::Icon,
        html_tag: "span".into(),
        self_closing: true,
        aria_role: Some("img".into()),
        props: vec![
            prop("name", "string", true, None, "Icon identifier"),
            prop("size", "string", false, Some("md"), "Size: xs|sm|md|lg|xl"),
            prop(
                "label",
                "string",
                false,
                None,
                "Accessible label (aria-label)"
            ),
            prop(
                "decorative",
                "bool",
                false,
                Some("false"),
                "Hide from screen readers"
            ),
        ],
        slots: vec![],
        default_classes: vec!["kx-icon".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "SVG icon with accessibility support".into(),
    });

    reg!(ComponentSchema {
        name: "image".into(),
        category: ComponentCategory::Media,
        html_tag: "img".into(),
        self_closing: true,
        aria_role: None,
        props: vec![
            prop("src", "string", true, None, "Image source URL"),
            prop(
                "alt",
                "string",
                true,
                None,
                "Alt text (required for accessibility)"
            ),
            prop("lazy", "bool", false, Some("true"), "Lazy load the image"),
            prop("placeholder", "string", false, None, "blur|skeleton|none"),
            prop("width", "number", false, None, "Width in pixels"),
            prop("height", "number", false, None, "Height in pixels"),
            prop("fit", "string", false, Some("cover"), "object-fit value"),
        ],
        slots: vec![],
        default_classes: vec!["kx-image".into()],
        runtime_features: vec![RuntimeFeature::Media],
        description: "Responsive image with lazy loading".into(),
    });

    reg!(ComponentSchema {
        name: "avatar".into(),
        category: ComponentCategory::Avatar,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop("src", "string", false, None, "Avatar image URL"),
            prop("name", "string", false, None, "Name for initials fallback"),
            prop(
                "size",
                "string",
                false,
                Some("md"),
                "Size: xs|sm|md|lg|xl|2xl"
            ),
            prop("status", "string", false, None, "online|offline|busy|away"),
            prop("shape", "string", false, Some("circle"), "circle|square"),
        ],
        slots: vec![],
        default_classes: vec!["kx-avatar".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "User avatar with image, initials, and status indicator".into(),
    });

    reg!(ComponentSchema {
        name: "card".into(),
        category: ComponentCategory::Content,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop(
                "variant",
                "string",
                false,
                Some("default"),
                "default|outline|elevated"
            ),
            prop(
                "clickable",
                "bool",
                false,
                Some("false"),
                "Adds hover/focus styles"
            ),
        ],
        slots: vec![slot(None, "Card content")],
        default_classes: vec!["kx-card".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Surface container card".into(),
    });

    reg!(ComponentSchema {
        name: "navbar".into(),
        category: ComponentCategory::Navigation,
        html_tag: "nav".into(),
        self_closing: false,
        aria_role: Some("navigation".into()),
        props: vec![
            prop("sticky", "bool", false, Some("false"), "Sticky positioning"),
            prop(
                "transparent",
                "bool",
                false,
                Some("false"),
                "Transparent background"
            ),
        ],
        slots: vec![
            slot(None, "Nav content"),
            slot(Some("end"), "Right-side items")
        ],
        default_classes: vec!["kx-navbar".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Top navigation bar".into(),
    });

    reg!(ComponentSchema {
        name: "footer".into(),
        category: ComponentCategory::Navigation,
        html_tag: "footer".into(),
        self_closing: false,
        aria_role: None,
        props: vec![],
        slots: vec![slot(None, "Footer content")],
        default_classes: vec!["kx-footer".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Page footer container".into(),
    });

    reg!(ComponentSchema {
        name: "sidebar".into(),
        category: ComponentCategory::Navigation,
        html_tag: "aside".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop("collapsed", "bool", false, Some("false"), "Collapsed state"),
            prop("width", "string", false, Some("240px"), "Sidebar width"),
        ],
        slots: vec![slot(None, "Sidebar content")],
        default_classes: vec!["kx-sidebar".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Collapsible sidebar navigation".into(),
    });

    // ── Feedback ────────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "toast".into(),
        category: ComponentCategory::Feedback,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: Some("alert".into()),
        props: vec![
            prop(
                "type",
                "string",
                false,
                Some("info"),
                "success|error|warning|info|loading"
            ),
            prop("message", "string", true, None, "Toast message"),
            prop(
                "duration",
                "number",
                false,
                Some("3000"),
                "Auto dismiss ms (0=manual)"
            ),
            prop(
                "position",
                "string",
                false,
                Some("top-right"),
                "Toast position"
            ),
        ],
        slots: vec![],
        default_classes: vec!["kx-toast".into()],
        runtime_features: vec![RuntimeFeature::Toast],
        description: "Notification toast message".into(),
    });

    reg!(ComponentSchema {
        name: "alert".into(),
        category: ComponentCategory::Feedback,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: Some("alert".into()),
        props: vec![
            prop(
                "type",
                "string",
                false,
                Some("info"),
                "success|error|warning|info"
            ),
            prop("title", "string", false, None, "Alert title"),
            prop(
                "dismissible",
                "bool",
                false,
                Some("false"),
                "Show close button"
            ),
        ],
        slots: vec![slot(None, "Alert content")],
        default_classes: vec!["kx-alert".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Inline alert/banner message".into(),
    });

    reg!(ComponentSchema {
        name: "badge".into(),
        category: ComponentCategory::Content,
        html_tag: "span".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop(
                "variant",
                "string",
                false,
                Some("default"),
                "default|primary|success|danger|warning"
            ),
            prop("size", "string", false, Some("md"), "sm|md|lg"),
        ],
        slots: vec![slot(None, "Badge text")],
        default_classes: vec!["kx-badge".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Status badge / pill".into(),
    });

    // ── Loaders ─────────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "spinner".into(),
        category: ComponentCategory::Loader,
        html_tag: "div".into(),
        self_closing: true,
        aria_role: Some("status".into()),
        props: vec![
            prop("size", "string", false, Some("md"), "xs|sm|md|lg|xl"),
            prop("color", "string", false, Some("primary"), "Color token"),
        ],
        slots: vec![],
        default_classes: vec!["kx-spinner".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Circular loading spinner".into(),
    });

    reg!(ComponentSchema {
        name: "skeleton".into(),
        category: ComponentCategory::Loader,
        html_tag: "div".into(),
        self_closing: true,
        aria_role: None,
        props: vec![
            prop("width", "string", false, Some("100%"), "Width"),
            prop("height", "string", false, Some("1rem"), "Height"),
            prop("rounded", "bool", false, Some("false"), "Circular skeleton"),
        ],
        slots: vec![],
        default_classes: vec!["kx-skeleton".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Loading placeholder skeleton".into(),
    });

    reg!(ComponentSchema {
        name: "skeleton-card".into(),
        category: ComponentCategory::Loader,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop(
                "count",
                "number",
                false,
                Some("1"),
                "Number of skeleton cards"
            ),
            prop("lines", "number", false, Some("3"), "Lines per card"),
        ],
        slots: vec![],
        default_classes: vec!["kx-skeleton-card".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Card-shaped loading skeleton".into(),
    });

    // ── Placeholders ────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "empty-state".into(),
        category: ComponentCategory::Placeholder,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop("icon", "string", false, None, "Icon name"),
            prop("title", "string", false, Some("Nothing here"), "Title text"),
            prop("description", "string", false, None, "Description text"),
        ],
        slots: vec![slot(Some("actions"), "Action buttons")],
        default_classes: vec!["kx-empty-state".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Empty state placeholder".into(),
    });

    // ── Overlay ─────────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "modal".into(),
        category: ComponentCategory::Overlay,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: Some("dialog".into()),
        props: vec![
            prop("id", "string", true, None, "Unique modal ID"),
            prop("title", "string", false, None, "Modal title"),
            prop("size", "string", false, Some("md"), "sm|md|lg|xl|full"),
            prop("closable", "bool", false, Some("true"), "Show close button"),
        ],
        slots: vec![
            slot(None, "Modal content"),
            slot(Some("footer"), "Modal footer with actions"),
        ],
        default_classes: vec!["kx-modal".into()],
        runtime_features: vec![RuntimeFeature::Overlay],
        description: "Accessible dialog modal with focus trap".into(),
    });

    reg!(ComponentSchema {
        name: "drawer".into(),
        category: ComponentCategory::Overlay,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: Some("dialog".into()),
        props: vec![
            prop("id", "string", true, None, "Drawer ID"),
            prop("side", "string", false, Some("right"), "right|left"),
            prop("title", "string", false, None, "Drawer title"),
        ],
        slots: vec![slot(None, "Drawer content")],
        default_classes: vec!["kx-drawer".into()],
        runtime_features: vec![RuntimeFeature::Overlay],
        description: "Slide-in drawer panel".into(),
    });

    reg!(ComponentSchema {
        name: "tooltip".into(),
        category: ComponentCategory::Overlay,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: Some("tooltip".into()),
        props: vec![
            prop("content", "string", true, None, "Tooltip text"),
            prop(
                "placement",
                "string",
                false,
                Some("top"),
                "top|bottom|left|right"
            ),
        ],
        slots: vec![slot(None, "Tooltip trigger")],
        default_classes: vec!["kx-tooltip".into()],
        runtime_features: vec![RuntimeFeature::Overlay],
        description: "Hover tooltip".into(),
    });

    // ── Pagination ───────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "pagination".into(),
        category: ComponentCategory::Navigation,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: Some("navigation".into()),
        props: vec![
            prop("page", "int", true, None, "Current page (1-based)"),
            prop("total", "int", true, None, "Total items"),
            prop("perPage", "int", false, Some("10"), "Items per page"),
            prop(
                "siblings",
                "int",
                false,
                Some("1"),
                "Pages on each side of current"
            ),
        ],
        slots: vec![],
        default_classes: vec!["kx-pagination".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Page navigation control".into(),
    });

    // ── Forms ────────────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "input".into(),
        category: ComponentCategory::Form,
        html_tag: "input".into(),
        self_closing: true,
        aria_role: None,
        props: vec![
            prop("type", "string", false, Some("text"), "HTML input type"),
            prop("placeholder", "string", false, None, "Placeholder text"),
            prop("value", "string", false, None, "Bound value"),
            prop("disabled", "bool", false, Some("false"), "Disabled state"),
            prop("error", "string", false, None, "Error message"),
            prop("label", "string", false, None, "Associated label"),
        ],
        slots: vec![],
        default_classes: vec!["kx-input".into()],
        runtime_features: vec![RuntimeFeature::Forms],
        description: "Text input field".into(),
    });

    reg!(ComponentSchema {
        name: "select".into(),
        category: ComponentCategory::Form,
        html_tag: "select".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop(
                "options",
                "list<record>",
                true,
                None,
                "Option items [{label, value}]"
            ),
            prop("value", "string", false, None, "Selected value"),
            prop("placeholder", "string", false, None, "Placeholder option"),
            prop("disabled", "bool", false, Some("false"), "Disabled state"),
        ],
        slots: vec![],
        default_classes: vec!["kx-select".into()],
        runtime_features: vec![RuntimeFeature::Forms],
        description: "Select dropdown".into(),
    });

    reg!(ComponentSchema {
        name: "textarea".into(),
        category: ComponentCategory::Form,
        html_tag: "textarea".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop("rows", "int", false, Some("4"), "Visible rows"),
            prop("placeholder", "string", false, None, "Placeholder text"),
            prop("value", "string", false, None, "Bound value"),
        ],
        slots: vec![],
        default_classes: vec!["kx-textarea".into()],
        runtime_features: vec![RuntimeFeature::Forms],
        description: "Multi-line text input".into(),
    });

    reg!(ComponentSchema {
        name: "checkbox".into(),
        category: ComponentCategory::Form,
        html_tag: "label".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop("checked", "bool", false, Some("false"), "Checked state"),
            prop("label", "string", false, None, "Checkbox label"),
            prop("disabled", "bool", false, Some("false"), "Disabled"),
        ],
        slots: vec![],
        default_classes: vec!["kx-checkbox".into()],
        runtime_features: vec![RuntimeFeature::Forms],
        description: "Checkbox input".into(),
    });

    reg!(ComponentSchema {
        name: "switch".into(),
        category: ComponentCategory::Form,
        html_tag: "label".into(),
        self_closing: false,
        aria_role: Some("switch".into()),
        props: vec![
            prop("checked", "bool", false, Some("false"), "On/off state"),
            prop("label", "string", false, None, "Switch label"),
            prop("disabled", "bool", false, Some("false"), "Disabled"),
        ],
        slots: vec![],
        default_classes: vec!["kx-switch".into()],
        runtime_features: vec![RuntimeFeature::Forms],
        description: "Toggle switch".into(),
    });

    // ── Content ──────────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "accordion".into(),
        category: ComponentCategory::Content,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop(
                "multiple",
                "bool",
                false,
                Some("false"),
                "Allow multiple open"
            ),
            prop(
                "default-open",
                "string",
                false,
                None,
                "ID of panel open by default"
            ),
        ],
        slots: vec![slot(None, "Accordion panels")],
        default_classes: vec!["kx-accordion".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Collapsible accordion panels".into(),
    });

    reg!(ComponentSchema {
        name: "tabs".into(),
        category: ComponentCategory::Navigation,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: Some("tablist".into()),
        props: vec![
            prop("active", "string", false, None, "Active tab ID"),
            prop("variant", "string", false, Some("line"), "line|pills|boxed"),
        ],
        slots: vec![slot(None, "Tab panels")],
        default_classes: vec!["kx-tabs".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Tab navigation".into(),
    });

    reg!(ComponentSchema {
        name: "table".into(),
        category: ComponentCategory::DataDisplay,
        html_tag: "table".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop(
                "striped",
                "bool",
                false,
                Some("false"),
                "Alternating row colors"
            ),
            prop(
                "hoverable",
                "bool",
                false,
                Some("true"),
                "Row hover highlight"
            ),
        ],
        slots: vec![slot(None, "Table content")],
        default_classes: vec!["kx-table".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Data table".into(),
    });

    // ── Marketing ────────────────────────────────────────────────────
    reg!(ComponentSchema {
        name: "hero".into(),
        category: ComponentCategory::Marketing,
        html_tag: "section".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop(
                "variant",
                "string",
                false,
                Some("centered"),
                "centered|split|full"
            ),
            prop("size", "string", false, Some("lg"), "sm|md|lg|xl"),
        ],
        slots: vec![
            slot(None, "Hero content"),
            slot(Some("actions"), "CTA buttons"),
            slot(Some("image"), "Hero image"),
        ],
        default_classes: vec!["kx-hero".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Hero section".into(),
    });

    reg!(ComponentSchema {
        name: "progress".into(),
        category: ComponentCategory::Loader,
        html_tag: "div".into(),
        self_closing: true,
        aria_role: Some("progressbar".into()),
        props: vec![
            prop("value", "number", false, Some("0"), "Progress 0-100"),
            prop("max", "number", false, Some("100"), "Maximum value"),
            prop("variant", "string", false, Some("primary"), "Color variant"),
            prop("size", "string", false, Some("md"), "sm|md|lg"),
            prop(
                "label",
                "bool",
                false,
                Some("false"),
                "Show percentage label"
            ),
        ],
        slots: vec![],
        default_classes: vec!["kx-progress".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Progress bar".into(),
    });

    reg!(ComponentSchema {
        name: "profile-card".into(),
        category: ComponentCategory::Avatar,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: None,
        props: vec![
            prop("name", "string", true, None, "User name"),
            prop("avatar", "string", false, None, "Avatar URL"),
            prop("role", "string", false, None, "Role/subtitle"),
            prop("bio", "string", false, None, "Bio text"),
        ],
        slots: vec![slot(Some("actions"), "Action buttons")],
        default_classes: vec!["kx-profile-card".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "User profile card".into(),
    });

    reg!(ComponentSchema {
        name: "breadcrumb".into(),
        category: ComponentCategory::Navigation,
        html_tag: "nav".into(),
        self_closing: false,
        aria_role: Some("navigation".into()),
        props: vec![
            prop(
                "items",
                "list<record>",
                true,
                None,
                "Breadcrumb items [{label, href}]"
            ),
            prop(
                "separator",
                "string",
                false,
                Some("/"),
                "Separator character"
            ),
        ],
        slots: vec![],
        default_classes: vec!["kx-breadcrumb".into()],
        runtime_features: vec![RuntimeFeature::Core],
        description: "Breadcrumb navigation".into(),
    });

    reg!(ComponentSchema {
        name: "section".into(),
        category: ComponentCategory::Primitive,
        html_tag: "section".into(),
        self_closing: false,
        aria_role: None,
        props: vec![],
        slots: vec![slot(None, "Section content")],
        default_classes: vec![],
        runtime_features: vec![],
        description: "HTML section wrapper".into(),
    });

    reg!(ComponentSchema {
        name: "container".into(),
        category: ComponentCategory::Primitive,
        html_tag: "div".into(),
        self_closing: false,
        aria_role: None,
        props: vec![prop(
            "size",
            "string",
            false,
            Some("lg"),
            "sm|md|lg|xl|2xl|full"
        ),],
        slots: vec![slot(None, "Container content")],
        default_classes: vec!["kx-container".into()],
        runtime_features: vec![],
        description: "Centered max-width container".into(),
    });

    m
}

pub fn get_component(name: &str) -> Option<&'static ComponentSchema> {
    COMPONENT_REGISTRY.get(name)
}

pub fn is_component(name: &str) -> bool {
    COMPONENT_REGISTRY.contains_key(name)
}

pub fn all_component_names() -> Vec<&'static str> {
    COMPONENT_REGISTRY.keys().map(|s| s.as_str()).collect()
}
