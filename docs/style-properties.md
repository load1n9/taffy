# Style Properties

Y = Supported in spec and implemented in Taffy
~Y = Implemented in Taffy, but not thoroughly tested
N = Supported in spec but not implemented in Taffy
\- = Not applicable to layout mode
1-5 = Priorities for a phased implementation of CSS Grid

| Property                     | Flex                                                                                        | Grid | Type                                  | Stack | Heap   | Description                                                                                 |
| ---                          | ---                                                                                         | ---  | ---                                   | ---   | -      | ---                                                                                         |
| **Layout Mode**              |                                                                                             |      |                                       |       |        |                                                                                             |
| `display`                    | Y                                                                                           | Y    | `Display`                             | 1     | -      | What layout strategy should be used?                                                        |
| **Position (Child)**         |                                                                                             |      |                                       |       | -      |                                                                                             |
| `position`                   | Y                                                                                           | Y    | `Position`                            | 1     | -      | Absolute vs. in-flow position                                                               |
| `inset`                      | Y                                                                                           | Y    | `Rect<LengthPercentageAuto>`          | 32    | -      | How should the position of this element be tweaked relative to the layout defined?          |
| **Item size**                |                                                                                             |      |                                       |       |        |                                                                                             |
| `size`                       | Y                                                                                           | Y    | `Size<Dimension>`                     | 16    | -      | The nominal height and width of item                                                        |
| `min_size`                   | Y                                                                                           | Y    | `Size<Dimension>`                     | 16    | -      | The minimum height and width of the item                                                    |
| `max_size`                   | Y                                                                                           | Y    | `Size<Dimension>`                     | 16    | -      | The maximum height and width of the item                                                    |
| `aspect_ratio`               | Y                                                                                           | 3    | `f32`                                 | 4     | -      | The preferred aspect ratio (calculated as width divided by height)                          |
| **Item spacing**             |                                                                                             |      |                                       |       |        |                                                                                             |
| `padding`                    | Y                                                                                           | ~Y   | `Rect<LengthPercentage>`              | 32    | -      | How large should the padding be on each side?                                               |
| `border`                     | Y                                                                                           | ~Y   | `Rect<LengthPercentage>`              | 32    | -      | How large should the border be on each side?                                                |
| `margin`                     | Y                                                                                           | ~Y   | `Rect<LengthPercentageAuto>`          | 32    | -      | How large should the margin be on each side?                                                |
| **Item spacing (Container)** |                                                                                             |      |                                       |       |        |                                                                                             |
| `gap`                        | Y                                                                                           | Y    | `Size<LengthPercentage>`              | 16    | -      | The size of the vertical and horizontal gaps between flex items / grid rows                 |
| **Alignment (Container)**    |                                                                                             |      |                                       |       |        |                                                                                             |
| `align_content`              | Y                                                                                           | Y    | `AlignContent`                        | 1     | -      | How should content contained within this item be aligned relative to the cross axis?        |
| `justify_content`            | Y                                                                                           | Y    | `AlignContent`                        | 1     | -      | How should content contained within this item be aligned relative to the main axis?         |
| `align_items`                | Y                                                                                           | Y    | `AlignItems`                          | 1     | -      | How should items be aligned relative to the cross axis?                                     |
| `justify_items`              | -                                                                                           | Y    | `AlignItems`                          | 1     | -      | How should items be aligned relative to the main axis?                                      |
| **Alignment (Child)**        |                                                                                             |      |                                       |       |        |                                                                                             |
| `align_self`                 | Y                                                                                           | Y    | `Option<AlignItems>`                  | 1     | -      | Should this item violate the cross axis alignment specified by its parent's [`AlignItems`]? |
| `justify_self`               | -                                                                                           | Y    | `Option<AlignItems>`                  | 1     | -      | Should this item violate the main axis alignment specified by its parent's [`AlignItems`]?  |
| **Flexbox (Container)**      |                                                                                             |      |                                       |       |        |                                                                                             |
| `flex_direction`             | Y                                                                                           | -    | `FlexDirection`                       | 1     | -      | Which direction does the main axis flow in?                                                 |
| `flex_wrap`                  | Y                                                                                           | -    | `FlexWrap`                            | 1     | -      | Should elements wrap, or stay in a single line?                                             |
| **Flexbox (Child)**          |                                                                                             |      |                                       |       |        |                                                                                             |
| `flex_basis`                 | Y                                                                                           | -    | `Dimension`                           | 8     | -      | Sets the initial main axis size of the item                                                 |
| `flex_grow`                  | Y                                                                                           | -    | `f32`                                 | 4     | -      | The relative rate at which this item grows when it is expanding to fill space               |
| `flex_shrink`                | Y                                                                                           | -    | `f32`                                 | 4     | -      | The relative rate at which this item shrinks when it is contracting to fit into space       |
| **CSS Grid (Container)**     |                                                                                             |      |                                       |       |        |                                                                                             |
| `grid_template_columns`      | -                                                                                           | Y    | `Vec<TrackSizingFunction>`            | 24    | 32 * N | The track sizing functions of the grid's explicit columns                                   |
| `grid_template_rows`         | -                                                                                           | Y    | `Vec<TrackSizingFunction>`            | 24    | 32 * N | The track sizing functions of the grid's explicit rows                                      |
| `grid_template_areas`        | -                                                                                           | 5    | -                                     | -     | -      | Defines named grid areas                                                                    |
| `grid_auto_rows`             | -                                                                                           | Y    | `Vec<NonRepeatedTrackSizingFunction>` | 24    | 20 * N | Track sizing functions for the grid's implicitly generated rows                             |
| `grid_auto_columns`          | -                                                                                           | Y    | `Vec<NonRepeatedTrackSizingFunction>` | 24    | 20 * N | Track sizing functions for the grid's implicitly generated columns                          |
| `grid_auto_flow`             | -                                                                                           | Y    | `GridAutoFlow`                        | 1     | -      | Whether auto-placed items are placed row-wise or column-wise. And sparsely or densely.      |
| **CSS Grid (Child)**         |                                                                                             |      |                                       |       |        |                                                                                             |
| `grid_row`                   | -                                                                                           | Y    | `Line<GridPlacement>`                 | 8     | -      | The vertical (row) placement of a grid item                                                 |
| `grid_column`                | -                                                                                           | Y    | `Line<GridPlacement>`                 | 8     | -      | The horizontal (row) placement of a grid item                                               |
| `grid_area`                  | -                                                                                           | 5    | -                                     | -     | -      | Accepts either shorthand row/column-start/end or a named grid area                          |


## Grouped by node type

The `display` style gets it only category.

### Common Styles

| `size`                  | The nominal height and width of item                                                        |
| `min_size`              | The minimum height and width of the item                                                    |
| `max_size`              | The maximum height and width of the item                                                    |
| `padding`               | How large should the padding be on each side?                                               |
| `border`                | How large should the border be on each side?                                                |
| `margin`                | How large should the margin be on each side?                                                |
| `aspect_ratio`          | The preferred aspect ratio (calculated as width divided by height)                          |

### Flex Container Styles

| `flex_direction`        | Which direction does the main axis flow in?                                                 |
| `flex_wrap`             | Should elements wrap, or stay in a single line?                                             |
| `gap`                   | The size of the vertical and horizontal gaps between flex items / grid rows                 |
| `align_content`         | How should content contained within this item be aligned relative to the cross axis?        |
| `justify_content`       | How should content contained within this item be aligned relative to the main axis?         |
| `align_items`           | How should items be aligned relative to the cross axis?                                     |

### Flex Item Styles

| `flex_basis`            | Sets the initial main axis size of the item                                                 |
| `flex_grow`             | The relative rate at which this item grows when it is expanding to fill space               |
| `flex_shrink`           | The relative rate at which this item shrinks when it is contracting to fit into space       |
| `align_self`            | Should this item violate the cross axis alignment specified by its parent's [`AlignItems`]? |
| `position`              | Absolute vs. in-flow position                                                               |
| `inset`                 | How should the position of this element be tweaked relative to the layout defined?          |

### Grid Container Styles

| `grid_template_columns` | The track sizing functions of the grid's explicit columns                                   |
| `grid_template_rows`    | The track sizing functions of the grid's explicit rows                                      |
| `grid_template_areas`   | Defines named grid areas                                                                    |
| `grid_auto_rows`        | Track sizing functions for the grid's implicitly generated rows                             |
| `grid_auto_columns`     | Track sizing functions for the grid's implicitly generated columns                          |
| `grid_auto_flow`        | Whether auto-placed items are placed row-wise or column-wise. And sparsely or densely.      |
| `gap`                   | The size of the vertical and horizontal gaps between flex items / grid rows                 |
| `align_content`         | How should content contained within this item be aligned relative to the cross axis?        |
| `justify_content`       | How should content contained within this item be aligned relative to the main axis?         |
| `align_items`           | How should items be aligned relative to the cross axis?                                     |
| `justify_items`         | How should items be aligned relative to the main axis?                                      |

### Grid Item Styles

| `grid_row`                   | The vertical (row) placement of a grid item                                                 |
| `grid_column`                | The horizontal (row) placement of a grid item                                               |
| `grid_area`                  | Accepts either shorthand row/column-start/end or a named grid area                          |
| `align_self`                 | Should this item violate the cross axis alignment specified by its parent's [`AlignItems`]? |
| `justify_self`               | Should this item violate the main axis alignment specified by its parent's [`AlignItems`]?  |
| `position`                   | Absolute vs. in-flow position                                                               |
| `inset`                      | How should the position of this element be tweaked relative to the layout defined?          |