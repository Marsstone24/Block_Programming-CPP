use std::fmt::Display;

use crate::{Action, UUID, utils::NavBar};

pub trait Window {
    fn uuid(&self) -> UUID;
    fn update(&mut self) -> Vec<Action>;
    fn render(&self);
}

#[derive(Clone)]
pub struct Menu<N> {
    uuid: UUID,
    is_shown: bool,
    nav_bar: NavBar<N>
}

impl<N: Display> Window for Menu<N> {
    fn uuid(&self) -> UUID {
        self.uuid
    }

    fn update(&mut self) -> Vec<Action> {
        self.nav_bar.update()
    }

    fn render(&self) {
        self.nav_bar.render();
    }
}

impl<N> Menu<N> {
    pub fn new(nav_bar: NavBar<N>) -> Self {
        Menu {
            uuid: UUID::new(),
            is_shown: false,
            nav_bar
        }
    }

    pub fn is_shown(&mut self, val: bool) {
        self.is_shown = val;
    }
}