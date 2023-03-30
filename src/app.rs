use crate::event::Event;
use crate::canvas::Canvas2D;

pub trait BeauxApp {
    fn init() -> Self;
    fn closed(&self) -> bool;
    fn close(&mut self);
    fn update(&mut self) {}
    fn draw(&mut self, canvas: &mut Canvas2D) {}
    fn events(&mut self, canvas: &mut Canvas2D) {
        for event in canvas.sdl.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => { self.close(); }
                _ => {}
            }
        }
    }
    fn run(&mut self, canvas: &mut Canvas2D) {
        while !self.closed() {
            self.update();
            self.draw(canvas);
            canvas.canvas.present();
        }
    }
}