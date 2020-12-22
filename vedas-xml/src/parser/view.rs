use treexml::Element;

pub fn view_parser(doc: &Element) {
    let view = doc.find_child(|tag| tag.name == "View");

    match view {
        None => {
            println!("No <View/> found!");
        }
        Some(view) => match view.children.is_empty() {
            true => println!("Generating empty application!"),
            false => {
                println!("-------------- View --------------");
                println!("{:?}", view.children)
            }
        },
    }
}
