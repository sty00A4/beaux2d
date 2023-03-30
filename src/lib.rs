#![allow(unused)]
pub extern crate sdl2;
pub mod vector;
pub mod color;
pub mod canvas;
pub mod app;
pub use sdl2::{event, keyboard, sys::{rand, random}, timer};

#[cfg(test)]
mod tests {
    use super::*;
}
