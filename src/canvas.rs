use sdl2::{
    self,
    Sdl, VideoSubsystem,
    video::Window, render::Canvas, rect::{Point, Rect},
};
use crate::{
    color::Color,
    vector::Vec2
};

pub struct Canvas2D {
    pub width: u32,
    pub height: u32,
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub canvas: Canvas<Window>,
}
impl Canvas2D {
    pub fn new(width: u32, height: u32, title: Option<&str>) -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video.window(title.unwrap_or("Window"), width, height)
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .build()
            .unwrap();
        Self { width, height, sdl, video, canvas }
    }
    pub fn set_color<C: Into<Color>>(&mut self, color: C) {
        self.canvas.set_draw_color(color.into());
    }
    pub fn get_color(&self) -> Color {
        self.canvas.draw_color().into()
    }
    pub fn background<C: Into<Color>>(&mut self, color: C) {
        self.canvas.set_draw_color(color.into());
        self.canvas.clear();
    }
    pub fn point(&mut self, pos: Vec2<i32>) {
        self.canvas.draw_point(pos);
    }
    pub fn line(&mut self, start: Vec2<i32>, end: Vec2<i32>) {
        self.canvas.draw_line(start, end);
    }
    pub fn rect(&mut self, rect: Rect) {
        self.canvas.draw_rect(rect);
    }
    pub fn rect_filled(&mut self, rect: Rect) {
        self.canvas.fill_rect(rect);
    }
}