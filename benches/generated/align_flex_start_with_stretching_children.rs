pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() },
            &[],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() },
            &[node000],
        )
        .unwrap();
    let node0 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node00]).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(500f32),
                    height: taffy::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::max_content()).unwrap();
}
