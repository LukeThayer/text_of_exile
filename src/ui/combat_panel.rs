use crate::app::{App, GameState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_enemy(frame, app, chunks[0]);
    render_combat_log(frame, app, chunks[1]);
}

fn render_enemy(frame: &mut Frame, app: &App, area: Rect) {
    let enemy = &app.enemy;
    let hp_bar = create_bar(enemy.current_hp, enemy.max_hp);

    let status_text = match app.state {
        GameState::PlayerTurn => "[ Your Turn ]",
        GameState::EnemyTurn => "[ Enemy Turn ]",
        GameState::Victory => "[ Victory! ]",
        GameState::GameOver => "[ Game Over ]",
    };

    let status_color = match app.state {
        GameState::PlayerTurn => Color::Green,
        GameState::EnemyTurn => Color::Yellow,
        GameState::Victory => Color::Cyan,
        GameState::GameOver => Color::Red,
    };

    let enemy_art = vec![
        "   ╭───────────────╮",
        "   │               │",
        "   │    (ಠ_ಠ)      │",
        "   │    /|\\       │",
        "   │    / \\        │",
        "   │               │",
        "   ╰───────────────╯",
    ];

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            status_text,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for art_line in enemy_art {
        lines.push(Line::from(art_line));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        &enemy.name,
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(vec![
        Span::raw("HP: "),
        Span::styled(hp_bar, Style::default().fg(Color::Red)),
        Span::raw(format!(" {}/{}", enemy.current_hp, enemy.max_hp)),
    ]));

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Combat"));

    frame.render_widget(paragraph, area);
}

fn render_combat_log(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = app
        .combat_log
        .iter()
        .map(|msg| Line::from(format!("  {}", msg)))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Combat Log"));

    frame.render_widget(paragraph, area);
}

fn create_bar(current: u32, max: u32) -> String {
    let width = 10;
    let filled = ((current as f32 / max as f32) * width as f32) as usize;
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
