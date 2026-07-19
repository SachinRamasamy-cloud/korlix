//! Design tokens: spacing, radius, shadows, breakpoints.

pub const SPACING: &[(&str, &str)] = &[
    ("0", "0px"),
    ("px", "1px"),
    ("0.5", "2px"),
    ("1", "4px"),
    ("1.5", "6px"),
    ("2", "8px"),
    ("2.5", "10px"),
    ("3", "12px"),
    ("3.5", "14px"),
    ("4", "16px"),
    ("5", "20px"),
    ("6", "24px"),
    ("7", "28px"),
    ("8", "32px"),
    ("9", "36px"),
    ("10", "40px"),
    ("11", "44px"),
    ("12", "48px"),
    ("14", "56px"),
    ("16", "64px"),
    ("20", "80px"),
    ("24", "96px"),
    ("28", "112px"),
    ("32", "128px"),
    ("36", "144px"),
    ("40", "160px"),
    ("44", "176px"),
    ("48", "192px"),
    ("52", "208px"),
    ("56", "224px"),
    ("60", "240px"),
    ("64", "256px"),
    ("72", "288px"),
    ("80", "320px"),
    ("96", "384px"),
];

pub const RADIUS: &[(&str, &str)] = &[
    ("none", "0px"),
    ("sm", "2px"),
    ("", "4px"),
    ("md", "6px"),
    ("lg", "8px"),
    ("xl", "12px"),
    ("2xl", "16px"),
    ("3xl", "24px"),
    ("full", "9999px"),
];

pub const SHADOWS: &[(&str, &str)] = &[
    ("sm", "0 1px 2px 0 rgba(0,0,0,.05)"),
    (
        "",
        "0 1px 3px 0 rgba(0,0,0,.1),0 1px 2px -1px rgba(0,0,0,.1)",
    ),
    (
        "md",
        "0 4px 6px -1px rgba(0,0,0,.1),0 2px 4px -2px rgba(0,0,0,.1)",
    ),
    (
        "lg",
        "0 10px 15px -3px rgba(0,0,0,.1),0 4px 6px -4px rgba(0,0,0,.1)",
    ),
    (
        "xl",
        "0 20px 25px -5px rgba(0,0,0,.1),0 8px 10px -6px rgba(0,0,0,.1)",
    ),
    ("2xl", "0 25px 50px -12px rgba(0,0,0,.25)"),
    ("inner", "inset 0 2px 4px 0 rgba(0,0,0,.05)"),
    ("none", "0 0 #0000"),
];

pub const BREAKPOINTS: &[(&str, u32)] = &[
    ("sm", 576),
    ("md", 768),
    ("lg", 992),
    ("xl", 1200),
    ("2xl", 1400),
];

pub const FONT_SIZES: &[(&str, &str, &str)] = &[
    // (name, font-size, line-height)
    ("xs", "0.75rem", "1rem"),
    ("sm", "0.875rem", "1.25rem"),
    ("base", "1rem", "1.5rem"),
    ("lg", "1.125rem", "1.75rem"),
    ("xl", "1.25rem", "1.75rem"),
    ("2xl", "1.5rem", "2rem"),
    ("3xl", "1.875rem", "2.25rem"),
    ("4xl", "2.25rem", "2.5rem"),
    ("5xl", "3rem", "1"),
    ("6xl", "3.75rem", "1"),
    ("7xl", "4.5rem", "1"),
    ("8xl", "6rem", "1"),
    ("9xl", "8rem", "1"),
];

pub const FONT_WEIGHTS: &[(&str, &str)] = &[
    ("thin", "100"),
    ("extralight", "200"),
    ("light", "300"),
    ("normal", "400"),
    ("medium", "500"),
    ("semibold", "600"),
    ("bold", "700"),
    ("extrabold", "800"),
    ("black", "900"),
];

pub const OPACITY: &[(&str, &str)] = &[
    ("0", "0"),
    ("5", "0.05"),
    ("10", "0.1"),
    ("20", "0.2"),
    ("25", "0.25"),
    ("30", "0.3"),
    ("40", "0.4"),
    ("50", "0.5"),
    ("60", "0.6"),
    ("70", "0.7"),
    ("75", "0.75"),
    ("80", "0.8"),
    ("90", "0.9"),
    ("95", "0.95"),
    ("100", "1"),
];

pub const Z_INDEX: &[(&str, &str)] = &[
    ("0", "0"),
    ("10", "10"),
    ("20", "20"),
    ("30", "30"),
    ("40", "40"),
    ("50", "50"),
    ("auto", "auto"),
];

pub const TRANSITIONS: &[(&str, &str)] = &[
    ("none",     "none"),
    ("all",      "all 150ms cubic-bezier(0.4,0,0.2,1)"),
    ("",         "color,background-color,border-color,outline-color,text-decoration-color,fill,stroke,opacity,box-shadow,transform,filter,backdrop-filter 150ms cubic-bezier(0.4,0,0.2,1)"),
    ("colors",   "color,background-color,border-color,outline-color,text-decoration-color,fill,stroke 150ms cubic-bezier(0.4,0,0.2,1)"),
    ("opacity",  "opacity 150ms cubic-bezier(0.4,0,0.2,1)"),
    ("shadow",   "box-shadow 150ms cubic-bezier(0.4,0,0.2,1)"),
    ("transform","transform 150ms cubic-bezier(0.4,0,0.2,1)"),
];

pub const DURATIONS: &[(&str, &str)] = &[
    ("0", "0s"),
    ("75", "75ms"),
    ("100", "100ms"),
    ("150", "150ms"),
    ("200", "200ms"),
    ("300", "300ms"),
    ("500", "500ms"),
    ("700", "700ms"),
    ("1000", "1000ms"),
];
