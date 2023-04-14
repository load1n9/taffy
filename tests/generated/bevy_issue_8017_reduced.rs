#[test]
fn bevy_issue_8017_reduced() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Percent(0.5f32) },
        ..Default::default()
    });
    taffy.set_children(node0, &[node00]).unwrap();
    let node10 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Percent(0.5f32) },
        ..Default::default()
    });
    taffy.set_children(node1, &[node10]).unwrap();
    let node = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        gap: taffy::geometry::Size {
            width: taffy::style::LengthPercentage::Points(8f32),
            height: taffy::style::LengthPercentage::Points(8f32),
        },
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(200f32),
            height: taffy::style::Dimension::Points(400f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 400f32, "height of node {:?}. Expected {}. Actual {}", node, 400f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node0, 196f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node00, 0f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node00, 196f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node1, 196f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 204f32, "y of node {:?}. Expected {}. Actual {}", node1, 204f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node10, 0f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node10, 196f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
}
