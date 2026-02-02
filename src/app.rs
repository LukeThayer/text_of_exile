use crate::game::{combat::Combat, enemy::Enemy, inventory::Inventory, player::Player};
use anyhow::Result;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,
    Victory,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Combat,
    Inventory,
    SelectCurrency,
}

pub struct App {
    pub state: GameState,
    pub input_mode: InputMode,
    pub player: Player,
    pub enemy: Enemy,
    pub inventory: Inventory,
    pub combat_log: Vec<String>,
    pub selected_inventory_index: usize,
    pub rng: StdRng,
}

impl App {
    pub fn new() -> Result<Self> {
        let rng = StdRng::from_entropy();

        Ok(Self {
            state: GameState::PlayerTurn,
            input_mode: InputMode::Combat,
            player: Player::new(),
            enemy: Enemy::new(),
            inventory: Inventory::new(),
            combat_log: vec!["A wild Goblin appears!".to_string()],
            selected_inventory_index: 0,
            rng,
        })
    }

    pub fn log(&mut self, message: String) {
        self.combat_log.push(message);
        // Keep only last 10 messages
        if self.combat_log.len() > 10 {
            self.combat_log.remove(0);
        }
    }

    pub fn player_attack(&mut self) {
        if self.state != GameState::PlayerTurn {
            return;
        }

        let damage = Combat::calculate_basic_attack(&self.player);
        let killed = self.enemy.take_damage(damage);

        self.log(format!("You attack for {} damage!", damage));

        if killed {
            self.on_enemy_death();
        } else {
            self.state = GameState::EnemyTurn;
            self.enemy_turn();
        }
    }

    pub fn use_skill(&mut self, skill_index: usize) {
        if self.state != GameState::PlayerTurn {
            return;
        }

        if skill_index >= self.player.skills.len() {
            self.log("No skill in that slot!".to_string());
            return;
        }

        let skill = &self.player.skills[skill_index];
        let skill_name = skill.name.clone();
        let mana_cost = skill.mana_cost;

        if self.player.current_mana < mana_cost {
            self.log(format!("Not enough mana for {}!", skill_name));
            return;
        }

        self.player.current_mana -= mana_cost;
        let damage = Combat::calculate_skill_damage(&self.player, skill_index);

        self.log(format!("You use {} for {} damage!", skill_name, damage));

        let killed = self.enemy.take_damage(damage);

        if killed {
            self.on_enemy_death();
        } else {
            self.state = GameState::EnemyTurn;
            self.enemy_turn();
        }
    }

    fn enemy_turn(&mut self) {
        let damage = Combat::calculate_enemy_attack(&self.enemy);
        let killed = self.player.take_damage(damage);

        self.log(format!("Goblin attacks for {} damage!", damage));

        if killed {
            self.state = GameState::GameOver;
            self.log("You have been defeated!".to_string());
        } else {
            self.state = GameState::PlayerTurn;
        }
    }

    fn on_enemy_death(&mut self) {
        self.log("The Goblin has been slain!".to_string());

        // Generate loot drops
        let drops = self.enemy.roll_drops(&mut self.rng);
        for item in drops {
            self.log(format!("Dropped: {}", item.name));
            self.inventory.add_item(item);
        }

        // Spawn new enemy
        self.enemy = Enemy::new();
        self.log("A new Goblin appears!".to_string());
        self.state = GameState::PlayerTurn;
    }

    pub fn toggle_inventory(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Combat => InputMode::Inventory,
            InputMode::Inventory | InputMode::SelectCurrency => InputMode::Combat,
        };
    }

    pub fn move_inventory_selection(&mut self, delta: i32) {
        let len = self.inventory.items.len();
        if len == 0 {
            return;
        }
        let new_index = (self.selected_inventory_index as i32 + delta).rem_euclid(len as i32);
        self.selected_inventory_index = new_index as usize;
    }

    pub fn equip_selected(&mut self) {
        if let Some(item) = self.inventory.items.get(self.selected_inventory_index) {
            if item.is_equipment() {
                let item = self.inventory.items.remove(self.selected_inventory_index);
                let old_item = self.player.equip(item);
                if let Some(old) = old_item {
                    self.inventory.add_item(old);
                }
                self.log("Item equipped!".to_string());
                if self.selected_inventory_index > 0 {
                    self.selected_inventory_index -= 1;
                }
            } else {
                self.log("Cannot equip this item.".to_string());
            }
        }
    }

    pub fn start_crafting(&mut self) {
        if let Some(item) = self.inventory.items.get(self.selected_inventory_index) {
            if item.is_equipment() {
                if self.inventory.has_currency() {
                    self.input_mode = InputMode::SelectCurrency;
                } else {
                    self.log("No currencies in inventory!".to_string());
                }
            } else {
                self.log("Select an equipment item to craft.".to_string());
            }
        }
    }

    pub fn apply_currency(&mut self, currency_index: usize) {
        let currency_items: Vec<usize> = self
            .inventory
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.is_currency())
            .map(|(i, _)| i)
            .collect();

        if let Some(&actual_index) = currency_items.get(currency_index) {
            let currency = self.inventory.items.remove(actual_index);
            if let Some(item) = self.inventory.items.get_mut(self.selected_inventory_index) {
                // Apply currency effect (simplified for demo)
                item.apply_currency(&currency);
                self.log(format!("Applied {} to item!", currency.name));
            }
        }

        self.input_mode = InputMode::Inventory;
    }
}
