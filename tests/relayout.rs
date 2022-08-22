use taffy::style::Dimension;

#[test]
fn relayout() {
    let mut taffy = taffy::Taffy::new();
    let node1 = taffy
        .new_leaf(taffy::style::FlexboxLayout {
            size: taffy::geometry::Size { width: Dimension::Points(8f32), height: Dimension::Points(80f32) },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                align_self: taffy::prelude::AlignSelf::Center,
                size: taffy::geometry::Size { width: Dimension::Auto, height: Dimension::Auto },
                // size: taffy::geometry::Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) },
                ..Default::default()
            },
            &[node1],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { width: Dimension::Percent(1f32), height: Dimension::Percent(1f32) },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    println!("0:");
    taffy.compute_layout(node, taffy::geometry::Size { width: 100f32.into(), height: 100f32.into() }).unwrap();
    let initial = taffy.layout(node).unwrap().location;
    let initial0 = taffy.layout(node0).unwrap().location;
    let initial1 = taffy.layout(node1).unwrap().location;
    for i in 1..10 {
        println!("\n\n{i}:");
        taffy.compute_layout(node, taffy::geometry::Size { width: 100f32.into(), height: 100f32.into() }).unwrap();
        assert_eq!(taffy.layout(node).unwrap().location, initial);
        assert_eq!(taffy.layout(node0).unwrap().location, initial0);
        assert_eq!(taffy.layout(node1).unwrap().location, initial1);
    }
}
