//! Establish the plot, margin, and grid quadrant sizes for a `PlotPanel`.

/// Description of the print (not screen) size of a plot panel, 
/// provided as inches at `print_dpi`, including the `PrintMargins`.
#[derive(PartialEq, Clone)]
pub struct PrintSize {
    pub width:  f32,
    pub height: f32,
}

/// Description of the print (not screen) margins for a plot panel, 
/// provided as inches at `print_dpi`.
#[derive(PartialEq, Clone)]
pub struct PrintMargins {
    pub bottom: f32, // in R-like order
    pub left:   f32,
    pub top:    f32,
    pub right:  f32,
}

/// Internal conversion of PrintSize and PrintMargins to pixels.
#[derive(PartialEq, Clone)]
pub struct PixelSize {
    pub width:  u32,
    pub height: u32,
}

#[derive(PartialEq, Clone)]
pub struct PixelMargins {
    pub bottom: u32, // in R-like order
    pub left:   u32,
    pub top:    u32,
    pub right:  u32,
}

#[derive(PartialEq, Clone)]
pub struct PixelGrid {
    pub upper_left:  PixelSize,
    pub title:       PixelSize,
    pub upper_right: PixelSize,
    pub y_axis:      PixelSize,
    pub plot:        PixelSize,
    pub legend:      PixelSize,
    pub lower_left:  PixelSize,
    pub x_axis:      PixelSize,
    pub lower_right: PixelSize,
}
#[derive(PartialEq, Clone)]
pub struct PlotPixels {
    pub panel:   PixelSize,
    pub margins: PixelMargins,
    pub grid:    PixelGrid,
}
