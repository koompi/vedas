use treexml::Element;

pub fn view_parser(doc: &Element) {
    let view = doc.find_child(|tag| tag.name == "View");

    match view {
        None => {
            println!("-------------- View --------------");
            println!("No <View/> found!");
        }
        Some(view) => match view.children.is_empty() {
            true => {
                println!("-------------- View --------------");
                println!("Generating empty application!");
            },
            false => {
                println!("-------------- View --------------");
                traverse(view.clone(), 0)
            }
        },
    }
}

fn traverse(view: Element, depth: u32) {
    view.children.iter().for_each(|child| {
        let mut pad = String::new();
        if depth > 0 { for i in 0..depth { pad.push_str("\t") } }
        println!("{}{}: {}", pad, child.name ,child.text.as_ref().unwrap_or(&String::new()));
        if !child.children.is_empty() {
            traverse(child.clone(), depth + 1)
        }
    })
}