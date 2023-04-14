#[test]
fn align_items_center_min_max_with_padding() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(62f32),
            height: taffy::style::Dimension::Points(62f32),
        },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        align_items: Some(taffy::style::AlignItems::Center),
        min_size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(320f32),
            height: taffy::style::Dimension::Points(72f32),
        },
        max_size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(320f32),
            height: taffy::style::Dimension::Points(504f32),
        },
        padding: taffy::geometry::Rect {
            left: zero(),
            right: zero(),
            top: taffy::style::LengthPercentage::Points(8f32),
            bottom: taffy::style::LengthPercentage::Points(8f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node, 320f32, size.width);
    assert_eq!(size.height, 78f32, "height of node {:?}. Expected {}. Actual {}", node, 78f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 62f32, "width of node {:?}. Expected {}. Actual {}", node0, 62f32, size.width);
    assert_eq!(size.height, 62f32, "height of node {:?}. Expected {}. Actual {}", node0, 62f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
}
