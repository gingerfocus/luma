mod app;
mod event;
mod prelude;
mod state;
// mod vault;
mod ui;

// mod serde {
//     pub extern crate serde as core;
pub extern crate serde_yaml as yaml;
//     pub extern crate toml;
// }

use crate::prelude::*;

fn main() -> anyhow::Result<()> {
    let file = std::env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("no file given"))?;

    let content = fs::read_to_string(&file)?;

    let mut state = yaml::from_str::<State>(&content).unwrap();

    let mut app = App::new();

    // --------------------------------------------
    app.init()?;
    app.draw(&state);

    while app.can_run() {
        let event = app.read_event();
        app.handle(event, &mut state);

        app.draw(&state);
    }

    app.deinit()?;
    // --------------------------------------------

    // let file = l.readline_with_initial("Save: ", (&file, ""))?;
    // let out = markdown.to_string();
    // fs::write(file, out)?;

    Ok(())
}
