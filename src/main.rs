#![feature(let_chains)]

mod app;
mod event;
mod node;
mod prelude;
mod traits;
// mod vault;

use crate::prelude::*;

fn main() -> anyhow::Result<()> {
    let file = std::env::args().skip(1).next().unwrap_or("new.md".into());

    let mut f = fs::File::open(&file)?;
    let mut content = String::new();
    let _read_size = f.read_to_string(&mut content)?;

    let markdown: Node = markdown::to_mdast(&content, &markdown::ParseOptions::default())
        .unwrap()
        .into();

    // create the line reader that is used to get input from the user
    // let mut l = rustyline::Editor::<(), rustyline::history::DefaultHistory>::with_config(
    //     rustyline::config::Builder::new()
    //         // .completion_type(rustyline::config::CompletionType::List)
    //         .edit_mode(rustyline::config::EditMode::Vi)
    //         .build(),
    // );

    let mut app = App::new(markdown);
    app.init()?;

    while app.can_run() {
        let event = app.read_event();
        app.handle(event);

        app.render();
    }

    app.deinit()?;

    // let file = l.readline_with_initial("Save: ", (&file, ""))?;
    // let out = markdown.to_string();
    // fs::write(file, out)?;

    Ok(())
}
