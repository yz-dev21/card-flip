use raylib::prelude::*;

mod card;

const CARD_SIZE: i32 = 100;
const GAP: i32 = 5;

fn get_count() -> i32 {
    use std::io::{stdin, stdout, Write};

    let mut s = String::new();

    print!("Count: ");
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Wrong string.");

    s.trim().parse::<i32>().unwrap()
}
fn init_cards(cards: &mut Vec<card::Card>, count: i32) {
    let mut x = 0.0;
    let mut y = 0.0;

    for cy in 0..count {
        for cx in 0..count {
            cards.push(card::Card::new(x, y, CARD_SIZE as f32, CARD_SIZE as f32));
            x += 100.0;

            if cx != count - 1 {
                x += GAP as f32;
            }
        }
        x = 0.0;
        y += 100.0;

        if cy != count - 1 {
            y += GAP as f32;
        }
    }
}
fn main() {
    let count = get_count();
    let window_size = CARD_SIZE * count + GAP * (count - 1);

    let (mut rl, thread) = raylib::init()
        .size(window_size, window_size)
        .title("card-flip")
        .build();

    let mut cards = Vec::<card::Card>::with_capacity((count * count) as usize);
    init_cards(&mut cards, count);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::WHITE);

            for c in &mut cards {
                c.draw(&mut d);
            }
        }
    }
}
