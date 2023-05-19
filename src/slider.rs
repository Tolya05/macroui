use macroquad::prelude::{
    Rect,
    draw_rectangle,
    Color,
    mouse_position,
    is_mouse_button_down,
    MouseButton::Left,
    draw_rectangle_lines
};

pub struct Slider {
    rect: Rect,
    value: f32,
    vertical: bool,
    color: Color,
}

impl Slider {
    pub fn new(rectangle: Rect, value: f32, is_vertical: bool, color: Color) -> Self {
        if value > 100.0 {
            Self { rect: rectangle, value: 100.0, vertical: is_vertical, color: color }
        }
        else if value < 0.0 {
            Self { rect: rectangle, value: 0.0, vertical: is_vertical, color: color }
        }
        else {
            Self { rect: rectangle, value: value, vertical: is_vertical, color: color }
        }
    }

    fn update(&mut self) {
        let mouse_pos = mouse_position();
        let mouse_rect = Rect {
            x: mouse_pos.0,
            y: mouse_pos.1,
            w: 5.0,
            h: 5.0,
        };
        if self.rect.overlaps(&mouse_rect) && is_mouse_button_down(Left) {
            let value_pos: f32;
            let mouse_overlap_pos: f32;
            if self.vertical {
                value_pos = self.value / 100.0;
                mouse_overlap_pos = (mouse_pos.1 - self.rect.y) / 100.0;
            }
            else {
                value_pos = self.value / 100.0;
                mouse_overlap_pos = (mouse_pos.0 - self.rect.x) / 100.0;
            }
            if mouse_overlap_pos > value_pos {
                self.value += 1.0;
            }
            else if mouse_overlap_pos < value_pos {
                self.value -= 1.0;
            }
            println!("Mouse Pos {} value pos {}", mouse_overlap_pos, value_pos);
        }
        if self.value > 100.0 {
            self.value = 100.0;
        }
        else if self.value < 0.0 {
            self.value = 0.0;
        }
    }

    pub fn draw(&mut self) {
        self.update();
        let inner_width: f32;
        let inner_height: f32;
        if self.vertical {
            inner_height = self.rect.h * (self.value / 100.0);
            inner_width = self.rect.w;
        }
        else {
            inner_width = self.rect.w * (self.value / 100.0);
            inner_height = self.rect.h;
        }
        draw_rectangle(self.rect.x, self.rect.y, inner_width, inner_height, self.color);
        draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 5.0, self.color);
    }

}