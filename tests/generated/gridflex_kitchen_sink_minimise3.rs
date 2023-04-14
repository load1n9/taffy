#[test]
fn gridflex_kitchen_sink_minimise3() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(20f32),
            height: taffy::style::Dimension::Points(20f32),
        },
        ..Default::default()
    });
    let node01 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node02 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node03 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node0 = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        grid_template_rows: vec![percent(0.3f32), percent(0.1f32)],
        grid_template_columns: vec![auto(), percent(0.1f32)],
        ..Default::default()
    });
    taffy.set_children(node0, &[node00, node01, node02, node03]).unwrap();
    let node = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), height: auto() },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node, 50f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node00, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node01, 2f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node01, 6f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node01, 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node02).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node02, 20f32, size.width);
    assert_eq!(size.height, 2f32, "height of node {:?}. Expected {}. Actual {}", node02, 2f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node02, 0f32, location.x);
    assert_eq!(location.y, 6f32, "y of node {:?}. Expected {}. Actual {}", node02, 6f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node03).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node03, 2f32, size.width);
    assert_eq!(size.height, 2f32, "height of node {:?}. Expected {}. Actual {}", node03, 2f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node03, 20f32, location.x);
    assert_eq!(location.y, 6f32, "y of node {:?}. Expected {}. Actual {}", node03, 6f32, location.y);
}
