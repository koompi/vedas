use treexml::Element;

pub fn config_parser(doc: &Element) {
    let config = doc.find_child(|tag| tag.name == "Config");

    match config {
        None => {
            println!("-------------- Conf --------------");
            println!("WARNING:");
            println!("No configuration was provided!");
            println!("Generating Appplication with default settings...");
        }
        Some(config) => match config.children.is_empty() {
            true => {
                println!("-------------- Conf --------------");
                println!("Generating Appplication with default settings...")
            },
            false => {
                println!("-------------- Conf --------------");
                println!("{:?}", config.children);
            }
        },
    }
}
