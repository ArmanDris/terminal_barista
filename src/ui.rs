use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(frame.area());

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(root[0]);

    let header = Block::new().bg(Color::LightMagenta);
    frame.render_widget(header, vertical_layout[0]);
    let mut selected_str = match app.src_selection {
        Some(val) => (val + 1).to_string(),
        None => "No selection".to_string(),
    };
    if let Some(ref msg) = app.tooltip {
        selected_str.push('\t');
        selected_str.push_str(msg);
    }
    let footer = Block::new().bg(Color::LightCyan);
    let footer_text = Paragraph::new(selected_str).fg(Color::Black).block(footer);
    frame.render_widget(footer_text, vertical_layout[2]);

    let mut cup_items: Vec<Constraint> = vec![];
    for _ in 0..app.cups.len() {
        cup_items.push(Constraint::Length(10));
    }

    let cups_layout = Layout::default()
        .direction(Direction::Horizontal)
        .flex(Flex::Center)
        .margin(2)
        .constraints(cup_items)
        .split(vertical_layout[1]);

    let mut liquid_items = vec![];
    for i in 0..app.cups.len() {
        let mut liquids = vec![];
        for _ in 0..app.cups[i].liquids.len() {
            liquids.push(Constraint::Length(5));
        }

        let items = Layout::default()
            .direction(Direction::Vertical)
            .constraints(liquids)
            .margin(1)
            .split(cups_layout[i]);
        liquid_items.push(items);
    }

    for (i, cup) in app.cups.iter().enumerate() {
        let mut selected_i = false;
        if let Some(src_selection) = app.src_selection {
            selected_i = (i as u32) == src_selection;
        }
        if selected_i {
            frame.render_widget(
                Block::bordered()
                    .title_bottom(format!("Cup {}", i + 1))
                    .title_alignment(Alignment::Center)
                    .title_style(Style::new().bg(Color::White).fg(Color::Black))
                    .border_set(symbols::border::DOUBLE),
                cups_layout[i],
            );
        } else {
            frame.render_widget(
                Block::bordered()
                    .title_bottom(format!("Cup {}", i + 1))
                    .title_alignment(Alignment::Center),
                cups_layout[i],
            );
        }
        for k in 0..cup.liquids.len() {
            let liquid = &cup.liquids[k];
            frame.render_widget(
                Block::bordered().bg(liquid.to_color()).fg(Color::Black),
                liquid_items[i][k],
            );
        }
    }
}
