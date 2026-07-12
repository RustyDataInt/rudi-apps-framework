//! Configuration for a `PlotPanel`.

// modules
mod sizes;
mod series;

// re-exports
pub use sizes::*;
pub use series::*;

/// Instructions on how to render a plot from a `DataFrame`.
#[derive(PartialEq, Clone)]
pub struct PlotConfig<X, Y>
where X: 'static + Clone + PartialEq + PartialOrd, 
      Y: 'static + Clone + PartialEq + PartialOrd
{
    pub series:  Vec<PlotSeries>, // at least one series is required
    pub x_label: Option<String>,  // axis lables omitted if None
    pub y_label: Option<String>,
    pub x_range: Option<(X, X)>,  // auto-range axes if None
    pub y_range: Option<(Y, Y)>,
    pub panel_size: PrintSize,    // panel width, height in inches at print_dpi
    pub margins:    PrintMargins, // bottom, left, top, right in inches at print_dpi
    pub print_dpi:  u16, // dots per inch for printing
    pub screen_dpi: u16, // dots per inch on screen
}
impl <X, Y> PlotConfig<X, Y>
where X: 'static + Clone + PartialEq + PartialOrd, 
      Y: 'static + Clone + PartialEq + PartialOrd
{
    /// Create a new `PlotConfig` to be filled with data series
    /// and other rendering options.
    pub fn builder() -> Self {
        Self {
            series: vec![],
            x_label: None,
            y_label: None,
            x_range: None,
            y_range: None,
            panel_size: PrintSize { width: 2.5, height: 2.0 },
            margins:    PrintMargins { bottom: 0.5, left: 0.5, top: 0.25, right: 0.05 },
            print_dpi:  300, // rarely need to change
            screen_dpi: 96,
        }
    }

    /// Add a new `PlotSeries` with the given X and Y column names
    /// and default values for `color` (sequential from the standard palette) 
    /// and `plot_as` (`SeriesType::Scatter`).
    pub fn series_with_defaults(mut self, x: &str, y: &str) -> Self {
        self.series.push(PlotSeries::new_with_defaults(x, y));
        self
    }
    /// Add a new `PlotSeries` with all fields specified.
    pub fn series(
        mut self, 
        x: &str, 
        y: &str,
        color:   &str,
        plot_as: SeriesType
    ) -> Self {
        self.series.push(PlotSeries::new(x, y, color, plot_as));
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
    /// Set the X-axis range for the plot configuration
    /// (default determines the range from the data).
    pub fn x_range(mut self, min: X, max: X) -> Self {
        self.x_range = Some((min, max));
        self
    }
    /// Set the Y-axis range for the plot configuration
    /// (default determines the range from the data).
    pub fn y_range(mut self, min: Y, max: Y) -> Self {
        self.y_range = Some((min, max));
        self
    }
    /// Set the print size in inches for the plot configuration 
    /// (default is 2.5 width x 2.0 height).
    pub fn panel_size(mut self, width: f32, height: f32) -> Self {
        self.panel_size = PrintSize { width, height };
        self
    }
    /// Set the print margins in inches for the plot configuration 
    /// (default is 0.5 bottom, 0.5 left, 0.25 top, 0.05 right).
    pub fn margins(mut self, bottom: f32, left: f32, top: f32, right: f32) -> Self {
        self.margins = PrintMargins { bottom, left, top, right };
        self
    }
    /// Set the print DPI for the plot configuration (default is 300).
    pub fn print_dpi(mut self, dpi: u16) -> Self {
        self.print_dpi = dpi;
        self
    }
    /// Set the screen DPI for the plot configuration (default is 96).
    pub fn screen_dpi(mut self, dpi: u16) -> Self {
        self.screen_dpi = dpi;
        self
    }

    /// Get the pixel reprsentation of the print size for the plot configuration.
    pub fn get_pixels(&self) -> PlotPixels {
        let dpi = self.print_dpi as f32;
        let panel = PixelSize {
            width:  (self.panel_size.width  * dpi) as u32,
            height: (self.panel_size.height * dpi) as u32,
        };
        let margins = PixelMargins {
            bottom: (self.margins.bottom * dpi) as u32,
            left:   (self.margins.left   * dpi) as u32,
            top:    (self.margins.top    * dpi) as u32,
            right:  (self.margins.right  * dpi) as u32,
        };
        let plot_width = panel.width - margins.left - margins.right;
        let plot_height = panel.height - margins.top - margins.bottom;
        let grid = PixelGrid {
            upper_left:  PixelSize { width: margins.left,  height: margins.top },
            title:       PixelSize { width: plot_width,    height: margins.top },
            upper_right: PixelSize { width: margins.right, height: margins.top },
            y_axis:      PixelSize { width: margins.left,  height: plot_height },
            plot:        PixelSize { width: plot_width,    height: plot_height },
            legend:      PixelSize { width: margins.right, height: plot_height },
            lower_left:  PixelSize { width: margins.left,  height: margins.bottom },
            x_axis:      PixelSize { width: plot_width,    height: margins.bottom },
            lower_right: PixelSize { width: margins.right, height: margins.bottom },
        };
        PlotPixels { 
            panel, 
            margins, 
            grid 
        }
    }
}
