#[macro_export]
macro_rules! slider {
   ($state:expr, $min:expr, $max:expr, $val:expr, $message:expr) => {
      iced::Slider::new($state, $min as f32..=$max as f32, $val as f32, $message)
   };
}