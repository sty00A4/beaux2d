use crate::event::Event;
use crate::canvas::Canvas2D;

pub trait BeauxApp {
    /// Initialize your app
    fn init() -> Self;

    /// This function checks if the program has been stopped, which is
    /// needed for the standard `event` function.
    /// Usually you'd have a variable in the
    /// struct like `closed`.
    /// ```rust
    /// struct App {
    ///     ...
    ///     closed: bool
    /// }
    /// impl App {
    ///     fn init() -> Self {...}
    ///     fn closed(&self) -> bool { self.closed }
    ///     fn close(&mut self) { self.closed = true }
    /// }
    /// ```
    fn closed(&self) -> bool;

    /// Closes the program in any way. Usually you'd have a variable in the
    /// struct like `closed`.
    /// ```rust
    /// struct App {
    ///     ...
    ///     closed: bool
    /// }
    /// impl App {
    ///     fn init() -> Self {...}
    ///     fn closed(&self) -> bool { self.closed }
    ///     fn close(&mut self) { self.closed = true }
    /// }
    /// ```
    fn close(&mut self);

    /// Step through the simulation (not required)
    fn update(&mut self) {}

    /// Draw function that is called every frame
    fn draw(&mut self, canvas: &mut Canvas2D) {}

    /// Event handling function called every frame.
    /// If not defined the function will be defined as following:
    /// ```rust
    /// fn event(&mut self, event: Event, canvas: &mut Canvas2D) {
    ///     match event {
    ///         Event::Quit { .. } => { self.close(); }
    ///         _ => {}
    ///     }
    /// }
    /// ```
    fn event(&mut self, event: Event, canvas: &mut Canvas2D) {
        match event {
            Event::Quit { .. } => { self.close(); }
            _ => {}
        }
    }
    
    /// Simple start of the program, can be overwritten but is not recommended
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