mod combat_panel;
mod inventory_panel;
mod player_panel;

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let main_area = chunks[0];
    let help_area = chunks[1];

    // Split main area into 3 columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(main_area);

    player_panel::render(frame, app, columns[0]);
    combat_panel::render(frame, app, columns[1]);
    inventory_panel::render(frame, app, columns[2]);

    render_help_bar(frame, app, help_area);
}

fn render_help_bar(frame: &mut Frame, app: &App, area: Rect) {
    use crate::app::{GameState, InputMode};
    use ratatui::{
        style::{Color, Style},
        widgets::{Block, Borders, Paragraph},
    };

    let help_text = match (&app.state, &app.input_mode) {
        (GameState::GameOver, _) => "[R] Restart  [Q] Quit",
        (_, InputMode::Combat) => "[SPACE] Attack  [1-4] Skills  [I/Tab] Inventory  [Q] Quit",
        (_, InputMode::Inventory) => "[↑↓/jk] Navigate  [E] Equip  [C] Craft  [I/Tab/Esc] Back",
        (_, InputMode::SelectCurrency) => "[1-3] Select Currency  [Esc] Cancel",
    };

    let paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Controls"));

    frame.render_widget(paragraph, area);
}
