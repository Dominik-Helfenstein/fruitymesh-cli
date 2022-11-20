use core::fmt;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

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
            Self::GLOBAL => std::path::PathBuf::from_str("templates/global.h")
                .expect("Path for global not found."),
            Self::VENDOR => std::path::PathBuf::from_str("templates/vendor.h")
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

    if !Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        exit(1);
    }

    let module_name: String = Input::with_theme(&theme)
        .with_prompt("Name")
        .interact_text()?;

    println!("Module name: {}", module_name);

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

    Ok(Some(ModuleConfig {
        name: module_name,
        module_type,
        template_path,
    }))
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    match args.action.as_str() {
        "new" => match args.sub_action.as_str() {
            "module" => {
                println!("Create a new module.");

                match prompt_module_config() {
                    Ok(config) => println!("Module user config: {:?}", config),
                    Err(err) => println!("Error getting module user config: {}", err),
                }
            }
            _ => {}
        },
        _ => {}
    }
}
