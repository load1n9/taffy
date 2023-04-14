#[test]
fn align_content_space_evenly_wrapped() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        ..Default::default()
    });
    let node3 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        ..Default::default()
    });
    let node4 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        ..Default::default()
    });
    let node5 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(10f32),
        },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        flex_wrap: taffy::style::FlexWrap::Wrap,
        align_content: Some(taffy::style::AlignContent::SpaceEvenly),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(100f32),
            height: taffy::style::Dimension::Points(100f32),
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
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node0, 18f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node1, 50f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node1, 18f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node2, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 45f32, "y of node {:?}. Expected {}. Actual {}", node2, 45f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node3, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node3, 10f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node3, 50f32, location.x);
    assert_eq!(location.y, 45f32, "y of node {:?}. Expected {}. Actual {}", node3, 45f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node4, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node4, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 73f32, "y of node {:?}. Expected {}. Actual {}", node4, 73f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node5, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node5, 10f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node5, 50f32, location.x);
    assert_eq!(location.y, 73f32, "y of node {:?}. Expected {}. Actual {}", node5, 73f32, location.y);
}
