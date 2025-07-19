use std::any::{Any};
use raylib::prelude::RaylibDraw;
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;

#[derive(Default, Copy, Clone)]
pub struct Line {
    pub(crate) start: Vector2,
    pub(crate) end: Vector2,
}

pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone, Copy)]
pub struct Transform2D {
    position: Vector2,
    size: Vector2,
}
impl Component for Transform2D {
    fn as_any(&self) -> &dyn Any { self}
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

// Functions
pub fn draw_line(draw_handle: &mut RaylibDrawHandle, line: &Line, color: Color) {
    draw_handle.draw_line(line.start.x as i32, line.start.y as i32, line.end.x as i32, line.end.y as i32, color);
}
