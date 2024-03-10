use raylib::prelude::*;
use std::ffi::CString;

pub struct Card {
    pub bounds: Rectangle,

    pub toggle: bool,
    pub value: CString,
}
impl Card {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Card {
        Card {
            bounds: Rectangle::new(x, y, width, height),
            toggle: false,
            value: CString::default(),
        }
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>) {
        d.gui_button(self.bounds, Some(self.value.as_c_str()));
    }
}