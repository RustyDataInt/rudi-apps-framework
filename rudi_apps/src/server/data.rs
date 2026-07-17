//! An app-level cache for loaded data objects. Data objects can be shared 
//! across app steps by providing closures to `ServerData` functions.

// imports
use std::sync::{LazyLock, Mutex};
use std::any::{Any, type_name};
use std::error::Error;
use std::collections::HashMap;

// constants
static SERVER_DATA: LazyLock<Mutex<ServerData>> = LazyLock::new(|| Mutex::new(ServerData::new()));
const GLOBAL_RESOURCE: &str = "_global_resource_";

/// A key for a single data object in the `ServerData` cache. 
/// 
/// `source_id` can be anything but is usually either `GLOBAL_RESOURCE` or the 
/// source_id of a pipeline data package. 
/// 
/// `content_name` is a string identifying the data object within the source, 
/// often the content_file_type of a data package.
/// 
/// `data_type` is the Rust type name of the data object, e.g., "Vec<u8>".
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct DataSource {
    pub source_id: String, 
    pub name:      String,
    pub data_type: String, // e.g., "DataFrame", "Vec<u8>", etc.
} 

/// `ServerData` holds data objects from multiple sources. See `DataSource` for
/// more details on how data object sources are keyed and identified.
pub struct ServerData(HashMap<DataSource, Box<dyn Any + Send + Sync>>);
impl ServerData {

    /// Create a new empty `ServerData` object. This function is called once
    /// at server (not app) startup to create a global data cache. Callers are 
    /// responsible for dropping data objects when they are no longer needed; 
    /// otherwise, a `ServerData` object acts as a persistent session cache.
    pub fn new() -> Self {
        ServerData(HashMap::new())
    }

    /// Insert a data object of any data type into the `ServerData` cache when
    /// the appropriate `DataSource` is known. 
    pub fn insert<T>(data_source: &DataSource, data_object: T)
    where T: Any + Send + Sync,
    {
        if let Ok(mut server_data) = SERVER_DATA.lock() {
            if !server_data.0.contains_key(&data_source) {
                server_data.0.insert(data_source.clone(), Box::new(data_object));
            }
        }
    }

    /// Load a data object of any data type into the `ServerData` cache using 
    /// the provided `create` function, or simply return the `DataSource` if it 
    /// is already present in the cache. `source_id` and `name` are passed 
    /// through to the `create` function if it is called.
    pub fn load<T, F>(
        source_id: &str, 
        name: &str, 
        create: F
    ) -> Result<DataSource, Box<dyn Error>> 
    where T: Any + Send + Sync,
          F: FnOnce(&str, &str) -> Result<T, Box<dyn Error>>, 
    {
        let data_source = DataSource {
            source_id: source_id.to_string(),
            name:      name.to_string(),
            data_type: type_name::<T>().to_string(),
        };
        if let Ok(mut server_data) = SERVER_DATA.lock() {
            if !server_data.0.contains_key(&data_source) {
                let data_object = create(source_id, name)?;
                server_data.0.insert(data_source.clone(), Box::new(data_object));
            }
        }
        Ok(data_source)
    }

    /// Load a data object of any data type into the `ServerData` cache using 
    /// `GLOBAL_RESOURCE` as `source_id`.
    pub fn load_global<T, F>(name: &str, create: F) -> Result<DataSource, Box<dyn Error>> 
    where T: Any + Send + Sync,
          F: FnOnce(&str, &str) -> Result<T, Box<dyn Error>>, 
    {
        Self::load::<T, F>(GLOBAL_RESOURCE, name, create)
    }

    /// Apply a closure to a data object of any data type in the `ServerData`
    /// cache, using a `DataSource` returned by `load()` or `load_global()`. 
    pub fn with_data_source<T, R, F>(data_source: &DataSource, op: F) -> Result<R, Box<dyn Error>>
    where T: Any,
          F: FnOnce(&T) -> Result<R, Box<dyn Error>>, 
    {
        if let Ok(server_data) = SERVER_DATA.lock() {
            if let Some(data_object) = server_data.0.get(data_source) {
                if let Some(data_ref) = data_object.downcast_ref::<T>() {
                    return op(data_ref);
                } else {
                    return Err(format!(
                        "ServerData object type mismatch for source_id: {}, name: {}. Expected: {}, found: {}",
                        data_source.source_id, 
                        data_source.name, 
                        data_source.data_type, 
                        type_name::<T>()).into()
                    );
                }
            } else {
                return Err(format!(
                    "ServerData object not found for source_id: {}, name: {}",
                    data_source.source_id, 
                    data_source.name).into()
                );
            }
        } else {
            return Err("Failed to lock ServerData".into());
        }
    }

    /// Update a data object in the cache by applying a closure to it.
    pub fn update<T, F>(data_source: &DataSource, op: F)
    where T: Any + Send + Sync,
          F: FnOnce(Option<T>) -> T, 
    {
        if let Ok(mut server_data) = SERVER_DATA.lock() {
            let current_opt_boxed = server_data.0.remove(data_source);
            let current_opt_t = current_opt_boxed
                .and_then(|b| b.downcast::<T>().ok())
                .map(|b| *b);
            let new_t = op(current_opt_t);
            server_data.0.insert(data_source.clone(), Box::new(new_t));
        } 
    }

    /// Drop a data object of any data type from the `ServerData` cache.
    pub fn drop(data_source: &DataSource) {
        if let Ok(mut server_data) = SERVER_DATA.lock() {
            server_data.0.remove(data_source);
        }
    }   
}


// /// `SourceData` holds data objects from a single source, e.g., a data package 
// /// or a global resource.
// /// 
// /// Data objects are stored in HashMaps keyed by object names assigned by the 
// /// calling app.
// /// 
// /// `data_frames` holds `rlike::DataFrame` objects, while `data_objects` holds 
// /// boxed arbitrary data types.
// struct SourceData {
//     data_frames:  HashMap<String, DataFrame>,
//     // data_objects: HashMap<String, Box<dyn Any>>,
// }
// impl SourceData {
//     /// Create a new empty `SourceData` object, with a `HashMap` to hold 
//     /// `DataFrame` and arbitrary data objects. Once `SourceData` object is
//     /// created for global resources and each keyed data source.
//     fn new() -> Self {
//         SourceData {
//             data_frames:  HashMap::new(),
//             // data_objects: HashMap::new(),
//         }
//     }
// }

// /// `ServerData` holds data objects from multiple sources, including global 
// /// resources not tied to any specific source, and a `by_source` map keyed by 
// /// source name, usually a data package `source_id`.
// pub struct ServerData{
//     global:    SourceData,
//     by_source: HashMap<String, SourceData>,
// }
// impl ServerData {
//     /// Create a new empty `ServerData` object. This function is called once
//     /// at app startup to create a global data cache. Callers are responsible
//     /// for dropping data objects when they are no longer needed; otherwise,
//     /// a `ServerData` object acts as a persistent session cache.
//     pub fn new() -> Self {
//         ServerData {
//             global:    SourceData::new(),
//             by_source: HashMap::new(),
//         }
//     }

//     /// Store a `DataFrame` object in the `ServerData` cache. If `source_id` is 
//     /// None, the `DataFrame` will be stored in the global cache.
//     pub fn set_data_frame(
//         // &mut self, 
//         source_id: Option<&str>, 
//         name:      &str, 
//         df:        DataFrame
//     ) {
//         if let Ok(mut server_data) = SERVER_DATA.lock() {
//             match source_id {
//                 Some(source_name) => {
//                     let source_data = server_data.by_source
//                         .entry(source_name.to_string())
//                         .or_insert_with(SourceData::new);
//                     source_data.data_frames.insert(name.to_string(), df);
//                 },
//                 None => {
//                     server_data.global.data_frames.insert(name.to_string(), df);
//                 }
//             }
//         }
//     }

//     // /// Store a boxed arbitrary data object in the `ServerData` cache. If 
//     // /// `source_id` is None, the data object will be stored in the global cache.
//     // pub fn set_data_object(
//     //     &mut self,
//     //     source_id: Option<&str>,
//     //     name:      &str,
//     //     object:    Box<dyn Any>
//     // ) {
//     //     match source_id {
//     //         Some(source_name) => {
//     //             let source_data = self.by_source
//     //                 .entry(source_name.to_string())
//     //                 .or_insert_with(SourceData::new);
//     //             source_data.data_objects.insert(name.to_string(), object);
//     //         },
//     //         None => {
//     //             self.global.data_objects.insert(name.to_string(), object);
//     //         }
//     //     }
//     // }

//     /// Retrieve a `DataFrame` object from the `ServerData` cache.
//     /// 
//     /// If a `create` function is provided, it will be called to attempt to
//     /// create the `DataFrame` if it is not already found in the cache.
//     /// Arguments `source_id` and `name` are passed through to `create()`.
//     pub fn get_data_frame<'a, F>(
//         // &mut self,
//         source_id: Option<&'a str>,
//         name:      &'a str,
//         create:    Option<F>
//     ) -> Option<MutexGuard<'static, DataFrame>> 
//     where F: FnOnce(Option<&'a str>, &'a str) -> Option<DataFrame>
//     {
//         if let Ok(mut server_data) = SERVER_DATA.lock() {
//             match source_id {
//                 Some(source_name) => {
//                     None
//                     // if !server_data.by_source.contains_key(source_name) {
//                     //     server_data.by_source.insert(source_name.to_string(), SourceData::new());
//                     // }
//                     // if !server_data.by_source.get(source_name).unwrap().data_frames.contains_key(name) {
//                     //     if let Some(create) = create {
//                     //         if let Some(df) = create(source_id, name) {
//                     //             server_data.by_source.get_mut(source_name)
//                     //                 .unwrap()
//                     //                 .data_frames
//                     //                 .insert(name.to_string(), df);
//                     //         }
//                     //     }
//                     // }
//                     // server_data.by_source.get(source_name)
//                     //     .and_then(|source_data| source_data.data_frames.get(name))
//                 },
//                 None => {
//                     if !server_data.global.data_frames.contains_key(name) {
//                         if let Some(create) = create {
//                             if let Some(df) = create(None, name) {
//                                 server_data.global.data_frames.insert(name.to_string(), df);
//                             }
//                         }
//                     }
//                     server_data.global.data_frames.get(name)
//                 }
//             }
//         } else {
//             None
//         }
//     }
// }
