//! Establish the canvas, margin, and grid quadrant sizes for a plot.

// imports
use super::*;

/// The print (not screen) size of a plot region. Depending on where it is used, 
/// a `PrintArea` may represent all or just a portion of a plot or canvas, and 
/// it may be in inches (as set by callers and users) or pixels (as set internally).
#[derive(PartialEq, Clone, Copy)]
pub struct PrintArea {
    pub width:  f32,
    pub height: f32,
}
impl PrintArea {
    /// Create a new `PrintArea` with the given width and height.
    pub fn new (width: f32, height: f32) -> Self {
        Self { 
            width: width,
            height: height,
        }
    }
}

/// Layout of the 3x3 grid covering a single plot (not a grid of plots).
#[derive(PartialEq, Clone, Copy)]
pub struct PrintGrid33 {
    // the entire canvas sub-area covered by the plot
    pub canvas:      PrintArea,
    pub canvas_offset_pixels: PrintArea, // offset of the sub-canvas from the top-left of the grid
    // subregions of the plot used to created interactive overlays
    pub upper_left:  PrintArea,
    pub title:       PrintArea,
    pub upper_right: PrintArea,
    pub y_axis:      PrintArea,
    pub plot:        PrintArea,
    pub legend:      PrintArea,
    pub lower_left:  PrintArea,
    pub x_axis:      PrintArea,
    pub lower_right: PrintArea,
    // flags for whether to draw axis labels, titles, and legends on a plot
    pub i1:            usize,
    pub j1:            usize,
    pub is_bottom_row: bool,
    pub is_left_col:   bool,
    pub is_top_row:    bool,
    pub is_right_col:  bool,
}
impl PrintGrid33 {
    /// Create a new `PrintGrid33` from a `PlotConfig`.
    pub fn pixels_from_config<X, Y>(
        config: &PlotConfig<X, Y>,
        n_rows: usize,
        n_cols: usize,
        i1:     usize,
        j1:     usize,
        canvas_offset_pixels: PrintArea,
        font_size725_pixels: u32,
        font_size675_pixels: u32,
    ) -> Self 
    where X: 'static + Clone + PartialEq + PartialOrd, 
          Y: 'static + Clone + PartialEq + PartialOrd
    {

        let axis_offset_pixels   = (
            TICK_SIZE_PX + 
            font_size675_pixels + 
            font_size725_pixels + 
            LABEL_PADDING_PX
        ) as f32;
        let title_offset_pixels  = (
            font_size725_pixels + 
            LABEL_PADDING_PX
        ) as f32;
        let legend_offset_pixels = LABEL_PADDING_PX as f32; //TODO: handle margin legends

        let plot_area_pixels = PrintArea::new(
            config.plot_area_inches.width  * config.print_dpi as f32,
            config.plot_area_inches.height * config.print_dpi as f32,
        );

        Self {
            canvas: PrintArea::new(
                axis_offset_pixels + plot_area_pixels.width  + legend_offset_pixels, 
                title_offset_pixels + plot_area_pixels.height + axis_offset_pixels
            ),
            canvas_offset_pixels,

            upper_left: PrintArea::new(
                axis_offset_pixels, 
                title_offset_pixels
            ),
            title: PrintArea::new(
                plot_area_pixels.width,
                title_offset_pixels
            ),
            upper_right: PrintArea::new(
                legend_offset_pixels, 
                title_offset_pixels
            ),

            y_axis: PrintArea::new(
                axis_offset_pixels, 
                plot_area_pixels.height
            ),
            plot: plot_area_pixels.clone(),
            legend: PrintArea::new(
                legend_offset_pixels, 
                plot_area_pixels.height
            ),

            lower_left: PrintArea::new(
                axis_offset_pixels, 
                axis_offset_pixels
            ),
            x_axis: PrintArea::new(
                plot_area_pixels.width,
                axis_offset_pixels
            ),
            lower_right: PrintArea::new(
                legend_offset_pixels, 
                axis_offset_pixels
            ),

            i1,
            j1,
            is_bottom_row: i1 == n_rows,
            is_left_col:   j1 == 1,
            is_top_row:    i1 == 1,
            is_right_col:  j1 == n_cols,
        }
    }
}
