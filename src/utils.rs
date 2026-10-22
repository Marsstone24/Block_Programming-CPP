use std::fmt::{Debug, Display};

use macroquad::{color::Color, input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::{Rect, Vec2, vec2}, shapes::draw_rectangle, text::{Font, TextParams, draw_text_ex, measure_text}};

use crate::ColorExt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    ToggleFileMenu,
    NewFile,
    OpenFile,
    NewWindow,
    OpenFolder,
    CloseFolder,
    Close
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug, Clone)]
pub struct NavBar<T> {
    pos: Vec2,
    direction: Direction,
    elements: Vec<Element<T>>,
    elem_w: f32,
    elem_h: f32
}

impl<T> NavBar<T> {
    pub fn new(x: f32, y: f32, direction: Direction, elem_w: f32, elem_h: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            direction,
            elements: Vec::new(),
            elem_w,
            elem_h
        }
    }
    
    pub fn add(&mut self, elem: Element<T>) {
        self.elements.push(elem);
    }

    pub fn get(&self, i: usize) -> Option<&Element<T>> {
        self.elements.get(i)
    }

    pub fn remove(&mut self, i: usize) {
        self.elements.remove(i);
    }

    fn calc_next_elem_pos(&self, prev_elem_pos: Vec2, w: f32, h: f32) -> Vec2 {
        let mut next_elem_pos = prev_elem_pos;

        match self.direction {
            Direction::Left => {
                next_elem_pos.x -= w;
            }
            Direction::Right => {
                next_elem_pos.x += w;
            }
            Direction::Up => {
                next_elem_pos.y -= h;
            }
            Direction::Down => {
                next_elem_pos.y += h;
            }
        };

        next_elem_pos
    }

    pub fn update(&mut self) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut prev_elem = self.pos;

        for i in 0..self.elements.len() {
            let (w, h) = (self.elem_w, self.elem_h);
            if let Some(a) = self.elements[i].update(prev_elem, w, h) {
                actions.push(a);
            }

            prev_elem = self.calc_next_elem_pos(prev_elem, w, h);
        }
        actions
    }
}

impl<T: Display> NavBar<T> {
    pub fn render(&self) {
        for i in 0..self.elements.len() {
            self.elements[i].render(self.elem_w, self.elem_h);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Element<T> {
    pub value: T,
    pub action: Action,
    pub pos: Vec2,
    pub base_color: Color,
    pub display_color: Color,
    pub font_color: Color,
    pub font_style: Option<Font>,
    pub hovered: bool
}

impl<T> Element<T> {
    pub fn new(value: T, action: Action, base_color: Color, font_color: Color, font_style: Option<Font>) -> Self {
        Self {
            value,
            action,
            base_color,
            display_color: base_color,
            pos: Vec2::new(0.0, 0.0),
            font_color,
            font_style,
            hovered: false
        }
    }

    fn contains(&mut self, point: Vec2, w: f32, h: f32) {
        self.hovered = Rect::new(self.pos.x, self.pos.y, w, h).contains(point);
    }

    fn fit_font_size(&self, text: &str, max_w: f32, max_h: f32) -> u16 {
        let ref_size = 100u16;
        let dims = measure_text(text, self.font_style.as_ref(), ref_size, 1.0);

        let scale_w = max_w / dims.width;
        let scale_h = max_h / ref_size as f32;

        (ref_size as f32 * scale_w.min(scale_h)).floor().max(1.0) as u16
    }

    pub fn update(&mut self, elem_pos: Vec2, w: f32, h: f32) -> Option<Action> {
        let m_pos = mouse_position();
        self.pos = elem_pos;

        self.contains(vec2(m_pos.0, m_pos.1), w, h);

        if self.hovered && is_mouse_button_pressed(MouseButton::Left) {
            self.display_color = self.base_color.brightness(-35.0);
            return Some(self.action);
        } else if self.hovered {
            self.display_color = self.base_color.brightness(-15.0);
        } else {
            self.display_color = self.base_color;
        }
        None
    }
}

impl<T: Display> Element<T> {
    pub fn render(&self, w: f32, h: f32) {
        draw_rectangle(self.pos.x, self.pos.y, w, h, self.display_color);

        let text = self.value.to_string();
        let size = self.fit_font_size(&text, w, h);
        let dims = measure_text(&text, self.font_style.as_ref(), size, 1.0);

        draw_text_ex(
            &text,
            self.pos.x + (w - dims.width) / 2.0,
            self.pos.y + (h - dims.height) / 2.0 + dims.offset_y,
            TextParams { font: self.font_style.as_ref(), font_size: size, font_scale: 1.0, font_scale_aspect: 1.0, color: self.font_color, rotation: 0.0 }
        );
    }
}