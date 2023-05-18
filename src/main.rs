use macroquad::prelude::*;
use macroui::button::Button;

#[macroquad::main("Test UI")]
async fn main() {
    let btn_rect = Rect {
        x: 100.0,
        y: 200.0,
        w: 75.0,
        h: 60.0,
    };
    let mut btn = Button::new(btn_rect, String::from("Hello"), 25.0, BLACK, WHITE);
    loop {
        clear_background(WHITE);

        btn.draw();

        if btn.clicked() {
            println!("Button Clicked!");
        }

        next_frame().await
    }
}
