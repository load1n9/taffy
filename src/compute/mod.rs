//! The layout algorithms themselves

pub(crate) mod flexbox;
pub(crate) mod leaf;

use crate::error::TaffyError;
use crate::geometry::{Point, Size};
use crate::layout::{AvailableSpace, Cache, Layout, RunMode, SizingMode};
use crate::style::{Display, Style};
use crate::sys::round;
use crate::tree::LayoutNode;

#[cfg(feature = "debug")]
use crate::debug::NODE_LOGGER;

/// Updates the stored layout of the provided `node` and its children
pub fn compute_layout<'tree>(
    root: &mut impl LayoutNode<'tree>,
    available_space: Size<AvailableSpace>,
) -> Result<(), TaffyError> {
    // Recursively compute node layout
    let size = compute_node_layout(root, Size::NONE, available_space, RunMode::PeformLayout, SizingMode::InherentSize);

    let layout = Layout { order: 0, size, location: Point::ZERO };
    *root.layout_mut() = layout;

    // Recursively round the layout's of this node and all children
    round_layout(root, 0.0, 0.0);

    Ok(())
}

/// Updates the stored layout of the provided `node` and its children
pub(crate) fn compute_node_layout<'node, 'tree: 'node>(
    node: &'node mut impl LayoutNode<'tree>,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
) -> Size<f32> {
    #[cfg(feature = "debug")]
    NODE_LOGGER.push_node(node);
    #[cfg(feature = "debug")]
    println!();

    // First we check if we have a cached result for the given input
    let cache_run_mode = if node.child_count() == 0 { RunMode::PeformLayout } else { run_mode };
    if let Some(cached_size) = compute_from_cache(node, known_dimensions, available_space, cache_run_mode, sizing_mode)
    {
        #[cfg(feature = "debug")]
        NODE_LOGGER.labelled_debug_log("CACHE", cached_size);
        #[cfg(feature = "debug")]
        NODE_LOGGER.labelled_debug_log("run_mode", run_mode);
        #[cfg(feature = "debug")]
        NODE_LOGGER.labelled_debug_log("sizing_mode", sizing_mode);
        #[cfg(feature = "debug")]
        NODE_LOGGER.labelled_debug_log("known_dimensions", known_dimensions);
        #[cfg(feature = "debug")]
        NODE_LOGGER.labelled_debug_log("available_space", available_space);
        #[cfg(feature = "debug")]
        NODE_LOGGER.pop_node();
        return cached_size;
    }

    #[cfg(feature = "debug")]
    NODE_LOGGER.log("COMPUTE");
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("run_mode", run_mode);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("sizing_mode", sizing_mode);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("known_dimensions", known_dimensions);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("available_space", available_space);

    // Attempt to shortcut size computation based on
    //  - KnownSize sizing constraints
    //  - The node's preferred sizes (width/heights) styles and AvailableSpace sizing constraints
    // (percentages resolve to pixel values if there is a definite AvailableSpace sizing constraint)
    // let style = tree.style(node);
    // // let known_node_size = available_space.
    // //     .zip_map(style.size, |constraint, style_size| style_size.maybe_resolve(constraint.as_option()));
    // let known_node_size = style
    //     .size
    //     .maybe_resolve(available_space.as_options())
    //     .zip_map(known_dimensions, |style_size, known_dimensions| known_dimensions.or(style_size));
    // if run_mode == RunMode::ComputeSize && known_node_size.width.is_some() && known_node_size.height.is_some() {
    //     let node_min_size = style.min_size.maybe_resolve(available_space.as_options());
    //     let node_max_size = style.max_size.maybe_resolve(available_space.as_options());
    //     return Size {
    //         width: known_node_size.width.maybe_max(node_min_size.width).maybe_min(node_max_size.width).unwrap_or(0.0),
    //         height: known_node_size
    //             .height
    //             .maybe_max(node_min_size.height)
    //             .maybe_min(node_max_size.height)
    //             .unwrap_or(0.0),
    //     };
    // }

    // If this is a leaf node we can skip a lot of this function in some cases
    let computed_size = if node.child_count() == 0 {
        #[cfg(feature = "debug")]
        NODE_LOGGER.log("Algo: leaf");
        self::leaf::compute(node, known_dimensions, available_space, run_mode, sizing_mode)
    } else {
        // println!("match {:?}", tree.style(node).display);
        match node.style::<Style>().unwrap().display {
            Display::Flex => {
                #[cfg(feature = "debug")]
                NODE_LOGGER.log("Algo: flexbox");
                self::flexbox::compute(node, known_dimensions, available_space, run_mode)
            }
            Display::None => {
                #[cfg(feature = "debug")]
                NODE_LOGGER.log("Algo: none");
                Size { width: 0.0, height: 0.0 }
            }
        }
    };

    // Cache result
    let cache_slot = (known_dimensions.width.is_some() as usize) + (known_dimensions.height.is_some() as usize * 2);
    *node.cache_mut(cache_slot) =
        Some(Cache { known_dimensions, available_space, run_mode: cache_run_mode, cached_size: computed_size });

    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("RESULT", computed_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.pop_node();

    computed_size
}

/// Try to get the computation result from the cache.
#[inline]
fn compute_from_cache<'tree>(
    node: &mut impl LayoutNode<'tree>,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
) -> Option<Size<f32>> {
    for idx in 0..4 {
        let entry = node.cache_mut(idx);
        #[cfg(feature = "debug")]
        NODE_LOGGER.labelled_debug_log("cache_entry", &entry);
        if let Some(entry) = entry {
            // Cached ComputeSize results are not valid if we are running in PerformLayout mode
            if entry.run_mode == RunMode::ComputeSize && run_mode == RunMode::PeformLayout {
                return None;
            }

            // if known_dimensions.width == entry.known_dimensions.width
            // && known_dimensions.height == entry.known_dimensions.height
            if (known_dimensions.width == entry.known_dimensions.width || known_dimensions.width == Some(entry.cached_size.width))
                && (known_dimensions.height == entry.known_dimensions.height || known_dimensions.height == Some(entry.cached_size.height))
                // && entry.available_space.width.is_roughly_equal(available_space.width)
                // && entry.available_space.height.is_roughly_equal(available_space.height)
                && (
                  known_dimensions.width.is_some()
                  || entry.available_space.width.is_roughly_equal(available_space.width)
                  || (sizing_mode == SizingMode::ContentSize && available_space.width.is_definite() && available_space.width.unwrap() >= entry.cached_size.width)
                )
                && (
                  known_dimensions.height.is_some()
                  || entry.available_space.height.is_roughly_equal(available_space.height)
                  || (sizing_mode == SizingMode::ContentSize && available_space.height.is_definite() && available_space.height.unwrap() >= entry.cached_size.height)
                )
            // && (entry.available_space.width.is_roughly_equal(available_space.width) || (available_space.width.is_definite() && available_space.width.unwrap() >= entry.cached_size.width))
            // && (entry.available_space.height.is_roughly_equal(available_space.height) || (available_space.height.is_definite() && available_space.height.unwrap() >= entry.cached_size.height))
            {
                return Some(entry.cached_size);
            }
        }
    }

    None
}

/// Rounds the calculated [`NodeData`] according to the spec
fn round_layout<'tree>(node: &'tree mut impl LayoutNode<'tree>, abs_x: f32, abs_y: f32) {
    let layout = node.layout_mut();
    let abs_x = abs_x + layout.location.x;
    let abs_y = abs_y + layout.location.y;

    layout.location.x = round(layout.location.x);
    layout.location.y = round(layout.location.y);

    layout.size.width = round(layout.size.width);
    layout.size.height = round(layout.size.height);

    // Satisfy the borrow checker here by re-indexing to shorten the lifetime to the loop scope
    for x in 0..node.child_count() {
        node.with_child_mut(x, move |child| round_layout(&mut child, abs_x, abs_y));
    }
}
