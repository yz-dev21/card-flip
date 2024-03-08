use raylib::prelude::*;
use std::ffi::CStr;

macro_rules! cstr {
    ($s:literal) => {{
        let s = CStr::from_bytes_with_nul(concat!($s, "\0").as_bytes());
        match s {
            Ok(s) => s,
            Err(_) => panic!("cstr cannot contain null bytes"),
        }
    }};
}

fn draw_grid(mut x: f32, mut y: f32, d: &mut RaylibDrawHandle<'_>) {
    for _ in 1..5 {
        for _ in 1..5 {
            d.gui_button(Rectangle::new(x, y, 100.0, 100.0), Some(cstr!("BTN")));
            x += 100.0;
        }
        x = 0.0;
        y += 100.0;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(400, 400).title("card-flip").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        {
            let x: f32 = 0.0;
            let y: f32 = 0.0;

            d.clear_background(Color::WHITE);
            draw_grid(x, y, &mut d);
        }
    }
}
