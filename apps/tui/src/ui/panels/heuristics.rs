use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Table, Row, Cell},
    Frame,
};
use crate::app::App;
use core_engine::heuristics::{TileType, SovereigntyState, RegionType, ParityType};

pub fn render(f: &mut Frame<'_>, area: Rect, app: &App) {
    let matrix = app.get_current_heuristics();

    // Mapping our 6 structural table coordinate configurations
    let target_spaces = [
        (RegionType::Corner2x2, ParityType::Even),
        (RegionType::Corner2x2, ParityType::Odd),
        (RegionType::Edge4x2,   ParityType::Even),
        (RegionType::Edge4x2,   ParityType::Odd),
        (RegionType::Center4x4, ParityType::Even),
        (RegionType::Center4x4, ParityType::Odd),
    ];

    // Build the grid headers including our new Aggregated Totals column
    let header_cells = vec![
        Cell::from("Tile:Sovereign").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        Cell::from("C:Evn"), Cell::from("C:Odd"),
        Cell::from("E:Evn"), Cell::from("E:Odd"),
        Cell::from("X:Evn"), Cell::from("X:Odd"),
        Cell::from("∑ SUM").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
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

    // Step through our 18 discrete tile and status rows
    for &(t_type, t_label) in &tile_types {
        for &(s_type, s_label, state_color) in &sovereignty_states {
            let row_title = format!("{}:{}", t_label, s_label);
            let mut cells = vec![Cell::from(row_title).style(Style::default().fg(state_color).add_modifier(Modifier::BOLD))];

            let mut row_sum: i32 = 0;

            // Compute values and aggregate total sum across all spatial dimensions
            for &(r_type, p_type) in &target_spaces {
                let val = matrix.values[t_type as usize][s_type as usize][r_type as usize][p_type as usize] as i32;
                row_sum += val;
                
                let cell_style = if val > 0 {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::Rgb(60, 60, 60))
                };
                
                cells.push(Cell::from(val.to_string()).style(cell_style));
            }

            // Append the compiled row total calculation cell onto the row tail
            let sum_style = if row_sum > 0 {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(70, 70, 70))
            };
            cells.push(Cell::from(row_sum.to_string()).style(sum_style));

            rows.push(Row::new(cells));
        }
    }

    // Set precise layout sizes to support the new totals dimension within bounds
    let column_widths = [
        ratatui::layout::Constraint::Length(13), // Title
        ratatui::layout::Constraint::Length(5),  // C:Evn
        ratatui::layout::Constraint::Length(5),  // C:Odd
        ratatui::layout::Constraint::Length(5),  // E:Evn
        ratatui::layout::Constraint::Length(5),  // E:Odd
        ratatui::layout::Constraint::Length(5),  // X:Evn
        ratatui::layout::Constraint::Length(5),  // X:Odd
        ratatui::layout::Constraint::Length(6),  // Cumulative Row Sum Column
    ];

    let matrix_table = Table::new(rows, column_widths)
        .header(header)
        .block(Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT))
        .column_spacing(1);

    f.render_widget(matrix_table, area);
}