#[test]
fn gap_column_gap_wrap_align_stretch() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
        ..Default::default()
    });
    let node3 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
        ..Default::default()
    });
    let node4 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        flex_wrap: taffy::style::FlexWrap::Wrap,
        align_content: Some(taffy::style::AlignContent::Stretch),
        gap: taffy::geometry::Size { width: taffy::style::LengthPercentage::Points(5f32), height: zero() },
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(300f32),
            height: taffy::style::Dimension::Points(300f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1, node2, node3, node4]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node, 300f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node0, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node0, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node1, 72f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node1, 150f32, size.height);
    assert_eq!(location.x, 76f32, "x of node {:?}. Expected {}. Actual {}", node1, 76f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node2, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node2, 150f32, size.height);
    assert_eq!(location.x, 153f32, "x of node {:?}. Expected {}. Actual {}", node2, 153f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node3, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node3, 150f32, size.height);
    assert_eq!(location.x, 229f32, "x of node {:?}. Expected {}. Actual {}", node3, 229f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node4, 300f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node4, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 150f32, "y of node {:?}. Expected {}. Actual {}", node4, 150f32, location.y);
}
