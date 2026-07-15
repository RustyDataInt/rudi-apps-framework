//! Handle the data series for a plot.

/// Drawing types for a plot data series.
#[derive(PartialEq, Clone)]
pub enum SeriesType {
    Lines,
    Points,
    Both,
    Histogram,
    Area,
    Bars, // for categorical data
}

/// Accessible line and point colors for a plot data series.
#[derive(PartialEq, Clone)]
pub enum SeriesColor {
    Blue,
    Red,
    Green,
    Black,
}
impl SeriesColor {
    /// Return the accessible RGB color values for a `SeriesColor`.
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        match self {
            SeriesColor::Blue  => (31, 119, 180),
            SeriesColor::Red   => (214, 39, 40),
            SeriesColor::Green => (44, 160, 44),
            SeriesColor::Black => (0, 0, 0),
        }
    }
    /// Return an indexed color from the standard palette for a `SeriesColor`.
    pub fn pick(index: usize) -> (u8, u8, u8) {
        match index {
            0 => SeriesColor::Blue.get_rgb(),
            1 => SeriesColor::Red.get_rgb(),
            2 => SeriesColor::Green.get_rgb(),
            _ => SeriesColor::Black.get_rgb(),
        }
    }
}

/// Rendering types for plot points.
#[derive(PartialEq, Clone)]
pub enum PointType {
    CircleFilled,
    SquareFilled,
    TriangleFilled,
    CircleOpen,
    SquareOpen,
    TriangleOpen,
    // discourage use of excessive point types...
}
impl PointType {
    // Return the accessible RGB color values for a `PointType`.
    // pub fn get_rgb(&self) -> (u8, u8, u8) {
    //     match self {
    //         PointType::CircleFilled   => (31, 119, 180),
    //         PointType::SquareFilled   => (214, 39, 40),
    //         PointType::TriangleFilled => (44, 160, 44),
    //         PointType::CircleOpen     => (31, 119, 180),
    //         PointType::SquareOpen     => (214, 39, 40),
    //         PointType::TriangleOpen   => (44, 160, 44),
    //     }
    // }
}

/// Description of one X-Y series to plot from a `DataFrame`.
#[derive(PartialEq, Clone)]
pub struct PlotSeries {

    // these fields are required from the caller
    pub x: String,
    pub y: String,
    pub plot_as: SeriesType,

    // these fields have robust defaults if not provided by the caller
    pub color: Option<SeriesColor>,
    pub point_type: PointType,
    pub opacity: f32,
    pub point_border: bool,
    pub point_size_points: f32,
    pub line_width_points: f32,
}
impl PlotSeries {

    /// Create a new `PlotSeries` where:
    /// 
    /// `x` and `y` are column names in the `DataFrame`.
    /// 
    /// `plot_as` defines the `SeriesType`, e.g., lines, points, etc.
    /// 
    /// If `color` is not provided, it will be assigned in series order from the 
    /// standard color palette defined by the `SeriesColor` enum.
    /// 
    /// If `point_type` is not provided for a `SeriesType` that needs it, 
    /// `PointType::CircleFilled` will be used.
    /// 
    /// If `opacity` is not provided for a `SeriesType` that needs it, fully 
    /// opaque points will be used.
    /// 
    /// If `point_border` is not provided for a `SeriesType` that needs it, 
    /// point borders will be used around any central fill.
    /// 
    /// If `point_size_points` is not provided for a `SeriesType` that needs it, 
    /// 2pt point symbols will be used.
    /// 
    /// If `line_width_points` is not provided for a `SeriesType` that needs it, 
    /// a 0.5pt line will be used.
    pub fn new_with_defaults(x: &str, y: &str, plot_as: SeriesType) -> Self {
        Self {
            x: x.to_string(),
            y: y.to_string(),
            plot_as: plot_as,
            color: None,
            point_type: PointType::CircleFilled,
            opacity: 1.0,
            point_border: true,
            point_size_points: 1.5,
            line_width_points: 0.5,
        }
    }

    /// Create a new `PlotSeries` with all options specified.
    pub fn new(
        x: &str, 
        y: &str, 
        plot_as: SeriesType, 
        color: Option<SeriesColor>,
        point_type: PointType,
        opacity: f32,
        point_border: bool,
        point_size_points: f32,
        line_width_points: f32
    ) -> Self {
        Self {
            x: x.to_string(),
            y: y.to_string(),
            plot_as,
            color,
            point_type,
            opacity,
            point_border,
            point_size_points,
            line_width_points,
        }
    }
}
