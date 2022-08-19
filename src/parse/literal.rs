use itertools::Itertools;
use swc_common::DUMMY_SP;
use swc_ecma_visit::swc_ecma_ast::{
    Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

use crate::{
    config::TailwindTheme,
    infer::{infer_type, Type},
};

pub fn parse_literal<'a>(theme: &TailwindTheme, s: &'a str) -> Result<ObjectLit, &'a str> {
    if let Some(pair) = s.split_once('-') {
        let literal = match pair {
            ("text", rest) => match infer_type(theme, rest) {
                Ok(Type::Screen(x)) => to_lit(&[("fontSize", &format!("{}em", x))]),
                Ok(Type::Color(x)) => to_lit(&[("color", x)]),
                _ => return Err(s),
            },
            ("font", rest) => match theme.font_family.get(rest) {
                Some(val) => to_lit(&[("fontFamily", &val.iter().join(", "))]),
                None => return Err(s),
            },
            ("shadow", rest) => match theme.box_shadow.get(rest) {
                Some(val) => to_lit(&[("boxShadow", "var(--tw-shadow)"), ("--tw-shadow", &val), ("--tw-shadow-colored", "0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color)")]),
                None => match theme.colors.get(rest) {
                    Some(val) => to_lit(&[
                        ("--tw-shadow-color", val),
                        ("--tw-shadow", "var(--tw-shadow-colored)"),
                    ]),
                    None => return Err(s),
                },
            },
            ("transition", rest) => match theme.transition_property.get(rest)  {
                Some(val) => to_lit(&[("transition-property", val)]),
                None => return Err(s),
            },
            ("delay", rest) => match theme.transition_delay.get(rest)  {
                Some(val) => to_lit(&[("transition-delay", val)]),
                None => return Err(s),
            },
            ("duration", rest) => match theme.transition_duration.get(rest)  {
                Some(val) => to_lit(&[("transition-duration", val)]),
                None => return Err(s),
            },
            ("ease", rest) => match theme.transition_timing_function.get(rest)  {
                Some(val) => to_lit(&[("transition-timing-function", val)]),
                None => return Err(s),
            },
            ("border", rest) => match infer_type(theme, rest) {
                Ok(Type::Scalar(x)) => to_lit(&[("borderWidth", &format!("{}px", x))]),
                Ok(Type::Color(x)) => to_lit(&[("borderColor", x)]),
                _ => return Err(s),
            },
            ("rounded", rest) => match theme.border_radius.get(rest) {
                Some(val) => to_lit(&[("borderRadius", val)]),
                None => return Err(s),
            },
            ("bg", rest) => match theme.colors.get(rest) {
                Some(c) => to_lit(&[("backgroundColor", c)]),
                _ => return Err(s),
            },
            ("h", rest) => to_lit(&[("height", &format!("{}em", rest,))]),
            ("w", rest) => to_lit(&[("width", &format!("{}em", rest,))]),
            ("p", rest) => to_lit(&[("padding", &format!("{}em", rest,))]),
            ("m", rest) => to_lit(&[("margin", &format!("{}em", rest,))]),
            _ => return Err(s),
        };

        Ok(literal)
    } else {
        Err(s)
    }
}

fn to_lit(items: &[(&str, &str)]) -> ObjectLit {
    ObjectLit {
        span: DUMMY_SP,
        props: items
            .iter()
            .map(|(key, value)| {
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Str(Str {
                        span: DUMMY_SP,
                        raw: None,
                        value: (*key).into(),
                    }),
                    value: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        raw: None,
                        value: (*value).into(),
                    }))),
                })))
            })
            .collect(),
    }
}
