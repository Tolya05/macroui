use macroquad::prelude::{Rect, draw_rectangle, draw_text, Color, measure_text, mouse_position, is_mouse_button_pressed, MouseButton::Left};

pub struct Button {
    rect: Rect,
    text: String,
    font_size: f32,
    rect_color: Color,
    text_color: Color,
    color_copy: Color,
}

impl Button {
    pub fn new(rect_pos: Rect, text: String, text_size: f32, rect_color: Color, text_color: Color) -> Self {
        Self { rect: rect_pos, text: text, font_size: text_size, rect_color: rect_color, text_color: text_color, color_copy: rect_color }
    }
    pub fn draw(&mut self) {
        let mouse_pos = mouse_position();
        let mouse_rect = Rect {
            x: mouse_pos.0,
            y: mouse_pos.1,
            w: 5.0,
            h: 5.0,
        };
        if self.rect.overlaps(&mouse_rect) && (self.rect_color.r >= self.color_copy.r - 0.08) {
            if self.rect_color.r < 0.2 {
                self.rect_color.r += 0.02;
            }
            else {
                self.rect_color.r -= 0.02;
            }
            if self.rect_color.g < 0.2 {
                self.rect_color.g += 0.02;
            }
            else {
                self.rect_color.g -= 0.02;
            }
            if self.rect_color.b < 0.2 {
                self.rect_color.b += 0.02;
            }
            else {
                self.rect_color.b -= 0.02;
            }
        }
        else if self.rect.overlaps(&mouse_rect) && (self.rect_color.r <= self.color_copy.r - 0.06) {

        }
        else {
            self.rect_color = self.color_copy.clone();
        }
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.rect_color);
        let text_size = measure_text(&self.text, None, self.font_size as u16, 1.0);
        draw_text(&self.text,self.rect.x + (self.rect.w / 2.0) - (text_size.width / 2.0), self.rect.y + (self.rect.h / 2.0) + text_size.height, self.font_size, self.text_color);
    }
    pub fn clicked(&self) -> bool {
        let mouse_pos = mouse_position();
        let mouse_rect = Rect {
            x: mouse_pos.0,
            y: mouse_pos.1,
            w: 5.0,
            h: 5.0,
        };
        if self.rect.overlaps(&mouse_rect) && is_mouse_button_pressed(Left) {
            true
        }
        else {
            false
        }
    }
}