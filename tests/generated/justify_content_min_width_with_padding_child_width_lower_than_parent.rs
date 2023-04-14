#[test]
fn justify_content_min_width_with_padding_child_width_lower_than_parent() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy.new_leaf(taffy::style::Style {
        align_content: Some(taffy::style::AlignContent::Stretch),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(199f32),
            height: taffy::style::Dimension::Points(100f32),
        },
        ..Default::default()
    });
    let node00 = taffy.new_leaf(taffy::style::Style {
        align_content: Some(taffy::style::AlignContent::Stretch),
        justify_content: Some(taffy::style::JustifyContent::Center),
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(400f32), height: auto() },
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::Points(100f32),
            right: taffy::style::LengthPercentage::Points(100f32),
            top: zero(),
            bottom: zero(),
        },
        ..Default::default()
    });
    taffy.set_children(node00, &[node000]).unwrap();
    let node0 = taffy.new_leaf(taffy::style::Style {
        align_content: Some(taffy::style::AlignContent::Stretch),
        ..Default::default()
    });
    taffy.set_children(node0, &[node00]).unwrap();
    let node = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        align_content: Some(taffy::style::AlignContent::Stretch),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(1080f32),
            height: taffy::style::Dimension::Points(1584f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node, 1080f32, size.width);
    assert_eq!(size.height, 1584f32, "height of node {:?}. Expected {}. Actual {}", node, 1584f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node0, 1080f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node00, 400f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node00, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 199f32, "width of node {:?}. Expected {}. Actual {}", node000, 199f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node000, 100f32, size.height);
    assert_eq!(location.x, 101f32, "x of node {:?}. Expected {}. Actual {}", node000, 101f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
}
