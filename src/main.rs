use core::fmt;
use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, BufReader};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::{error::Error, fs::File};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use handlebars::{to_json, Handlebars, Helper, Context, RenderContext, Output, HelperResult, JsonRender};
use serde_json::Map;
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

// TODO: get rid of this and walk through folder instead
#[derive(Debug, FromPrimitive, ToPrimitive)]
enum ModuleType {
    GLOBAL = 0,
    VENDOR = 1,
}

impl ModuleType {
    fn from_usize(value: usize) -> Self {
        match value {
            0 => Self::GLOBAL,
            1 => Self::VENDOR,
            _ => Self::GLOBAL,
        }
    }

    fn get_template_path(&self) -> std::path::PathBuf {
        match *self {
            Self::GLOBAL => std::path::PathBuf::from_str("templates/GlobalModule")
                .expect("Path for global not found."),
            Self::VENDOR => std::path::PathBuf::from_str("templates/VendorModule")
                .expect("Path for vendor not found."),
        }
    }
}

impl fmt::Display for ModuleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct ModuleConfig {
    module_type: ModuleType,
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
    let module_type = ModuleType::from_usize(module_type_index);
    let template_path = module_type.get_template_path();

    let template_keys_path = format!("{}.json", template_path.to_str().expect("unable to get str from template path"));
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
        module_type,
        template_path,
        template_replacements,
    }))
}

fn replace_file(module_config: ModuleConfig) {
    let template_path = format!("{}.h", module_config.template_path.to_str().expect("Could not convert template path to str"));
    let mut template_file: File =
        File::open(template_path).expect("Unable to open template file");
    let mut file_buf = String::new();
    template_file
        .read_to_string(&mut file_buf)
        .expect("Unable to read template file");

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_helper("upper", Box::new(upper_helper));

    let header_file_str = format!("{}.h", module_config
                .template_path
                .to_str()
                .expect("Could not convert template path to string"));
    const HEADER_TEMPLATE_NAME: &str = "header-template";
    handlebars
        .register_template_file(
            HEADER_TEMPLATE_NAME,
            header_file_str,
        )
        .expect("Unable to register template file");

    let mut output_file =
        File::create("templates/Output.h").expect("Unable to create output file");

    handlebars
        .render_to_write(HEADER_TEMPLATE_NAME, &module_config.template_replacements, &mut output_file)
        .expect("Could not render output file");
}

fn upper_helper (h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
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
        let filename = path.expect("Could not read filename.").path().display().to_string().replace("./templates/", "");
        let name = filename.replace(".cpp", "").replace(".h", "").replace(".json", "").replace(".c", "").replace(".hpp", "");
        let has_json = filename.contains(".json");
        let has_header = filename.contains(".cpp") || filename.contains(".c");
        let has_source = filename.contains(".h") || filename.contains(".hpp");
        if let Some(current_module_type) = module_type_readings.iter_mut().filter(|m| m.name == name).next() {
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
    let module_types: Vec<String> = module_type_readings.iter().filter(|m| m.has_json && m.has_header && m.has_source).map(|m| {
        // if !m.has_json {
        //     panic!("{} does not have a json file! The json file must have an array of strings. Each string is one key that will be replaced in the template files.", m.name);
        //     return;
        // }
        // if !m.has_header {
        //     panic!("{} does not have a header file! Currently, header and source templates are necessary.", m.name);
        // }
        // if !m.has_source {
        //     panic!("{} does not have a source file! Currently, header and source templates are necessary.", m.name);
        // }
        return m.name.clone();
    }).collect();

    module_types
}

fn main() {
    let args = Args::parse();

    match args.action.as_str() {
        "new" => match args.sub_action.as_str() {
            "module" => {
                println!("Create a new module.");

                match prompt_module_config() {
                    Ok(config) => {
                        println!("Module user config: {:?}", config);
                        let module_config = config.unwrap();
                        replace_file(module_config);
                    }
                    Err(err) => println!("Error getting module user config: {}", err),
                }
            }
            _ => {}
        },
        _ => {}
    }
}
