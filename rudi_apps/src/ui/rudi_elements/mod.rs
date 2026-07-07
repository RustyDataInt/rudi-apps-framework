//! A `RudiElement` is an extension of a Rust Dioxus `Element`
//! that makes the underlying component and current value 
//! stateful, namespaced, and serializable so that they are
//! included in all bookmarks.

// modules
mod primitives;

// re-exports
pub use primitives::*;

// imports
use std::any::type_name;
use serde::{de::DeserializeOwned, Serialize};
use serde_json as json;
use dioxus::prelude::*;
use crate::server::*;

/// `Namespace` is a newtype for `String` used to provide 
/// a namespace to a RudiElement's children via 
/// `use_context_provider(|| Namespace::from(&RudiElement))`.
#[derive(Clone, PartialEq)]
pub struct Namespace(pub String);
impl From<&RudiElement> for Namespace {
    fn from(this: &RudiElement) -> Self {
        Namespace(this.id.clone())
    }
}

/// A `RudiElement` is an extension of a Rust Dioxus `Element`
/// that makes the underlying component and current value 
/// stateful, namespaced, and serializable so that they are
/// included in all bookmarks.
#[derive(Clone, PartialEq)]
pub struct RudiElement{
    pub namespace: Namespace, // of the parent, NOT including the name of this element
    pub name:      String,
    pub id:        String, // the fully-qualified globally-unique id of this element
    pub data_type: String,
}
impl RudiElement {

    /// Internal function to construct a new `RudiElement`.
    fn new_rudi_element<T>(
        name:      &str, 
        namespace: Namespace,
    ) -> RudiElement {
        let name = name.replace("-", "_");
        let id = format!("{}-{}", &namespace.0, &name);
        RudiElement {
            namespace,
            name,
            id,
            data_type: type_name::<T>().to_string(),
        }
    }

    /// Create a new top-level app-step `RudiElement` with the given name.
    /// The namespace is set to "app", thus the step id will be "app-<step_name>".
    /// 
    /// Usually, the app step data type `T` is `()`, but it can be other types.
    pub fn app_step<T>(
        name: &str, 
    ) -> RudiElement {
        let namespace = Namespace("app".to_string());
        Self::new_rudi_element::<T>(name, namespace)
    }

    /// Create a new `RudiElement` with the given name and data type. 
    /// The namespace is inherited from the parent `RudiElement` via 
    /// `use_context::<Namespace>()`.
    pub fn new<T>(
        name: &str, 
    ) -> RudiElement {
        let namespace = use_context::<Namespace>();
        Self::new_rudi_element::<T>(name, namespace)
    }
    
    /// Set the current value of this `RudiElement` into the server state.
    pub fn set_state<T: Serialize>(&self, value: &T) {
        let mut server_state = use_context::<Signal<ServerState>>();
        let id = self.id.clone();
        let json = json::to_value(value).unwrap();
        server_state
            .write()
            .set_input(id, json);
    }

    /// Get the recorded value of this `RudiElement` from the server state.
    pub fn get_state<T: DeserializeOwned>(&self) -> Option<T> {
        let server_state = use_context::<Signal<ServerState>>();
        let value_opt = server_state
            .read()
            .get_input(&self.id)
            .and_then(|v| json::from_value::<T>(v).ok());
        value_opt
    }

    /// Get the initial value of a child of this `RudiElement` from 
    /// the server state, with a default value. This function is used
    /// to restore a bookmarked state of a child on app load.
    /// By using `peek`, we do not subscribe to the changing server
    /// state during app use, so this function executes once per call.
    pub fn get_initial_state<T: DeserializeOwned>(&self, child_name: &str, default: T) -> T {
        let server_state = consume_context::<Signal<ServerState>>();
        let value_opt = server_state
            .peek()
            .get_input(&format!("{}-{}", self.id, child_name))
            .and_then(|v| json::from_value::<T>(v).ok());
        value_opt.unwrap_or(default)
    }
}
