#[test]
fn align_items_center_justify_content_center() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy.new_leaf_with_measure(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(10f32),
                height: taffy::style::Dimension::Points(10f32),
            },
            ..Default::default()
        },
        taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT: &str = "\n      ";
            super::measure_standard_text(known_dimensions, available_space, TEXT, super::WritingMode::Horizontal, None)
        }),
    );
    let node00 = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        align_items: Some(taffy::style::AlignItems::Center),
        justify_content: Some(taffy::style::JustifyContent::Center),
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Percent(1f32) },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Points(10f32),
            right: taffy::style::LengthPercentageAuto::Points(10f32),
            top: zero(),
            bottom: zero(),
        },
        ..Default::default()
    });
    taffy.set_children(node00, &[node000]).unwrap();
    let node0 = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        align_items: Some(taffy::style::AlignItems::Center),
        justify_content: Some(taffy::style::JustifyContent::Center),
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(50f32) },
        ..Default::default()
    });
    taffy.set_children(node0, &[node00]).unwrap();
    let node = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(500f32),
            height: taffy::style::Dimension::Points(500f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 500f32, "width of node {:?}. Expected {}. Actual {}", node, 500f32, size.width);
    assert_eq!(size.height, 500f32, "height of node {:?}. Expected {}. Actual {}", node, 500f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 500f32, "width of node {:?}. Expected {}. Actual {}", node0, 500f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node0, 50f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node00, 10f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node00, 50f32, size.height);
    assert_eq!(location.x, 245f32, "x of node {:?}. Expected {}. Actual {}", node00, 245f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node000, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node000, 20f32, location.y);
}
