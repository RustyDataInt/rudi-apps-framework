//! Configuration constants and types for data analysis plots. This module
//! enforces many of the opinionated design decisions for RuDI plots.

// modules
mod sizes;
mod series;

// re-exports
pub use sizes::*;
pub use series::*;

// imports
use crate::ui::*;

// constants
pub const ARIAL: &str = "Arial";
pub const POINTS_90: f64 = 9.0;
pub const POINTS_80: f64 = 8.0;
pub const POINTS_725: f64 = 7.25;
pub const POINTS_675: f64 = 6.75;
pub const TICK_SIZE_PX: u32 = 12;
pub const LABEL_PADDING_PX: u32 = 20;

/// `PlotConfig` includes all plot configuration options set by the caller 
/// and/or modified by the user (see `PlotGridConfig` for calculated values).
/// These values are recorded independently in one `PlotConfig` instance for 
/// each plot in a grid.
/// 
/// At least one `series` is required to render a plot.
/// 
/// If `x_range` or `y_range` are None, axis ranges will be determined from the 
/// data.
/// 
/// If `x_label`, `y_label`, or `title` are None, the respective elements will 
/// be omitted, although their labeling areas will occupy the same space because
/// a user may subsequently add these elements via the UI.
/// 
/// `plot_area_inches` defaults to 1.25 inches wide x 1.0 inches high, NOT 
/// including the axis, title, and legend labeling areas. A canvas area without
/// a right-side legend will add about 0.35 inches to the width and height of 
/// the final plot image. 
/// 
/// `print_dpi` defaults to 600. All plots are rendered at `canvas_size` and 
/// `print_dpi` and downsized for the screen to ensure WYSIWYG image downloads.
#[derive(PartialEq, Clone)]
pub struct PlotConfig<X, Y>
where X: 'static + Clone + PartialEq + PartialOrd, 
      Y: 'static + Clone + PartialEq + PartialOrd
{
    pub series:  Vec<PlotSeries>,
    pub x_range: Option<(X, X)>,
    pub y_range: Option<(Y, Y)>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub title:   Option<String>,
    pub plot_area_inches: PrintArea,
    pub print_dpi: u16,
}
impl <X, Y> PlotConfig<X, Y>
where X: 'static + Clone + PartialEq + PartialOrd, 
      Y: 'static + Clone + PartialEq + PartialOrd
{
    /// Create a new `PlotConfig` to be filled with data series and other 
    /// drawing options. Most callers should subsequently call builder methods:
    ///     - `series()` or its shortcuts `lines()`, `points()`, etc.
    ///     - `x_label()` (by default no axis labels or title are shown)
    ///     - `y_label()`
    ///     - `title()`
    /// Depending on the nature of the plot, the following may be used to 
    /// override default behavior:
    ///     - `x_range()` (default ranges are determined from the data)
    ///     - `y_range()`
    ///     - `plot_area_inches()` (default is 1.25 x 1.0 inches)
    ///     - `print_dpi()` (default is 600)
    pub fn builder() -> Self {
        Self {
            series: vec![],
            x_range: None,
            y_range: None,
            x_label: None,
            y_label: None,
            title:   None,
            plot_area_inches: PrintArea { width: 1.25, height: 1.0 },
            print_dpi:  600,
        }
    }

    /// Add a new `PlotSeries` with the given X and Y column names and default 
    /// values for `color` (sequential from the standard palette) and `plot_as` 
    /// (`SeriesType::Scatter`).
    pub fn series_with_defaults(mut self, x: &str, y: &str, plot_as: SeriesType) -> Self {
        self.series.push(PlotSeries::new_with_defaults(x, y, plot_as));
        self
    }
    // /// Add a new `PlotSeries` with all fields specified.
    // pub fn series(
    //     mut self, 
    //     x: &str, 
    //     y: &str,
    //     color:   &str,
    //     plot_as: SeriesType
    // ) -> Self {
    //     self.series.push(PlotSeries::new(x, y, color, plot_as));
    //     self
    // }
    /// Set the title for the plot configuration (default is None).
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
    /// Set the X-axis label for the plot configuration (default is None).
    pub fn x_label(mut self, label: &str) -> Self {
        self.x_label = Some(label.to_string());
        self
    }
    /// Set the Y-axis label for the plot configuration (default is None).
    pub fn y_label(mut self, label: &str) -> Self {
        self.y_label = Some(label.to_string());
        self
    }
    /// Set the X-axis range for the plot configuration (default determines the 
    /// range from the data).
    pub fn x_range(mut self, min: X, max: X) -> Self {
        self.x_range = Some((min, max));
        self
    }
    /// Set the Y-axis range for the plot configuration (default determines the 
    /// range from the data).
    pub fn y_range(mut self, min: Y, max: Y) -> Self {
        self.y_range = Some((min, max));
        self
    }
    // /// Set the print size in inches for the plot configuration 
    // /// (default is 2.5 width x 2.0 height).
    // pub fn canvas_size(mut self, width: f32, height: f32) -> Self {
    //     self.canvas_size = PrintSize { width, height };
    //     self
    // }
    // /// Set the print margins in inches for the plot configuration 
    // /// (default is 0.5 bottom, 0.5 left, 0.25 top, 0.05 right).
    // pub fn margins(mut self, bottom: f32, left: f32, top: f32, right: f32) -> Self {
    //     self.margins = PrintMargins { bottom, left, top, right };
    //     self
    // }
    // /// Set the print DPI for the plot configuration (default is 300).
    // pub fn print_dpi(mut self, dpi: u16) -> Self {
    //     self.print_dpi = dpi;
    //     self
    // }




    // /// Get the pixel reprsentation of the print size for the plot configuration.
    // pub fn get_pixels(&self) -> PlotPixels {
    //     let dpi = self.print_dpi as f32;
    //     let panel = PixelSize {
    //         width:  (self.canvas_size.width  * dpi) as u32,
    //         height: (self.canvas_size.height * dpi) as u32,
    //     };
    //     let margins = PixelMargins {
    //         bottom: (self.margins.bottom * dpi) as u32,
    //         left:   (self.margins.left   * dpi) as u32,
    //         top:    (self.margins.top    * dpi) as u32,
    //         right:  (self.margins.right  * dpi) as u32,
    //     };
    //     let plot_width = panel.width - margins.left - margins.right;
    //     let plot_height = panel.height - margins.top - margins.bottom;
    //     let grid = PixelGrid {
    //         upper_left:  PixelSize { width: margins.left,  height: margins.top },
    //         title:       PixelSize { width: plot_width,    height: margins.top },
    //         upper_right: PixelSize { width: margins.right, height: margins.top },
    //         y_axis:      PixelSize { width: margins.left,  height: plot_height },
    //         plot:        PixelSize { width: plot_width,    height: plot_height },
    //         legend:      PixelSize { width: margins.right, height: plot_height },
    //         lower_left:  PixelSize { width: margins.left,  height: margins.bottom },
    //         x_axis:      PixelSize { width: plot_width,    height: margins.bottom },
    //         lower_right: PixelSize { width: margins.right, height: margins.bottom },
    //     };
    //     PlotPixels { 
    //         panel, 
    //         margins, 
    //         grid 
    //     }
    // }
}

/// `PlotGridConfig` includes all plot configuration values calculated from:
///     - RuDI constants
///     - a grid's n_rows and n_cols, provided by the caller
///     - as many `PlotConfigs` as needed to fill the grid
/// This structure is instantiated by framework components that build plots.
#[derive(PartialEq, Clone)]
pub struct PlotGridConfig {
    pub n_rows:  usize, // number of rows and columns in a grid of plots
    pub n_cols:  usize, // filled from top-left to bottom-right, row-major order
    pub n_plots: usize, // n_rows * n_cols
    pub dpi:     f32,   // the print DPI for all plots in the grid
    pub grid33_pixels: Vec<PrintGrid33>, // subdivision of the canvas per plot
    pub canvas_area_pixels: PrintArea, // the total canvas size over all grid plots
    pub font_size90_pixels:  u32, // the biggest recommended text size
    pub font_size80_pixels:  u32, // a bigger font for labeling items
    pub font_size725_pixels: u32, // used for axis labels and titles
    pub font_size675_pixels: u32, // used for axis tick labels
}
impl PlotGridConfig {
    /// Create a new `PlotGridConfig` from:
    ///     - RuDI constants
    ///     - a grid's `n_rows` and `n_cols`, provided by the caller
    ///     - as many `PlotConfigs` as needed to fill the grid
    pub fn new<X, Y> (
        configs: &[&PlotConfig<X, Y>],
        n_rows: usize,
        n_cols: usize,
    ) -> PlotGridConfig 
    where X: 'static + Clone + PartialEq + PartialOrd, 
          Y: 'static + Clone + PartialEq + PartialOrd
    {
        let n_plots = n_rows * n_cols;

        let dpi = configs[0].print_dpi as f32;

        let font_size90_pixels  = PlotGridConfig::font_pixels(POINTS_90,  dpi);
        let font_size80_pixels  = PlotGridConfig::font_pixels(POINTS_80,  dpi);
        let font_size725_pixels = PlotGridConfig::font_pixels(POINTS_725, dpi);
        let font_size675_pixels = PlotGridConfig::font_pixels(POINTS_675, dpi);

        let mut grid33_pixels:Vec<PrintGrid33> = Vec::new();
        let mut canvas_area_pixels = PrintArea { width: 0.0, height: 0.0 };
        let mut canvas_offset_pixels = PrintArea { width: 0.0, height: 0.0 };

        for i1 in 1..=n_rows {
            for j1 in 1..=n_cols {
                let plot_index = (i1 - 1) * n_cols + (j1 - 1);
                if plot_index < configs.len() {
                    let grid33_px = PrintGrid33::pixels_from_config(
                        &configs[plot_index], n_rows, n_cols, i1, j1,
                        canvas_offset_pixels,
                        font_size725_pixels, font_size675_pixels
                    );
                    canvas_area_pixels.width += grid33_px.canvas.width;
                    canvas_area_pixels.height += grid33_px.canvas.height;
                    if j1 == n_cols {
                        canvas_offset_pixels.width = 0.0;
                        canvas_offset_pixels.height += grid33_px.canvas.height;
                    } else {
                        canvas_offset_pixels.width += grid33_px.canvas.width;
                    }
                    grid33_pixels.push(grid33_px);
                }
            }
        }

        PlotGridConfig {
            n_rows,
            n_cols,
            n_plots,
            dpi,
            grid33_pixels,
            canvas_area_pixels,
            font_size90_pixels,
            font_size80_pixels,
            font_size725_pixels,
            font_size675_pixels,
        }
    }
    fn font_pixels(points: f64, dpi: f32) -> u32 {
        (points / POINTS_PER_INCH * dpi as f64) as u32
    }
}

// /// `PlotGridConfig` includes configuration for arranging
// /// multiple plots in a grid layout.
// #[derive(PartialEq, Clone)]
// pub struct PlotGridConfig<X, Y>
// where X: 'static + Clone + PartialEq + PartialOrd, 
//       Y: 'static + Clone + PartialEq + PartialOrd
// {
//     pub titles: Vec<(String, u32)>, // (title, left_offset in pixels)
// }
// impl <X, Y> PlotGridConfig<X, Y>
// where X: 'static + Clone + PartialEq + PartialOrd, 
//       Y: 'static + Clone + PartialEq + PartialOrd
// {
//     pub fn from_config(
//         cfgs: &[PlotConfig<X, Y>],
//         drvs: &[PlotConfig<X, Y>],
//         n_rows: u8,
//         n_cols: u8,
//     ){
//         let left_offset = 
//     }
// }
