use elvis_core::{style::Style, Class};

/// Parse class to web string
pub fn parse_class(class: &Class) -> &'static str {
    match class {
        Class::Center => "center",
        Class::Flex => "flex",
        Class::Col => "col",
        Class::Row => "row",
        Class::Empty => "",
    }
}

fn camel_snake(camel: &str) -> String {
    let mut res = "".to_string();
    camel.trim().chars().enumerate().for_each(|(n, c)| {
        if n > 0 && c.is_ascii_uppercase() {
            res.push_str("-");
        }
        res.push(c);
    });

    res.to_lowercase()
}

macro_rules! parse_style {
    (
        $s:expr,
        [$($ns:ident,)*],
        [$($ss:ident,)*]
    ) => {{
        match $s {
            $(
                Style::$ns(v) => format!(
                    "{}: {}",
                    camel_snake(stringify!($ns)),
                    v.to_string()
                ),
            )*
            $(
                Style::$ss(v) => format!(
                    "{}: {}",
                    camel_snake(stringify!($ss)),
                    v.to_string()
                ),
            )*
        }
    }}
}

/// Parse Style to String
pub fn parse_style(s: &Style) -> String {
    parse_style! {s, [
        // Box
        Height,
        Width,
        Padding,
        Margin,

        // Typo
        FontWeight,
        FontSize,
        FontStretch,
        LineHeight,

        // Color
        Color,
        BackgroundColor,

        // Flex
        AlignItems,
        JustifyContent,
        FlexGrow,
        FlexOrder,
        Wrap,

        // Grid
        GridAutoColumns,
        GridAutoRows,
        GridAutoFlow,
        GridColumnGap,
        GridRowGap,
        GridTemplateColumns,
        GridTemplateRows,

        // Column
        ColumnCount,
        ColumnGap,
        ColumnRuleColor,
        ColumnRuleStyle,

        // Flex
        FlexBasis,
        FlexDirection,
        FlexPosition,

        // Grid
        GridAuto,
        GridFlow,
        GridTemplate,
    ], [
        FontStyle,
    ]}
}