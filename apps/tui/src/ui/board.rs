use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};
use crate::app::{App, SelectionState};

pub fn render_board(f: &mut Frame<'_>, area: Rect, app: &App) {
    // 1. Divide the vertical board space into 8 uniform rows
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2); 8])
        .split(area);

    // Grid coordinates flow from rank 7 down to rank 0 (top to bottom on screen)
    for r in 0..8 {
        let rank = 7 - r; 
        
        // 2. Divide each row horizontally into 8 uniform columns
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(5); 8])
            .split(rows[r as usize]);

        for file in 0..8 {
            let current_idx = rank * 8 + file;
            
            // Extract piece presence from underlying bitboards
            let has_p1 = app.game_state.p1_pieces.has_bit(current_idx);
            let has_p2 = app.game_state.p2_pieces.has_bit(current_idx);

            // Text display asset configuration
            let cell_text = if has_p1 { " 🅟🅜 " } else if has_p2 { " 🅟🅝 " } else { " . " };

            // Generate standard alternating checkerboard background patterns
            let mut bg_color = if (rank + file) % 2 == 0 { 
                Color::Rgb(30, 30, 30) 
            } else { 
                Color::Rgb(45, 45, 45) 
            };
            
            let mut fg_color = if has_p1 { 
                Color::Cyan 
            } else if has_p2 { 
                Color::Magenta 
            } else { 
                Color::DarkGray 
            };
            
            let mut modifier = Modifier::empty();

            // 3. Highlight states based on selection matrix layers
            if let SelectionState::PieceSelected { index, valid_moves } = app.selection {
                if index == current_idx {
                    // Active moving piece anchor
                    bg_color = Color::Blue;
                    fg_color = Color::White;
                } else if valid_moves.has_bit(current_idx) {
                    // Legal destination cells projected in green
                    bg_color = Color::Green;
                    fg_color = Color::Black;
                    modifier = Modifier::BOLD;
                }
            }

            // 4. Highlight current user navigation cursor location
            if app.cursor_x == file && app.cursor_y == rank {
                bg_color = Color::Yellow;
                fg_color = Color::Black;
                modifier = Modifier::BOLD;
            }

            // 5. Draw the single cell widget
            let cell_paragraph = Paragraph::new(cell_text)
                .alignment(Alignment::Center)
                .style(Style::default().fg(fg_color).bg(bg_color).add_modifier(modifier));

            f.render_widget(cell_paragraph, cols[file as usize]);
        }
    }
}