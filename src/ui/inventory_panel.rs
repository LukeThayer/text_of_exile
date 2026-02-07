use crate::app::{App, InputMode};
use crate::game::inventory::InventoryEntry;
use loot_core::{item::Modifier, AffixScope, DamageType, StatType};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier as StyleModifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let is_active = matches!(
        app.input_mode,
        InputMode::Inventory | InputMode::SelectCurrency
    );

    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let mut lines = vec![Line::from("")];

    if app.inventory.is_empty() {
        lines.push(Line::from(Span::styled(
            "  (empty)",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, entry) in app.inventory.iter().enumerate() {
            let is_selected = is_active && i == app.selected_inventory_index;

            let prefix = if is_selected { " > " } else { "   " };

            let rarity_color = match entry.rarity() {
                "magic" => Color::Blue,
                "rare" => Color::Yellow,
                "unique" => Color::Rgb(175, 95, 0),
                _ => Color::White,
            };

            let rarity_label = match entry.rarity() {
                "magic" => " [Magic]",
                "rare" => " [Rare]",
                "unique" => " [Unique]",
                _ => "",
            };

            let name = entry.name();

            let mut line_spans = vec![Span::raw(prefix)];

            if is_selected {
                line_spans.push(Span::styled(
                    format!("{}{}", name, rarity_label),
                    Style::default()
                        .fg(rarity_color)
                        .add_modifier(StyleModifier::BOLD),
                ));
            } else {
                line_spans.push(Span::styled(
                    format!("{}{}", name, rarity_label),
                    Style::default().fg(rarity_color),
                ));
            }

            lines.push(Line::from(line_spans));

            // Show item details for selected item
            if is_selected {
                if let InventoryEntry::Item(item) = entry {
                    // Show weapon damage
                    if let Some(ref damage) = item.damage {
                        for dmg in &damage.damages {
                            let type_color = damage_type_color(dmg.damage_type);
                            lines.push(Line::from(Span::styled(
                                format!(
                                    "     {}-{} {} damage",
                                    dmg.min,
                                    dmg.max,
                                    format_damage_type(dmg.damage_type)
                                ),
                                Style::default().fg(type_color),
                            )));
                        }
                        if damage.attack_speed != 1.0 {
                            lines.push(Line::from(Span::styled(
                                format!("     {:.2} attacks/sec", damage.attack_speed),
                                Style::default().fg(Color::Gray),
                            )));
                        }
                        if damage.critical_chance > 0.0 {
                            lines.push(Line::from(Span::styled(
                                format!("     {:.1}% crit chance", damage.critical_chance),
                                Style::default().fg(Color::Gray),
                            )));
                        }
                    }

                    // Show defenses
                    if let Some(armour) = item.defenses.armour {
                        lines.push(Line::from(Span::styled(
                            format!("     {} Armour", armour),
                            Style::default().fg(Color::Gray),
                        )));
                    }
                    if let Some(evasion) = item.defenses.evasion {
                        lines.push(Line::from(Span::styled(
                            format!("     {} Evasion", evasion),
                            Style::default().fg(Color::Gray),
                        )));
                    }
                    if let Some(es) = item.defenses.energy_shield {
                        lines.push(Line::from(Span::styled(
                            format!("     {} Energy Shield", es),
                            Style::default().fg(Color::Cyan),
                        )));
                    }

                    // Show implicit modifier
                    if let Some(ref implicit) = item.implicit {
                        lines.push(Line::from(Span::styled(
                            format!("     {} (implicit)", format_modifier(implicit)),
                            Style::default().fg(Color::Magenta),
                        )));
                    }

                    // Separator if there are affixes
                    if !item.prefixes.is_empty() || !item.suffixes.is_empty() {
                        lines.push(Line::from(Span::styled(
                            "     ───────────────",
                            Style::default().fg(Color::DarkGray),
                        )));
                    }

                    // Show prefix modifiers
                    for prefix_mod in &item.prefixes {
                        lines.push(Line::from(Span::styled(
                            format!("     {}", format_modifier(prefix_mod)),
                            Style::default().fg(Color::Cyan),
                        )));
                    }

                    // Show suffix modifiers
                    for suffix_mod in &item.suffixes {
                        lines.push(Line::from(Span::styled(
                            format!("     {}", format_modifier(suffix_mod)),
                            Style::default().fg(Color::Green),
                        )));
                    }
                }
            }
        }
    }

    // Currency selection mode
    if app.input_mode == InputMode::SelectCurrency {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "─── Select Currency ───",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(StyleModifier::BOLD),
        )));

        for (i, (_, currency_id)) in app.inventory.get_currencies().iter().enumerate() {
            lines.push(Line::from(format!("  [{}] {}", i + 1, currency_id)));
        }
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Inventory")
            .border_style(border_style),
    );

    frame.render_widget(paragraph, area);
}

fn format_modifier(m: &Modifier) -> String {
    let scope_marker = match m.scope {
        AffixScope::Local => "(local)",
        AffixScope::Global => "",
    };

    let stat_name = format_stat_type(m.stat);

    if let Some(max_val) = m.value_max {
        // Damage range modifier
        format!(
            "+{}-{} {} {} T{}",
            m.value, max_val, stat_name, scope_marker, m.tier
        )
    } else {
        // Single value modifier
        let sign = if m.value >= 0 { "+" } else { "" };
        format!(
            "{}{} {} {} T{}",
            sign, m.value, stat_name, scope_marker, m.tier
        )
        .trim()
        .to_string()
    }
}

fn format_stat_type(stat: StatType) -> &'static str {
    match stat {
        StatType::AddedPhysicalDamage => "Physical Damage",
        StatType::AddedFireDamage => "Fire Damage",
        StatType::AddedColdDamage => "Cold Damage",
        StatType::AddedLightningDamage => "Lightning Damage",
        StatType::AddedChaosDamage => "Chaos Damage",
        StatType::IncreasedPhysicalDamage => "% Phys Damage",
        StatType::IncreasedFireDamage => "% Fire Damage",
        StatType::IncreasedColdDamage => "% Cold Damage",
        StatType::IncreasedLightningDamage => "% Lightning Damage",
        StatType::IncreasedChaosDamage => "% Chaos Damage",
        StatType::IncreasedAttackSpeed => "% Attack Speed",
        StatType::IncreasedCriticalChance => "% Crit Chance",
        StatType::AddedLife => "Life",
        StatType::AddedMana => "Mana",
        StatType::AddedArmour => "Armour",
        StatType::AddedEvasion => "Evasion",
        StatType::AddedEnergyShield => "Energy Shield",
        StatType::IncreasedArmour => "% Armour",
        StatType::IncreasedEvasion => "% Evasion",
        StatType::FireResistance => "% Fire Res",
        StatType::ColdResistance => "% Cold Res",
        StatType::LightningResistance => "% Lightning Res",
        StatType::ChaosResistance => "% Chaos Res",
        StatType::AddedStrength => "Strength",
        StatType::AddedDexterity => "Dexterity",
        StatType::AddedIntelligence => "Intelligence",
        _ => "???",
    }
}

fn format_damage_type(dt: DamageType) -> &'static str {
    match dt {
        DamageType::Physical => "physical",
        DamageType::Fire => "fire",
        DamageType::Cold => "cold",
        DamageType::Lightning => "lightning",
        DamageType::Chaos => "chaos",
    }
}

fn damage_type_color(dt: DamageType) -> Color {
    match dt {
        DamageType::Physical => Color::Gray,
        DamageType::Fire => Color::Red,
        DamageType::Cold => Color::Cyan,
        DamageType::Lightning => Color::Yellow,
        DamageType::Chaos => Color::Magenta,
    }
}
