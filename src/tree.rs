//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout

use core::any::Any;

use slotmap::DefaultKey;

use crate::{
    error::TaffyResult,
    layout::{AvailableSpace, Cache, Layout, SizingMode},
    prelude::*,
};

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indices. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = &'a DefaultKey>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn children(&self, node: Node) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, node: Node) -> usize;

    /// Returns true if the node has no children
    fn is_childless(&self, node: Node) -> bool;

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, node: Node, index: usize) -> Node;

    /// Get any available parent for this node
    fn parent(&self, node: Node) -> Option<Node>;

    // todo: allow abstractions over this so we don't prescribe how layout works
    // for reference, CSS cascades require context, and storing a full flexbox layout for each node could be inefficient
    //
    /// Get the [`Style`] for this Node.
    fn style(&self, node: Node) -> &Style;

    /// Get the node's output "Final Layout"
    fn layout(&self, node: Node) -> &Layout;

    /// Modify the node's output layout
    fn layout_mut(&mut self, node: Node) -> &mut Layout;

    /// Mark a node as dirty to tell Taffy that something has changed and it needs to be recomputed.
    ///
    /// Commonly done if the style of the node has changed.
    fn mark_dirty(&mut self, node: Node) -> TaffyResult<()>;

    /// Measure a node. Taffy uses this to force reflows of things like text and overflowing content.
    fn measure_node(
        &self,
        node: Node,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
    ) -> Size<f32>;

    /// Node needs to be measured
    fn needs_measure(&self, node: Node) -> bool;

    /// Get a cache entry for this Node by index
    fn cache_mut(&mut self, node: Node, index: usize) -> &mut Option<Cache>;
}

/// Any type implementing LayoutAlgorithm can be used by Taffy to compute a Node's layout
pub trait LayoutAlgorithm {
    /// Measure the size of this node. Taffy uses this to force reflows of things like text and overflowing content.
    fn measure<'tree, Node: LayoutNode<'tree>>(
        node: &mut Node,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform full recursive layout of this node
    fn perform_layout<'tree, Node: LayoutNode<'tree>>(
        node: &mut Node,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    // /// Calculate this node's baseline
    // fn calculate_baseline<Node: LayoutNode>(
    //     node: &mut Node,
    //     known_dimensions: Size<Option<f32>>,
    //     available_space: Size<AvailableSpace>,
    //     sizing_mode: SizingMode
    // ) -> f32;
}

/// Any item that implements Node can be laid out using Taffy's algorithms.
pub trait LayoutNode<'tree> {
    // type This : LayoutNode<'tree>;
    type Child<'subtree>: LayoutNode<'subtree> where Self : 'subtree;
    // where
        // 'tree: 'subtree,
        // Self: 'subtree,
        // Self: 'tree;

    /// Get the number of children for this node
    fn child_count(&self) -> usize;

    /// Get an immutable reference to a new instance of Self (the type implementing LayoutNode)
    // fn child<'this, 'subtree>(&'this self, index: usize) -> Self::Child<'subtree> where 'this : 'tree, 'tree : 'subtree;

    /// Get a mutable reference to a new instance of Self (the type implementing LayoutNode)
    // fn child_mut<'subtree>(&mut self, child_index: usize) -> Self::Child<'subtree> where 'tree : 'subtree;
    fn with_child_mut<'this, 'subtree, T: 'static, Callback>(&'this mut self, index: usize, callback: Callback) -> T
    where
        'this: 'tree,
        'tree: 'subtree,
        Self: 'subtree,
        Callback: FnOnce(Self::Child<'subtree>) -> T;

    /// Get the style of the type specified by the generic parameter for this node
    fn style<T: Any>(&self) -> Option<&T>;

    /// Get the node's output "Final Layout"
    fn layout(&self) -> &Layout;

    /// Modify the node's output layout
    fn layout_mut(&mut self) -> &mut Layout;

    /// Get a cache entry for this node by index
    fn cache_mut(&mut self, index: usize) -> &mut Option<Cache>;

    /// See LayoutAlgorithm trait for documentation
    fn measure(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// See LayoutAlgorithm trait for documentation
    fn perform_layout(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        // is_hidden: bool,
    ) -> Size<f32>;

    // /// See LayoutAlgorithm trait for documentation
    // fn calculate_baseline<Node: LayoutNode>(
    //     &mut self,
    //     known_dimensions: Size<Option<f32>>,
    //     available_space: Size<AvailableSpace>,
    //     sizing_mode: SizingMode
    // ) -> f32;
}
