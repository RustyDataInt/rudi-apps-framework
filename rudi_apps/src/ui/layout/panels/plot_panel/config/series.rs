//! Handle the data series for a `PlotPanel`.

/// Rendering types for a plot series.
#[derive(PartialEq, Clone)]
pub enum SeriesType {
    Scatter,
    Line,
    Bar,
    Area,
}

/// Description of one XY series to plot from a `DataFrame`.
#[derive(PartialEq, Clone)]
pub struct PlotSeries {
    pub x: String, // column names in data_frame
    pub y: String,
    pub color:   Option<String>,
    pub plot_as: Option<SeriesType>,
}
impl PlotSeries {
    /// Create a new `PlotSeries` with the given X and Y column names
    /// and default values for color (sequential from the standard palette) 
    /// and plot_as (SeriesType::Scatter).
    pub fn new_with_defaults(x: &str, y: &str) -> Self {
        Self {
            x: x.to_string(),
            y: y.to_string(),
            color:   None,
            plot_as: Some(SeriesType::Scatter),
        }
    }
    /// Create a new `PlotSeries` with all fields specified.
    pub fn new(
        x: &str, 
        y: &str, 
        color:   &str, 
        plot_as: SeriesType
    ) -> Self {
        Self {
            x: x.to_string(),
            y: y.to_string(),
            color:   Some(color.to_string()),
            plot_as: Some(plot_as),
        }
    }
}
