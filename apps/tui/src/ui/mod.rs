use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};
use crate::app::{App, ActivePanelTab};

// Declare our separate sub-component rendering modules
pub mod board;
pub mod panels;

pub fn render(f: &mut Frame<'_>, app: &App) {
    // 1. Split screen into Main Workspace and Bottom Status Bar
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // 2. Split Main Workspace into Left (Board) and Right (Data Panels)
    let workspace_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(main_layout[0]);

    let board_area = workspace_layout[0];
    let right_panel_area = workspace_layout[1];
    let status_area = main_layout[1];

    // 3. Render the core 8x8 board on the left
    board::render_board(f, board_area, app);
    
    // 4. Render the tabbed telemetry panel wrapper on the right
    render_tabbed_panel(f, right_panel_area, app);

    // 5. Render the bottom status bar
    let status_bar = Paragraph::new(app.message_log.as_str())
        .block(Block::default().borders(Borders::ALL).title(" System Telemetry Log "));
    f.render_widget(status_bar, status_area);
}

/// Draws the tab headers across the top of the right panel and delegates
/// body drawing to the sub-panels based on the currently chosen tab state.
fn render_tabbed_panel(f: &mut Frame<'_>, area: Rect, app: &App) {
    // Carve out a top row for the navigation tab titles
    let panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let tab_header_area = panel_layout[0];
    let tab_content_area = panel_layout[1];

    // Define the exact titles for our four sub-menus
    let tab_titles = vec!["[1] Help Map", "[2] Game State", "[3] Sovereignty", "[4] AI Engine"];

    // Build the tab widget highlighting the active panel selection
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT))
        .select(app.active_tab as usize)
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        );

    f.render_widget(tabs, tab_header_area);

    // Delegate content rendering based on what tab is currently highlighted
    match app.active_tab {
        ActivePanelTab::ControlMap => panels::control_map::render(f, tab_content_area, app),
        ActivePanelTab::GameState => panels::game_state::render(f, tab_content_area, app),
        ActivePanelTab::HeuristicsTable => panels::heuristics::render(f, tab_content_area, app),
        ActivePanelTab::AIMoveAnalysis => panels::ai_moves::render(f, tab_content_area, app),
    }
}