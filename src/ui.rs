use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen};

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

    let header_text = Paragraph::new("terminal barista ✨☕️")
        .block(Block::new().padding(Padding::new(0, 0, vertical_layout[2].height / 2, 0)))
        .alignment(Alignment::Center);
    frame.render_widget(header_text, vertical_layout[0]);

    let parsed_tooltip = match &app.tooltip {
        Some(str) => str,
        None => "",
    };
    let footer_text = Paragraph::new(parsed_tooltip)
        .block(Block::new().padding(Padding::new(0, 0, vertical_layout[2].height / 2, 0)))
        .alignment(Alignment::Center);
    frame.render_widget(footer_text, vertical_layout[2]);

    let mut cup_top_margin_required = 0;
    while vertical_layout[0].height - cup_top_margin_required != 0 {
        cup_top_margin_required += 1;
    }
    let middle_section_divisible_by_three = Rect {
        x: vertical_layout[1].x,
        y: vertical_layout[1].y + cup_top_margin_required,
        width: vertical_layout[1].width,
        height: vertical_layout[1].height - cup_top_margin_required,
    };

    let mut cup_items: Vec<Constraint> = vec![];
    for _ in 0..app.cups.len() {
        cup_items.push(Constraint::Length(10));
    }

    let cups_layout = Layout::default()
        .direction(Direction::Horizontal)
        .flex(Flex::Center)
        .margin(2)
        .constraints(cup_items)
        .split(middle_section_divisible_by_three);

    let mut liquid_items = vec![];
    for i in 0..app.cups.len() {
        let mut liquids = vec![];
        for _ in 0..app.cups[i].capacity {
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
                Block::new()
                    .title_bottom(format!("[{}]", i + 1))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                    .border_type(BorderType::Rounded)
                    .title_style(Style::new().bg(Color::White).fg(Color::Black)),
                cups_layout[i],
            );
        } else {
            frame.render_widget(
                Block::new()
                    .title_bottom(format!("[{}]", i + 1))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                    .border_type(BorderType::Rounded),
                cups_layout[i],
            );
        }
        for k in 0..cup.liquids.len() {
            let liquid = &cup.liquids[k];
            let offset = cup.capacity - cup.liquids.len();
            let max_index = cup.liquids.len() - 1;
            // The liquids are displayed top to bottom, but
            // we want to display bottom to top. So we need
            // to render starting from the bottom
            frame.render_widget(
                Block::new()
                    .bg(liquid.to_color())
                    .fg(Color::Black)
                    .padding(Padding::uniform(3)),
                liquid_items[i][max_index - k + offset],
            );
        }
    }

    if app.current_screen == CurrentScreen::Finished {
        let parent_layout = middle_section_divisible_by_three;
        let victory_text_width = parsed_tooltip.chars().count();
        let victory_box_width = std::cmp::max((victory_text_width as u16) + 6, 60);
        let victory_box_height = 7;
        let victory_box = Rect {
            x: parent_layout.x + (parent_layout.width / 2) - (victory_box_width / 2),
            y: parent_layout.y + (parent_layout.height / 2) - (victory_box_height / 2),
            width: victory_box_width,
            height: victory_box_height,
        };
        let victory_text = Paragraph::new("You win 🥳 \n \n New Game [Enter]")
            .alignment(Alignment::Center)
            .block(Block::new().padding(Padding::new(0, 0, victory_box.height / 2 - 1, 0)))
            .bg(Color::White)
            .fg(Color::Black);
        frame.render_widget(victory_text, victory_box);
    }
}
