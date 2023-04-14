#[test]
fn gap_row_gap_align_items_stretch() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
        ..Default::default()
    });
    let node3 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
        ..Default::default()
    });
    let node4 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
        ..Default::default()
    });
    let node5 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        flex_wrap: taffy::style::FlexWrap::Wrap,
        align_items: Some(taffy::style::AlignItems::Stretch),
        align_content: Some(taffy::style::AlignContent::Stretch),
        gap: taffy::geometry::Size {
            width: taffy::style::LengthPercentage::Points(10f32),
            height: taffy::style::LengthPercentage::Points(20f32),
        },
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(100f32),
            height: taffy::style::Dimension::Points(200f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1, node2, node3, node4, node5]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0, 20f32, size.width);
    assert_eq!(size.height, 90f32, "height of node {:?}. Expected {}. Actual {}", node0, 90f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node1, 20f32, size.width);
    assert_eq!(size.height, 90f32, "height of node {:?}. Expected {}. Actual {}", node1, 90f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node1, 30f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node2, 20f32, size.width);
    assert_eq!(size.height, 90f32, "height of node {:?}. Expected {}. Actual {}", node2, 90f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node2, 60f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node3, 20f32, size.width);
    assert_eq!(size.height, 90f32, "height of node {:?}. Expected {}. Actual {}", node3, 90f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 110f32, "y of node {:?}. Expected {}. Actual {}", node3, 110f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node4, 20f32, size.width);
    assert_eq!(size.height, 90f32, "height of node {:?}. Expected {}. Actual {}", node4, 90f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node4, 30f32, location.x);
    assert_eq!(location.y, 110f32, "y of node {:?}. Expected {}. Actual {}", node4, 110f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node5, 20f32, size.width);
    assert_eq!(size.height, 90f32, "height of node {:?}. Expected {}. Actual {}", node5, 90f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node5, 60f32, location.x);
    assert_eq!(location.y, 110f32, "y of node {:?}. Expected {}. Actual {}", node5, 110f32, location.y);
}
