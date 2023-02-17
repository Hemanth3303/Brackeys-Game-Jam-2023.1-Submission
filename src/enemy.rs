use macroquad::prelude::*;

pub struct Enemy {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Enemy {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        return Self { pos, size };
    }
    
    pub fn draw(self: &Self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, RED);
    }
}