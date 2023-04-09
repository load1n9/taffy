use rand::distributions::uniform::SampleRange;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use taffy::node::Node as TaffyNode;
use taffy::style::Style as TaffyStyle;
use taffy::Taffy as TaffyTree;

use super::{BuildTree, BuildTreeExt, GenStyle};

pub struct TaffyTreeBuilder<R: Rng, G: GenStyle<TaffyStyle>> {
    rng: R,
    style_generator: G,
    tree: TaffyTree,
    root: TaffyNode,
}

// Implement the BuildTree trait
impl<R: Rng, G: GenStyle<TaffyStyle>> BuildTree<R, G> for TaffyTreeBuilder<R, G> {
    type Tree = TaffyTree;
    type Node = TaffyNode;

    fn with_rng(mut rng: R, mut style_generator: G) -> Self {
        let mut tree = TaffyTree::new();
        let root = tree.new_leaf(style_generator.create_root_style(&mut rng)).unwrap();
        TaffyTreeBuilder { rng, style_generator, tree, root }
    }

    fn random_usize(&mut self, range: impl SampleRange<usize>) -> usize {
        self.rng.gen_range(range)
    }

    fn create_leaf_node(&mut self) -> Self::Node {
        let style = self.style_generator.create_leaf_style(&mut self.rng);
        self.tree.new_leaf(style).unwrap()
    }

    fn create_container_node(&mut self, children: &[Self::Node]) -> Self::Node {
        let style = self.style_generator.create_container_style(&mut self.rng);
        self.tree.new_with_children(style, children).unwrap()
    }

    fn total_node_count(&mut self) -> usize {
        self.tree.total_node_count()
    }

    fn set_root_children(&mut self, children: &[Self::Node]) {
        self.tree.set_children(self.root, children).unwrap();
    }

    fn into_tree_and_root(self) -> (Self::Tree, Self::Node) {
        (self.tree, self.root)
    }
}

impl<G: GenStyle<TaffyStyle>> BuildTreeExt<G> for TaffyTreeBuilder<ChaCha8Rng, G> {}
