//! Plot rendering in a `PlotPanel` using `plotters`.

// // imports
// use dioxus::prelude::*;
// use dioxus_icons::lucide::axis_3d;
// use dioxus_web::WebEventExt;
// use plotters::prelude::*;
// use plotters_canvas::CanvasBackend;
// use rlike::data_frame::DataFrame;
// use super::*;

// /// Perform data processing and render the plot image. 
// #[component]
// pub fn PlotPanelPlot<X, Y>(
//     df:     Signal<DataFrame>, 
//     config: Signal<PlotConfig<X, Y>>,
// ) -> Element 
// where X: 'static + Clone + PartialEq + PartialOrd,
//       Y: 'static + Clone + PartialEq + PartialOrd
// {
//     let this = RudiElement::new::<()>("plot");
//     let mut canvas_id = use_signal(|| None::<String>);
//     let _df = df.read();
//     let cfg = config.read();
//     let px = cfg.get_pixels();

//     // establish the print-sized PNG dimensions, how all plots are rendered
//     // use persistent buffers for image rendering at a given print size
//     let dim = use_memo(move || {
//         let cfg = config.read();
//         let w = (cfg.panel_size.width  * cfg.print_dpi as f32) as u32;
//         let h = (cfg.panel_size.height * cfg.print_dpi as f32) as u32;
//         ((w * h * 3) as usize, (w, h))
//     });

//     const ARIAL: &str = "Arial";
//     const POINTS_PER_INCH: f64 = 72.0;
//     const POINTS_75: f64 = 7.5;
//     const POINTS_65: f64 = 6.5;
//     const TICK_SIZE_PX: u32 = 12;

//     let font_size_px_75 = (POINTS_75 / POINTS_PER_INCH * cfg.print_dpi as f64) as u32; // 31px at 300dpi
//     let font_size_px_65 = (POINTS_65 / POINTS_PER_INCH * cfg.print_dpi as f64) as u32; // 28px at 300dpi
//     let axis_label_area_px = TICK_SIZE_PX + font_size_px_65 + font_size_px_75 + 20;
//     let caption_area_px = font_size_px_75 + 20;

//     // render the plot once the canvas is mounted
//     use_effect(move || {
//         // async move {
//         // }
//         if canvas_id.read().is_none() { return; }
//         if let Some(canvas_id) = &*canvas_id.read(){
//             let canvas = CanvasBackend::new(canvas_id).expect("CanvasBackend::new failed");
//             let panel = canvas.into_drawing_area();
//             panel.fill(&WHITE).expect("panel.fill failed");

//             let title_style = (ARIAL, font_size_px_75).into_text_style(&panel);
//             panel.draw_text("Plot Title", &title_style, (200, 10)).expect("draw_text failed");
            
//             let mut chart = ChartBuilder::on(&panel)
//                 .set_label_area_size(LabelAreaPosition::Bottom, axis_label_area_px)
//                 .set_label_area_size(LabelAreaPosition::Left, axis_label_area_px)
//                 // .set_label_area_size(LabelAreaPosition::Top, caption_area_px)
//             //     .set_label_area_size(LabelAreaPosition::Top, px.margins.top)
//             //     .set_label_area_size(LabelAreaPosition::Right, px.margins.right)
//                 // .margin_bottom(px.margins.bottom)
//                 // .margin_left(px.margins.left)
//                 .margin_top(caption_area_px)
//                 .margin_right(px.margins.right)
//                 // .caption("Plot Title", (ARIAL, font_size_px_75))
//                 .build_cartesian_2d(0..10, 0..10).expect("build_cartesian_2d failed");
//             chart.configure_mesh()
//                 .set_all_tick_mark_size(TICK_SIZE_PX)
//                 .label_style((ARIAL, font_size_px_65))
//                 .axis_desc_style((ARIAL, font_size_px_75))
//                 .x_desc("X axis 0123")
//                 .y_desc("Y axis 0123")
//                 .draw()
//                 .expect("chart.configure_mesh().draw() failed");
//             chart.draw_series(
//                 LineSeries::new(
//                     (0..10).map(|x| (x, 10 - x)), 
//                     Into::<ShapeStyle>::into(&BLUE).stroke_width(2).filled()
//                 ).point_size(5),
//             ).expect("chart.draw_series failed");
//             panel.present().expect("panel.present failed");
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


// // use image::codecs::png::PngEncoder as RgbToPng;
// // use image::{ColorType, ImageEncoder};
// // use base64::{engine::general_purpose::STANDARD as PngToBase64, Engine as _};

//     // let mut rgb = use_memo(move || vec![255u8; dim.read().0] );
//     // let mut png_bytes = use_memo(move || Vec::<u8>::with_capacity(dim.read().0));

//         // // use plotters to render the plot
//     // {
//     //     let mut rgb_mut_guard = rgb.write();
//     //     let rgb_mut: &mut Vec<u8> = rgb_mut_guard.as_mut();
//     //     let backend = CanvasBackend::new(rgb_mut, dim.read().1);
//     //     let drawing_area = backend.into_drawing_area();
//     //     drawing_area.fill(&WHITE)?;
//     //     let mut chart = ChartBuilder::on(&drawing_area)
//     //         .set_label_area_size(LabelAreaPosition::Bottom, px.margins.bottom)
//     //         .set_label_area_size(LabelAreaPosition::Left, px.margins.left)
//     //         .set_label_area_size(LabelAreaPosition::Top, px.margins.top)
//     //         .set_label_area_size(LabelAreaPosition::Right, px.margins.right)
//     //         .build_cartesian_2d(0..100, 0..100)?;
//     //     // chart.configure_mesh().draw()?;
//     //     chart.draw_series(
//     //         LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK),
//     //     )?;
//     //     drawing_area.present()?;
//     // }

//     // // convert RGB array to base64-encoded PNG
//     // let mut png_bytes_mut_guard = png_bytes.write();
//     // let png_bytes_mut: &mut Vec<u8> = png_bytes_mut_guard.as_mut();
//     // let rgb_to_png = RgbToPng::new(png_bytes_mut);
//     // rgb_to_png.write_image(rgb.read().as_ref(), dim.read().1.0, dim.read().1.1, ColorType::Rgb8.into())?;
//     // drop(png_bytes_mut_guard);
//     // let png_bytes_guard = png_bytes.read();
//     // let png_bytes_ref: &Vec<u8> = png_bytes_guard.as_ref();
//     // let base64 = PngToBase64.encode(png_bytes_ref);

    
//         // img { class: "plot-panel-img", src: "data:image/png;base64,{base64}" }