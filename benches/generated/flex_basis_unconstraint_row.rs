pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_basis: taffy::style::Dimension::Points(50f32),
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::max_content()).unwrap();
}
