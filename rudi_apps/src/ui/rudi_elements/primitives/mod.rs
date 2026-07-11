//! Components that defines inputs for primitive data types. 

// modules
mod numeric_input;
mod text_input;
mod boolean_switch;
mod select_input;
mod radio_group;

// re-exports
pub use numeric_input::NumericInput;
pub use text_input::TextInput;
pub use boolean_switch::BooleanSwitch;
pub use select_input::SelectInput;
pub use radio_group::RadioGroupInput;
