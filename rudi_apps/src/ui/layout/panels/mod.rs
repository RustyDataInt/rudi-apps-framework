//! Broadly reusable panel components that establish page layout.

// modules
mod display_panel;
mod input_panel;
mod table_panel;
mod plot_panel;

// re-exports
pub use display_panel::*;
pub use input_panel::*;
pub use table_panel::*;
pub use plot_panel::*;

// imports
use std::any::type_name;
use dioxus::prelude::*;
use rlike::data_frame::prelude::*;
use crate::server::*;

// constants
const CALLER_INDEX_COL: &str = "rudi_caller_index";

/// As needed, convert `Vec<T>` to a `DataFrame` for use in data panels. If
/// `data_frame` is set it is used as is, otherwise `data_vec` is converted to
/// a `DataFrame` for internal use, noting that both forms are cached in
/// `ServerData` with type-specific `DataSource` values.
fn get_df<T> (
    data_vec:       Option<Resource<DataSource>>,
    mut data_frame: Option<Resource<DataSource>>,
    add_caller_index: bool,
) -> Option<Resource<DataSource>>
where T: 'static + Clone + PartialEq + PartialOrd
{
    // convert a caller's `Vec<T>` into a `DataFrame`
    if data_frame.is_none() && data_vec.is_some() {
        let data_vec = data_vec.unwrap();
        let data_vec_source = data_vec.read().clone();
        if let Some(data_vec_source) = data_vec_source {
            data_frame = Some(use_resource(move || {
                let mut data_frame_source = data_vec_source.clone();
                async move {
                    data_frame_source.data_type = type_name::<DataFrame>().to_string();
                    
                    // TODO: convert `Vec<T>` to `DataFrame`
                    ServerData::insert(&data_frame_source, df_new!());

                    data_frame_source
                }
            }));
        }
    }

    // for tables, add a caller index column to the `DataFrame` so that row 
    // selections always reflect the source data row regardless of any sorting 
    // or filtering applied to the `DataFrame` by the user.
    if add_caller_index && data_frame.is_some() {
        let data_frame_source = data_frame.unwrap();
        let data_frame_source = data_frame_source.read().clone();
        if let Some(df) = data_frame_source {
            ServerData::update::<DataFrame, _>(&df, |df_opt| {
                if let Some(mut df) = df_opt {
                    if !df.col_names().contains(&CALLER_INDEX_COL.to_string()) {
                        let i = (0..df.n_row()).collect::<Vec<usize>>().to_rl();
                        df.add_col(CALLER_INDEX_COL, i);
                    }
                    df
                } else {
                    df_new!()
                }
            });
        }
    }
    data_frame
}
