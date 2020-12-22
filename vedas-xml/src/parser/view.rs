use super::converter;
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
            }
            false => {
                println!("-------------- View --------------");
                // traverse(view.clone(), 0, None)
                traverse(&view, 0, "", "", "")
            }
        },
    }
}

fn traverse(view: &Element, mut depth: usize, prepend: &str, append: &str, parent: &str) {
    let mut tab = String::new();
    if depth > 0 {
        for i in 0..depth {
            tab.push_str("\t")
        }
    }

    if !view.children.is_empty() {
        for child in view.children.iter() {
            match child.name.as_ref() {
                "Container" => {
                    println!("{}{}container!(", tab, prepend);
                    if !child.children.is_empty() {
                        // println!("{}{:#?}", tab, child.name);
                        traverse(child, depth + 1, "", "", "");
                    } else {
                        println!("{}{}{:#?}{}", tab, prepend, child.name, append);
                    }
                    println!("{})", tab);
                    if parent == "Column" {
                        println!("{})", tab);
                    }
                }
                "Column" => {
                    if !child.children.is_empty() {
                        println!("{}{}col!()", tab, prepend);
                        traverse(child, depth + 1, ".push(", ")", "Column");
                        println!("{}{}", tab, append);
                    }
                    // else {
                    //     println!("{}{}{:#?}{}", tab, prepend, child.name, append);
                    // }
                    // if parent == "Column" {
                    //     println!("{})", tab);
                    // }
                }
                "Text" => {
                    println!(
                        "{}{}text!(\"{}\"){}",
                        tab,
                        prepend,
                        child.text.as_ref().unwrap_or(&String::new()),
                        append
                    )
                }
                "Button" => {
                    println!("{}{}btn!(){}", tab, prepend, append)
                }
                _ => {
                    if !child.children.is_empty() {
                        println!("{}{}{:#?}{}", tab, prepend, child.name, append);
                        traverse(child, depth + 1, "", "", "");
                    } else {
                        println!("{}{}{:#?}{}", tab, prepend, child.name, append);
                    }
                }
            }
        }
    }
}
