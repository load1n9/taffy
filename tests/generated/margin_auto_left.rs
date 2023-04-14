#[test]
fn margin_auto_left() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(50f32),
        },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Auto,
            right: zero(),
            top: zero(),
            bottom: zero(),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Points(50f32),
        },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        align_items: Some(taffy::style::AlignItems::Center),
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(200f32),
            height: taffy::style::Dimension::Points(200f32),
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
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node0, 50f32, size.height);
    assert_eq!(location.x, 100f32, "x of node {:?}. Expected {}. Actual {}", node0, 100f32, location.x);
    assert_eq!(location.y, 75f32, "y of node {:?}. Expected {}. Actual {}", node0, 75f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node1, 50f32, size.height);
    assert_eq!(location.x, 150f32, "x of node {:?}. Expected {}. Actual {}", node1, 150f32, location.x);
    assert_eq!(location.y, 75f32, "y of node {:?}. Expected {}. Actual {}", node1, 75f32, location.y);
}
