#[test]
fn measure_child_constraint_padding_parent() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy . new_leaf_with_measure (taffy :: style :: Style { .. Default :: default () } , taffy :: tree :: MeasureFunc :: Raw (| known_dimensions , available_space | { const TEXT : & str = "HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH" ; super :: measure_standard_text (known_dimensions , available_space , TEXT , super :: WritingMode :: Horizontal , None) }) ,) ;
    let node = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(50f32),
            height: taffy::style::Dimension::Auto,
        },
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::Points(10f32),
            right: taffy::style::LengthPercentage::Points(10f32),
            top: taffy::style::LengthPercentage::Points(10f32),
            bottom: taffy::style::LengthPercentage::Points(10f32),
        },
        ..Default::default()
    });
    taffy.set_children(node, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node, 50f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node0, 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0, 100f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node0, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node0, 10f32, location.y);
}
