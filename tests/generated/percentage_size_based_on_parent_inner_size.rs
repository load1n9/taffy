#[test]
fn percentage_size_based_on_parent_inner_size() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Percent(0.5f32),
            height: taffy::style::Dimension::Percent(0.5f32),
        },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        flex_direction: taffy::style::FlexDirection::Column,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(200f32),
            height: taffy::style::Dimension::Points(400f32),
        },
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::Points(20f32),
            right: taffy::style::LengthPercentage::Points(20f32),
            top: taffy::style::LengthPercentage::Points(20f32),
            bottom: taffy::style::LengthPercentage::Points(20f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
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
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node0, 80f32, size.width);
    assert_eq!(size.height, 180f32, "height of node {:?}. Expected {}. Actual {}", node0, 180f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node0, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node0, 20f32, location.y);
}
