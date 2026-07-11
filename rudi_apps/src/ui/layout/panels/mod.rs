//! Broadly reusable app components that establish page layout.
//! These are organizing wrappers, not namespaced components.

// modules
mod display_panel;
mod input_panel;
mod plot_panel;
mod table_panel;

// re-exports
pub use display_panel::*;
pub use input_panel::*;
pub use plot_panel::*;
pub use table_panel::*;

// imports
use dioxus::prelude::*;
use rlike::data_frame::prelude::*;

/// Get a `Signal<DataFrame>` for a `PlotPanel` or `TablePanel`
/// from its `data` and `data_frame` properties.
pub fn get_df<T> (
    data: Option<Signal<Vec<T>>>,
    mut data_frame: Option<Signal<DataFrame>>,
) -> Option<Signal<DataFrame>>
where T: 'static + 
    Clone + 
    PartialEq + PartialOrd + 
{
    if data_frame.is_none() && data.is_some() {
        // TODO: convert Vec<T> to DataFrame
        let df = use_signal::<DataFrame>(|| df_new!());
        data_frame = Some(df);
        // then remove this!
        data_frame.take();
    }
    data_frame
}
