use core::fmt;
use std::collections::BTreeMap;
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{error::Error, fs::File};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};
use titlecase::titlecase;

// use serde::Serialize;
// use serde_json::Result;

extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    action: String,
    sub_action: String,
}

enum Action {
    NEW(SubActionNew),
}

enum SubActionNew {
    MODULE,
}

#[derive(Debug)]
struct ModuleConfig {
    template_path: PathBuf,
    template_replacements: BTreeMap<String, String>,
}

fn prompt_module_config() -> Result<Option<ModuleConfig>, Box<dyn Error>> {
    let theme = ColorfulTheme::default();
    println!("Welcome to the setup wizard");

    let module_types = read_module_types();
    println!("If a module type is not listed here it is because one of the header, source or json files is missing.");

    let module_type_index = match Select::with_theme(&theme)
        .with_prompt("Module Type")
        .default(0)
        .items(&module_types[..])
        .interact_opt()?
    {
        Some(index) => index,
        None => 0,
    };
    let module_type = module_types[module_type_index].clone();
    let template_path = PathBuf::from_str(format!("templates/{module_type}").as_str()).unwrap();

    let template_keys_path = format!(
        "{}.json",
        template_path
            .to_str()
            .expect("unable to get str from template path")
    );
    let template_keys_file = File::open(template_keys_path)?;
    let reader = BufReader::new(template_keys_file);
    let template_keys: Vec<String> = serde_json::from_reader(reader)?;

    let mut template_replacements: BTreeMap<String, String> = BTreeMap::new();
    for key in template_keys {
        let value: String = Input::with_theme(&theme)
            .with_prompt(titlecase(key.replace("_", " ").as_str()))
            .interact_text()?;
        template_replacements.insert(key, value);
    }

    Ok(Some(ModuleConfig {
        template_path,
        template_replacements,
    }))
}

fn replace_files(module_config: ModuleConfig) {
    let header_path = format!(
        "{}.h",
        module_config
            .template_path
            .to_str()
            .expect("Could not convert template path to header string")
    );
    let source_path = format!(
        "{}.cpp",
        module_config
            .template_path
            .to_str()
            .expect("Could not convert template path to source string")
    );

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_helper("upper", Box::new(upper_helper));

    const HEADER_TEMPLATE_NAME: &str = "header-template";
    const SOURCE_TEMPLATE_NAME: &str = "source-template";
    handlebars
        .register_template_file(HEADER_TEMPLATE_NAME, &header_path)
        .expect("Unable to register header template file");
    handlebars
        .register_template_file(SOURCE_TEMPLATE_NAME, &source_path)
        .expect("Unable to register source template file");

    dbg!(&module_config.template_replacements);
    let module_file_name = if let Some(module_name) =
        module_config.template_replacements.get("module_name")
    {
        module_name
    } else {
        module_config.template_path.to_str().expect("Could not convert template path to str")
                            .split("/").last().expect("Could not find last element in path that has been split with '/'. Maybe there was no '/' in the path?")
    };

    if !Path::new("output").exists() {
        fs::create_dir("output").expect("Could not create output dir");
    }
    let mut header_output_file = File::create(format!("output/{module_file_name}.h"))
        .expect("Unable to create output header file");
    let mut source_output_file = File::create(format!("output/{module_file_name}.cpp"))
        .expect("Unable to create output source file");

    handlebars
        .render_to_write(
            HEADER_TEMPLATE_NAME,
            &module_config.template_replacements,
            &mut header_output_file,
        )
        .expect("Could not render output header file");
    handlebars
        .render_to_write(
            SOURCE_TEMPLATE_NAME,
            &module_config.template_replacements,
            &mut source_output_file,
        )
        .expect("Could not render output source file");

    println!("Successfully created output/{module_file_name}.h and output/{module_file_name}.cpp");
    println!("Make sure to rename the files to the desired name.")
}

fn upper_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(param.value().render().to_uppercase().as_str())?;
    Ok(())
}

#[derive(Default, Debug)]
struct ModuleTypeReading {
    name: String,
    has_json: bool,
    has_header: bool,
    has_source: bool,
}

fn read_module_types() -> Vec<String> {
    let paths = fs::read_dir("./templates").expect("Could not read templates dir.");
    let mut module_type_readings: Vec<ModuleTypeReading> = Vec::new();
    for path in paths {
        // TODO: just use regex
        let filename = path
            .expect("Could not read filename.")
            .path()
            .display()
            .to_string()
            .replace("./templates/", "");
        let name = filename
            .replace(".cpp", "")
            .replace(".h", "")
            .replace(".json", "")
            .replace(".c", "")
            .replace(".hpp", "");
        let has_json = filename.contains(".json");
        let has_header = filename.contains(".cpp") || filename.contains(".c");
        let has_source = filename.contains(".h") || filename.contains(".hpp");
        if let Some(current_module_type) = module_type_readings
            .iter_mut()
            .filter(|m| m.name == name)
            .next()
        {
            if has_json {
                current_module_type.has_json = true;
            }
            if has_header {
                current_module_type.has_header = true;
            }
            if has_source {
                current_module_type.has_source = true;
            }
        } else {
            let mut current_module_type = ModuleTypeReading {
                name,
                ..Default::default()
            };
            if has_json {
                current_module_type.has_json = true;
            }
            if has_header {
                current_module_type.has_header = true;
            }
            if has_source {
                current_module_type.has_source = true;
            }
            module_type_readings.push(current_module_type);
        }
    }
    let module_types: Vec<String> = module_type_readings
        .iter()
        .filter(|m| m.has_json && m.has_header && m.has_source)
        .map(|m| {
            return m.name.clone();
        })
        .collect();

    module_types
}

fn main() {
    println!("Create a new module.");

    match prompt_module_config() {
        Ok(config) => {
            println!("Module user config: {:?}", config);
            let module_config = config.unwrap();
            replace_files(module_config);
        }
        Err(err) => println!("Error getting module user config: {}", err),
    }
}
