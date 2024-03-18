use std::u8;

use tui::{
    style::{Color, Stylize},
    widgets::Block,
    Frame,
};

use crate::Luma;

pub fn draw(f: &mut Frame<'_>, luma: &Luma) {
    let area = f.size();
    let mut r = 0u8;
    let mut g = 0u8;

    let w = area.width;
    let step = (w / u8::MAX as u16) as u8 + 1;

    for col in area.columns() {
        let a = Block::new().bg(Color::Rgb(r, 0, 0));
        f.render_widget(a, col);
        r = r.overflowing_add(step).0;
        g = g.overflowing_sub(step).0;
    }

    // let render = format!("{:?}", luma);
    // f.render_widget(Paragraph::new(render), area)
}
