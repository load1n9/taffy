#[test]
fn align_baseline_child_margin_percent() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(50f32),
        },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
        },
        ..Default::default()
    });
    let node10 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Percent(0.01f32),
            right: taffy::style::LengthPercentageAuto::Percent(0.01f32),
            top: taffy::style::LengthPercentageAuto::Percent(0.01f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(0.01f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(20f32),
        },
        ..Default::default()
    });
    taffy.set_children(node1, &[node10]).unwrap();
    let node = taffy.new_leaf(taffy::style::Style {
        align_items: Some(taffy::style::AlignItems::Baseline),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(100f32),
            height: taffy::style::Dimension::Points(100f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node0, 50f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node1, 50f32, location.x);
    assert_eq!(location.y, 45f32, "y of node {:?}. Expected {}. Actual {}", node1, 45f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node10, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node10, 10f32, size.height);
    assert_eq!(location.x, 1f32, "x of node {:?}. Expected {}. Actual {}", node10, 1f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node10, 1f32, location.y);
}
