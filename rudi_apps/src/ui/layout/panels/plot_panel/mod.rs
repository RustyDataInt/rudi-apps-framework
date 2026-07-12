//! A component for a standardized `DisplayPanel` carrying a single
//! interactive plot, built from an `rlike::DataFrame` and rendered 
//! with `plotters`.

// modules
mod config;

// re-exports
pub use config::*;

// imports
use dioxus::prelude::*;
use plotters::prelude::*;
use image::codecs::png::PngEncoder as RgbToPng;
use image::{ColorType, ImageEncoder};
use base64::{engine::general_purpose::STANDARD as PngToBase64, Engine as _};
use rlike::data_frame::DataFrame;
use crate::ui::*;
use super::*;

// constants
const DEFAULT_INPUT_WIDTH: u16 = 75; // i.e., a single-column input width, in pixels

/// Dioxus Properties for an `PlotPanel`.
#[derive(PartialEq, Clone, Props)]
pub struct PlotPanelProps<T, X, Y> 
where T: 'static + Clone + PartialEq + PartialOrd,
      X: 'static + Clone + PartialEq + PartialOrd,
      Y: 'static + Clone + PartialEq + PartialOrd
{
    // required
    data:       Option<Signal<Vec<T>>>, // either `data` or `data_frame` is required
    data_frame: Option<Signal<DataFrame>>,
    config:     Signal<PlotConfig<X, Y>>,
    // optional
    default_input_width: Option<InputWidth>,
    // from DisplayPanelProps
    name:      String,
    title:     Option<String>,
    n_columns: u8,
    min_width: Option<String>,
}

/// A component for a standardized display panel carrying a single
/// interactive plot rendered with `plotters`.
/// 
/// A `PlotPanel` requires a value for either `data` or `data_frame` 
/// to provide the data to plot as a `Vec<T>` or `rlike::DataFrame`, 
/// respectively.
/// 
/// `config` provides the instructions for how to intially render 
/// the plot; these initial values may then be modified by the user.
/// 
/// `name` defines the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// If provided, `title` will be displayed in the panel header, otherwise
/// a thin solid header will be shown.
/// 
/// `n_columns` and `min_width` are passed to the `FluidSpan` to control
/// the size of the panel in the `FluidPage`.
#[component]
pub fn PlotPanel<T, X, Y>(props: PlotPanelProps<T, X, Y>) -> Element 
where T: 'static + Clone + PartialEq + PartialOrd,
      X: 'static + Clone + PartialEq + PartialOrd,
      Y: 'static + Clone + PartialEq + PartialOrd
{
    let default_input_width = props.default_input_width
        .unwrap_or(InputWidth(DEFAULT_INPUT_WIDTH));
    use_context_provider(|| default_input_width);

    rsx!{
        DisplayPanel {
            name: props.name,
            title: props.title,
            n_columns: props.n_columns,
            min_width: props.min_width,
            if let Some(df) = get_df(props.data, props.data_frame, false) {
                div { class: "plot-panel-contents",
                    PlotPanelPlot { df, config: props.config }
                    PlotPanelOverlay { df, config: props.config }
                }
            } else {
                div { class: "display-panel-error", "Missing data!" }
            }
        }
    }
}

/// Perform data processing and render the plot image. 
#[component]
pub fn PlotPanelPlot<X, Y>(
    df:     Signal<DataFrame>, 
    config: Signal<PlotConfig<X, Y>>,
) -> Element 
where X: 'static + Clone + PartialEq + PartialOrd,
      Y: 'static + Clone + PartialEq + PartialOrd
{
    let _df = df.read();
    let config = config.read();
    let w = (config.panel_size.width  * config.print_dpi as f32) as u32;
    let h = (config.panel_size.height * config.print_dpi as f32) as u32;

    // use plotters to render the plot
    let mut rgb = vec![255u8; (w * h * 3) as usize];
    {
        let backend = BitMapBackend::with_buffer(&mut rgb, (w, h));
        let drawing_area = backend.into_drawing_area();
        let mut chart = ChartBuilder::on(&drawing_area)
            .build_cartesian_2d(0..100, 0..100)?;
        chart.draw_series(
            LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK),
        )?;
        drawing_area.present()?;
    }

    // convert RGB array to base64 PNG
    let mut png_bytes: Vec<u8> = Vec::new();
    let rgb_to_png = RgbToPng::new(&mut png_bytes);
    rgb_to_png.write_image(&rgb, w, h, ColorType::Rgb8.into())?;
    let base64 = PngToBase64.encode(&png_bytes);

    // return the img panel
    rsx!{
        div { class: "plot-panel-plot",
            img { src: "data:image/png;base64,{base64}" }
        }
    }
}

/// Overlay the plot with invisible interactive elements. 
#[component]
pub fn PlotPanelOverlay<X, Y>(
    df:     Signal<DataFrame>, 
    config: Signal<PlotConfig<X, Y>>,
) -> Element 
where X: 'static + Clone + PartialEq + PartialOrd,
      Y: 'static + Clone + PartialEq + PartialOrd
{
    let _df = df.read();
    let config = config.read();
    let px = config.get_pixels();
    let mut scalar = use_signal(|| 0.0);
    rsx!{
        div {
            class: "plot-panel-overlay-wrapper",
            grid_template_columns: "{px.margins.left}fr {px.grid.plot.width}fr {px.margins.right}fr",
            grid_template_rows: "{px.margins.top}fr {px.grid.plot.height}fr {px.margins.bottom}fr",

            onresize: move |e| {
                let new_px = e.data.get_content_box_size();
                if let Ok(new_px) = new_px {
                    scalar.set(new_px.width / px.panel.width as f64);
                }
            },

            div { class: "plot-panel-overlay-quadrant plot-panel-overlay-upper-left" }
            div { class: "plot-panel-overlay-quadrant plot-panel-overlay-title" }
            div { class: "plot-panel-overlay-quadrant plot-panel-overlay-upper-right" }

            div { class: "plot-panel-overlay-quadrant plot-panel-overlay-y-axis" }
            div {
                class: "plot-panel-overlay-quadrant plot-panel-overlay-plot",
                onclick: move |e| {
                    let coords = e.data.coordinates().element(); // (x, y)
                    let scalar = scalar.read();
                    let screen_width_px = px.grid.plot.width as f64 * *scalar;
                    let screen_height_px = px.grid.plot.height as f64 * *scalar;
                    log::info!(
                        "Plot clicked at: {} {} %", coords.x / screen_width_px, coords.y /
                        screen_height_px
                    );
                },
            }
            // UiEvent {
            //     bubble_state: true,
            //     prevent_default: false,
            //     data: MouseData {
            //         coordinates: Coordinates {
            //             screen: (943.0, 366.0),
            //             client: (345.0, 257.0),
            //             element: (66.0, 45.0), // so, within the plot quadrant
            //             page: (345.0, 257.0)
            //         },
            //         modifiers: Modifiers(0x0),
            //         held_buttons: EnumSet(),
            //         trigger_button: Some(Primary)
            //     }
            // }
            div { class: "plot-panel-overlay-quadrant plot-panel-overlay-legend" }

            div { class: "plot-panel-overlay-quadrant plot-panel-overlay-lower-left" }
            div { class: "plot-panel-overlay-quadrant plot-panel-overlay-x-axis" }
            div { class: "plot-panel-overlay-quadrant plot-panel-overlay-lower-right" }
        }
    }
}

