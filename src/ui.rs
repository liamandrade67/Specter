use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;
use crate::modules;
use crate::screens::Screen;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = Paragraph::new("Specter v0.1").block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    match app.screen {
        Screen::Dashboard => {
            draw_list(frame, chunks[1], "Categories", &modules::categories(), app.selected_category);
        }
        Screen::Category => {
            let tools = modules::tools_for_category(app.selected_category);
            let name = modules::categories()[app.selected_category];
            draw_list(frame, chunks[1], name, &tools, app.selected_tool);
        }
        Screen::Tool => {
            let tools = modules::tools_for_category(app.selected_category);
            let tool_name = tools[app.selected_tool];
            let block = Block::default().borders(Borders::ALL).title(tool_name);
            let paragraph = Paragraph::new(format!("{tool_name} — not implemented yet.")).block(block);
            frame.render_widget(paragraph, chunks[1]);
        }
    }

    let help = match app.screen {
        Screen::Dashboard => "↑↓ Navigate  Enter Select  q Quit",
        Screen::Category => "↑↓ Navigate  Enter Select  Esc Back  q Quit",
        Screen::Tool => "Esc Back  q Quit",
    };
    let footer = Paragraph::new(help).block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}

fn draw_list(frame: &mut Frame, area: Rect, title: &str, items: &[&str], selected: usize) {
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let line = if i == selected {
                Line::from(Span::styled(
                    format!("> {item}"),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::raw(format!("  {item}")))
            };
            ListItem::new(line)
        })
        .collect();

    let list = List::new(list_items).block(Block::default().borders(Borders::ALL).title(title.to_string()));
    frame.render_widget(list, area);
}
