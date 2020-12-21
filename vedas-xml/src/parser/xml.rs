use super::{config::config_parser, view::view_parser};
use treexml::Element;

pub fn xml_parser(doc: Element) {
    config_parser(&doc);
}
