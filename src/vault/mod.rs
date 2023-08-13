mod app;
mod enums;
mod output;
mod state;
mod traits;
mod utils;

pub use self::{
    app::App,
    output::{Message, Output},
    state::args::Args,
};

fn run(args: Args) {
    let mut app = App::new();
    app.handle(args);
}
