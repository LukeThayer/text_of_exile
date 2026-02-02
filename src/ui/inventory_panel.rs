use crate::app::{App, InputMode};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
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

    if app.inventory.items.is_empty() {
        lines.push(Line::from(Span::styled(
            "  (empty)",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, item) in app.inventory.items.iter().enumerate() {
            let is_selected = is_active && i == app.selected_inventory_index;

            let prefix = if is_selected { " > " } else { "   " };

            let rarity_color = match item.rarity {
                crate::game::inventory::Rarity::Common => Color::White,
                crate::game::inventory::Rarity::Magic => Color::Blue,
                crate::game::inventory::Rarity::Rare => Color::Yellow,
            };

            let mut line_spans = vec![Span::raw(prefix)];

            if is_selected {
                line_spans.push(Span::styled(
                    format!("{} {}", item.name, item.rarity_symbol()),
                    Style::default()
                        .fg(rarity_color)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                line_spans.push(Span::styled(
                    format!("{} {}", item.name, item.rarity_symbol()),
                    Style::default().fg(rarity_color),
                ));
            }

            lines.push(Line::from(line_spans));

            // Show stats for selected item
            if is_selected && item.is_equipment() {
                if item.stats.damage > 0 {
                    lines.push(Line::from(Span::styled(
                        format!("     Damage: {}", item.stats.damage),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
                if item.stats.defense > 0 {
                    lines.push(Line::from(Span::styled(
                        format!("     Defense: {}", item.stats.defense),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
                for affix in &item.affixes {
                    lines.push(Line::from(Span::styled(
                        format!("     + {}", affix),
                        Style::default().fg(Color::Cyan),
                    )));
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
                .add_modifier(Modifier::BOLD),
        )));

        for (i, (_, currency)) in app.inventory.get_currencies().iter().enumerate() {
            lines.push(Line::from(format!("  [{}] {}", i + 1, currency.name)));
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
