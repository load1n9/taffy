#[test]
fn absolute_layout_align_items_and_justify_content_flex_end() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        position: taffy::style::Position::Absolute,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(60f32),
            height: taffy::style::Dimension::Points(40f32),
        },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        align_items: Some(taffy::style::AlignItems::FlexEnd),
        justify_content: Some(taffy::style::JustifyContent::FlexEnd),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(110f32),
            height: taffy::style::Dimension::Points(100f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 110f32, "width of node {:?}. Expected {}. Actual {}", node, 110f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node0, 60f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node0, 50f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node0, 60f32, location.y);
}
