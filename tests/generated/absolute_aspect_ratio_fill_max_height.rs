#[test]
fn absolute_aspect_ratio_fill_max_height() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf_with_measure(
        taffy::style::Style {
            position: taffy::style::Position::Absolute,
            max_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), height: auto() },
            aspect_ratio: Some(3f32),
            ..Default::default()
        },
        taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT: &str =
                "HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH";
            super::measure_standard_text(
                known_dimensions,
                available_space,
                TEXT,
                super::WritingMode::Horizontal,
                Some(3f32),
            )
        }),
    );
    let node = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Flex,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(400f32),
            height: taffy::style::Dimension::Points(300f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 17f32, "height of node {:?}. Expected {}. Actual {}", node0, 17f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
