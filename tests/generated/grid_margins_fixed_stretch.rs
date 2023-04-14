#[test]
fn grid_margins_fixed_stretch() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        align_self: Some(taffy::style::AlignSelf::Stretch),
        justify_self: Some(taffy::style::JustifySelf::Stretch),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(20f32),
            height: taffy::style::Dimension::Points(20f32),
        },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Points(4f32),
            right: taffy::style::LengthPercentageAuto::Points(2f32),
            top: taffy::style::LengthPercentageAuto::Points(1f32),
            bottom: taffy::style::LengthPercentageAuto::Points(3f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        grid_template_rows: vec![points(40f32), points(40f32), points(40f32)],
        grid_template_columns: vec![points(40f32), points(40f32), points(40f32)],
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::Points(40f32),
            right: taffy::style::LengthPercentage::Points(20f32),
            top: taffy::style::LengthPercentage::Points(10f32),
            bottom: taffy::style::LengthPercentage::Points(30f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1, node2, node3, node4, node5]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 180f32, "width of node {:?}. Expected {}. Actual {}", node, 180f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node, 160f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 44f32, "x of node {:?}. Expected {}. Actual {}", node0, 44f32, location.x);
    assert_eq!(location.y, 11f32, "y of node {:?}. Expected {}. Actual {}", node0, 11f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node1, 80f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node1, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node2, 120f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node2, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node3, 40f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node3, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node4, 80f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node4, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node5, 120f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node5, 50f32, location.y);
}
