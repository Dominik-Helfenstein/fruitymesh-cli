use core::fmt;
use std::io::{Read, BufReader};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::{error::Error, fs::File};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use handlebars::{to_json, Handlebars, Helper, Context, RenderContext, Output, HelperResult, JsonRender};
use serde_json::Map;

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

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum ModuleType {
    GLOBAL = 0,
    VENDOR = 1,
}

// #[derive()]
struct GlobalModuleTemplate {
    module_name: String,
    module_description: String,
    vendor_id: u32,
    vendor_module_id: u32,
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
    name: String,
    module_type: ModuleType,
    template_path: PathBuf,
}

fn prompt_module_config() -> Result<Option<ModuleConfig>, Box<dyn Error>> {
    let theme = ColorfulTheme::default();
    println!("Welcome to the setup wizard");

    let module_type_index = match Select::with_theme(&theme)
        .with_prompt("Type")
        .default(0)
        .item(ModuleType::GLOBAL.to_string())
        .item(ModuleType::VENDOR.to_string()) // TODO: replace with iterative
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
    println!("{:?}", template_keys);

    let module_name: String = Input::with_theme(&theme)
        .with_prompt("Name")
        .interact_text()?;

    Ok(Some(ModuleConfig {
        name: module_name,
        module_type,
        template_path,
    }))
}

fn replace_file(module_config: ModuleConfig) {
    let mut template_file: File =
        File::open(module_config.template_path.as_path()).expect("Unable to open template file");
    let mut file_buf = String::new();
    template_file
        .read_to_string(&mut file_buf)
        .expect("Unable to read template file");

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_helper("upper", Box::new(upper_helper));

    let header_str = format!("{}.h", module_config
                .template_path
                .to_str()
                .expect("Could not convert template path to string"));
    handlebars
        .register_template_file(
            &module_config.name[..],
            header_str,
        )
        .expect("Unable to register template file");

    let mut output_file =
        File::create("templates/OutputGlobalModule.h").expect("Unable to create output file");

    let mut data = Map::new();
    data.insert("module_name".to_string(), to_json(&module_config.name[..]));
    data.insert("module_description".to_string(), to_json("description"));
    data.insert("vendor_id".to_string(), to_json("VeNdOr_Id"));
    data.insert("vendor_module_id".to_string(), to_json("VeNdOr_MoDuLe_Id"));

    handlebars
        .render_to_write(&module_config.name[..], &data, &mut output_file)
        .expect("Could not render output file");
}

fn upper_helper (h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(param.value().render().to_uppercase().as_str())?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

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
