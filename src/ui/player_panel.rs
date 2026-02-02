use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let player = &app.player;

    let hp_bar = create_bar(player.current_hp, player.max_hp, Color::Red);
    let mana_bar = create_bar(player.current_mana, player.max_mana, Color::Blue);

    let weapon_name = player
        .equipped_weapon
        .as_ref()
        .map(|w| w.name.as_str())
        .unwrap_or("(none)");
    let armor_name = player
        .equipped_armor
        .as_ref()
        .map(|a| a.name.as_str())
        .unwrap_or("(none)");

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("HP:   "),
            Span::styled(hp_bar, Style::default().fg(Color::Red)),
            Span::raw(format!(" {}/{}", player.current_hp, player.max_hp)),
        ]),
        Line::from(vec![
            Span::raw("Mana: "),
            Span::styled(mana_bar, Style::default().fg(Color::Blue)),
            Span::raw(format!(" {}/{}", player.current_mana, player.max_mana)),
        ]),
        Line::from(""),
        Line::from(format!("STR: {}", player.strength)),
        Line::from(format!("DEX: {}", player.dexterity)),
        Line::from(format!("INT: {}", player.intelligence)),
        Line::from(""),
        Line::from(format!("Attack:  {}", player.calculate_attack())),
        Line::from(format!("Defense: {}", player.calculate_defense())),
        Line::from(""),
        Line::from(Span::styled(
            "─── Equipment ───",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("Weapon: {}", weapon_name)),
        Line::from(format!("Armor:  {}", armor_name)),
        Line::from(""),
        Line::from(Span::styled(
            "─── Skills ───",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ];

    let mut all_lines = lines;

    for (i, skill) in player.skills.iter().enumerate() {
        all_lines.push(Line::from(format!(
            "[{}] {} ({}mp)",
            i + 1,
            skill.name,
            skill.mana_cost
        )));
    }

    let paragraph = Paragraph::new(all_lines)
        .block(Block::default().borders(Borders::ALL).title("Player"));

    frame.render_widget(paragraph, area);
}

fn create_bar(current: u32, max: u32, _color: Color) -> String {
    let width = 8;
    let filled = ((current as f32 / max as f32) * width as f32) as usize;
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
