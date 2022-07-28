
use crate::sys::GridTrackVec;
use crate::node::Node;
use crate::geometry::Size;
use crate::style::Dimension;

struct AreaOccupancyMatrix {
  areas: Vec<u16>,
  num_rows: u16,
}


/// Track sizing function
/// 
/// Each function is either
///   - A FIXED sizing function (<length> or resolvable <percentage>).
///   - An INTRINSIC sizing function (min-content, max-content, auto, fit-content()).
///   - A FLEXIBLE sizing function (<flex>).
///
/// [Specification](https://www.w3.org/TR/css3-grid-layout/#layout-algorithm)
enum GridTrackSizingFunction {
  Fixed(Dimension),
  MinContent,
  MaxContent,
  Auto,
  Flex(f32),
}

enum GridTrackKind {
  Track,
  Gutter,
}

struct GridTrack {
  kind: GridTrackKind,
  min_track_sizing_function: GridTrackSizingFunction,
  max_track_sizing_function: GridTrackSizingFunction,
  base_size: f32,
  growth_limit: f32, // Note: can be infinity
  infinitely_growable: bool, // https://www.w3.org/TR/css3-grid-layout/#infinitely-growable
}

struct GridLine {

}

enum AvailableSpace {
  Definite(f32),
  MinContent,
  MaxContent,
}


enum GridPosition {
  Auto,
  Line(u8),
}

struct GridItem {
  node: Node,
  min_content_contribution: Option<Size<f32>>,
  max_content_contribution: Option<Size<f32>>,
  row_start: GridPosition,
  row_end: GridPosition,
  column_start: GridPosition,
  column_end: GridPosition,
}

impl GridItem {
  fn new(node: Node) -> Self {
      GridItem {
        node,
        min_content_contribution: None,
        max_content_contribution: None,
        row_start: GridPosition::Auto,
        row_end: GridPosition::Auto,
        column_start: GridPosition::Auto,
        column_end: GridPosition::Auto,
      }
  }
}

struct Grid {
  width: AvailableSpace,
  height: AvailableSpace,
  columns: GridTrackVec<GridTrack>,
  rows: GridTrackVec<GridTrack>,
  area_occupancy_matrix: AreaOccupancyMatrix,
  column_gutters: GridTrackVec<GridLine>,
  row_gutters: GridTrackVec<GridLine>,
  items: Vec<GridItem>,
}