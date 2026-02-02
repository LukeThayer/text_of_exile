use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use stat_core::EquipmentSlot;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let player = &app.player;
    let stats = &player.stats;

    let max_life = player.max_life();
    let max_mana = player.max_mana();

    let hp_bar = create_bar(stats.current_life, max_life, Color::Red);
    let mana_bar = create_bar(stats.current_mana, max_mana, Color::Blue);

    let weapon_name = player
        .get_equipped(EquipmentSlot::MainHand)
        .map(|w| w.name.as_str())
        .unwrap_or("(unarmed)");
    let armor_name = player
        .get_equipped(EquipmentSlot::BodyArmour)
        .map(|a| a.name.as_str())
        .unwrap_or("(none)");

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("HP:   "),
            Span::styled(hp_bar, Style::default().fg(Color::Red)),
            Span::raw(format!(" {:.0}/{:.0}", stats.current_life, max_life)),
        ]),
        Line::from(vec![
            Span::raw("Mana: "),
            Span::styled(mana_bar, Style::default().fg(Color::Blue)),
            Span::raw(format!(" {:.0}/{:.0}", stats.current_mana, max_mana)),
        ]),
        Line::from(""),
        Line::from(format!("STR: {:.0}", stats.strength.compute())),
        Line::from(format!("DEX: {:.0}", stats.dexterity.compute())),
        Line::from(format!("INT: {:.0}", stats.intelligence.compute())),
        Line::from(""),
        Line::from(format!(
            "Damage: {:.0}-{:.0}",
            stats.weapon_physical_min, stats.weapon_physical_max
        )),
        Line::from(format!("Armour: {:.0}", stats.armour.compute())),
        Line::from(format!("Evasion: {:.0}", stats.evasion.compute())),
        Line::from(""),
        Line::from(Span::styled(
            "--- Equipment ---",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("Weapon: {}", weapon_name)),
        Line::from(format!("Armor:  {}", armor_name)),
        Line::from(""),
        Line::from(Span::styled(
            "--- Skills ---",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ];

    let mut all_lines = lines;

    for (i, skill) in player.skills.iter().enumerate() {
        let mana_cost = player.skill_mana_costs.get(i).copied().unwrap_or(0);
        all_lines.push(Line::from(format!(
            "[{}] {} ({}mp)",
            i + 1,
            skill.name,
            mana_cost
        )));
    }

    let paragraph = Paragraph::new(all_lines)
        .block(Block::default().borders(Borders::ALL).title("Player"));

    frame.render_widget(paragraph, area);
}

fn create_bar(current: f64, max: f64, _color: Color) -> String {
    let width = 8;
    let ratio = if max > 0.0 { current / max } else { 0.0 };
    let filled = (ratio * width as f64) as usize;
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
