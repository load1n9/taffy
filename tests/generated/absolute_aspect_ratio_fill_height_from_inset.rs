#[test]
fn absolute_aspect_ratio_fill_height_from_inset() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        position: taffy::style::Position::Absolute,
        aspect_ratio: Some(3f32),
        inset: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Percent(0.1f32),
            right: taffy::style::LengthPercentageAuto::Percent(0.1f32),
            top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            bottom: auto(),
        },
        ..Default::default()
    });
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
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node0, 320f32, size.width);
    assert_eq!(size.height, 107f32, "height of node {:?}. Expected {}. Actual {}", node0, 107f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node0, 40f32, location.x);
    assert_eq!(location.y, 15f32, "y of node {:?}. Expected {}. Actual {}", node0, 15f32, location.y);
}
