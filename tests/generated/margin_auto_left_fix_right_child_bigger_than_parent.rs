#[test]
fn margin_auto_left_fix_right_child_bigger_than_parent() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(72f32),
            height: taffy::style::Dimension::Points(72f32),
        },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Auto,
            right: taffy::style::LengthPercentageAuto::Points(10f32),
            top: zero(),
            bottom: zero(),
        },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        justify_content: Some(taffy::style::JustifyContent::Center),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(52f32),
            height: taffy::style::Dimension::Points(52f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 52f32, "width of node {:?}. Expected {}. Actual {}", node, 52f32, size.width);
    assert_eq!(size.height, 52f32, "height of node {:?}. Expected {}. Actual {}", node, 52f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 42f32, "width of node {:?}. Expected {}. Actual {}", node0, 42f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node0, 72f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
