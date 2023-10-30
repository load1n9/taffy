#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum ReturnCode {
  // Operation suceeded
  RETURN_CODE_OK,
  // The style pointer passed was null
  RETURN_CODE_NULL_STYLE_POINTER,
  // An enum value was specified that was outside the range of valid value for this enum
  RETURN_CODE_INVALID_ENUM_VALUE,
  // A Points unit was specified but is not valid in this context
  RETURN_CODE_INVALID_NONE,
  // A Points unit was specified but is not valid in this context
  RETURN_CODE_INVALID_POINTS,
  // A Percent unit was specified but is not valid in this context
  RETURN_CODE_INVALID_PERCENT,
  // A MinContent unit was specified but is not valid in this context
  RETURN_CODE_INVALID_MIN_CONTENT,
  // A MaxContent unit was specified but is not valid in this context
  RETURN_CODE_INVALID_MAX_CONTENT,
  // A FitContentPx unit was specified but is not valid in this context
  RETURN_CODE_INVALID_FIT_CONTENT_PX,
  // A FitContentPercent unit was specified but is not valid in this context
  RETURN_CODE_INVALID_FIT_CONTENT_PERCENT,
  // An Auto unit was specified but is not valid in this context
  RETURN_CODE_INVALID_AUTO,
  // An Fr unit was specified but is not valid in this context
  RETURN_CODE_INVALID_FR,
  // A NaN value was specified but is not valid in this context
  RETURN_CODE_UNEXPECTED_NA_N,
  // A infinite value was specified but is not valid in this context
  RETURN_CODE_UNEXPECTED_INFINITY,
  // A negative value was specified but is not valid in this context
  RETURN_CODE_UNEXPECTED_NEGATIVE,
} ReturnCode;

typedef enum StyleValueUnit {
  // A none value (used to unset optional fields)
  STYLE_VALUE_UNIT_NONE,
  // Fixed Length (pixel) value
  STYLE_VALUE_UNIT_LENGTH,
  // Percentage value
  STYLE_VALUE_UNIT_PERCENT,
  // Min-content size
  STYLE_VALUE_UNIT_MIN_CONTENT,
  // Max-content size
  STYLE_VALUE_UNIT_MAX_CONTENT,
  // fit-content() function with a pixel limit
  STYLE_VALUE_UNIT_FIT_CONTENT_PX,
  // fit-content() function with a percentage limit
  STYLE_VALUE_UNIT_FIT_CONTENT_PERCENT,
  // Automatic values
  STYLE_VALUE_UNIT_AUTO,
  // fr unit
  STYLE_VALUE_UNIT_FR,
} StyleValueUnit;

// Sets the distribution of space between and around content items
// For Flexbox it controls alignment in the cross axis
// For Grid it controls alignment in the block axis
//
// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
typedef enum TaffyAlignContent {
  // Items are aligned according to their algorithm-specific default value
  // This is equivalent to not setting a value in CSS
  TAFFY_ALIGN_CONTENT_NORMAL,
  // Items are packed toward the start of the axis
  TAFFY_ALIGN_CONTENT_START,
  // Items are packed toward the end of the axis
  TAFFY_ALIGN_CONTENT_END,
  // Items are packed towards the flex-relative start of the axis.
  //
  // For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
  // to End. In all other cases it is equivalent to Start.
  TAFFY_ALIGN_CONTENT_FLEX_START,
  // Items are packed towards the flex-relative end of the axis.
  //
  // For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
  // to Start. In all other cases it is equivalent to End.
  TAFFY_ALIGN_CONTENT_FLEX_END,
  // Items are centered around the middle of the axis
  TAFFY_ALIGN_CONTENT_CENTER,
  // Items are stretched to fill the container
  TAFFY_ALIGN_CONTENT_STRETCH,
  // The first and last items are aligned flush with the edges of the container (no gap)
  // The gap between items is distributed evenly.
  TAFFY_ALIGN_CONTENT_SPACE_BETWEEN,
  // The gap between the first and last items is exactly THE SAME as the gap between items.
  // The gaps are distributed evenly
  TAFFY_ALIGN_CONTENT_SPACE_EVENLY,
  // The gap between the first and last items is exactly HALF the gap between items.
  // The gaps are distributed evenly in proportion to these ratios.
  TAFFY_ALIGN_CONTENT_SPACE_AROUND,
} TaffyAlignContent;

// Used to control how child nodes are aligned.
// For Flexbox it controls alignment in the cross axis
// For Grid it controls alignment in the block axis
//
// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
typedef enum TaffyAlignItems {
  // Items are aligned according to their algorithm-specific default value
  // This is equivalent to not setting a value in CSS
  TAFFY_ALIGN_ITEMS_NORMAL,
  // Items are packed toward the start of the axis
  TAFFY_ALIGN_ITEMS_START,
  // Items are packed toward the end of the axis
  TAFFY_ALIGN_ITEMS_END,
  // Items are packed towards the flex-relative start of the axis.
  //
  // For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
  // to End. In all other cases it is equivalent to Start.
  TAFFY_ALIGN_ITEMS_FLEX_START,
  // Items are packed towards the flex-relative end of the axis.
  //
  // For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
  // to Start. In all other cases it is equivalent to End.
  TAFFY_ALIGN_ITEMS_FLEX_END,
  // Items are packed along the center of the cross axis
  TAFFY_ALIGN_ITEMS_CENTER,
  // Items are aligned such as their baselines align
  TAFFY_ALIGN_ITEMS_BASELINE,
  // Stretch to fill the container
  TAFFY_ALIGN_ITEMS_STRETCH,
} TaffyAlignItems;

// Sets the layout used for the children of this node
//
// The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
typedef enum TaffyDisplay {
  // The children will follow the block layout algorithm
  TAFFY_DISPLAY_BLOCK,
  // The children will follow the flexbox layout algorithm
  TAFFY_DISPLAY_FLEX,
  // The children will follow the CSS Grid layout algorithm
  TAFFY_DISPLAY_GRID,
  // The children will not be laid out, and will follow absolute positioning
  TAFFY_DISPLAY_NONE,
} TaffyDisplay;

typedef enum TaffyEdge {
  // The top edge of the box
  TAFFY_EDGE_TOP,
  // The bottom edge of the box
  TAFFY_EDGE_BOTTOM,
  // The left edge of the box
  TAFFY_EDGE_LEFT,
  // The right edge of the box
  TAFFY_EDGE_RIGHT,
  // Both the top and bottom edges of the box
  TAFFY_EDGE_VERTICAL,
  // Both the left and right edges of the box
  TAFFY_EDGE_HORIZONTAL,
  // All four edges of the box
  TAFFY_EDGE_ALL,
} TaffyEdge;

// The direction of the flexbox layout main axis.
//
// There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
// Adding items will cause them to be positioned adjacent to each other along the main axis.
// By varying this value throughout your tree, you can create complex axis-aligned layouts.
//
// Items are always aligned relative to the cross axis, and justified relative to the main axis.
//
// The default behavior is [`FlexDirection::Row`].
//
// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
typedef enum TaffyFlexDirection {
  // Defines +x as the main axis
  //
  // Items will be added from left to right in a row.
  TAFFY_FLEX_DIRECTION_ROW,
  // Defines +y as the main axis
  //
  // Items will be added from top to bottom in a column.
  TAFFY_FLEX_DIRECTION_COLUMN,
  // Defines -x as the main axis
  //
  // Items will be added from right to left in a row.
  TAFFY_FLEX_DIRECTION_ROW_REVERSE,
  // Defines -y as the main axis
  //
  // Items will be added from bottom to top in a column.
  TAFFY_FLEX_DIRECTION_COLUMN_REVERSE,
} TaffyFlexDirection;

// Controls whether flex items are forced onto one line or can wrap onto multiple lines.
//
// Defaults to [`FlexWrap::NoWrap`]
//
// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
typedef enum TaffyFlexWrap {
  // Items will not wrap and stay on a single line
  TAFFY_FLEX_WRAP_NO_WRAP,
  // Items will wrap according to this item's [`FlexDirection`]
  TAFFY_FLEX_WRAP_WRAP,
  // Items will wrap in the opposite direction to this item's [`FlexDirection`]
  TAFFY_FLEX_WRAP_WRAP_REVERSE,
} TaffyFlexWrap;

// Controls whether grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
//
// The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later.
// This may cause items to appear out-of-order, when doing so would fill in holes left by larger items.
//
// Defaults to [`GridAutoFlow::Row`]
//
// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow)
typedef enum TaffyGridAutoFlow {
  // Items are placed by filling each row in turn, adding new rows as necessary
  TAFFY_GRID_AUTO_FLOW_ROW,
  // Items are placed by filling each column in turn, adding new columns as necessary.
  TAFFY_GRID_AUTO_FLOW_COLUMN,
  // Combines `Row` with the dense packing algorithm.
  TAFFY_GRID_AUTO_FLOW_ROW_DENSE,
  // Combines `Column` with the dense packing algorithm.
  TAFFY_GRID_AUTO_FLOW_COLUMN_DENSE,
} TaffyGridAutoFlow;

// How children overflowing their container should affect layout
//
// In CSS the primary effect of this property is to control whether contents of a parent container that overflow that container should
// be displayed anyway, be clipped, or trigger the container to become a scroll container. However it also has secondary effects on layout,
// the main ones being:
//
//   - The automatic minimum size Flexbox/CSS Grid items with non-`Visible` overflow is `0` rather than being content based
//   - `Overflow::Scroll` nodes have space in the layout reserved for a scrollbar (width controlled by the `scrollbar_width` property)
//
// In Taffy, we only implement the layout related secondary effects as we are not concerned with drawing/painting. The amount of space reserved for
// a scrollbar is controlled by the `scrollbar_width` property. If this is `0` then `Scroll` behaves identically to `Hidden`.
//
// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
typedef enum TaffyOverflow {
  // The automatic minimum size of this node as a flexbox/grid item should be based on the size of it's content.
  TAFFY_OVERFLOW_VISIBLE,
  // The automatic minimum size of this node as a flexbox/grid item should be `0`.
  TAFFY_OVERFLOW_HIDDEN,
  // The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
  // for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
  TAFFY_OVERFLOW_SCROLL,
} TaffyOverflow;

// The positioning strategy for this item.
//
// This controls both how the origin is determined for the [`Style::position`] field,
// and whether or not the item will be controlled by flexbox's layout algorithm.
//
// WARNING: this enum follows the behavior of [CSS's `position` property](https://developer.mozilla.org/en-US/docs/Web/CSS/position),
// which can be unintuitive.
//
// [`Position::Relative`] is the default value, in contrast to the default behavior in CSS.
typedef enum TaffyPosition {
  // The offset is computed relative to the final position given by the layout algorithm.
  // Offsets do not affect the position of any other items; they are effectively a correction factor applied at the end.
  TAFFY_POSITION_RELATIVE,
  // The offset is computed relative to this item's closest positioned ancestor, if any.
  // Otherwise, it is placed relative to the origin.
  // No space is created for the item in the page layout, and its size will not be altered.
  //
  // WARNING: to opt-out of layouting entirely, you must use [`Display::None`] instead on your [`Style`] object.
  TAFFY_POSITION_ABSOLUTE,
} TaffyPosition;

typedef struct TaffyNodeId TaffyNodeId;

typedef struct TaffyStyle TaffyStyle;

typedef struct TaffyIntResult {
  enum ReturnCode return_code;
  int32_t value;
} TaffyIntResult;

typedef const struct TaffyStyle *TaffyStyleConstRef;

typedef struct TaffyStyle *TaffyStyleMutRef;

typedef struct StyleValue {
  // The value. If the unit is variant that doesn't require a value (e.g. Auto) then the value is ignored.
  float value;
  enum StyleValueUnit unit;
} StyleValue;

typedef struct TaffyStyleValueResult {
  enum ReturnCode return_code;
  struct StyleValue value;
} TaffyStyleValueResult;

typedef struct TaffyFloatResult {
  enum ReturnCode return_code;
  float value;
} TaffyFloatResult;

// For all fields, zero represents not set
typedef struct GridPlacement {
  int16_t start;
  int16_t end;
  uint16_t span;
} GridPlacement;

typedef struct TaffyGridPlacementResult {
  enum ReturnCode return_code;
  struct GridPlacement value;
} TaffyGridPlacementResult;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct TaffyIntResult TaffyStyle_GetDisplay(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetDisplay(TaffyStyleMutRef raw_style, enum TaffyDisplay value);

struct TaffyIntResult TaffyStyle_GetPosition(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetPosition(TaffyStyleMutRef raw_style, enum TaffyPosition value);

struct TaffyIntResult TaffyStyle_GetOverflowX(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetOverflowX(TaffyStyleMutRef raw_style, enum TaffyOverflow value);

struct TaffyIntResult TaffyStyle_GetOverflowY(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetOverflowY(TaffyStyleMutRef raw_style, enum TaffyOverflow value);

struct TaffyIntResult TaffyStyle_GetAlignContent(TaffyStyleConstRef raw_style);

struct TaffyIntResult TaffyStyle_GetAlignItems(TaffyStyleConstRef raw_style);

struct TaffyIntResult TaffyStyle_GetAlignSelf(TaffyStyleConstRef raw_style);

struct TaffyIntResult TaffyStyle_GetJustifyContent(TaffyStyleConstRef raw_style);

struct TaffyIntResult TaffyStyle_GetJustifyItems(TaffyStyleConstRef raw_style);

struct TaffyIntResult TaffyStyle_GetJustifySelf(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetAlignContent(TaffyStyleMutRef raw_style, enum TaffyAlignContent value);

enum ReturnCode TaffyStyle_SetAlignItems(TaffyStyleMutRef raw_style, enum TaffyAlignItems value);

enum ReturnCode TaffyStyle_SetAlignSelf(TaffyStyleMutRef raw_style, enum TaffyAlignItems value);

enum ReturnCode TaffyStyle_SetJustifyContent(TaffyStyleMutRef raw_style, enum TaffyAlignContent value);

enum ReturnCode TaffyStyle_SetJustifyItems(TaffyStyleMutRef raw_style, enum TaffyAlignItems value);

enum ReturnCode TaffyStyle_SetJustifySelf(TaffyStyleMutRef raw_style, enum TaffyAlignItems value);

struct TaffyIntResult TaffyStyle_GetFlexDirection(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetFlexDirection(TaffyStyleMutRef raw_style, enum TaffyFlexDirection value);

struct TaffyIntResult TaffyStyle_GetFlexWrap(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetFlexWrap(TaffyStyleMutRef raw_style, enum TaffyFlexWrap value);

struct TaffyIntResult TaffyStyle_GetGridAutoFlow(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetGridAutoFlow(TaffyStyleMutRef raw_style, enum TaffyGridAutoFlow value);

struct TaffyStyleValueResult TaffyStyle_GetWidth(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetWidth(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetHeight(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetHeight(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetMinWidth(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetMinWidth(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetMinHeight(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetMinHeight(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetMaxWidth(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetMaxWidth(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetMaxHeight(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetMaxHeight(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetInsetTop(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetInsetTop(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetInsetBottom(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetInsetBottom(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetInsetLeft(TaffyStyleConstRef raw_style);

struct TaffyStyleValueResult TaffyStyle_GetInsetRight(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetInsetLeft(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

enum ReturnCode TaffyStyle_SetInsetRight(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetMarginTop(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetMarginTop(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetMarginBottom(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetMarginBottom(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetMarginLeft(TaffyStyleConstRef raw_style);

struct TaffyStyleValueResult TaffyStyle_GetMarginRight(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetMarginLeft(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

enum ReturnCode TaffyStyle_SetMarginRight(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetPaddingTop(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetPaddingTop(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetPaddingBottom(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetPaddingBottom(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetPaddingLeft(TaffyStyleConstRef raw_style);

struct TaffyStyleValueResult TaffyStyle_GetPaddingRight(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetPaddingLeft(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

enum ReturnCode TaffyStyle_SetPaddingRight(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetBorderTop(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetBorderTop(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetBorderBottom(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetBorderBottom(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetBorderLeft(TaffyStyleConstRef raw_style);

struct TaffyStyleValueResult TaffyStyle_GetBorderRight(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetBorderLeft(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

enum ReturnCode TaffyStyle_SetBorderRight(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetColumnGap(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetColumnGap(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyStyleValueResult TaffyStyle_GetRowGap(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetRowGap(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyFloatResult TaffyStyle_GetAspectRatio(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetAspectRatio(TaffyStyleMutRef raw_style, float value);

struct TaffyFloatResult TaffyStyle_GetScrollbarWidth(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetScrollbarWidth(TaffyStyleMutRef raw_style, float value);

struct TaffyStyleValueResult TaffyStyle_GetFlexBasis(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetFlexBasis(TaffyStyleMutRef raw_style, float value, enum StyleValueUnit unit);

struct TaffyFloatResult TaffyStyle_GetFlexGrow(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetFlexGrow(TaffyStyleMutRef raw_style, float value);

struct TaffyFloatResult TaffyStyle_GetFlexShrink(TaffyStyleConstRef raw_style);

enum ReturnCode TaffyStyle_SetFlexShrink(TaffyStyleMutRef raw_style, float value);

// Function to set all the value of margin
enum ReturnCode TaffyStyle_SetMargin(TaffyStyleMutRef raw_style, enum TaffyEdge edge, struct StyleValue value);

// Get grid item's column placement
struct TaffyGridPlacementResult TaffyStyleGetGridColumn(TaffyStyleMutRef raw_style);

// Set grid item's column placement
enum ReturnCode TaffyStyleSetGridColumn(TaffyStyleMutRef raw_style, struct GridPlacement placement);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
