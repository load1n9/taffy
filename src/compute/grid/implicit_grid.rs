//! This module is not required for spec compliance, but is used as a performance optimisation
//! to reduce the number of allocations required when creating a grid.
use crate::geometry::Line;
use crate::style::{GridPlacement, Style};
use core::cmp::{max, min};

use super::types::TrackCounts;
use super::util::css_grid_line_into_origin_zero_coords;

/// Estimate the number of rows and columns in the grid
/// This is used as a performance optimisation to pre-size vectors and reduce allocations. It also forms a necessary step
/// in the auto-placement
///   - The estimates for the explicit and negative implicit track counts are exact.
///   - However, the estimates for the positive explicit track count is a lower bound as auto-placement can affect this
///     in ways which are impossible to predict until the auto-placement algorithm is run.
///
/// Note that this function internally mixes use of grid track numbers and grid line numbers
pub(crate) fn compute_grid_size_estimate<'a>(
    explicit_col_count: u16,
    explicit_row_count: u16,
    child_styles_iter: impl Iterator<Item = &'a Style>,
) -> (TrackCounts, TrackCounts) {
    // Iterate over children, producing an estimate of the min and max grid lines (in origin-zero coordinates where)
    // along with the span of each itme
    let (col_min, col_max, col_max_span, row_min, row_max, row_max_span) =
        get_known_child_positions(child_styles_iter, explicit_col_count, explicit_row_count);

    // Compute *track* count estimates for each axis from:
    //   - The explicit track counts
    //   - The origin-zero coordinate min and max grid line variables
    let negative_implicit_inline_tracks = col_min.unsigned_abs();
    let explicit_inline_tracks = explicit_col_count;
    let mut positive_implicit_inline_tracks = max(explicit_col_count, col_max.unsigned_abs()) - explicit_col_count;
    let negative_implicit_block_tracks = row_min.unsigned_abs();
    let explicit_block_tracks = explicit_row_count;
    let mut positive_implicit_block_tracks = max(explicit_row_count, row_max.unsigned_abs()) - explicit_row_count;

    // In each axis, adjust positive track estimate if any items have a span that does not fit within
    // the total number of tracks in the estimate
    let tot_inline_tracks = negative_implicit_inline_tracks + explicit_inline_tracks + positive_implicit_inline_tracks;
    if tot_inline_tracks < col_max_span {
        positive_implicit_inline_tracks = col_max_span - explicit_inline_tracks - negative_implicit_inline_tracks;
    }

    let tot_block_tracks = negative_implicit_block_tracks + explicit_block_tracks + positive_implicit_block_tracks;
    if tot_block_tracks < row_max_span {
        positive_implicit_block_tracks = row_max_span - explicit_block_tracks - negative_implicit_block_tracks;
    }

    let column_counts =
        TrackCounts::from_raw(negative_implicit_inline_tracks, explicit_inline_tracks, positive_implicit_inline_tracks);

    let row_counts =
        TrackCounts::from_raw(negative_implicit_block_tracks, explicit_block_tracks, positive_implicit_block_tracks);

    (column_counts, row_counts)
}

/// Iterate over children, producing an estimate of the min and max grid *lines* along with the span of each item
///
/// Min and max grid lines are returned in origin-zero coordinates)
/// The span is measured in tracks spanned
fn get_known_child_positions<'a>(
    children_iter: impl Iterator<Item = &'a Style>,
    explicit_col_count: u16,
    explicit_row_count: u16,
) -> (i16, i16, u16, i16, i16, u16) {
    let (mut col_min, mut col_max, mut col_max_span, mut row_min, mut row_max, mut row_max_span) = (0, 0, 0, 0, 0, 0);
    children_iter.for_each(|child_style: &Style| {
        // Note: that the children reference the lines in between (and around) the tracks not tracks themselves,
        // and thus we must subtract 1 to get an accurate estimate of the number of tracks
        let (child_col_min, child_col_max, child_col_span) =
            child_min_line_max_line_span(child_style.grid_column, explicit_col_count);
        let (child_row_min, child_row_max, child_row_span) =
            child_min_line_max_line_span(child_style.grid_row, explicit_row_count);
        col_min = min(col_min, child_col_min);
        col_max = max(col_max, child_col_max);
        col_max_span = max(col_max_span, child_col_span);
        row_min = min(row_min, child_row_min);
        row_max = max(row_max, child_row_max);
        row_max_span = max(row_max_span, child_row_span);
    });

    (col_min, col_max, col_max_span, row_min, row_max, row_max_span)
}

/// Helper function for `compute_grid_size_estimate`
/// Produces a conservative estimate of the greatest and smallest grid lines used by a single grid item
///
/// Values are returned in origin-zero coordinates
#[inline]
fn child_min_line_max_line_span(line: Line<GridPlacement>, explicit_track_count: u16) -> (i16, i16, u16) {
    use GridPlacement::*;

    // 8.3.1. Grid Placement Conflict Handling
    // A. If the placement for a grid item contains two lines, and the start line is further end-ward than the end line, swap the two lines.
    // B. If the start line is equal to the end line, remove the end line.
    // C. If the placement contains two spans, remove the one contributed by the end grid-placement property.
    // D. If the placement contains only a span for a named line, replace it with a span of 1.

    let into_oz = |grid_line: i16| css_grid_line_into_origin_zero_coords(grid_line, explicit_track_count);

    let min = match (line.start, line.end) {
        // Both tracks specified
        (Line(track1), Line(track2)) => {
            let track1_oz = into_oz(track1);
            let track2_oz = into_oz(track2);
            // See rules A and B above
            if track1_oz == track2_oz {
                track1_oz
            } else {
                min(track1_oz, track2_oz)
            }
        }

        // Start track specified
        (Line(track), Auto) => into_oz(track),
        (Line(track), Span(_)) => into_oz(track),

        // End track specified
        (Auto, Line(track)) => into_oz(track),
        (Span(span), Line(track)) => into_oz(track) - (span as i16),

        // Only spans or autos
        // We ignore spans here by returning 1 which never effect the estimate as these are accounted for separately
        (Auto | Span(_), Auto | Span(_)) => 1,
    };

    let max = match (line.start, line.end) {
        // Both tracks specified
        (Line(track1), Line(track2)) => {
            let track1_oz = into_oz(track1);
            let track2_oz = into_oz(track2);
            // See rules A and B above
            if track1_oz == track2_oz {
                track1_oz + 1
            } else {
                max(track1_oz, track2_oz)
            }
        }

        // Start track specified
        (Line(track), Auto) => into_oz(track) + 1,
        (Line(track), Span(span)) => into_oz(track) + (span as i16),

        // End track specified
        (Auto, Line(track)) => into_oz(track),
        (Span(_), Line(track)) => into_oz(track),

        // Only spans or autos
        // We ignore spans here by returning 1 which never effect the estimate as these are accounted for separately
        (Auto | Span(_), Auto | Span(_)) => 1,
    };

    // Calculate span only for indefinitely placed items as we don't need for other items (whose required space will
    // be taken into account by min and max)
    let span = match (line.start, line.end) {
        (Auto | Span(_), Auto | Span(_)) => line.indefinite_span(),
        _ => 1,
    };

    (min, max, span)
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    mod test_child_min_max_line {
        use super::super::child_min_line_max_line_span;
        use crate::geometry::Line;
        use crate::style::GridPlacement::*;

        #[test]
        fn child_min_max_line_auto() {
            let (min_col, max_col, span) = child_min_line_max_line_span(Line { start: Line(5), end: Span(6) }, 6);
            assert_eq!(min_col, 4);
            assert_eq!(max_col, 10);
            assert_eq!(span, 1);
        }

        #[test]
        fn child_min_max_line_negative_track() {
            let (min_col, max_col, span) = child_min_line_max_line_span(Line { start: Line(-5), end: Span(3) }, 6);
            assert_eq!(min_col, 2);
            assert_eq!(max_col, 5);
            assert_eq!(span, 1);
        }
    }

    mod test_intial_grid_sizing {
        use super::super::compute_grid_size_estimate;
        use crate::compute::grid::util::test_helpers::*;
        use crate::style::GridPlacement::*;

        #[test]
        fn explicit_grid_sizing_with_children() {
            let explicit_col_count = 6;
            let explicit_row_count = 8;
            let child_styles = vec![
                (Line(1), Span(2), Line(2), Auto).into_grid_child(),
                (Line(-4), Auto, Line(-2), Auto).into_grid_child(),
            ];
            let (inline, block) =
                compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles.iter());
            assert_eq!(inline.negative_implicit, 0);
            assert_eq!(inline.explicit, explicit_col_count);
            assert_eq!(inline.positive_implicit, 0);
            assert_eq!(block.negative_implicit, 0);
            assert_eq!(block.explicit, explicit_row_count);
            assert_eq!(block.positive_implicit, 0);
        }

        #[test]
        fn negative_implicit_grid_sizing() {
            let explicit_col_count = 4;
            let explicit_row_count = 4;
            let child_styles = vec![
                (Line(-6), Span(2), Line(-8), Auto).into_grid_child(),
                (Line(4), Auto, Line(3), Auto).into_grid_child(),
            ];
            let (inline, block) =
                compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles.iter());
            assert_eq!(inline.negative_implicit, 1);
            assert_eq!(inline.explicit, explicit_col_count);
            assert_eq!(inline.positive_implicit, 0);
            assert_eq!(block.negative_implicit, 3);
            assert_eq!(block.explicit, explicit_row_count);
            assert_eq!(block.positive_implicit, 0);
        }
    }
}
