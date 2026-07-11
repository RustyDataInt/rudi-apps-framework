//! Components that defines inputs for primitive data types. 

// modules
mod numeric_input;
mod text_input;
mod boolean_switch;
mod select;

// re-exports
pub use numeric_input::NumericInput;
pub use text_input::TextInput;
pub use boolean_switch::BooleanSwitch;
pub use select::SelectInput;
