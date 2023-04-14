#[test]
fn rounding_inner_node_controversy_combined() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Percent(1f32) },
        ..Default::default()
    });
    let node10 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
        ..Default::default()
    });
    let node110 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
        ..Default::default()
    });
    let node11 = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        flex_grow: 1f32,
        size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
        ..Default::default()
    });
    taffy.set_children(node11, &[node110]).unwrap();
    let node12 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        flex_grow: 1f32,
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Percent(1f32) },
        ..Default::default()
    });
    taffy.set_children(node1, &[node10, node11, node12]).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Percent(1f32) },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(640f32),
            height: taffy::style::Dimension::Points(320f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1, node2]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 640f32, "width of node {:?}. Expected {}. Actual {}", node, 640f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node, 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 213f32, "width of node {:?}. Expected {}. Actual {}", node0, 213f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node0, 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 214f32, "width of node {:?}. Expected {}. Actual {}", node1, 214f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node1, 320f32, size.height);
    assert_eq!(location.x, 213f32, "x of node {:?}. Expected {}. Actual {}", node1, 213f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 214f32, "width of node {:?}. Expected {}. Actual {}", node10, 214f32, size.width);
    assert_eq!(size.height, 107f32, "height of node {:?}. Expected {}. Actual {}", node10, 107f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node11).unwrap();
    assert_eq!(size.width, 214f32, "width of node {:?}. Expected {}. Actual {}", node11, 214f32, size.width);
    assert_eq!(size.height, 106f32, "height of node {:?}. Expected {}. Actual {}", node11, 106f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node11, 0f32, location.x);
    assert_eq!(location.y, 107f32, "y of node {:?}. Expected {}. Actual {}", node11, 107f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node110).unwrap();
    assert_eq!(size.width, 214f32, "width of node {:?}. Expected {}. Actual {}", node110, 214f32, size.width);
    assert_eq!(size.height, 106f32, "height of node {:?}. Expected {}. Actual {}", node110, 106f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node110, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node110, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node12).unwrap();
    assert_eq!(size.width, 214f32, "width of node {:?}. Expected {}. Actual {}", node12, 214f32, size.width);
    assert_eq!(size.height, 107f32, "height of node {:?}. Expected {}. Actual {}", node12, 107f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node12, 0f32, location.x);
    assert_eq!(location.y, 213f32, "y of node {:?}. Expected {}. Actual {}", node12, 213f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 213f32, "width of node {:?}. Expected {}. Actual {}", node2, 213f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node2, 320f32, size.height);
    assert_eq!(location.x, 427f32, "x of node {:?}. Expected {}. Actual {}", node2, 427f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
}
