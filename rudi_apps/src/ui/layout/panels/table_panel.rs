//! A component for a standardized `DisplayPanel` carrying a single
//! interactive table, built from an `rlike::DataFrame` and rendered 
//! without additional dependencies.

// imports
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronUp, ChevronDown};
use rlike::data_frame::prelude::*;
use crate::ui::*;
use super::*;

// constants
const DEFAULT_INPUT_WIDTH: u16 = 75; // i.e., a single-column input width, in pixels
const CHEVRON_ICON_SIZE: u16 = 16; // size of the chevron icons in pixels

/// Instructions on how to render an HTML table from a `DataFrame`.
#[derive(PartialEq, Clone, Props)]
pub struct TableConfig {
    pub columns:        Option<Vec<String>>,
    pub sort_columns:   Option<Vec<String>>,
    pub sort_ascending: Option<Vec<bool>>,
    pub max_rows:       i32,
}

/// Dioxus Properties for an `TablePanel`.
#[derive(PartialEq, Clone, Props)]
pub struct TablePanelProps<T> 
where T: 'static + 
    Clone + 
    PartialEq + PartialOrd + 
{
    // required
    data: Option<Signal<Vec<T>>>, // one is required
    data_frame: Option<Signal<DataFrame>>,
    config: TableConfig,
    // optional
    selected_row: Option<Signal<usize>>,
    default_input_width: Option<InputWidth>,
    // from DisplayPanelProps
    name:      String,
    title:     Option<String>,
    n_columns: u8,
    min_width: Option<String>,
}

/// A component for a standardized display panel carrying a single
/// interactive table rendered without additional dependencies.
/// 
/// An `PlotPanel` requires a value for either the `data` or `data_frame` 
/// property to provide the data to plot as a `Vec<T>` or `rlike::DataFrame`, 
/// respectively.
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
pub fn TablePanel<T>(props: TablePanelProps<T>) -> Element 
where T: 'static + 
    Clone + 
    PartialEq + PartialOrd
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
            if let Some(df) = get_df(props.data, props.data_frame) {
                div { class: "table-panel-contents",
                    TablePanelTable {
                        df,
                        config: props.config,
                        selected_row: props.selected_row,
                    }
                }
            } else {
                div { class: "data-panel-error", "Missing data!" }
            }
        }
    }
}

/// Perform data processing and render the table. 
#[component]
pub fn TablePanelTable(
    df: Signal<DataFrame>, 
    config: TableConfig,
    selected_row: Option<Signal<usize>>,
) -> Element {
    let df = df.read();
    let columns = config.columns.unwrap_or_else(|| df.col_names().clone() );
    let ths = columns.iter().map(|col| {
        let col = col.clone();
        rsx!{
            th {
                key: "{col}",
                onclick: move |_| {
                    log::info!("Table column header clicked: {col}");
                },
                {col.clone()}
                ChevronUp { size: CHEVRON_ICON_SIZE }
            }
        }
    });
    let trs = (0..df.n_row()).map(|i| { 
        let tds = columns.iter().map(|col| {
            let value = df.cell_string(col, i);
            rsx!{
                td { key: "{col}-{i}", {value} }
            }
        });
        rsx!{
            tr {
                key: "row-{i}",
                onclick: move |_| {
                    log::info!("Table row clicked: {i}");
                    if let Some(selected_row) = &mut selected_row {
                        selected_row.set(i);
                    }
                },
                {tds}
            }
        }
    });
    rsx!{
        table { class: "table-panel-table",
            tr { class: "table-panel-table-header", {ths} }
            {trs}
        }
    }
}
