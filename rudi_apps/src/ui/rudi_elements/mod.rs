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

/// `Namespace` is a newtype wrapper for a `String` that is used 
/// to provide a namespace to a RudiElement's children via 
/// `use_context_provider(|| Namespace::from(&this))`.
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

    /// Create a new `RudiElement` with the given name and namespace.
    /// A `namespace` is required except for top-level elements, most
    /// notably app steps, in which case the namespace will be "app".
    pub fn app_step<T>(
        name: &str, 
    ) -> RudiElement {
        let namespace = Namespace("app".to_string());
        Self::new_rudi_element::<T>(name, namespace)
    }

    /// Create a new `RudiElement` with the given name and namespace.
    /// A `namespace` is required except for top-level elements, most
    /// notably app steps, in which case the namespace will be "app".
    pub fn new<T>(
        name: &str, 
    ) -> RudiElement {
        log::info!("Creating new RudiElement with name: {}", name);
        log::info!("TRYING: use_context::<Namespace>()");
        let namespace = use_context::<Namespace>();
        log::info!("SUCCESS: use_context::<Namespace>() {}", namespace.0);
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
}
