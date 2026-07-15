//! The interactive overlay for a `PlotPanel`.

// imports
use dioxus::prelude::*;
use rlike::data_frame::DataFrame;
// use crate::ui::async_delay;
use super::*;

// constants
const CROSSHAIR_PIXEL_BASE64: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYGDwDwAEhQEBAZ89LwAAAABJRU5ErkJggg==";
const DEBOUNCE_DELAY_MS: u32 = 50;

/// Place horizontal and vertical crosshairs on the plot
/// between the img and the interactive overlay.
#[component]
pub fn PlotPanelCrosshairs() -> Element {
    let mut cursor_x = use_signal(|| 50_u32);
    let mut cursor_y = use_signal(|| 50_u32);
    rsx!{
        div {
            class: "plot-panel-crosshairs-wrapper",

            // move the crosshairs to the mouse position when the user moves the mouse over the overlay
            onmousemove: move |e| {
                // spawn(async move {
                //     async_delay(DEBOUNCE_DELAY_MS).await;
                //     // delay_is_over.set(true);
                // });
                let coords = e.data.coordinates().element(); // (x, y)
                if coords.x > 1.0 {
                    cursor_x.set((coords.x + 2.0) as u32);
                }
                if coords.y > 1.0 {
                    cursor_y.set((coords.y + 2.0) as u32);
                }
            },
            img {
                class: "crosshair-vertical",
                left: "{cursor_x}px",
                src: "{CROSSHAIR_PIXEL_BASE64}",
            }
            img {
                class: "crosshair-horizontal",
                top: "{cursor_y}px",
                src: "{CROSSHAIR_PIXEL_BASE64}",
            }
        }
    }
}

/// Overlay the plot with invisible interactive elements that
/// match the plot inner and outer regions, and render at a 
/// higher z-index than the plot itself, so that the user can
/// interact with the plot without actually touching the img.
#[component]
pub fn PlotPanelOverlay<X, Y>(
    df:     Signal<DataFrame>, 
    config: Signal<PlotConfig<X, Y>>,
) -> Element 
where X: 'static + Clone + PartialEq + PartialOrd,
      Y: 'static + Clone + PartialEq + PartialOrd
{
    rsx!{}

    // // establish the scale between the print-sized PNG
    // // and the dynamic changing screen size of the panel
    // let _df = df.read();
    // let config = config.read();
    // let px = config.get_pixels();
    // let mut screen_print_ratio = use_signal(|| 0.0);

    // // render a single div at the same size, on top of the img
    // rsx!{
    //     div {
    //         class: "plot-panel-overlay-wrapper",

    //         // update the relative scale when the user resizes the browser
    //         onresize: move |e| {
    //             let new_px = e.data.get_content_box_size();
    //             if let Ok(new_px) = new_px {
    //                 screen_print_ratio.set(new_px.width / px.panel.width as f64);
    //             }
    //         },

    //         // fill the single overlay div with a 3x3 grid of quadrants that match the plot layout
    //         // (other required attibutes are set in rudi_apps_layout.css, these are dynamic)
    //         grid_template_columns: "{px.margins.left}fr {px.grid.plot.width}fr {px.margins.right}fr",
    //         grid_template_rows: "{px.margins.top}fr {px.grid.plot.height}fr {px.margins.bottom}fr",

    //         // upper-left quadrant is unused at present
    //         div { class: "plot-panel-overlay-quadrant plot-panel-overlay-upper-left" }

    //         // upper-central quadrant matches the title area clicks to title update
    //         div { class: "plot-panel-overlay-quadrant plot-panel-overlay-title" }

    //         // upper-right quadrant is unused at present
    //         div { class: "plot-panel-overlay-quadrant plot-panel-overlay-upper-right" }

    //         // middle-left quadrant matches the Y-axis area clicks to Y-axis update
    //         div { class: "plot-panel-overlay-quadrant plot-panel-overlay-y-axis" }

    //         // middle-central quadrant matches the plot area clicks to point selection
    //         div {
    //             class: "plot-panel-overlay-quadrant plot-panel-overlay-plot",
    //             onclick: move |e| {
    //                 let coords = e.data.coordinates().element(); // (x, y)
    //                 let screen_print_ratio = screen_print_ratio.read();
    //                 let screen_width_px = px.grid.plot.width as f64 * *screen_print_ratio;
    //                 let screen_height_px = px.grid.plot.height as f64 * *screen_print_ratio;
    //                 log::info!(
    //                     "Plot clicked at: {} {} %", coords.x / screen_width_px, coords.y /
    //                     screen_height_px
    //                 );
    //             },
    //         }

    //         // middle-right quadrant matches the legend area clicks to legend update
    //         div { class: "plot-panel-overlay-quadrant plot-panel-overlay-legend" }

    //         // lower-left quadrant is unused at present
    //         div { class: "plot-panel-overlay-quadrant plot-panel-overlay-lower-left" }

    //         // lower-central quadrant matches the X-axis area clicks to X-axis update
    //         div { class: "plot-panel-overlay-quadrant plot-panel-overlay-x-axis" }

    //         // lower-right quadrant is unused at present
    //         div { class: "plot-panel-overlay-quadrant plot-panel-overlay-lower-right" }
    //     }
    // }
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
