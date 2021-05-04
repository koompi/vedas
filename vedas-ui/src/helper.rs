use crate::constants::PREFIX_CLASS;
use wasm_bindgen_test::*;

#[cfg(feature = "layouts")]
use rand::{distributions::Alphanumeric, thread_rng, Rng};
#[cfg(feature = "layouts")]
use web_sys::{window, HtmlElement};
#[cfg(feature = "layouts")]
use yew::utils;

#[cfg(feature = "layouts")]
pub fn create_style(style: String, value: String, wrap: String) {
    let element = get_html_element_by_class(&wrap, 0);

    element.style().set_property(&style, &value).unwrap();
}

#[cfg(feature = "layouts")]
pub fn get_random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[cfg(feature = "layouts")]
pub fn get_html_element_by_class(class_name: &str, index: u32) -> HtmlElement {
    utils::document()
        .get_elements_by_class_name(class_name)
        .get_with_index(index)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
}

pub fn get_prefix_class<S: ToString>(suffix_class: Option<S>, custom_prefix_class: Option<S>) -> String {
    if let Some(custom_class) = custom_prefix_class {
        custom_class.to_string()
    } else if let Some(suffix_class) = suffix_class {
        if !PREFIX_CLASS.is_empty() {
            format!("{}-{}", PREFIX_CLASS, suffix_class.to_string())
        } else {
            suffix_class.to_string()
        }
    } else {
        String::from(PREFIX_CLASS)
    }
}

pub fn get_prefix_concat_with<S: ToString>(prefix_class: &str, suffix_class: S) -> String {
    let suffix_class = suffix_class.to_string();

    if !suffix_class.is_empty() {
        format!("{}-{}", prefix_class, suffix_class)
    } else {
        String::default()
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(feature = "layouts")]
#[wasm_bindgen_test]
fn should_set_style_prop() {
    let body = window().unwrap().document().unwrap().body().unwrap();

    let element = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    element.set_text_content(Some("item"));
    element.set_class_name("item");
    body.append_child(&element).unwrap();

    create_style(
        "padding".to_string(),
        "10px".to_string(),
        "item".to_string(),
    );

    let item = window()
        .unwrap()
        .document()
        .unwrap()
        .get_elements_by_class_name("item")
        .get_with_index(0)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    let value = item.style().get_property_value("padding").unwrap();

    assert_eq!(value, "10px");
}

#[cfg(feature = "layouts")]
#[wasm_bindgen_test]
fn should_generate_random_string() {
    let mut random_values: Vec<String> = vec![];
    let mut i = 0;
    loop {
        random_values.push(get_random_string(10));
        i += 1;

        if i == 1000 {
            break;
        }
    }

    for (i, value) in random_values.iter().enumerate() {
        let mut index = 0;
        let repeat = &random_values.iter().any(move |random_value| {
            let exist = random_value == value && i != index;
            index += 1;
            exist
        });
        assert_eq!(*repeat, false);
    }
}
