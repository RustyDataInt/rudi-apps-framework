//! An app-level cache for loaded data objects. Data
//! objects can be shared across app steps.

// imports
use std::any::Any;
use std::collections::HashMap;
use rlike::data_frame::DataFrame;

/// `SourceData` holds data objects from a single source,
/// e.g., a data package or a global resource.
/// 
/// Data objects are stored in HashMaps keyed by object 
/// names assigned by the calling app.
/// 
/// `data_frames` holds `rlike::DataFrame` objects, while
/// `data_objects` holds boxed arbitrary data types.
pub struct SourceData {
    data_frames:  HashMap<String, DataFrame>,
    data_objects: HashMap<String, Box<dyn Any>>,
}
impl SourceData {
    /// Create a new empty `SourceData` object.
    pub fn new() -> Self {
        SourceData {
            data_frames:  HashMap::new(),
            data_objects: HashMap::new(),
        }
    }
}

/// `ServerData` holds data objects from multiple sources,
/// including global resources not tied to any specific source. 
/// 
/// The `by_source` map is keyed by source name, usually a
/// data package `source_id`.
pub struct ServerData{
    global:    SourceData,
    by_source: HashMap<String, SourceData>,
}
impl ServerData {
    /// Create a new empty `ServerData` object .
    pub fn new() -> Self {
        ServerData {
            global:    SourceData::new(),
            by_source: HashMap::new(),
        }
    }

    /// Store a `DataFrame` object in the `ServerData` cache.
    pub fn set_data_frame(
        &mut self, 
        source_id: Option<&str>, 
        name:      &str, 
        df:        DataFrame
    ) {
        match source_id {
            Some(source_name) => {
                let source_data = self.by_source.entry(source_name.to_string())
                    .or_insert_with(SourceData::new);
                source_data.data_frames.insert(name.to_string(), df);
            },
            None => {
                self.global.data_frames.insert(name.to_string(), df);
            }
        }
    }

    // /// Retrieve a `DataFrame` object from the `ServerData` cache.
    // pub fn get_data_frame<F>(
    //     &self,
    //     source_id: Option<&str>,
    //     name:      &str,
    //     create: F
    // ) -> Option<&DataFrame> 
    // where F: FnOnce() -> Option<DataFrame>
    // {
    //     // match source_id {
    //     //     Some(source_name) => {
    //     //         self.by_source.get(source_name)
    //     //             .and_then(|source_data| source_data.data_frames.get(name))
    //     //             .or_else(|| {
    //     //                 let df = create();
    //     //                 self.by_source.get(source_name)
    //     //                     .unwrap()
    //     //                     .data_frames
    //     //                     .insert(name.to_string(), df);
    //     //                 self.by_source.get(source_name)
    //     //                     .unwrap()
    //     //                     .data_frames
    //     //                     .get(name)
    //     //             })
    //     //     },
    //     //     None => {
    //             self.global.data_frames.get(name).or_else(|| {
    //                 let df = create();
    //                 self.global.data_frames.insert(name.to_string(), df);
    //                 self.global.data_frames.get(name)
    //             })
    //         // }
    //     }
    // }
}
