use super::{config::config_parser, view::view_parser};
use treexml::Element;

pub fn parse(doc: Element) {
    // config_parser(&doc);
    view_parser(&doc);
}
