use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::{App, GameMode};

pub fn render(f: &mut Frame<'_>, area: Rect, app: &App) {
    let mut text = Vec::new();

    // 1. General Info & Instructions Header
    text.push(Line::from(Span::styled("--- Global Navigation Commands ---", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));
    text.push(Line::from("  [Arrow Keys]  - Navigate Selector Grid"));
    text.push(Line::from("  [Space / Ent] - Select Piece / Confirm Movement Command"));
    text.push(Line::from("  [ESC]         - Flush Active Selection Coordinates"));
    text.push(Line::from("  [Tab]         - Toggle Operational Mode (Strict <-> Freeform)"));
    text.push(Line::from("  [[ / ]]       - Cycle Left/Right Through Side Menu Tabs"));
    text.push(Line::from("  [R]           - Reset Environment to Starting Layout"));
    text.push(Line::from("  [Q]           - Force Graceful Terminal Termination"));
    text.push(Line::from(""));

    // 2. AI Controller Allocation Instructions
    text.push(Line::from(Span::styled("--- AI Engine Assignments ---", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));
    text.push(Line::from("  [4] - Toggle Player 1 Controller (Human <-> AI)"));
    text.push(Line::from("  [5] - Toggle Player 2 Controller (Human <-> AI)"));
    text.push(Line::from(""));

    // 3. Dynamic Contextual Sandbox Mapping Info
    text.push(Line::from(Span::styled("--- Freeform Sandbox Paint Tools ---", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));
    
    if app.mode == GameMode::Freeform {
        text.push(Line::from(vec![
            Span::raw("  Status: "),
            Span::styled("ACTIVE", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]));
        text.push(Line::from("  [1] - Force Spawn Player 1 Piece at Cursor Index"));
        text.push(Line::from("  [2] - Force Spawn Player 2 Piece at Cursor Index"));
        text.push(Line::from("  [3] - Clear Contents / Delete Piece at Cursor Index"));
        text.push(Line::from("  [T] - Manually Force Swap Active Turn Ownership"));
    } else {
        text.push(Line::from(vec![
            Span::raw("  Status: "),
            Span::styled("DISABLED", Style::default().fg(Color::DarkGray)),
            Span::raw(" (Switch via [Tab] to use paint tools)"),
        ]));
    }

    // 4. Wrap everything in a clean block matching the sub-layout
    let panel_block = Paragraph::new(text)
        .block(Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT))
        .wrap(Wrap { trim: true });

    f.render_widget(panel_block, area);
}