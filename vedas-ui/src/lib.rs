#![recursion_limit = "512"]
mod constants;
mod helper;
mod components;
pub mod styles;
pub mod utils;

#[cfg(feature = "button")]
pub use components::button::{self, Button, ButtonGroup};
#[cfg(feature = "card")]
pub use components::card::{self, Card};
#[cfg(feature = "carousel")]
pub use components::carousel::{self, Carousel};
#[cfg(feature = "dropdown")]
pub use components::dropdown::{self, Dropdown, DropdownItem};
#[cfg(feature = "forms")]
pub use components::forms::{
    self,
    form_textarea::{self, FormTextArea},
    form_submit::{self, FormSubmit},
    form_select::{self, FormSelect},
    form_label::{self, FormLabel},
    form_input::{self, FormInput},
    form_group::{self, FormGroup},
    form_file::{self, FormFile},
    form_component::{self, Form},
};
#[cfg(feature = "layouts")]
pub use components::layouts::{
    self,
    container::{self, Container},
    item::{self, Item},
};
#[cfg(feature = "modal")]
pub use components::modal::{self, Modal};
#[cfg(feature = "navbar")]
pub use components::navbar::{
    self,
    navbar_item::{self, NavbarItem},
    navbar_dropdown::{self, Dropdown, DropdownItem},
    navbar_container::{self, NavbarContainer},
    navbar_component::{self, Navbar},
};
#[cfg(feature = "spinner")]
pub use components::spinner::{self, Spinner};
#[cfg(feature = "text")]
pub use components::text::{self, Text};
