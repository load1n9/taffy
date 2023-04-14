#[test]
fn align_self_center_undefined_max_height() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(240f32),
            height: taffy::style::Dimension::Points(44f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        align_self: Some(taffy::style::AlignSelf::Center),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(40f32),
            height: taffy::style::Dimension::Points(56f32),
        },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(280f32), height: auto() },
        min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(52f32) },
        ..Default::default()
    });
    taffy.set_children(node, &[node0, node1]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 280f32, "width of node {:?}. Expected {}. Actual {}", node, 280f32, size.width);
    assert_eq!(size.height, 56f32, "height of node {:?}. Expected {}. Actual {}", node, 56f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 240f32, "width of node {:?}. Expected {}. Actual {}", node0, 240f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node0, 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 56f32, "height of node {:?}. Expected {}. Actual {}", node1, 56f32, size.height);
    assert_eq!(location.x, 240f32, "x of node {:?}. Expected {}. Actual {}", node1, 240f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}
