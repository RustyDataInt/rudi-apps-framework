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

// constants
const CALLER_INDEX_COL: &str = "rudi_caller_index";

/// Get a `Signal<DataFrame>` for a `PlotPanel` or `TablePanel`
/// from its `data` and `data_frame` properties. If `data_frame` 
/// is set it is used as is, otherwise `data` must be provided
/// in which case the `Vec<T>` is converted to a `DataFrame`.
fn get_df<T> (
    data: Option<Signal<Vec<T>>>,
    mut data_frame: Option<Signal<DataFrame>>,
    add_caller_index: bool,
) -> Option<Signal<DataFrame>>
where T: 'static + Clone + PartialEq + PartialOrd
{
    // convert a caller's `Vec<T>` to a `DataFrame`
    if data_frame.is_none() && data.is_some() {
        // TODO: convert Vec<T> to DataFrame
        let data = data.unwrap();
        let _data = data.read();
        let df = use_signal::<DataFrame>(|| df_new!());
        data_frame = Some(df);
        // then remove this!
        data_frame.take();
    }

    // for tables, add a caller index column to the `DataFrame` so that
    // row selections always reflect the source data row regardless of
    // any sorting or filtering applied to the `DataFrame` by the user.
    if add_caller_index && data_frame.is_some() {
        let mut df = data_frame.unwrap();
        if !df.peek().col_names().contains(&CALLER_INDEX_COL.to_string()) {
            let i = (0..df.peek().n_row()).collect::<Vec<usize>>().to_rl();
            df.write().add_col(CALLER_INDEX_COL, i);
        }
    }
    data_frame
}
