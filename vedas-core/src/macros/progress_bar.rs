#[macro_export]
macro_rules! progress_bar {
   ($val:expr, $min:expr, $max:expr) => {
      iced::ProgressBar::new($min as f32..=$max as f32, $val as f32)
   };
}