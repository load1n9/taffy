#[test]
fn percentage_size_based_on_parent_inner_size() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.5f32),
                    height: taffy::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(20f32),
                    end: taffy::style::Dimension::Points(20f32),
                    top: taffy::style::Dimension::Points(20f32),
                    bottom: taffy::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::max_content()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 200f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 400f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 80f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 180f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 20f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 20f32);
}
