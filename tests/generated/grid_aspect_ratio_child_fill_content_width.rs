#[test]
fn grid_aspect_ratio_child_fill_content_width() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf_with_measure(
        taffy::style::Style { aspect_ratio: Some(2f32), ..Default::default() },
        taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT: &str = "HHHH";
            super::measure_standard_text(
                known_dimensions,
                available_space,
                TEXT,
                super::WritingMode::Horizontal,
                Some(2f32),
            )
        }),
    );
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(20f32) },
        ..Default::default()
    });
    let node = taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Grid, ..Default::default() });
    taffy.set_children(node, &[node0, node1]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
}
