use crate::event::Event;
use crate::canvas::Canvas2D;

pub trait BeauxApp {
    fn init() -> Self;
    fn closed(&self) -> bool;
    fn close(&mut self);
    fn update(&mut self) {}
    fn draw(&mut self, canvas: &mut Canvas2D) {}
    fn event(&mut self, event: Event, canvas: &mut Canvas2D) {
        match event {
            Event::Quit { .. } => { self.close(); }
            _ => {}
        }
    }
    fn run(&mut self, canvas: &mut Canvas2D) {
        let mut event_pump = canvas.sdl.event_pump().unwrap();
        while !self.closed() {
            for event in event_pump.poll_iter() {
                self.event(event, canvas);
            }
            self.update();
            self.draw(canvas);
            canvas.canvas.present();
        }
    }
}