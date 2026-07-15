//! A component for a standardized `DisplayPanel` carrying a single interactive 
//! table, built from an `rlike::DataFrame` and rendered without additional 
//! dependencies.

// imports
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronUp, ChevronDown};
use rlike::data_frame::prelude::*;
use crate::ui::*;
use super::*;

// constants
const MAJOR_SORT_ICON_SIZE: u16 = 20;
const MINOR_SORT_ICON_SIZE: u16 = 16;
const DEFAULT_MAX_ROWS:     u8 = 20;

/// Row selection types for a table.
#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum TableSelectMode {
    None,
    Single,
    Multiple,
}

/// Instructions on how to render an HTML table from a `DataFrame`.
#[derive(PartialEq, Clone, Props)]
pub struct TableConfig {
    pub columns:      Option<Vec<String>>, // the SELECT columns, default to all
    pub major_sort:   Option<String>, // the first ORDER BY column
    pub major_negate: bool,
    pub minor_sort:   Option<String>, // the second ORDER BY column
    pub minor_negate: bool,
    pub max_rows:     u8,
    pub select_mode:  TableSelectMode, // single or multi row selection
}
impl TableConfig {
    /// Create a new `TableConfig` with the given row selection mode.
    /// 
    /// Any initial default sorting and filtering should be applied to a 
    /// `DataFrame` by the caller prior to passing it to `TablePanel`. The user 
    /// will be able to resort as needed, or apply additional filters, but can 
    /// never recover rows not present in the `DataFrame` passed to `TablePanel`.
    pub fn new(select_mode: TableSelectMode) -> Self {
        Self {
            columns: None,
            major_sort:   None,
            major_negate: false,
            minor_sort:   None,
            minor_negate: false,
            max_rows:     DEFAULT_MAX_ROWS,
            select_mode:  select_mode,
        }
    }
    /// Create a new `TableConfig` with the given columns and row selection mode.
    /// 
    /// Any initial default sorting and filtering should be applied to a 
    /// `DataFrame` by the caller prior to passing it to `TablePanel`. The user 
    /// will be able to resort as needed, or apply additional filters, but can 
    /// never recover rows not present in the `DataFrame` passed to `TablePanel`.
    pub fn new_with_columns(columns: &[&str], select_mode: TableSelectMode) -> Self {
        Self {
            columns: Some(columns.iter().map(|s| s.to_string()).collect()),
            major_sort:   None,
            major_negate: false,
            minor_sort:   None,
            minor_negate: false,
            max_rows:     DEFAULT_MAX_ROWS,
            select_mode:  select_mode,
        }
    }
}

/// Dioxus Properties for an `TablePanel`.
#[derive(PartialEq, Clone, Props)]
pub struct TablePanelProps<T> 
where T: 'static + Clone + PartialEq + PartialOrd
{
    // required
    data:       Option<Signal<Vec<T>>>, // either `data` or `data_frame` is required
    data_frame: Option<Signal<DataFrame>>,
    config:     Signal<TableConfig>,
    // optional
    selected_rows: Option<Signal<Vec<usize>>>, // required if `select_mode` is not `None`
    default_input_width: Option<InputWidth>,
    // from DisplayPanelProps
    name:      String,
    title:     Option<String>,
    n_columns: u8,
    min_width: Option<String>,
}

/// A component for a standardized display panel carrying a single interactive 
/// table rendered without additional dependencies.
/// 
/// A `TablePanel` requires a value for either `data` or `data_frame` to provide 
/// the data to display as a `Vec<T>` or `rlike::DataFrame`, respectively.
/// 
/// `config` provides instructions for how to intially display the table; these 
/// initial values may then be modified by the user.
/// 
/// `name` is used to define the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// If provided, `title` will be displayed in the panel header, otherwise
/// a thin solid header will be shown.
/// 
/// `n_columns` and `min_width` are passed to the `FluidSpan` to control
/// the size of the panel in the `FluidPage`.
#[component]
pub fn TablePanel<T>(props: TablePanelProps<T>) -> Element 
where T: 'static + Clone + PartialEq + PartialOrd
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
            if let Some(df) = get_df(props.data, props.data_frame, true) {
                div { class: "table-panel-contents",
                    TablePanelTable {
                        df,
                        config: props.config,
                        selected_rows: props.selected_rows,
                    }
                }
            } else {
                div { class: "display-panel-error", "Missing data!" }
            }
        }
    }
}

/// Perform data processing and render the table. 
#[component]
pub fn TablePanelTable(
    df:     Signal<DataFrame>, 
    config: Signal<TableConfig>,
    selected_rows: Option<Signal<Vec<usize>>>,
) -> Element {
    let current_config = config.read().clone();
    let columns = current_config.columns.unwrap_or_else(|| df.read().col_names().clone() );
    let columns = columns.iter().filter(|col| col != &&CALLER_INDEX_COL).cloned().collect::<Vec<String>>();

    // render the table header and handling sorting
    let ths = columns.iter().map(|col| {
        let col = col.clone();
        let chevron = if current_config.major_sort == Some(col.clone()) {
            if current_config.major_negate {
                rsx!{
                    ChevronDown { class: "table-major-sort", size: MAJOR_SORT_ICON_SIZE }
                }
            } else {
                rsx!{
                    ChevronUp { class: "table-major-sort", size: MAJOR_SORT_ICON_SIZE }
                }
            }
        } else if current_config.minor_sort == Some(col.clone()) {
            if current_config.minor_negate {
                rsx!{
                    ChevronDown { class: "table-minor-sort", size: MINOR_SORT_ICON_SIZE }
                }
            } else {
                rsx!{
                    ChevronUp { class: "table-minor-sort", size: MINOR_SORT_ICON_SIZE }
                }
            }
        } else {
            rsx!{}
        };
        rsx!{
            th {
                key: "{col}",
                onclick: move |_| {
                    let mut config = config.write();
                    // let mut df = df.write();
                    if config.major_sort == Some(col.clone()) {
                        config.major_negate = !config.major_negate;
                    } else {
                        config.minor_sort = config.major_sort.take();
                        config.minor_negate = config.major_negate;
                        config.major_sort = Some(col.clone());
                        config.major_negate = false;
                    }
                },
                {col.clone()}
                {chevron}
            }
        }
    });
    
    // render the table rows and handle row selection
    let trs = (0..df.read().n_row()).map(|i| { 
        let caller_i: usize = df.read().cell(CALLER_INDEX_COL, i).unwrap();
        let tds = columns.iter().map(|col| {
            let value = df.read().cell_string(col, i);
            rsx!{
                td { key: "{col}-{caller_i}", {value} }
            }
        });
        rsx!{
            tr {
                key: "row-{i}",
                onclick: move |_| {
                    let config = config.read();
                    if let Some(selected_rows) = &mut selected_rows {
                        if config.select_mode != TableSelectMode::None {
                            if selected_rows.read().contains(&caller_i) {
                                selected_rows.write().retain(|&i| i != caller_i);
                            } else {
                                selected_rows.write().push(caller_i);
                            }
                        }
                    }
                },
                {tds}
            }
        }
    });

    // assemble the final table
    rsx!{
        table { class: "table-panel-table",
            tr { class: "table-panel-table-header", {ths} }
            {trs}
        }
    }
}
