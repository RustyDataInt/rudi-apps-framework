//! Handle plot, i.e., data chart, creation using plotters. These functions 
//! apply specifically to creating and drawing on a canvas, not wrapping that 
//! canvas into a display panel.

// modules
mod config;

// re-exports
pub use config::*;

// imports
use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use rlike::data_frame::DataFrame;
use crate::ui::*;

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

                    log::info!("Canvas mounted with id: {}", web_sys_element.id());

                    canvas_id.set(Some(web_sys_element.id()));
                },
            }
        }
    }
}

/// Draw canvas-level (titles, legends) and chart-level elements (series, axes).
pub fn draw_plots<X, Y> (
    _df:        &DataFrame,
    configs:   &[&PlotConfig<X, Y>],
    grid:      &PlotGridConfig,
    canvas_id: &str,
)
where X: 'static + Clone + PartialEq + PartialOrd,
      Y: 'static + Clone + PartialEq + PartialOrd
{
    let backend = CanvasBackend::new(canvas_id).expect("new_canvas CanvasBackend::new failed");
    let canvas = backend.into_drawing_area();
    canvas.fill(&WHITE).expect("new_canvas canvas.fill failed");

    let title_style = (ARIAL, grid.font_size725_pixels).into_text_style(&canvas);
    for i in 0..grid.n_plots {
        let config = configs[i];
        let g33 = &grid.grid33_pixels[i];
        if g33.is_top_row {
            if let Some(title) = &config.title {
                let x = g33.canvas_offset_pixels.width + 
                        g33.upper_left.width + 
                        g33.plot.width / 2.0;
                let y = g33.canvas_offset_pixels.height + 
                        g33.title.height / 2.0;
                canvas.draw_text(title, &title_style, (x as i32, y as i32))
                    .expect("new_canvas canvas.draw_text failed");
            }
        }
        if g33.is_right_col {
            // TODO: draw legend in middle right area
        }
    }

    let areas = if grid.n_plots == 1 {
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
    };
    
    for i in 0..grid.n_plots {
        let config = configs[i];
        let g33 = &grid.grid33_pixels[i];
        let area = &areas[i];
        let chart = &mut ChartBuilder::on(area);
        if g33.is_bottom_row {
            chart.set_label_area_size(
                LabelAreaPosition::Bottom, 
                g33.x_axis.height
            );
        } else {
            chart.margin_bottom(g33.x_axis.height);
        }
        if g33.is_left_col {
            chart.set_label_area_size(
                LabelAreaPosition::Left, 
                g33.y_axis.width
            );
        } else {
            chart.margin_left(g33.y_axis.width);
        }
        let mut chart = chart
            .margin_top(g33.title.height)
            .margin_right(g33.legend.width)
            .build_cartesian_2d(0..10, 0..10).expect("build_cartesian_2d failed");
        chart.configure_mesh()
            .set_all_tick_mark_size(TICK_SIZE_PX)
            .label_style((ARIAL, grid.font_size675_pixels))
            .axis_desc_style((ARIAL, grid.font_size725_pixels))
            .x_desc(config.x_label.as_deref().unwrap_or(""))
            .y_desc(config.y_label.as_deref().unwrap_or(""))
            .draw()
            .expect("chart.configure_mesh().draw() failed");
        for series in &config.series {
            let line_width_pixels = (series.line_width_points * grid.dpi / POINTS_PER_INCH as f32).round() as u32;
            let point_size_pixels = (series.point_size_points * grid.dpi / POINTS_PER_INCH as f32).round() as u32;
            // match series.plot_as {
            //     SeriesType::Lines => {
                    chart.draw_series(
                        LineSeries::new(
                            (0..10).map(|x| (x, 10 - x)), 
                            Into::<ShapeStyle>::into(&BLUE).stroke_width(line_width_pixels).filled()
                        ).point_size(point_size_pixels),
                    ).expect("draw_plots chart.draw_series failed");
                // },
                // SeriesType::Points => {
                //     chart.draw_series(
                //         PointSeries::of_element(
                //             (0..10).map(|x| (x, 10 - x)), 
                //             series.point_size_points,
                //             Into::<ShapeStyle>::into(&RED).filled(),
                //             &|c, s, st| {
                //                 return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                //                 + Circle::new((0,0), s, st.filled()); // At this point, the new pixel coordinate is established
                //             },
                //         ),
                //     ).expect("draw_plots chart.draw_series failed");
                // },
                // SeriesType::Both => {
                //     chart.draw_series(
                //         LineSeries::new(
                //             (0..10).map(|x| (x, 10 - x)), 
                //             Into::<ShapeStyle>::into(&BLUE).stroke_width(2).filled()
                //         ).point_size(5),
                //     ).expect("draw_plots chart.draw_series failed");
                // },
            // }
        }
        chart.draw_series(
            LineSeries::new(
                (0..10).map(|x| (x, 10 - x)), 
                Into::<ShapeStyle>::into(&BLUE).stroke_width(2).filled()
            ).point_size(5),
        ).expect("draw_plots chart.draw_series failed");
        area.present().expect("draw_plots area.present failed");
    }
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
