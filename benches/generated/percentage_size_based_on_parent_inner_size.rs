pub fn compute() {
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
}
