#[macro_export]
macro_rules! space {
   ($width:expr, $height:expr) => {
      iced::Space::new($width, $height)
   };
}

#[macro_export]
macro_rules! w_space {
   ($width:expr) => {
      iced::Space::with_width($width)
   };
}

#[macro_export]
macro_rules! h_space {
   ($width:expr) => {
      iced::Space::with_height($width)
   };
}