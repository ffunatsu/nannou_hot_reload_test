#![allow(unused)]

use nannou::prelude::*;

pub struct Model {
    window: Entity,
    pub was_updated: bool,
    state: State,
}

impl Model {
    pub fn for_window(window: Entity) -> Self {
        Self {
            window,
            state: State::default(),
            was_updated: false,
        }
    }
}

#[derive(Default, Debug)]
pub struct State {}

#[unsafe(no_mangle)]
pub fn update(app: &App, model: &mut Model) {

}

#[unsafe(no_mangle)]
pub fn view(app: &App, model: &Model) {
    let draw = app.draw();
    draw.background().color(BLACK);
}
