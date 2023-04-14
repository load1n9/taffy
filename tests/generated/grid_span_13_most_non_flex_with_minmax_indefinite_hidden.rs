#[test]
fn grid_span_13_most_non_flex_with_minmax_indefinite_hidden() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf_with_measure(
        taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Hidden, y: taffy::style::Overflow::Hidden },
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Span(13u16) },
            ..Default::default()
        },
        taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT: &str = "HHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHH";
            super::measure_standard_text(known_dimensions, available_space, TEXT, super::WritingMode::Horizontal, None)
        }),
    );
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node6 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node9 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node10 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node11 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node12 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node13 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        grid_template_rows: vec![points(40f32), points(40f32)],
        grid_template_columns: vec![
            min_content(),
            max_content(),
            fit_content(points(20f32)),
            auto(),
            points(10f32),
            percent(0.2f32),
            minmax(points(2f32), auto()),
            minmax(points(2f32), points(4f32)),
            minmax(points(2f32), min_content()),
            minmax(points(2f32), max_content()),
            minmax(min_content(), max_content()),
            minmax(min_content(), auto()),
            minmax(max_content(), auto()),
        ],
        ..Default::default()
    });
    taffy
        .set_children(
            node,
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8, node9, node10, node11, node12, node13],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 322f32, "width of node {:?}. Expected {}. Actual {}", node, 322f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node, 80f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 322f32, "width of node {:?}. Expected {}. Actual {}", node0, 322f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 16f32, "width of node {:?}. Expected {}. Actual {}", node1, 16f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node1, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 95f32, "width of node {:?}. Expected {}. Actual {}", node2, 95f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 16f32, "x of node {:?}. Expected {}. Actual {}", node2, 16f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node2, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node3, 0f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 111f32, "x of node {:?}. Expected {}. Actual {}", node3, 111f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node4, 0f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 111f32, "x of node {:?}. Expected {}. Actual {}", node4, 111f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node5, 10f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5, 40f32, size.height);
    assert_eq!(location.x, 111f32, "x of node {:?}. Expected {}. Actual {}", node5, 111f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node5, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node6).unwrap();
    assert_eq!(size.width, 64f32, "width of node {:?}. Expected {}. Actual {}", node6, 64f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node6, 40f32, size.height);
    assert_eq!(location.x, 121f32, "x of node {:?}. Expected {}. Actual {}", node6, 121f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node6, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node7).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node7, 2f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node7, 40f32, size.height);
    assert_eq!(location.x, 185f32, "x of node {:?}. Expected {}. Actual {}", node7, 185f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node7, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node8).unwrap();
    assert_eq!(size.width, 4f32, "width of node {:?}. Expected {}. Actual {}", node8, 4f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8, 40f32, size.height);
    assert_eq!(location.x, 187f32, "x of node {:?}. Expected {}. Actual {}", node8, 187f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node8, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node9).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node9, 2f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node9, 40f32, size.height);
    assert_eq!(location.x, 191f32, "x of node {:?}. Expected {}. Actual {}", node9, 191f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node9, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node10, 2f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node10, 40f32, size.height);
    assert_eq!(location.x, 193f32, "x of node {:?}. Expected {}. Actual {}", node10, 193f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node10, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node11).unwrap();
    assert_eq!(size.width, 16f32, "width of node {:?}. Expected {}. Actual {}", node11, 16f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node11, 40f32, size.height);
    assert_eq!(location.x, 195f32, "x of node {:?}. Expected {}. Actual {}", node11, 195f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node11, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node12).unwrap();
    assert_eq!(size.width, 15f32, "width of node {:?}. Expected {}. Actual {}", node12, 15f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node12, 40f32, size.height);
    assert_eq!(location.x, 211f32, "x of node {:?}. Expected {}. Actual {}", node12, 211f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node12, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node13).unwrap();
    assert_eq!(size.width, 96f32, "width of node {:?}. Expected {}. Actual {}", node13, 96f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node13, 40f32, size.height);
    assert_eq!(location.x, 226f32, "x of node {:?}. Expected {}. Actual {}", node13, 226f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node13, 40f32, location.y);
}
