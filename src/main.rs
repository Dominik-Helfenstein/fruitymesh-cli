use std::process::exit;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("No action provided. Possible actions are: add");
        exit(1);
    }

    let action: &str = match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("The following can be added: module");
                exit(1);
            }

            match args[2].as_str() {
                "module" => {}
                _ => {
                    println!("Action 'add {}' not found.", args[2]);
                    exit(1);
                }
            }

            args[1].as_str()
        }
        _ => {
            println!("Action '{}' not found.", args[1]);
            exit(1);
        }
    };
}
