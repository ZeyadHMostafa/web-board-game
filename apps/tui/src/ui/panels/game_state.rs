use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::{App, ControllerAgent, GameMode};
use core_engine::rules::state::Player;
use core_engine::luts::EngineLUTs;

pub fn render(f: &mut Frame<'_>, area: Rect, app: &App) {
    let mut text = Vec::new();

    // 1. Operational Engine Mode Status Line
    let mode_span = match app.mode {
        GameMode::Strict => Span::styled("STRICT (RULES ENFORCED)", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        GameMode::Freeform => Span::styled("FREEFORM (SANDBOX)", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    };
    text.push(Line::from(vec![Span::raw("Engine Mode:     "), mode_span]));

    // 2. Turn Control Indicator
    let turn_span = match app.game_state.active_player {
        Player::P1 => Span::styled("PLAYER 1 (Cyan)", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Player::P2 => Span::styled("PLAYER 2 (Magenta)", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
    };
    text.push(Line::from(vec![Span::raw("Active Turn:     "), turn_span]));
    text.push(Line::from("----------------------------------------"));

    // 3. Controller Architecture Metadata Profiles
    let p1_agent_span = match app.p1_agent {
        ControllerAgent::Human => Span::styled("HUMAN", Style::default().fg(Color::LightBlue)),
        ControllerAgent::AI => Span::styled("AI ENGINE (1-PLY PICKER)", Style::default().fg(Color::LightYellow).add_modifier(Modifier::ITALIC)),
    };
    text.push(Line::from(vec![Span::raw("Player 1 Agent:  "), p1_agent_span]));

    let p2_agent_span = match app.p2_agent {
        ControllerAgent::Human => Span::styled("HUMAN", Style::default().fg(Color::LightBlue)),
        ControllerAgent::AI => Span::styled("AI ENGINE (1-PLY PICKER)", Style::default().fg(Color::LightYellow).add_modifier(Modifier::ITALIC)),
    };
    text.push(Line::from(vec![Span::raw("Player 2 Agent:  "), p2_agent_span]));
    text.push(Line::from("----------------------------------------"));

    // 4. Structural Population Counts (Hardware Bitboard Population Density)
    let p1_count = app.game_state.p1_pieces.count_ones();
    let p2_count = app.game_state.p2_pieces.count_ones();
    text.push(Line::from(vec![
        Span::raw("Player 1 Pieces: "), 
        Span::styled(p1_count.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    ]));
    text.push(Line::from(vec![
        Span::raw("Player 2 Pieces: "), 
        Span::styled(p2_count.to_string(), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
    ]));
    text.push(Line::from("----------------------------------------"));

    // 5. Raw Machine Evaluation Position Scoring Metric Output
    let raw_score = app.get_position_score();
    let score_color = if raw_score > 0 {
        Color::LightGreen
    } else if raw_score < 0 {
        Color::LightRed
    } else {
        Color::White
    };
    
    text.push(Line::from(vec![
        Span::raw("Active Evaluator Score: "),
        Span::styled(
            format!("{:+}", raw_score),
            Style::default().fg(score_color).add_modifier(Modifier::BOLD)
        ),
        Span::raw(" (Perspective: Turn Owner)")
    ]));
    text.push(Line::from(""));

    // 6. Match Termination Banner Display
    if app.game_state.is_lost() {
        let victor = match app.game_state.active_player {
            Player::P1 => "PLAYER 2 (MAGENTA) TRIUMPHANT",
            Player::P2 => "PLAYER 1 (CYAN) TRIUMPHANT",
        };
        text.push(Line::from(Span::styled(
            format!(" 🏁 TERMINAL NODE DETECTED: {} ", victor),
            Style::default().fg(Color::White).bg(Color::Red).add_modifier(Modifier::BOLD)
        )));
    }

    let panel_block = Paragraph::new(text)
        .block(Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT))
        .wrap(Wrap { trim: true });

    f.render_widget(panel_block, area);
}