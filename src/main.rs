use macroquad::prelude::*;
use macroui::slider::Slider;

#[macroquad::main("Test UI")]
async fn main() {
    let slider_rect = Rect {
        x: 250.0,
        y: 200.0,
        w: 75.0,
        h: 60.0,
    };
    let mut slider = Slider::new(
        slider_rect,
        25.0,
        true,
        WHITE
    );
    loop {
        clear_background(BLACK);

        slider.draw();

        next_frame().await
    }
}
