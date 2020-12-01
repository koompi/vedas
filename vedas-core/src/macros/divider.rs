#[macro_export]
macro_rules! h_divider {
   () => {
      iced::Rule::horizontal(1)
   };
   ($spacing:expr) => {
      iced::Rule::horizontal($spacing)
   };
}

#[macro_export]
macro_rules! v_divider {
   () => {
      iced::Rule::vertical(1)
   };
   ($spacing:expr) => {
      iced::Rule::vertical($spacing)
   };
}