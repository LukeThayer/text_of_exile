use crate::app::{App, GameState, InputMode};
use crossterm::event::{KeyCode, KeyEvent};

/// Handle input and return true if the app should quit
pub fn handle_input(app: &mut App, key: KeyEvent) -> bool {
    // Global quit
    if key.code == KeyCode::Char('q') {
        return true;
    }

    match app.input_mode {
        InputMode::Combat => handle_combat_input(app, key),
        InputMode::Inventory => handle_inventory_input(app, key),
        InputMode::SelectCurrency => handle_currency_selection(app, key),
    }

    false
}

fn handle_combat_input(app: &mut App, key: KeyEvent) {
    if app.state == GameState::GameOver {
        // R to restart
        if key.code == KeyCode::Char('r') {
            *app = App::new().unwrap();
        }
        return;
    }

    if app.state != GameState::PlayerTurn {
        return;
    }

    match key.code {
        KeyCode::Char(' ') => app.player_attack(),
        KeyCode::Char('1') => app.use_skill(0),
        KeyCode::Char('2') => app.use_skill(1),
        KeyCode::Char('3') => app.use_skill(2),
        KeyCode::Char('4') => app.use_skill(3),
        KeyCode::Char('i') | KeyCode::Tab => app.toggle_inventory(),
        _ => {}
    }
}

fn handle_inventory_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => app.move_inventory_selection(-1),
        KeyCode::Down | KeyCode::Char('j') => app.move_inventory_selection(1),
        KeyCode::Char('e') | KeyCode::Enter => app.equip_selected(),
        KeyCode::Char('c') => app.start_crafting(),
        KeyCode::Char('i') | KeyCode::Tab | KeyCode::Esc => app.toggle_inventory(),
        _ => {}
    }
}

fn handle_currency_selection(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('1') => app.apply_currency(0),
        KeyCode::Char('2') => app.apply_currency(1),
        KeyCode::Char('3') => app.apply_currency(2),
        KeyCode::Esc => app.input_mode = InputMode::Inventory,
        _ => {}
    }
}
