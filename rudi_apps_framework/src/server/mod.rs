//! The apps framework server modules is generally only
//! used by a tool suite's server `build.rs` and `main.rs`
//! at build time and runtime.

// modules
pub mod config;

// imports
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use config::*;

/// Build app configurations for server `main.rs`. Called 
/// by build.rs` at compile time to read a tool suite's 
/// set of supported apps.
pub fn build(cargo_manifest_dir: &str, out_dir: &str){
    let server_dir = Path::new(cargo_manifest_dir);
    let apps_dir = server_dir.parent().unwrap().parent().unwrap();
    let suite_dir = apps_dir.parent().unwrap();
    let suite_name = suite_dir.file_name().unwrap().to_str().unwrap();

    let suite_config_file = suite_dir.join("suite_config.toml");
    let suite_config_toml = fs::read_to_string(&suite_config_file).unwrap();
    let suite_config: SuiteConfig = toml::from_str(&suite_config_toml).unwrap();

    let server_code_rs = Path::new(out_dir).join("generated_server_code.rs");
    let mut server_code_file = fs::File::create(&server_code_rs).unwrap();
    let server_config_toml = Path::new(&out_dir).join("server_config.toml");
    let mut server_config_file = fs::File::create(&server_config_toml).unwrap();

    let app_step_routes: Vec<(String, String)> = Vec::new();
    let server_config = ServerConfig{
        dir:  suite_dir,
        name: suite_name,
        suite_config:  suite_config,
        app_configs:   HashMap::new(),
        app_overviews: HashMap::new(),
    };

    // examine all directories in apps_dir for an app config file
    for entry in fs::read_dir(apps_dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let app_config_file = entry.path().join("app_config.toml");
            if app_config_file.exists() {
                let app_config_toml = fs::read_to_string(&app_config_file).unwrap();
                let app_config: AppConfig = toml::from_str(&app_config_toml).unwrap();

                writeln!(server_code_file, "use {}::*;", app_config.name.clone()).unwrap();
                println!("cargo:rerun-if-changed={}", app_config_file.display());

                for app_step_config in app_config.app_steps {
                    app_step_routes.push((
                        format!(
                            "    #[route(\"/{}/{}\")]", 
                            app_config.name.clone(), 
                            app_step_config.name.clone()
                        ),
                        format!(
                            "    {} \{\},", 
                            app_step_config.component.clone()
                        ),
                    ));
                }
                
                let app_overview_file = entry.path().join("app_overview.md");
                let app_overview = fs::read_to_string(&app_overview_file)
                    .unwrap_or("No app overview available.".into());

                server_config.app_configs.insert(app_config.name.clone(), app_config);
                server_config.app_overviews.insert(app_config.name.clone(), app_overview);
            }
        }
    }

    writeln!(server_code_file, "").unwrap();
    writeln!(server_code_file, "/// App router paths.").unwrap();
    writeln!(server_code_file, "#[derive(Debug, Clone, Routable, PartialEq)]").unwrap();
    writeln!(server_code_file, "#[rustfmt::skip]").unwrap();
    writeln!(server_code_file, "enum RudiRouter {").unwrap();
    writeln!(server_code_file, "    #[route(\"/\")]").unwrap();
    writeln!(server_code_file, "    AppChooser \{\}").unwrap();
    for (route, component) in app_step_routes {
        writeln!(server_code_file, "{}", route).unwrap();
        writeln!(server_code_file, "{}", component).unwrap();
    }
    writeln!(server_code_file, "}").unwrap();

    let server_config_toml = toml::to_string(&server_config).unwrap();
    fs::write(server_config_file, server_config_toml).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
