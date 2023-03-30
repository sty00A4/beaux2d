#![allow(unused)]
mod vector;
mod color;
mod canvas;
mod app;
use sdl2::{event, keyboard, sys::{rand, random}};

#[cfg(test)]
mod tests {
    use super::*;
}
