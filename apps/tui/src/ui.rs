use crate::app::{App, GameMode, SelectionState};
use core_engine::rules::state::Player;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame<'_>, app: &App) {
    // 1. Divide the screen vertically for the main workspace vs bottom status bar
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area()); // Note: Frame::size was also modernized to Frame::area()

    let workspace_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_layout[0]);

    let board_area = workspace_layout[0];
    let panel_area = workspace_layout[1];
    let status_area = main_layout[1];

    // ========================================================================
    // PASS 1: RENDER THE 8x8 INTERACTIVE BOARD GRID
    // ========================================================================
    render_board(f, board_area, app);

    // ========================================================================
    // PASS 2: RENDER CONTROL DECK & STATUS PANEL
    // ========================================================================
    render_panel(f, panel_area, app);

    // ========================================================================
    // PASS 3: RENDER THE BOTTOM TELEMETRY LANE
    // ========================================================================
    let status_bar = Paragraph::new(app.message_log.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Telemetry Status "));
    f.render_widget(status_bar, status_area);
}

fn render_board(f: &mut Frame<'_>, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2); 8]) // 2 character high padding per rank row
        .split(area);

    for r in 0..8 {
        // Render ranks from 7 down to 0 to mirror top-down engine orientation
        let rank = 7 - r; 
        
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(5); 8]) // 5 character wide block columns
            .split(rows[r as usize]);

        for file in 0..8 {
            let current_idx = rank * 8 + file;
            
            let has_p1 = app.game_state.p1_pieces.has_bit(current_idx);
            let has_p2 = app.game_state.p2_pieces.has_bit(current_idx);

            let cell_text = if has_p1 { " 🅟🅜 " } else if has_p2 { " 🅟🅝 " } else { " . " };

            let mut bg_color = if (rank + file) % 2 == 0 { Color::Rgb(30, 30, 30) } else { Color::Rgb(45, 45, 45) };
            let mut fg_color = if has_p1 { Color::Cyan } else if has_p2 { Color::Magenta } else { Color::DarkGray };
            let mut modifier = Modifier::empty();

            if let SelectionState::PieceSelected { index, valid_moves } = app.selection {
                if index == current_idx {
                    bg_color = Color::Blue;
                    fg_color = Color::White;
                } else if valid_moves.has_bit(current_idx) {
                    bg_color = Color::Green;
                    fg_color = Color::Black;
                    modifier = Modifier::BOLD;
                }
            }

            if app.cursor_x == file && app.cursor_y == rank {
                bg_color = Color::Yellow;
                fg_color = Color::Black;
                modifier = Modifier::BOLD;
            }

            let cell_paragraph = Paragraph::new(cell_text)
                .alignment(Alignment::Center)
                .style(Style::default().fg(fg_color).bg(bg_color).add_modifier(modifier));

            f.render_widget(cell_paragraph, cols[file as usize]);
        }
    }
}

fn render_panel(f: &mut Frame<'_>, area: Rect, app: &App) {
    let mut text = Vec::new();

    let mode_span = match app.mode {
        GameMode::Strict => Span::styled("STRICT (GAME-MODE)", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        GameMode::Freeform => Span::styled("FREEFORM (SANDBOX)", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    };
    text.push(Line::from(vec![Span::raw("Current Operating Engine: "), mode_span]));
    text.push(Line::from(""));

    let turn_span = match app.game_state.active_player {
        Player::P1 => Span::styled("PLAYER 1 (Cyan)", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Player::P2 => Span::styled("PLAYER 2 (Magenta)", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
    };
    text.push(Line::from(vec![Span::raw("Active Player Control: "), turn_span]));
    text.push(Line::from(""));

    let index_string = format!("Index: {} | Grid Coordinate: {}{}", app.cursor_index(), (b'A' + app.cursor_x) as char, app.cursor_y + 1);
    text.push(Line::from(vec![Span::raw(index_string)]));
    text.push(Line::from("----------------------------------------"));
    text.push(Line::from("Global Keyboard Mappings:"));
    text.push(Line::from("  [Arrow Keys]  - Navigate Selector Grid"));
    text.push(Line::from("  [Space / Ent] - Select / Move Intended Piece"));
    text.push(Line::from("  [Tab]         - Toggle Operational Engine Mode"));
    text.push(Line::from("  [ESC]         - Deselect Selection Node / Clear Move Targets"));
    text.push(Line::from("  [Q]           - Force Terminate Execution Loop"));

    if app.mode == GameMode::Freeform {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled("Sandbox Direct Tooling:", Style::default().fg(Color::Yellow))));
        text.push(Line::from("  [1]           - Spawn Player 1 Piece here"));
        text.push(Line::from("  [2]           - Spawn Player 2 Piece here"));
        text.push(Line::from("  [3]           - Clear / Wipe Square content"));
        text.push(Line::from("  [T]           - Switch Active Player turn state"));
    }

    if app.game_state.is_lost(&app.luts) {
        text.push(Line::from(""));
        let victor = match app.game_state.active_player {
            Player::P1 => "PLAYER 2 (MAGENTA) WINS!",
            Player::P2 => "PLAYER 1 (CYAN) WINS!",
        };
        text.push(Line::from(Span::styled(
            format!(" GAME OVER: {} ", victor),
            Style::default().fg(Color::White).bg(Color::Red).add_modifier(Modifier::SLOW_BLINK | Modifier::BOLD)
        )));
    }

    let panel_paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Core Controller Interface "))
        .wrap(Wrap { trim: true });

    f.render_widget(panel_paragraph, area);
}