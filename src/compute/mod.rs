//! The layout algorithms themselves

pub(crate) mod common;
pub(crate) mod leaf;

#[cfg(feature = "block_layout")]
pub(crate) mod block;

#[cfg(feature = "flexbox")]
pub(crate) mod flexbox;

#[cfg(feature = "grid")]
pub(crate) mod grid;

pub use leaf::compute_leaf_layout;

#[cfg(feature = "block_layout")]
pub use self::block::compute_block_layout;

#[cfg(feature = "flexbox")]
pub use self::flexbox::compute_flexbox_layout;

#[cfg(feature = "grid")]
pub use self::grid::compute_grid_layout;

use crate::geometry::{Line, Point, Size};
use crate::style::AvailableSpace;
use crate::tree::{Layout, LayoutInput, LayoutOutput, LayoutTree, LayoutTreeExt, NodeId, RunMode, SizingMode};
use crate::util::debug::{debug_log, debug_log_node, debug_pop_node, debug_push_node};
use crate::util::sys::round;

/// Updates the stored layout of the provided `node` and its children
pub fn compute_layout(
    tree: &mut impl LayoutTree,
    root: NodeId,
    available_space: Size<AvailableSpace>,
    use_rounding: bool,
) {
    // Recursively compute node layout
    let size_and_baselines = tree.perform_child_layout(
        root,
        Size::NONE,
        available_space.into_options(),
        available_space,
        SizingMode::InherentSize,
        Line::FALSE,
    );

    let layout = Layout { order: 0, size: size_and_baselines.size, location: Point::ZERO };
    *tree.unrounded_layout_mut(root) = layout;
    *tree.final_layout_mut(root) = layout;

    // If rounding is enabled, recursively round the layout's of this node and all children
    if use_rounding {
        round_layout(tree, root, 0.0, 0.0);
    }
}

/// Updates the stored layout of the provided `node` and its children
pub(crate) fn compute_cached_layout<Tree: LayoutTree + ?Sized>(
    tree: &mut Tree,
    node: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput {
    debug_push_node!(node);

    let LayoutInput { known_dimensions, available_space, run_mode, .. } = inputs;
    let has_children = tree.child_count(node) > 0;

    // First we check if we have a cached result for the given input
    let cache_run_mode = if !has_children { RunMode::PerformLayout } else { run_mode };
    let cache_entry = tree.cache_mut(node).get(known_dimensions, available_space, cache_run_mode);
    if let Some(cached_size_and_baselines) = cache_entry {
        debug_log!("CACHE", dbg:cached_size_and_baselines.size);
        debug_log_node!(known_dimensions, parent_size, available_space, run_mode, sizing_mode);
        debug_pop_node!();
        return cached_size_and_baselines;
    }

    let computed_size_and_baselines = tree.compute_child_layout(node, inputs);

    // Cache result
    tree.cache_mut(node).store(known_dimensions, available_space, cache_run_mode, computed_size_and_baselines);

    debug_log!("RESULT", dbg:computed_size_and_baselines.size);
    debug_pop_node!();

    computed_size_and_baselines
}

/// Rounds the calculated [`Layout`] to exact pixel values
///
/// In order to ensure that no gaps in the layout are introduced we:
///   - Always round based on the cumulative x/y coordinates (relative to the viewport) rather than
///     parent-relative coordinates
///   - Compute width/height by first rounding the top/bottom/left/right and then computing the difference
///     rather than rounding the width/height directly
/// See <https://github.com/facebook/yoga/commit/aa5b296ac78f7a22e1aeaf4891243c6bb76488e2> for more context
///
/// In order to prevent innacuracies caused by rounding already-rounded values, we read from `unrounded_layout`
/// and write to `final_layout`.
pub fn round_layout(tree: &mut impl LayoutTree, node_id: NodeId, cumulative_x: f32, cumulative_y: f32) {
    let unrounded_layout = *tree.unrounded_layout_mut(node_id);
    let layout = &mut tree.final_layout_mut(node_id);

    let cumulative_x = cumulative_x + unrounded_layout.location.x;
    let cumulative_y = cumulative_y + unrounded_layout.location.y;

    layout.location.x = round(unrounded_layout.location.x);
    layout.location.y = round(unrounded_layout.location.y);
    layout.size.width = round(cumulative_x + unrounded_layout.size.width) - round(cumulative_x);
    layout.size.height = round(cumulative_y + unrounded_layout.size.height) - round(cumulative_y);

    let child_count = tree.child_count(node_id);
    for index in 0..child_count {
        let child = tree.child(node_id, index);
        round_layout(tree, child, cumulative_x, cumulative_y);
    }
}

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
pub fn compute_hidden_layout(tree: &mut impl LayoutTree, node: NodeId) -> LayoutOutput {
    /// Recursive function to apply hidden layout to all descendents
    fn compute_hidden_layout_inner(tree: &mut impl LayoutTree, node: NodeId, order: u32) {
        *tree.unrounded_layout_mut(node) = Layout::with_order(order);
        *tree.final_layout_mut(node) = Layout::with_order(order);
        tree.cache_mut(node).clear();
        for order in 0..tree.child_count(node) {
            compute_hidden_layout_inner(tree, tree.child(node, order), order as _);
        }
    }

    for order in 0..tree.child_count(node) {
        compute_hidden_layout_inner(tree, tree.child(node, order), order as _);
    }

    LayoutOutput::HIDDEN
}

#[cfg(test)]
mod tests {
    use super::compute_hidden_layout;
    use crate::geometry::{Point, Size};
    use crate::style::{Display, Style};
    use crate::Taffy;

    #[test]
    fn hidden_layout_should_hide_recursively() {
        let mut taffy: Taffy<()> = Taffy::new();

        let style: Style = Style { display: Display::Flex, size: Size::from_lengths(50.0, 50.0), ..Default::default() };

        let grandchild_00 = taffy.new_leaf(style.clone()).unwrap();
        let grandchild_01 = taffy.new_leaf(style.clone()).unwrap();
        let child_00 = taffy.new_with_children(style.clone(), &[grandchild_00, grandchild_01]).unwrap();

        let grandchild_02 = taffy.new_leaf(style.clone()).unwrap();
        let child_01 = taffy.new_with_children(style.clone(), &[grandchild_02]).unwrap();

        let root = taffy
            .new_with_children(
                Style { display: Display::None, size: Size::from_lengths(50.0, 50.0), ..Default::default() },
                &[child_00, child_01],
            )
            .unwrap();

        compute_hidden_layout(&mut taffy.as_layout_tree(), root.into());

        // Whatever size and display-mode the nodes had previously,
        // all layouts should resolve to ZERO due to the root's DISPLAY::NONE
        for (node, _) in taffy.nodes.iter().filter(|(node, _)| *node != root.into()) {
            if let Ok(layout) = taffy.layout(node.into()) {
                assert_eq!(layout.size, Size::zero());
                assert_eq!(layout.location, Point::zero());
            }
        }
    }
}
