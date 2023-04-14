#[test]
fn gridflex_kitchen_sink() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
        ..Default::default()
    });
    let node00100 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
        ..Default::default()
    });
    let node00101 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node00102 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node00103 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node0010 = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        grid_template_rows: vec![percent(0.3f32), percent(0.1f32)],
        grid_template_columns: vec![auto(), percent(0.1f32)],
        ..Default::default()
    });
    taffy.set_children(node0010, &[node00100, node00101, node00102, node00103]).unwrap();
    let node001 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), height: auto() },
        ..Default::default()
    });
    taffy.set_children(node001, &[node0010]).unwrap();
    let node00 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    taffy.set_children(node00, &[node000, node001]).unwrap();
    let node01 = taffy.new_leaf_with_measure(
        taffy::style::Style { ..Default::default() },
        taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT: &str = "HH";
            super::measure_standard_text(known_dimensions, available_space, TEXT, super::WritingMode::Horizontal, None)
        }),
    );
    let node02 = taffy.new_leaf_with_measure(
        taffy::style::Style { ..Default::default() },
        taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT: &str = "HH";
            super::measure_standard_text(known_dimensions, available_space, TEXT, super::WritingMode::Horizontal, None)
        }),
    );
    let node03 = taffy.new_leaf_with_measure(
        taffy::style::Style { ..Default::default() },
        taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT: &str = "HH";
            super::measure_standard_text(known_dimensions, available_space, TEXT, super::WritingMode::Horizontal, None)
        }),
    );
    let node0 = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        grid_template_rows: vec![fr(1f32), fr(1f32)],
        grid_template_columns: vec![fr(1f32), fr(1f32)],
        ..Default::default()
    });
    taffy.set_children(node0, &[node00, node01, node02, node03]).unwrap();
    let node = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node0, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node00, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node00, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node000, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node001).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node001, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node001, 10f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node001, 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0010).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0010, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0010, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0010, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0010, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00100).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00100, 20f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00100, 3f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00101).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00101, 2f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00101, 3f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00101, 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00102).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00102, 20f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00102, 1f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00102, 0f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00102, 3f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00103).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00103, 2f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00103, 1f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00103, 20f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00103, 3f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node01, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node01, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node01, 70f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node02).unwrap();
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node02, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node02, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node02, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node03).unwrap();
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node03, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node03, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node03, 70f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
}
