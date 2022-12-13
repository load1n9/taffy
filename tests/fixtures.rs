// this declaration is necessary to "mount" the generated code where cargo can see it
// this allows us to both keep code generation scoped to a singe directory for fs events
// and to keep each test in a separate file
mod generated;

use core::str::FromStr;

use html_parser::{Dom, Element, Node};
use swc_common::input::StringInput;
use swc_common::source_map::BytePos;
use swc_css_ast::{self as ast, ComponentValue, DeclarationName, Ident, StyleBlock};
use swc_css_parser::parse_string_input;
use swc_css_parser::parser::ParserConfig;
use taffy::prelude::*;

const HTML: &str = r###"
  <div id="test-root" style="flex-direction: row; flex-wrap: wrap; width: 100px; height: 200px; column-gap: 10px; row-gap: 20px; align-items:flex-end;">
    <div style="left: 20px;right: 30%;top:auto;bottom:0"></div>
    <div style="flex-basis: 40px;flex-grow: 2.2;flex-shrink: 1"></div>
    <div style="width: 20px;aspect-ratio: 0.345"></div>
    <div style="width: 20px;"></div>
    <div style="width: 20px;"></div>
    <div style="width: 20px;"></div>
  </div>
"###;

const HTML2: &str = r###"
  <div id="test-root" style="width: 140px; display: grid; grid-auto-flow: column dense;grid-auto-rows: 1fr min-content max-content auto 40px minmax(20px, 40px) 40px;grid-template-rows: repeat(auto-fill, 40px 20px) ;">
    <div></div>
  </div>
"###;

#[test]
fn parse_html() {
    let tree = Dom::parse(HTML2).unwrap();
    let root = &tree.children[0];

    parse_node(root);

    assert_eq!(true, false);
}

fn parse_node(node: &Node) {
    match node {
        Node::Element(element) => parse_element(element),
        Node::Text(text) => println!("{text}"),
        Node::Comment(text) => println!("{text}"),
    }
}

fn parse_element(element: &Element) {
    // dbg!(&element);

    println!("{}", element.name);
    let styles = element.attributes.get("style");
    if let Some(Some(styles)) = styles {
        println!("  {styles}");
        parse_style(styles);
    }

    for child in &element.children {
        parse_node(child);
    }
}

fn parse_style(style_text: &str) -> Style {
    let input = StringInput::new(style_text, BytePos(0), BytePos(style_text.len() as u32));
    let mut errors = vec![];
    let parsed_styles = parse_string_input::<Vec<StyleBlock>>(input, ParserConfig::default(), &mut errors).unwrap();

    // dbg!(errors);
    // dbg!(parsed_styles);

    let declarations = parsed_styles.iter().filter_map(|style_block| match style_block {
        StyleBlock::Declaration(decl) => Some(decl),
        _ => None,
    });

    let mut style = Style::default();
    for decl in declarations {
        let DeclarationName::Ident(name) = &decl.name else { panic!("Declaration name is not an ident"); };
        match ident_as_str(name) {
            "display" => {
                let ComponentValue::Ident(ident) = &decl.value[0] else { continue; };
                style.display = match ident_as_str(ident) {
                    "grid" => Display::Grid,
                    "none" => Display::None,
                    "flex" | _ => Display::Flex,
                };
            }
            "position" => {
                let ComponentValue::Ident(ident) = &decl.value[0] else { continue; };
                style.position_type = match ident_as_str(ident) {
                    "absolute" => PositionType::Absolute,
                    "relative" | _ => PositionType::Relative,
                };
            }

            // Position
            "left" => {
                let Some(left) = parse_length_percentage_auto(&decl.value[0]) else { continue; };
                style.position.left = left;
            }
            "right" => {
                let Some(right) = parse_length_percentage_auto(&decl.value[0]) else { continue; };
                style.position.right = right;
            }
            "top" => {
                let Some(top) = parse_length_percentage_auto(&decl.value[0]) else { continue; };
                style.position.top = top;
            }
            "bottom" => {
                let Some(bottom) = parse_length_percentage_auto(&decl.value[0]) else { continue; };
                style.position.bottom = bottom;
            }
            "inset" => {
                let Some([top, right, bottom, left]) = four_value_shorthand(&decl.value, parse_length_percentage_auto) else { continue; };
                style.position.top = top;
                style.position.right = right;
                style.position.bottom = bottom;
                style.position.left = left;
            }

            // Sizes
            "width" => {
                let Some(width) = parse_dimension(&decl.value[0]) else { continue; };
                style.size.width = width;
            }
            "height" => {
                let Some(height) = parse_dimension(&decl.value[0]) else { continue; };
                style.size.height = height;
            }
            "min-width" => {
                let Some(width) = parse_dimension(&decl.value[0]) else { continue; };
                style.min_size.width = width;
            }
            "min-height" => {
                let Some(height) = parse_dimension(&decl.value[0]) else { continue; };
                style.min_size.height = height;
            }
            "max-width" => {
                let Some(width) = parse_dimension(&decl.value[0]) else { continue; };
                style.max_size.width = width;
            }
            "max-height" => {
                let Some(height) = parse_dimension(&decl.value[0]) else { continue; };
                style.max_size.height = height;
            }

            // Aspect ratio
            "aspect-ratio" => {
                let Some(aspect_ratio) = parse_float(&decl.value[0]) else { continue; };
                style.aspect_ratio = Some(aspect_ratio);
            }

            // Margin
            "margin-left" => {
                let Some(left) = parse_length_percentage_auto(&decl.value[0]) else { continue; };
                style.margin.left = left;
            }
            "margin-right" => {
                let Some(right) = parse_length_percentage_auto(&decl.value[0]) else { continue; };
                style.margin.right = right;
            }
            "margin-top" => {
                let Some(top) = parse_length_percentage_auto(&decl.value[0]) else { continue; };
                style.margin.top = top;
            }
            "margin-bottom" => {
                let Some(bottom) = parse_length_percentage_auto(&decl.value[0]) else { continue; };
                style.margin.bottom = bottom;
            }
            "margin" => {
                let Some([top, right, bottom, left]) = four_value_shorthand(&decl.value, parse_length_percentage_auto) else { continue; };
                style.margin.top = top;
                style.margin.right = right;
                style.margin.bottom = bottom;
                style.margin.left = left;
            }

            // Padding
            "padding-left" => {
                let Some(left) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.padding.left = left;
            }
            "padding-right" => {
                let Some(right) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.padding.right = right;
            }
            "padding-top" => {
                let Some(top) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.padding.top = top;
            }
            "padding-bottom" => {
                let Some(bottom) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.padding.bottom = bottom;
            }
            "padding" => {
                let Some([top, right, bottom, left]) = four_value_shorthand(&decl.value, parse_length_percentage) else { continue; };
                style.padding.top = top;
                style.padding.right = right;
                style.padding.bottom = bottom;
                style.padding.left = left;
            }

            // Border
            "border-left" => {
                let Some(left) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.border.left = left;
            }
            "border-right" => {
                let Some(right) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.border.right = right;
            }
            "border-top" => {
                let Some(top) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.border.top = top;
            }
            "border-bottom" => {
                let Some(bottom) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.border.bottom = bottom;
            }
            "border" => {
                let Some([top, right, bottom, left]) = four_value_shorthand(&decl.value, parse_length_percentage) else { continue; };
                style.border.top = top;
                style.border.right = right;
                style.border.bottom = bottom;
                style.border.left = left;
            }

            // Alignment
            "align-content" => {
                style.align_content = parse_string(&decl.value);
            }
            "justify-content" => {
                style.justify_content = parse_string(&decl.value);
            }
            "align-items" => {
                style.align_items = parse_string(&decl.value);
            }
            "justify-items" => {
                style.justify_items = parse_string(&decl.value);
            }
            "align-self" => {
                style.align_self = parse_string(&decl.value);
            }
            "justify-self" => {
                style.justify_self = parse_string(&decl.value);
            }

            // Gap
            "column-gap" => {
                let Some(column_gap) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.gap.width = column_gap;
            }
            "row-gap" => {
                let Some(row_gap) = parse_length_percentage(&decl.value[0]) else { continue; };
                style.gap.height = row_gap;
            }
            "gap" => {
                let Some([row_gap, column_gap]) = two_value_shorthand(&decl.value, parse_length_percentage) else { continue; };
                style.gap.height = row_gap;
                style.gap.width = column_gap;
            }

            // Flex Container
            // TODO

            // Flex Child
            "flex-basis" => {
                let Some(flex_basis) = parse_dimension(&decl.value[0]) else { continue; };
                style.flex_basis = flex_basis;
            }
            "flex-grow" => {
                let Some(flex_grow) = parse_float(&decl.value[0]) else { continue; };
                style.flex_grow = flex_grow;
            }
            "flex-shrink" => {
                let Some(flex_shrink) = parse_float(&decl.value[0]) else { continue; };
                style.flex_shrink = flex_shrink;
            }

            // Grid Container
            "grid-template-columns" => {
                let Some(track_list) = parse_vec(&decl.value, parse_repeatable_track_definition) else { continue; };
                style.grid_template_columns = track_list;
            }
            "grid-template-rows" => {
                let Some(track_list) = parse_vec(&decl.value, parse_repeatable_track_definition) else { continue; };
                style.grid_template_rows = track_list;
            }
            "grid-auto-columns" => {
                let Some(track_list) = parse_vec(&decl.value, parse_track_sizing_function) else { continue; };
                style.grid_auto_columns = track_list;
            }
            "grid-auto-rows" => {
                let Some(track_list) = parse_vec(&decl.value, parse_track_sizing_function) else { continue; };
                style.grid_auto_rows = track_list;
            }
            "grid-auto-flow" => {
                let Some(auto_flow) = parse_string(&decl.value) else { continue; };
                style.grid_auto_flow = auto_flow;
            }

            // Grid Child
            // TODO
            _ => {}
        };
        // dbg!(decl);
    }

    dbg!(&style.grid_auto_flow);

    style
}

fn ident_as_str(ident: &Ident) -> &str {
    ident.raw.as_ref().unwrap()
}

fn try_component_value_ident_as_str(raw_value: &ComponentValue) -> Option<&str> {
    match raw_value {
        ComponentValue::Ident(ident) => Some(&ident.raw.as_ref().unwrap()),
        _ => None,
    }
}

fn parse_float(raw_value: &ComponentValue) -> Option<f32> {
    match raw_value {
        ComponentValue::Integer(integer) => Some(points(integer.value as f32)),
        ComponentValue::Number(number) => Some(points(number.value as f32)),
        _ => {
            dbg!(raw_value);
            None
        }
    }
}

fn parse_string<T: FromStr>(raw_values: &[ComponentValue]) -> Option<T> {
    match raw_values.len() {
        1 => try_component_value_ident_as_str(&raw_values[0])?.parse().ok(),
        2 => {
            let string = raw_values.iter().filter_map(try_component_value_ident_as_str).collect::<Vec<_>>().join(" ");
            string.parse().ok()
        }
        _ => None,
    }
}

fn parse_dimension(raw_value: &ComponentValue) -> Option<Dimension> {
    match raw_value {
        ComponentValue::Percentage(percentage) => Some(Dimension::Percent(percentage.value.value as f32)),
        ComponentValue::Dimension(ast::Dimension::Length(length)) if ident_as_str(&length.unit) == "px" => {
            Some(points(length.value.value as f32))
        }
        ComponentValue::Integer(integer) if integer.value == 0 => Some(points(integer.value as f32)),
        ComponentValue::Ident(ident) => match ident_as_str(ident) {
            "auto" => auto(),
            _ => None,
        },
        _ => {
            dbg!(raw_value);
            None
        }
    }
}

fn parse_length_percentage_auto(raw_value: &ComponentValue) -> Option<LengthPercentageAuto> {
    parse_dimension(raw_value).and_then(|dim| dim.try_into().ok())
}

fn parse_length_percentage(raw_value: &ComponentValue) -> Option<LengthPercentage> {
    parse_dimension(raw_value).and_then(|dim| dim.try_into().ok())
}

fn parse_repeatable_track_definition(raw_value: &ComponentValue) -> Option<TrackSizingFunction> {
    match parse_track_sizing_function(raw_value) {
        Some(sizing_function) => Some(TrackSizingFunction::Single(sizing_function)),
        None => match raw_value {
            ComponentValue::Function(function) => match ident_as_str(&function.name) {
                "repeat" => {
                    let args = &function.value;
                    if args.len() < 3 || !matches!(args[1], ComponentValue::Delimiter(_)) {
                        return None;
                    }

                    // Get fill type
                    let repetition_kind = match &args[0] {
                        ComponentValue::Ident(ident) => {
                            match ident_as_str(&ident) {
                                "auto-fill" => GridTrackRepetition::AutoFill,
                                // "auto-fit" => GridTrackRepetition::AutoFit,
                                _ => return None,
                            }
                        }
                        // ComponentValue::Integer(integer) => GridTrackRepetition::Count(integer.value as f32),
                        _ => return None,
                    };

                    let tracks = args[2..].iter().map(parse_track_sizing_function).collect::<Option<Vec<_>>>()?;
                    Some(repeat(repetition_kind, tracks))
                }
                _ => None,
            },
            _ => {
                dbg!(raw_value);
                None
            }
        },
    }
}

fn parse_track_sizing_function(raw_value: &ComponentValue) -> Option<NonRepeatedTrackSizingFunction> {
    match parse_max_track_sizing_function(raw_value) {
        Some(max) => {
            let min = max.clone().try_into().unwrap_or(auto());
            Some(minmax(min, max))
        }
        None => match raw_value {
            ComponentValue::Function(function) => match ident_as_str(&function.name) {
                "minmax" => {
                    let args = &function.value;
                    if args.len() != 3 || !matches!(args[1], ComponentValue::Delimiter(_)) {
                        return None;
                    }
                    let min = parse_max_track_sizing_function(&args[0])?;
                    let min = min.try_into().ok()?;
                    let max = parse_max_track_sizing_function(&args[2])?;
                    Some(minmax(min, max))
                }
                _ => None,
            },
            _ => {
                dbg!(raw_value);
                None
            }
        },
    }
}

// MaxTrackSizingFunction has a TryInto impl for MinTrackSizingFunction, so this function is also
// used from parsing MinTrackSizingFunction
fn parse_max_track_sizing_function(raw_value: &ComponentValue) -> Option<MaxTrackSizingFunction> {
    match raw_value {
        ComponentValue::Percentage(percentage) => Some(percent(percentage.value.value as f32)),
        ComponentValue::Dimension(ast::Dimension::Length(length)) if ident_as_str(&length.unit) == "px" => {
            Some(points(length.value.value as f32))
        }
        ComponentValue::Dimension(ast::Dimension::Flex(flex_fraction)) => Some(flex(flex_fraction.value.value as f32)),
        ComponentValue::Integer(integer) if integer.value == 0 => Some(points(integer.value as f32)),
        ComponentValue::Ident(ident) => match ident_as_str(ident) {
            "auto" => auto(),
            "min-content" => min_content(),
            "max-content" => max_content(),
            _ => None,
        },
        _ => None,
    }
}

fn parse_vec<T>(raw_values: &[ComponentValue], parser: impl Fn(&ComponentValue) -> Option<T>) -> Option<Vec<T>> {
    let results: Vec<T> = raw_values.into_iter().filter_map(|value| parser(value)).collect();
    if results.len() == raw_values.len() {
        Some(results)
    } else {
        None
    }
}

fn two_value_shorthand<T: Clone>(
    raw_values: &[ComponentValue],
    parser: impl Fn(&ComponentValue) -> Option<T>,
) -> Option<[T; 2]> {
    match raw_values.len() {
        1 => {
            let Some(value) = parser(&raw_values[0]) else { return None; };
            Some([value.clone(), value.clone()])
        }
        2 => {
            let Some(value1) = parser(&raw_values[0]) else { return None; };
            let Some(value2) = parser(&raw_values[1]) else { return None; };
            Some([value1, value2])
        }
        _ => None,
    }
}

fn four_value_shorthand<T: Clone>(
    raw_values: &[ComponentValue],
    parser: impl Fn(&ComponentValue) -> Option<T>,
) -> Option<[T; 4]> {
    match raw_values.len() {
        1 => {
            let Some(value) = parser(&raw_values[0]) else { return None; };
            Some([value.clone(), value.clone(), value.clone(), value.clone()])
        }
        2 => {
            let Some(value1) = parser(&raw_values[0]) else { return None; };
            let Some(value2) = parser(&raw_values[1]) else { return None; };
            Some([value1.clone(), value2.clone(), value1.clone(), value2.clone()])
        }
        4 => {
            let Some(value1) = parser(&raw_values[0]) else { return None; };
            let Some(value2) = parser(&raw_values[1]) else { return None; };
            let Some(value3) = parser(&raw_values[2]) else { return None; };
            let Some(value4) = parser(&raw_values[3]) else { return None; };
            Some([value1, value2, value3, value4])
        }
        _ => None,
    }
}
