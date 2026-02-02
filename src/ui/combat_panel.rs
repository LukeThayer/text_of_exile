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
    let max_life = enemy.max_life();
    let current_life = enemy.current_life();
    let hp_bar = create_bar(current_life, max_life);

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
        Span::raw(format!(" {:.0}/{:.0}", current_life, max_life)),
    ]));

    // Enemy defensive stats
    let armour = enemy.stats.armour.compute();
    let fire_res = enemy.stats.fire_resistance.compute();
    let cold_res = enemy.stats.cold_resistance.compute();
    let lightning_res = enemy.stats.lightning_resistance.compute();

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Armour: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.0}", armour), Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled("Res: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.0}%", fire_res), Style::default().fg(Color::Red)),
        Span::raw("/"),
        Span::styled(format!("{:.0}%", cold_res), Style::default().fg(Color::Cyan)),
        Span::raw("/"),
        Span::styled(format!("{:.0}%", lightning_res), Style::default().fg(Color::Yellow)),
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

fn create_bar(current: f64, max: f64) -> String {
    let width = 10;
    let ratio = if max > 0.0 { current / max } else { 0.0 };
    let filled = (ratio * width as f64) as usize;
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
