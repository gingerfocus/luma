mod app;
mod event;
mod luma;
mod mode;
mod prelude;
mod state;
mod ui;

// mod vault;

pub extern crate serde_yaml as yaml;

use ui::Screen;

use crate::prelude::*;

pub enum LumaMessage {
    Redraw,
    SetMode(Mode),
    Exit,
    Nothing,
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("no file given"))?;

    // let mut state = State::default();
    let mut mode = Mode::default();
    let mut luma: Luma = yaml::from_str(&fs::read_to_string(&file)?)?;

    let mut screen = Screen::default();
    // HACK: make the screen valid during the transition period
    screen.init()?;

    let mut app = App::new();

    // --------------------------------------------
    app.init()?;
    app.draw(&luma, &mode);

    while can_run(&app, &screen) {
        let event = read_event();
        match app.handle(event, &mut luma, &mode)? {
            LumaMessage::Redraw => app.draw(&luma, &mode),
            LumaMessage::SetMode(m) => {
                mode = m;
                app.draw(&luma, &mode);
            }
            LumaMessage::Exit => app.run = false,
            LumaMessage::Nothing => {}
        }
    }

    app.deinit()?;
    // --------------------------------------------

    println!("Save file? (y/N)");
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    if response.trim().starts_with('y') {
        println!("Save as? (Default: {})", file);
        let mut save_file = String::new();
        io::stdin().read_line(&mut save_file)?;
        // remove new line from reading stding
        save_file = save_file.trim().to_string();
        if save_file.is_empty() {
            save_file = file;
        }
        fs::write(save_file, yaml::to_string(&luma)?.as_bytes())?;
    }

    Ok(())
}

fn can_run(app: &App, screen: &Screen) -> bool {
    app.run && screen.is_valid
}

fn read_event() -> Event {
    if crossterm::event::poll(std::time::Duration::from_millis(500)).unwrap() {
        crossterm::event::read().map(Into::into).unwrap()
    } else {
        Event::Tick
    }
}
