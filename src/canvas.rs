use sdl2::{
    self,
    Sdl, VideoSubsystem,
    video::Window, render::Canvas, rect::{Point, Rect},
};
use crate::{
    pixels::Color,
    vector::Vec2
};

/// A wrapper to control the window and canvas, as well as the SDL context
pub struct Canvas2D {
    pub width: u32,
    pub height: u32,
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub canvas: Canvas<Window>,
}
impl Canvas2D {
    /// Create a new window with a canvas in it
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

    /// set the color for any drawing operation
    pub fn set_color<C: Into<Color>>(&mut self, color: C) {
        self.canvas.set_draw_color(color.into());
    }

    /// get the color for the drawing operations
    pub fn get_color(&self) -> Color {
        self.canvas.draw_color().into()
    }

    /// draw the background with a color
    pub fn background<C: Into<Color>>(&mut self, color: C) {
        self.canvas.set_draw_color(color.into());
        self.canvas.clear();
    }

    /// draw a singel pixel at a position
    pub fn point(&mut self, pos: Vec2<i32>) {
        self.canvas.draw_point(pos);
    }

    /// draw a line from one position to another
    pub fn line(&mut self, start: Vec2<i32>, end: Vec2<i32>) {
        self.canvas.draw_line(start, end);
    }

    /// draw a rectangle (not filled)
    pub fn rect(&mut self, rect: Rect) {
        self.canvas.draw_rect(rect);
    }

    /// draw a filled rectangle
    pub fn rect_filled(&mut self, rect: Rect) {
        self.canvas.fill_rect(rect);
    }
}