#[test]
fn grid_percent_items_width_and_margin() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), height: auto() },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
        },
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::Points(3f32),
            right: taffy::style::LengthPercentage::Points(3f32),
            top: taffy::style::LengthPercentage::Points(3f32),
            bottom: taffy::style::LengthPercentage::Points(3f32),
        },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(200f32), height: auto() },
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::Points(3f32),
            right: taffy::style::LengthPercentage::Points(3f32),
            top: taffy::style::LengthPercentage::Points(3f32),
            bottom: taffy::style::LengthPercentage::Points(3f32),
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
    assert_eq!(size.height, 31f32, "height of node {:?}. Expected {}. Actual {}", node, 31f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node0, 87f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node0, 6f32, size.height);
    assert_eq!(location.x, 13f32, "x of node {:?}. Expected {}. Actual {}", node0, 13f32, location.x);
    assert_eq!(location.y, 13f32, "y of node {:?}. Expected {}. Actual {}", node0, 13f32, location.y);
}
