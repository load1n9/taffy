#[test]
fn grid_percent_tracks_definite_underflow() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        grid_template_rows: vec![percent(0.3f32), percent(0.6f32)],
        grid_template_columns: vec![percent(0.1f32), percent(0.2f32), percent(0.3f32)],
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(120f32),
            height: taffy::style::Dimension::Points(60f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1, node2, node3, node4, node5]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node0, 12f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node0, 18f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node1, 24f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node1, 18f32, size.height);
    assert_eq!(location.x, 12f32, "x of node {:?}. Expected {}. Actual {}", node1, 12f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node2, 36f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node2, 18f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node2, 36f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node3, 12f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node3, 36f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node3, 18f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node4, 24f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node4, 36f32, size.height);
    assert_eq!(location.x, 12f32, "x of node {:?}. Expected {}. Actual {}", node4, 12f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node4, 18f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node5, 36f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node5, 36f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node5, 36f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node5, 18f32, location.y);
}
