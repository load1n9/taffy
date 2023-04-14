#[test]
fn align_baseline_child_multiline() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(60f32),
        },
        ..Default::default()
    });
    let node10 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(25f32),
            height: taffy::style::Dimension::Points(20f32),
        },
        ..Default::default()
    });
    let node11 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(25f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        ..Default::default()
    });
    let node12 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(25f32),
            height: taffy::style::Dimension::Points(20f32),
        },
        ..Default::default()
    });
    let node13 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(25f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        flex_wrap: taffy::style::FlexWrap::Wrap,
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), height: auto() },
        ..Default::default()
    });
    taffy.set_children(node1, &[node10, node11, node12, node13]).unwrap();
    let node = taffy.new_leaf(taffy::style::Style {
        align_items: Some(taffy::style::AlignItems::Baseline),
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), height: auto() },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node, 80f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node0, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node1, 50f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node1, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node10, 25f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node10, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node11).unwrap();
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node11, 25f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node11, 10f32, size.height);
    assert_eq!(location.x, 25f32, "x of node {:?}. Expected {}. Actual {}", node11, 25f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node11, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node12).unwrap();
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node12, 25f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node12, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node12, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node12, 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node13).unwrap();
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node13, 25f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node13, 10f32, size.height);
    assert_eq!(location.x, 25f32, "x of node {:?}. Expected {}. Actual {}", node13, 25f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node13, 20f32, location.y);
}
