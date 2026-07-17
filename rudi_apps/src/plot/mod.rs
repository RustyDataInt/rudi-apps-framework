//! Handle plot, i.e., data chart, creation using plotters. These functions 
//! apply specifically to creating and drawing on a canvas, not wrapping that 
//! canvas into a display panel.

// modules
mod config;

// re-exports
pub use config::*;

// imports
use std::ops::Range;
use std::fmt::Debug;
use std::error::Error;
use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use plotters::prelude::*;
use plotters::coord::{ranged1d::{AsRangedCoord, ValueFormatter}, Shift};
use plotters_canvas::CanvasBackend;
use rlike::data_frame::prelude::*;
use rlike::data_frame::column::get::ColVec;
use crate::ui::*;
use crate::server::*;

/// Create an HTML canvas element and set a signal with its id once it is 
/// mounted in DOM. 
#[component]
pub fn PlotCanvas(
    grid: ReadSignal<Option<PlotGridConfig>>,
    canvas_id: Signal<Option<String>>,
) -> Element {
    let this = RudiElement::new::<()>("canvas");
    rsx!{
        if let Some(grid) = &*grid.read() {
            canvas {
                id: this.id,
                class: "plot-panel-canvas",
                width: "{grid.canvas_area_pixels.width}",
                height: "{grid.canvas_area_pixels.height}",
                onmounted: move |element| {
                    let web_sys_element = element.as_web_event();

                    //////////////////////
                    log::info!("Canvas mounted with id: {}", web_sys_element.id());

                    canvas_id.set(Some(web_sys_element.id()));
                },
            }
        }
    }
}

/// Draw canvas-level (titles, legends) and chart-level elements (series, axes)
/// onto an HTML canvas element.
pub fn draw_plots<X, Y> (
    data_frame: &DataSource,
    configs:    &[&PlotConfig<X, Y>], // one or more plot configs for a grid of plots
    grid:       &PlotGridConfig, // grid layout configuration with derived values
    canvas_id:  &str, // CSS id of the HMTL canvas element to draw on
) -> Result<(), Box<dyn Error>>
where X: 'static + Default + Clone + Copy + PartialEq + PartialOrd + Debug,
      Y: 'static + Default + Clone + Copy + PartialEq + PartialOrd + Debug,
      Range<X>: AsRangedCoord<Value = X>,
      Range<Y>: AsRangedCoord<Value = Y>,
      <Range<X> as AsRangedCoord>::CoordDescType: ValueFormatter<X>,
      <Range<Y> as AsRangedCoord>::CoordDescType: ValueFormatter<Y>,
      Vec<Option<X>>: ColVec,
      Vec<Option<Y>>: ColVec,
{
    ServerData::with_data_source::<DataFrame, (), _>(&data_frame, |df| {
        let backend = CanvasBackend::new(canvas_id)
            .ok_or("could not create canvas backend")?;
        let canvas = backend.into_drawing_area();
        canvas.fill(&WHITE)?;
        add_canvas_elements(configs, grid, &canvas)?;
        let areas = split_by_breakpointsplot(grid, canvas);
        add_charts(df, configs, grid, areas)?;
        Ok(())
    })
}

/// Draw canvas-level elements, i.e., titles, legends.
fn add_canvas_elements<X, Y>(
    configs: &[&PlotConfig<X, Y>],
    grid:    &PlotGridConfig, 
    canvas:  &DrawingArea<CanvasBackend, Shift>
) -> Result<(), Box<dyn Error>>
where X: 'static + Clone + PartialOrd,
      Y: 'static + Clone + PartialOrd,
{
    let title_style = (ARIAL, grid.font_size725_pixels).into_text_style(canvas);
    for i in 0..grid.n_plots {
        let cfg = configs[i];
        let g33 = &grid.grid33_pixels[i];
        add_plot_titles(g33, cfg, canvas, &title_style)?;
        add_plot_legends(g33, cfg, canvas)?;
    }
    Ok(())
}

/// Add titles to all top-row plots.
fn add_plot_titles<X, Y>(
    g33:    &PrintGrid33,
    cfg:    &PlotConfig<X, Y>,
    canvas: &DrawingArea<CanvasBackend, Shift>,
    style:  &TextStyle,
) -> Result<(), Box<dyn Error>>
where X: 'static + Clone + PartialOrd,
      Y: 'static + Clone + PartialOrd
{
    if g33.is_top_row {
        if let Some(title) = &cfg.title {
            let x = g33.canvas_offset_pixels.width + 
                         g33.upper_left.width + 
                         g33.plot.width / 2.0;
            let y = g33.canvas_offset_pixels.height + 
                         g33.title.height / 2.0;
            canvas.draw_text(title, &style, (x as i32, y as i32))?
        }
    }
    Ok(())
}

/// Add legends to all right-col plots.
fn add_plot_legends<X, Y>(
    g33:    &PrintGrid33,
    _cfg:    &PlotConfig<X, Y>,
    _canvas: &DrawingArea<CanvasBackend, Shift>,
) -> Result<(), Box<dyn Error>>
where X: 'static + Clone + PartialOrd,
      Y: 'static + Clone + PartialOrd
{
    if g33.is_right_col {
        // TODO: implmenent margin legends
    }
    Ok(())
}

/// Calculate breakpoints for splitting the canvas into a plot grid and return 
/// the resulting canvas areas.
fn split_by_breakpointsplot(
    grid:    &PlotGridConfig, 
    canvas:  DrawingArea<CanvasBackend, Shift>
) -> Vec<DrawingArea<CanvasBackend, Shift>> {
    if grid.n_plots == 1 {
        vec![canvas]
    } else {
        let xs  = if grid.n_cols == 1 {
            Vec::new()
        } else {
            (0..grid.n_cols - 1).map(|i| {
                grid.grid33_pixels[i].canvas.width
            }).collect()
        };
        let ys  = if grid.n_rows == 1 {
            Vec::new()
        } else {
            (0..grid.n_plots).step_by(grid.n_cols).map(|i| {
                grid.grid33_pixels[i].canvas.height
            }).collect()
        };
        canvas.split_by_breakpoints(&xs, &ys)
    }
}

/// Add a grid of plots/charts to the canvas one at a time.
fn add_charts<X, Y>(
    df:      &DataFrame,
    configs: &[&PlotConfig<X, Y>],
    grid:    &PlotGridConfig, 
    areas:   Vec<DrawingArea<CanvasBackend, Shift>>
) -> Result<(), Box<dyn Error>>
where X: 'static + Default + Clone + Copy + PartialOrd,
      Y: 'static + Default + Clone + Copy + PartialOrd,
      Range<X>: AsRangedCoord<Value = X>,
      Range<Y>: AsRangedCoord<Value = Y>,
      <Range<X> as AsRangedCoord>::CoordDescType: ValueFormatter<X>,
      <Range<Y> as AsRangedCoord>::CoordDescType: ValueFormatter<Y>,
      Vec<Option<X>>: ColVec,
      Vec<Option<Y>>: ColVec,
{
    for i in 0..grid.n_plots {
        let cfg = configs[i];
        let g33 = &grid.grid33_pixels[i];
        let area = &areas[i];
        let chart_builder = &mut ChartBuilder::on(area);
        
        prepare_axis_label_areas(g33, chart_builder);

        // if ranges are not provided, calculate them from the first data series
        let x_range = cfg.x_range.clone().unwrap_or_else(||{
            get_axis_range::<X>(df, &cfg.series[0].x)
        });
        let y_range = cfg.y_range.clone().unwrap_or_else(||{
            get_axis_range::<Y>(df, &cfg.series[0].y)
        });
        let mut chart = chart_builder
            .margin_top(g33.title.height)
            .margin_right(g33.legend.width)
            .build_cartesian_2d(x_range, y_range)?;
        chart.configure_mesh()
            .set_all_tick_mark_size(TICK_SIZE_PX)
            .label_style((ARIAL, grid.font_size675_pixels))
            .axis_desc_style((ARIAL, grid.font_size725_pixels))
            .x_desc(cfg.x_label.as_deref().unwrap_or(""))
            .y_desc(cfg.y_label.as_deref().unwrap_or(""))
            .draw()?;

        // draw data series one at a time ...
        for i in 0..cfg.series.len() {
            let series = &cfg.series[i];

            let x = df.get_ref::<X>(&series.x);
            let y = df.get_ref::<Y>(&series.y);
            let d = x.into_iter()
                .zip(y)
                .filter_map(|(opt_x, opt_y)| {
                    match (opt_x, opt_y) {
                        (Some(x), Some(y)) => Some((*x, *y)),
                        _ => None,
                    }
                });

            let color = series.color.unwrap_or(SeriesColor::pick(i)).get_rgb();

            let line_width_pixels = (series.line_width_points * grid.dpi / POINTS_PER_INCH as f32).round() as u32;
            let line_style = ShapeStyle{
                color: color.mix(series.opacity),
                filled: false,
                stroke_width: line_width_pixels,
            };
            match series.plot_as {
                SeriesType::Lines | SeriesType::Both => {
                    chart.draw_series(LineSeries::new(d.clone(), line_style),)?;
                },
                _ => {}
            };

            let point_size_pixels = (series.point_size_points * grid.dpi / POINTS_PER_INCH as f32).round() as u32;
            let point_style = ShapeStyle{
                color: color.mix(series.opacity),
                filled: series.point_type.get_filled(),
                stroke_width: if series.point_border { line_width_pixels / 2 } else { 0 },
            };
            match series.plot_as {
                SeriesType::Points | SeriesType::Both => {
                    match series.point_type {
                        PointType::CircleFilled | PointType::CircleOpen => {
                            chart.draw_series(
                                d.map(|p| Circle::new(p, point_size_pixels, point_style)),
                            )?;
                        },
                        PointType::TriangleFilled | PointType::TriangleOpen => {
                            chart.draw_series(
                                d.map(|p| TriangleMarker::new(p, point_size_pixels, point_style)),
                            )?;
                        },
                        PointType::Cross => {
                            chart.draw_series(
                                d.map(|p| Cross::new(p, point_size_pixels, point_style)),
                            )?;
                        },
                    }
                },
                _ => {}
            }
        }

        // present this chart to the canvas
        area.present().expect("draw_plots area.present failed");
    }
    Ok(())
}

/// Set the margins areas in prepartion for drawing axis labels.
fn prepare_axis_label_areas(
    g33: &PrintGrid33,
    chart_builder: &mut ChartBuilder<CanvasBackend>
) {
    // x-axis labels on bottom-row plots
    if g33.is_bottom_row {
        chart_builder.set_label_area_size(
            LabelAreaPosition::Bottom, 
            g33.x_axis.height
        );
    } else {
        chart_builder.margin_bottom(g33.x_axis.height);
    }

    //y-axis labels on left-col plots
    if g33.is_left_col {
        chart_builder.set_label_area_size(
            LabelAreaPosition::Left, 
            g33.y_axis.width
        );
    } else {
        chart_builder.margin_left(g33.y_axis.width);
    }
}

/// Get the range of values in a column of a `DataFrame` for plotting. Notably,
/// this function will panic if NaN is encountered when T is a float type.
fn get_axis_range<T>(
    df: &DataFrame, 
    col: &str
) -> Range<T> 
where T: 'static + Default + Clone + Copy + PartialOrd,
      Vec<Option<T>>: ColVec,
{
    let x: Vec<T> = df
        .get_ref::<T>(col)
        .iter()
        .flatten()
        .copied()
        .collect();
    let x_min = x.iter()
        .reduce(|min, val| if val < min { val } else { min })
        .copied()
        .unwrap_or_default();
    let x_max = x.iter()
        .reduce(|max, val| if val > max { val } else { max })
        .copied()
        .unwrap_or_default();
    x_min..x_max
}






// /// Draw canvas-level (titles, legends) and chart-level elements (series, axes).
// fn draw_plot<X, Y> (
//     df:      &DataFrame,
//     configs: &PlotConfig<X, Y>,
//     g33:     &PrintGrid33<X, Y>,
//     canvas:  &DrawingArea<CanvasBackend, Shift>,
// )
// where X: 'static + Clone + PartialEq + PartialOrd,
//       Y: 'static + Clone + PartialEq + PartialOrd
// {
//     chart.configure_mesh()
//         .set_all_tick_mark_size(TICK_SIZE_PX)
//         .label_style((ARIAL, font_size_px_65))
//         .axis_desc_style((ARIAL, font_size_px_75))
//         .x_desc("X axis 0123")
//         .y_desc("Y axis 0123")
//         .draw()
//         .expect("chart.configure_mesh().draw() failed");
//     chart.draw_series(
//         LineSeries::new(
//             (0..10).map(|x| (x, 10 - x)), 
//             Into::<ShapeStyle>::into(&BLUE).stroke_width(2).filled()
//         ).point_size(5),
//     ).expect("chart.draw_series failed");
//     panel.present().expect("panel.present failed");
// }


// /// Perform data processing and render the plot image. 
// #[component]
// pub fn RudiPlot<X, Y>(
//     df:        ReadSignal<DataFrame>, 
//     config:    ReadSignal<PlotConfig<X, Y>>,
//     grid:      ReadSignal<PlotGridConfig<X, Y>>,
//     canvas_id: Signal<Option<String>>,
// ) -> Element 
// where X: 'static + Clone + PartialEq + PartialOrd,
//       Y: 'static + Clone + PartialEq + PartialOrd
// {

//     // render the plot once the canvas is mounted
//     use_effect(move || {




//         }
//     });

//     // return the image canvas element
//     rsx!{
//         canvas {
//             id: this.id,
//             class: "plot-panel-canvas",
//             width: "{dim.read().1.0}",
//             height: "{dim.read().1.1}",
//             onmounted: move |element| {
//                 let web_sys_element = element.as_web_event();
//                 log::info!("Canvas mounted with id: {}", web_sys_element.id());
//                 canvas_id.set(Some(web_sys_element.id()));
//             },
//         }
//     }
// }
