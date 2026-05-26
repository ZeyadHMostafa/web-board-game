use crate::app::{App, GameMode, SelectionState, RightPanelMode};
use core_engine::rules::state::Player;
use core_engine::heuristics::{TileType, SovereigntyState, RegionType, ParityType};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table, Cell, Wrap},
    Frame,
};

pub fn render(f: &mut Frame<'_>, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let workspace_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(main_layout[0]);

    let board_area = workspace_layout[0];
    let panel_area = workspace_layout[1];
    let status_area = main_layout[1];

    render_board(f, board_area, app);
    
    // Evaluate display dynamically according to active side menu panel configuration
    match app.panel_mode {
        RightPanelMode::ControlPanel => render_control_panel(f, panel_area, app),
        RightPanelMode::HeuristicMatrix => render_heuristic_matrix(f, panel_area, app),
    }

    let status_bar = Paragraph::new(app.message_log.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Telemetry Status "));
    f.render_widget(status_bar, status_area);
}

fn render_board(f: &mut Frame<'_>, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2); 8])
        .split(area);

    for r in 0..8 {
        let rank = 7 - r; 
        
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(5); 8])
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

fn render_control_panel(f: &mut Frame<'_>, area: Rect, app: &App) {
    let mut text = Vec::new();

    let mode_span = match app.mode {
        GameMode::Strict => Span::styled("STRICT (GAME-MODE)", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        GameMode::Freeform => Span::styled("FREEFORM (SANDBOX)", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    };
    text.push(Line::from(vec![Span::raw("Current Operating Engine: "), mode_span]));

    let turn_span = match app.game_state.active_player {
        Player::P1 => Span::styled("PLAYER 1 (Cyan)", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Player::P2 => Span::styled("PLAYER 2 (Magenta)", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
    };
    text.push(Line::from(vec![Span::raw("Active Player Control: "), turn_span]));

    let index_string = format!("Index: {} | Grid Coordinate: {}{}", app.cursor_index(), (b'A' + app.cursor_x) as char, app.cursor_y + 1);
    text.push(Line::from(vec![Span::raw(index_string)]));
    text.push(Line::from("----------------------------------------"));
    text.push(Line::from("Global Keyboard Mappings:"));
    text.push(Line::from("  [Arrow Keys]  - Navigate Selector Grid"));
    text.push(Line::from("  [Space / Ent] - Select / Move Intended Piece"));
    text.push(Line::from("  [Tab]         - Toggle Operational Engine Mode"));
    text.push(Line::from("  [M]           - Swap Side Menu Panel Viewports"));
    text.push(Line::from("  [ESC]         - Deselect Node  |  [Q] - Quit"));

    if app.mode == GameMode::Freeform {
        text.push(Line::from(Span::styled("Sandbox Direct Tooling: [1] P1  [2] P2  [3] Clear  [T] Switch Turn", Style::default().fg(Color::Yellow))));
    }

    if app.game_state.is_lost(&app.luts) {
        let victor = match app.game_state.active_player {
            Player::P1 => "PLAYER 2 (MAGENTA) WINS!",
            Player::P2 => "PLAYER 1 (CYAN) WINS!",
        };
        text.push(Line::from(Span::styled(
            format!(" GAME OVER: {} ", victor),
            Style::default().fg(Color::White).bg(Color::Red).add_modifier(Modifier::BOLD)
        )));
    }

    let panel_paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Core Controller Interface "))
        .wrap(Wrap { trim: true });

    f.render_widget(panel_paragraph, area);
}

fn render_heuristic_matrix(f: &mut Frame<'_>, area: Rect, app: &App) {
    let matrix = app.get_current_heuristics();

    let target_spaces = [
        (RegionType::Corner2x2, ParityType::Even),
        (RegionType::Corner2x2, ParityType::Odd),
        (RegionType::Edge4x2,   ParityType::Even),
        (RegionType::Edge4x2,   ParityType::Odd),
        (RegionType::Center4x4, ParityType::Even),
        (RegionType::Center4x4, ParityType::Odd),
    ];

    let header_cells = vec![
        Cell::from("Tile / Sovereignty").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        Cell::from("C:Evn").style(Style::default().fg(Color::Gray)),
        Cell::from("C:Odd").style(Style::default().fg(Color::Gray)),
        Cell::from("E:Evn").style(Style::default().fg(Color::Gray)),
        Cell::from("E:Odd").style(Style::default().fg(Color::Gray)),
        Cell::from("X:Evn").style(Style::default().fg(Color::Gray)),
        Cell::from("X:Odd").style(Style::default().fg(Color::Gray)),
    ];
    let header = Row::new(header_cells).height(1);

    let tile_types = [
        (TileType::Empty,        "Empt"),
        (TileType::AlliedPiece,  "Ally"),
        (TileType::EnemyPiece,   "Enmy"),
    ];

    let sovereignty_states = [
        (SovereigntyState::AllyDominates,    "DomAlly", Color::Cyan),
        (SovereigntyState::EnemyDominates,   "DomEnmy", Color::Magenta),
        (SovereigntyState::AllyUncontested,  "UncAlly", Color::Green),
        (SovereigntyState::EnemyUncontested, "UncEnmy", Color::Rgb(139, 0, 139)),
        (SovereigntyState::TiedConflict,     "TiedCfl", Color::Yellow),
        (SovereigntyState::NoConflict,       "NoConfl", Color::DarkGray),
    ];

    let mut rows = Vec::new();
    for &(t_type, t_label) in &tile_types {
        for &(s_type, s_label, state_color) in &sovereignty_states {
            let row_title = format!("{}:{}", t_label, s_label);
            let mut cells = vec![Cell::from(row_title).style(Style::default().fg(state_color).add_modifier(Modifier::BOLD))];

            for &(r_type, p_type) in &target_spaces {
                let val = matrix.values[t_type as usize][s_type as usize][r_type as usize][p_type as usize];
                
                let cell_style = if val > 0 {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::Rgb(60, 60, 60))
                };
                
                cells.push(Cell::from(val.to_string()).style(cell_style));
            }
            rows.push(Row::new(cells));
        }
    }

    let column_widths = [
        Constraint::Length(14),
        Constraint::Length(6),  
        Constraint::Length(6),  
        Constraint::Length(6),  
        Constraint::Length(6),  
        Constraint::Length(6),  
        Constraint::Length(6),  
    ];

    let matrix_table = Table::new(rows, column_widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Spatial Sovereignty Tensor (Press [M] to Return) "))
        .column_spacing(1);

    f.render_widget(matrix_table, area);
}