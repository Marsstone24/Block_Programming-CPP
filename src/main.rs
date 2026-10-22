use block_programming::{ColorExt, Direction, Element, NavBar, Program, Settings, window_conf, windows::Menu};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let settings = Settings::load();
    let mut program = Program::new();
    
    let mut top_navbar: NavBar<String> = NavBar::new(0.0, 0.0, Direction::Right, 40., 20.);
    top_navbar.add(Element::new("File".to_string(),  settings.background_color.brightness(20.0), BLACK, None));

    program.add_window(Box::new(Menu::new(top_navbar)));

    loop {
        clear_background(settings.background_color);


        
        program.update();
        program.render();

        next_frame().await;
    }
}

fn file() {

}