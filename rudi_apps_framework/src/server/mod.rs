//! The apps framework server module is generally only used by
//! a tool suite's shared/server `build.rs` and `main.rs` at
//! build time and runtime.

// modules
pub mod config;
pub mod state;
pub mod ui;

// imports
use std::path::Path;
use std::collections::HashMap;
use std::fs;

// re-exports
pub use config::*;
pub use state::*;
pub use ui::*;

// assets
pub static RUDI_LOGO_ICO:      Asset = asset!("/assets/favicon.ico");
pub static RUDI_FRAMEWORK_CSS: Asset = asset!("/assets/rudi_apps_framework.css");
pub static RUDI_FRAMEWORK_JS:  Asset = asset!("/assets/rudi_apps_framework.js");

/// Build app configurations for server `main.rs`. Called 
/// by `build.rs` at compile time to read a tool suite's 
/// set of supported apps.
pub fn build(cargo_manifest_dir: &str, out_dir: &str){

    // learn about the tool suite context
    let server_dir = Path::new(cargo_manifest_dir);
    let apps_dir = server_dir.parent().unwrap().parent().unwrap();
    let suite_dir = apps_dir.parent().unwrap();
    let suite_name = suite_dir.file_name().unwrap().to_str().unwrap();

    // read the suite config
    let suite_config_file = apps_dir.join("suite_config.toml");
    let suite_config_toml = fs::read_to_string(&suite_config_file).unwrap();
    let suite_config: SuiteConfig = toml::from_str(&suite_config_toml).unwrap();

    // initalize the configuration object built from app configs
    let mut server_config = ServerConfig{
        dir:  suite_dir,
        name: suite_name,
        suite_config:  suite_config,
        app_configs:   HashMap::new(),
        app_overviews: HashMap::new(),
    };

    // establish the output files
    let app_imports_rs = Path::new(out_dir).join("app_imports.rs");
    let mut app_imports_file = fs::File::create(&app_imports_rs).unwrap();
    let app_step_components_rs = Path::new(out_dir).join("app_step_components.rs");
    let mut app_step_components_file = fs::File::create(&app_step_components_rs).unwrap();
    let server_config_toml = Path::new(&out_dir).join("server_config.toml");
    let mut server_config_file = fs::File::create(&server_config_toml).unwrap();

    // examine all directories in apps_dir for an app config file
    for app_dir in fs::read_dir(apps_dir).unwrap() {
        let app_dir = app_dir.unwrap();
        if app_dir.file_type().unwrap().is_dir() {
            let app_config_file = app_dir.path().join("app_config.toml");
            if app_config_file.exists() {
                let app_config_toml = fs::read_to_string(&app_config_file).unwrap();
                let mut app_config: AppConfig = toml::from_str(&app_config_toml).unwrap();

                writeln!(app_imports_file, "use {}::*;", app_config.name.clone()).unwrap();
                println!("cargo:rerun-if-changed={}", app_config_file.display());

                let app_step_display = r#"{ "display: block;" } else { "display: none;" }"#;
                let mut app_step_order = 1;
                
                writeln!(app_step_components_file, "Some(app_name) if app_name == \"{}\" => {", app_config.name.clone()).unwrap();
                for app_step_config in app_config.app_steps {
                    app_step_config.order = app_step_order;
                    writeln!(app_step_components_file, "    div {").unwrap();
                    writeln!(app_step_components_file, "        class: \"app-step-content\",").unwrap();
                    writeln!(app_step_components_file, "        style: \{ if app_step_name() == Some(\"{}\".to_string()) {} \},", 
                        app_step_config.name.clone(), 
                        app_step_display
                    ).unwrap();
                    writeln!(app_step_components_file, "        \"data-app\": \"{}\",",     app_config.name.clone()).unwrap();
                    writeln!(app_step_components_file, "        \"data-app-step\": \"{}\"", app_step_config.name.clone()).unwrap();
                    writeln!(app_step_components_file, "        {} \{\}",                   app_step_config.component.clone()).unwrap();
                    writeln!(app_step_components_file, "    }").unwrap();
                    app_step_order += 1;
                }
                writeln!(app_step_components_file, "},").unwrap();
                
                let app_overview_file = app_dir.path().join("app_overview.md");
                let app_overview = fs::read_to_string(&app_overview_file)
                    .unwrap_or("No app overview available.".into());

                server_config.app_configs.insert(app_config.name.clone(), app_config);
                server_config.app_overviews.insert(app_config.name.clone(), app_overview);
            }
        }
    }

    // write the server config to TOML
    let server_config_toml = toml::to_string(&server_config).unwrap();
    fs::write(server_config_file, server_config_toml).unwrap();

    // finish
    println!("cargo:rerun-if-changed=build.rs");
}
