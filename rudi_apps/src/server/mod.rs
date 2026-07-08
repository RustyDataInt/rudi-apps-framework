//! The apps framework `server` module is generally only used 
//! directly by a tool suite's shared/server `build.rs` and 
//! `main.rs` at build time and runtime, respectively.
//! 
//! The `state::ServerState` and, less commonly, 
//! `config::ServerConfig` objects are used indirectly in apps 
//! by calling `use_context::<Signal<ServerState>>()` and
//! `use_context::<ServerConfig>()`, respectively.

// RuDI developer note: script:
//     `rudi-suite-template/apps/dioxus/shared/server/build.rs`
// rarely changes even if updates are made to the shared builder:
//     `rudi-apps-framework/rudi_apps/src/server/mod.rs::build()`
// so `println!("cargo:rerun-if-changed=build.rs");` doesn't do much.
// When working on changes to the shared `build()` function
// it is best to touch this file when compiling:
//     `touch build.rs && dx build`
// Changes to the user's config files are separately tracked to 
// trigger builds, these comments only apply when updating `build()`.

// modules
pub mod config;
pub mod state;

// imports
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use dioxus::prelude::*;

// re-exports
pub use config::*;
pub use state::*;

// assets
pub static RUDI_LOGO_ICO:       Asset = asset!("/assets/favicon.ico");
pub static RUDI_THEME_CSS:      Asset = asset!("/assets/rudi_apps_theme.css");
pub static RUDI_LAYOUT_CSS:     Asset = asset!("/assets/rudi_apps_layout.css");
pub static RUDI_FRAMEWORK_JS:   Asset = asset!("/assets/rudi_apps.js");
pub static DX_COMPONENTS_THEME: Asset = asset!("/assets/dx-components-theme.css");

/// Build app configurations for server `main.rs`. Called 
/// by `build.rs` at compile time to read a tool suite's 
/// set of supported apps via its TOML configuration files.
pub fn build(cargo_manifest_dir: &str, out_dir: &str){

    // learn about the tool suite context
    let server_dir_path = Path::new(cargo_manifest_dir);
    let apps_dir_path = server_dir_path.parent().unwrap().parent().unwrap();
    let suite_dir_path = apps_dir_path.parent().unwrap();
    let suite_dir = suite_dir_path.to_str().unwrap();
    let suite_name = suite_dir_path.file_name().unwrap().to_str().unwrap();

    // read the suite config
    let suite_config_path_buf = apps_dir_path.join("suite_config.toml");
    let suite_config_toml = fs::read_to_string(&suite_config_path_buf).unwrap();
    let suite_config: SuiteConfig = toml::from_str(&suite_config_toml).unwrap();

    // initalize the configuration object built from app configs
    let mut server_config = ServerConfig{
        dir:  suite_dir.to_string(),
        name: suite_name.to_string(),
        suite_config:  suite_config,
        app_configs:   HashMap::new(),
        app_overviews: HashMap::new(),
    };

    // establish the output files
    let app_imports_rs = Path::new(out_dir).join("app_imports.rs");
    let mut app_imports_file = fs::File::create(&app_imports_rs).unwrap();
    let app_matcher_rs = Path::new(out_dir).join("app_matcher.rs");
    let mut app_matcher_file = fs::File::create(&app_matcher_rs).unwrap();
    let server_config_path_buf = Path::new(&out_dir).join("server_config.toml");
    // let mut server_config_file = fs::File::create(&server_config_toml).unwrap();

    // initialize the app_matcher
    writeln!(app_matcher_file, "match server_state.read().get_app() {{").unwrap();

    // examine all directories in apps_dir for an app config file
    for app_dir in fs::read_dir(apps_dir_path).unwrap() {
        let app_dir = app_dir.unwrap();
        if app_dir.file_type().unwrap().is_dir() {
            let app_config_file = app_dir.path().join("app_config.toml");
            if app_config_file.exists() {
                let app_config_toml = fs::read_to_string(&app_config_file).unwrap();
                let mut app_config: AppConfig = toml::from_str(&app_config_toml).unwrap();

                // import the app
                writeln!(app_imports_file, "use {}::*;", app_config.name.clone()).unwrap();
                println!("cargo:rerun-if-changed={}", app_config_file.display());

                // add the app step components for each matchable app
                writeln!(app_matcher_file, "    Some(app_name) if app_name == \"{}\" => rsx!{{", app_config.name.clone()).unwrap();
                add_app_step_match(
                    &mut app_matcher_file, 
                    &app_config.name, 
                    "app_overview", 
                    "AppOverview"
                );
                let mut app_step_order = 1;
                for app_step_config in &mut app_config.app_steps {
                    app_step_config.order = app_step_order;
                    if app_step_config.tooltip.is_none() {
                        app_step_config.tooltip = Some(app_step_config.title.clone());
                    }
                    let app_step_instructions_file = app_dir.path().join(format!("src/{}/instructions.md", app_step_config.name));
                    app_step_config.instructions = fs::read_to_string(&app_step_instructions_file).ok();
                    let app_step_settings_file = app_dir.path().join(format!("src/{}/settings.yml", app_step_config.name));
                    app_step_config.settings = fs::read_to_string(&app_step_settings_file).ok();
                    add_app_step_match(
                        &mut app_matcher_file, 
                        &app_config.name, 
                        &app_step_config.name, 
                        &app_step_config.component
                    );
                    app_step_order += 1;
                }
                writeln!(app_matcher_file, "    }},").unwrap();

                // collect any available app overview
                let app_overview_file = app_dir.path().join("app_overview.md");
                let app_overview = fs::read_to_string(&app_overview_file).unwrap_or("".into());

                // collect data into the server_config
                server_config.app_overviews.insert(app_config.name.clone(), app_overview);
                server_config.app_configs.insert(app_config.name.clone(), app_config);
            }
        }
    }

    // finish the app_matcher, defaulting to AppChooser until an app is chosen
    writeln!(app_matcher_file, "    Some(app_name) => rsx!{{ AppNotFound {{app_name}} }},").unwrap();
    writeln!(app_matcher_file, "    _ => rsx!{{ AppChooser {{}} }},").unwrap();
    writeln!(app_matcher_file, "}}").unwrap();

    // write the server config to TOML
    let server_config_toml = toml::to_string(&server_config).unwrap();
    fs::write(server_config_path_buf, server_config_toml).unwrap();

    // finish
    println!("cargo:rerun-if-changed=build.rs");
}

/// Add a match arm to the app_matcher for a single app step,
/// including the app overview step.
fn add_app_step_match(
    app_matcher_file: &mut File, 
    app_name:         &str, 
    app_step_name:    &str, 
    component_name:   &str
) {
    const APP_STEP_DISPLAY: &str = r#"{ "display: block;" } else { "display: none;" }"#;
    writeln!(app_matcher_file, "        div {{").unwrap();
    writeln!(app_matcher_file, "            class: \"app-step-content\",").unwrap();
    writeln!(app_matcher_file, "            style: {{ if app_step_name() == Some(\"{}\".to_string()) {} }},", 
        app_step_name, 
        APP_STEP_DISPLAY
    ).unwrap();
    writeln!(app_matcher_file, "           \"data-app\": \"{}\",",       app_name).unwrap();
    writeln!(app_matcher_file, "            \"data-app-step\": \"{}\",", app_step_name).unwrap();
    writeln!(app_matcher_file, "            {} {{}}",                    component_name).unwrap();
    writeln!(app_matcher_file, "        }}").unwrap();
}
