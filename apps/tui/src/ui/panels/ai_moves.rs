use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Table, Row, Cell},
    Frame,
};
use crate::app::App;
use core_engine::luts::EngineLUTs;
use core_engine::rules::moves::Move;
use core_engine::ai::EvaluationScore;

struct RatedMove {
    action: Move,
    score: i32,
}

pub fn render(f: &mut Frame<'_>, area: Rect, app: &App) {
    // 1. Gather all immediate legal actions using the global reference LUTs
    let legal_moves = app.game_state.generate_legal_moves(EngineLUTs::get_engine_luts());
    let mut rated_moves = Vec::with_capacity(legal_moves.len());

    // 2. Perform a 1-ply lookahead simulation pass for telemetry reporting
    for current_move in legal_moves {
        let mut virtual_state = app.game_state.clone();
        virtual_state.make_move(current_move);

        // Score the result from the opponent's new turn perspective
        let raw_score = app.evaluator.evaluate(&virtual_state);
        
        // Invert the score back to represent the value to the active player
        let score = match raw_score {
            EvaluationScore::Value(v) => -v,
            EvaluationScore::Mating(_) => i32::MAX,
            EvaluationScore::Mated(_) => i32::MIN,
        };

        rated_moves.push(RatedMove { action: current_move, score });
    }

    // 3. Sort moves in descending order so the absolute best action sits at index 0
    rated_moves.sort_by(|a, b| b.score.cmp(&a.score));

    // 4. Construct the Analysis Grid Table UI
    let header_cells = vec![
        Cell::from("Rank").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        Cell::from("Origin").style(Style::default().fg(Color::Gray)),
        Cell::from("Target").style(Style::default().fg(Color::Gray)),
        Cell::from("1-Ply Score").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
    ];
    let header = Row::new(header_cells).height(1);

    let mut rows = Vec::new();
    
    // Display up to the top 12 moves to prevent crowding the terminal panel view
    for (i, rated) in rated_moves.iter().take(12).enumerate() {
        let rank_num = (i + 1).to_string();
        
        // Convert internal square indices into legible chessboard-style labels (e.g., A1, E4)
        let from_sq = rated.action.from_square();
        let to_sq = rated.action.to_square();
        
        let from_coord = format!("{}{}", (b'A' + (from_sq % 8)) as char, (from_sq / 8) + 1);
        let to_coord = format!("{}{}", (b'A' + (to_sq % 8)) as char, (to_sq / 8) + 1);

        // Apply a bold green highlight to row 1 to emphasize the optimal move recommendation
        let row_style = if i == 0 {
            Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let row_cells = vec![
            Cell::from(format!("#{}", rank_num)),
            Cell::from(from_coord),
            Cell::from(to_coord),
            Cell::from(format!("{:+}", rated.score)),
        ];
        
        rows.push(Row::new(row_cells).style(row_style));
    }

    // Handle the fallback state where no legal moves remain
    if rows.is_empty() {
        rows.push(Row::new(vec![
            Cell::from("N/A"),
            Cell::from("--"),
            Cell::from("--"),
            Cell::from("Mated / No Moves"),
        ]).style(Style::default().fg(Color::Red)));
    }

    let column_widths = [
        ratatui::layout::Constraint::Length(6),  // Rank index (#1, #2)
        ratatui::layout::Constraint::Length(10), // Source coordinate string
        ratatui::layout::Constraint::Length(10), // Target coordinate string
        ratatui::layout::Constraint::Length(14), // Position evaluation score
    ];

    let table = Table::new(rows, column_widths)
        .header(header)
        .block(Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT))
        .column_spacing(2);

    f.render_widget(table, area);
}