use anyhow::{Context, Ok, Result};
use clap::Parser;
use secrets_cli::{
    config::Config,
    opts::{Action, Opts},
    utils::{check_file, check_folder},
};
use serde_json::{from_str, to_string, to_string_pretty, Value};
use std::io::Write;
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let config = Config::create()?;
    let opts = Opts::parse();
    match opts.action {
        Action::Copy(val) => {
            let project_path = PathBuf::from(val.project.unwrap_or("var".to_string()));
            let variable_file_path = config.secrets_path.join(project_path);
            check_file(&variable_file_path).context(format!(
                "Variables file not found. Create it at {:?}",
                variable_file_path
            ))?;
            let var_str = show_variables(&variable_file_path)?;

            let mut child = std::process::Command::new(config.clipboard_command)
                .stdin(std::process::Stdio::piped())
                .spawn()
                .context("Clipboard not found")?;
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(var_str.as_bytes())
                .context("Clipboard not written")?;
        }
        Action::Show(val) => {
            let project_path = PathBuf::from(val.project.unwrap_or("var".to_string()));
            let variable_file_path = config.secrets_path.join(project_path);
            check_file(&variable_file_path).context(format!(
                "Variables file not found. Create it at {:?}",
                variable_file_path
            ))?;

            let var_str = show_variables(&variable_file_path)?;

            println!("{}", var_str);
        }
        Action::Fish(val) => {
            let project_path = PathBuf::from(val.project.unwrap_or("".to_string()));
            let variable_file_path = config.secrets_path.join(project_path).join("var");
            check_file(&variable_file_path).context(format!(
                "Variables file not found. Create it at {:?}",
                variable_file_path
            ))?;
            let var_str = show_variables(&variable_file_path)?;
            for ele in var_str.split('\n') {
                println!("set -Ux {}", ele);
            }
        }
        Action::Set(val) => {
            println!("Setting secrets path to {:?}", val.path);
            println!("Setting clipboard command to {:?}", val.clipboard);
            check_folder(&val.path)?;
            set_config(&val.path, &val.clipboard, &config.config_path)?;
        }
        Action::Config => {
            println!("{:?}", config);
        }
    }
    Ok(())
}

/**
 * Set the secrets path in the config file
 * @param path Path to the secrets folder
 * @return Result
 */
fn set_config(path: &PathBuf, clipboard: &str, config_path: &PathBuf) -> Result<()> {
    // secrets path
    let secrets_json: serde_json::Value = from_str(&to_string(path).context("Json not valid")?)
        .with_context(|| format!("Json not valid: {:?}", path))?;
    let clipboard_json: serde_json::Value =
        from_str(&to_string(clipboard).context("Json not valid")?)
            .with_context(|| format!("Json not valid: {:?}", clipboard))?;

    // read config
    let mut config_string = std::fs::read_to_string(config_path).context("Config not found")?;
    let mut config_json: Value = from_str(&config_string).context("Config not valid")?;

    // write config
    config_json["secrets_path"] = secrets_json;
    config_json["clipboard_command"] = clipboard_json;
    config_string = to_string_pretty(&config_json)
        .with_context(|| format!("Config not valid {:?}", config_string))?;

    // save config
    fs::write(config_path, config_string)
        .with_context(|| format!("Config not written to {:?}", config_path))?;

    Ok(())
}

fn show_variables(file_path: &PathBuf) -> Result<String> {
    let var_file = fs::read_to_string(file_path).context("Variables file not found")?;
    let lines = var_file.split('\n').collect::<Vec<&str>>();
    let mut var_str: String = "".to_string();
    for ele in lines {
        // split only the first =
        let mut ele = ele.splitn(2, '=').collect::<Vec<&str>>();
        if ele.len() == 2 {
            ele[0] = ele[0].trim();
            ele[1] = ele[1].trim();
            var_str.push_str(&format!("    {}={} \\\n", ele[0], ele[1]));
            // println!(r"    {}={} \", ele[0], ele[1]);
        }
    }
    Ok(var_str)
}
