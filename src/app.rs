use crate::game::{
    combat::{format_combat_result, Combat},
    enemy::Enemy,
    inventory::Inventory,
    player::Player,
    GameConfig,
};
use anyhow::Result;
use loot_core::Item as LootItem;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::Arc;

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
    pub config: Arc<GameConfig>,
}

impl App {
    pub fn new() -> Result<Self> {
        let rng = StdRng::from_entropy();
        let config = Arc::new(GameConfig::load()?);

        Ok(Self {
            state: GameState::PlayerTurn,
            input_mode: InputMode::Combat,
            player: Player::new(&config),
            enemy: Enemy::new(),
            inventory: Inventory::new(),
            combat_log: vec!["A wild Goblin appears!".to_string()],
            selected_inventory_index: 0,
            rng,
            config,
        })
    }

    pub fn log(&mut self, message: String) {
        self.combat_log.push(message);
        // Keep only last 20 messages for detailed combat info
        if self.combat_log.len() > 20 {
            self.combat_log.remove(0);
        }
    }

    pub fn player_attack(&mut self) {
        if self.state != GameState::PlayerTurn {
            return;
        }

        let result = Combat::player_basic_attack(&self.player, &mut self.enemy);

        // Log the combat result
        for line in format_combat_result(&result, "You", result.is_killing_blow) {
            self.log(line);
        }

        if self.enemy.is_dead() {
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

        let skill_name = self.player.skills[skill_index].name.clone();
        let mana_cost = self.player.skill_mana_costs[skill_index];

        if self.player.stats.current_mana < mana_cost as f64 {
            self.log(format!("Not enough mana for {}!", skill_name));
            return;
        }

        self.player.stats.current_mana -= mana_cost as f64;
        let result = Combat::player_skill_attack(&self.player, &mut self.enemy, skill_index);

        // Log with skill name
        self.log(format!("You use {}!", skill_name));
        for line in format_combat_result(&result, "  →", false) {
            self.log(line);
        }

        if self.enemy.is_dead() {
            self.on_enemy_death();
        } else {
            self.state = GameState::EnemyTurn;
            self.enemy_turn();
        }
    }

    fn enemy_turn(&mut self) {
        let result = Combat::enemy_attack(&self.enemy, &mut self.player);

        for line in format_combat_result(&result, "Goblin", false) {
            self.log(line);
        }

        if !self.player.stats.is_alive() {
            self.state = GameState::GameOver;
            self.log("You have been defeated!".to_string());
        } else {
            self.state = GameState::PlayerTurn;
        }
    }

    fn on_enemy_death(&mut self) {
        self.log("The Goblin has been slain!".to_string());

        // Roll drops using tables_core
        let rarity_mult = 1.0 + self.player.stats.item_rarity_increased;
        let quantity_mult = 1.0 + self.player.stats.item_quantity_increased;

        match self
            .config
            .tables
            .roll("goblin", rarity_mult, quantity_mult, 10, &mut self.rng)
        {
            Ok(drops) => {
                use tables_core::DropsExt;

                // Generate items
                for item_drop in drops.get_items() {
                    let seed: u64 = self.rng.gen();
                    match self.config.generator.generate(item_drop.base_type, seed) {
                        Ok(mut item) => {
                            // Apply any currencies specified in the drop
                            for currency_id in item_drop.currencies {
                                if let Ok(new_item) =
                                    self.config.generator.apply_currency(&item, currency_id)
                                {
                                    item = new_item;
                                }
                            }
                            // Log item with damage info if it's a weapon
                            if let Some(ref dmg) = item.damage {
                                let dmg_str: Vec<String> = dmg
                                    .damages
                                    .iter()
                                    .map(|d| format!("{}-{}", d.min, d.max))
                                    .collect();
                                self.log(format!(
                                    "Dropped: {} ({})",
                                    item.name,
                                    dmg_str.join(", ")
                                ));
                            } else {
                                self.log(format!("Dropped: {}", item.name));
                            }
                            self.inventory.add_item(item);
                        }
                        Err(e) => {
                            self.log(format!("Failed to generate item: {}", e));
                        }
                    }
                }

                // Add currencies
                for currency_drop in drops.get_currencies() {
                    for _ in 0..currency_drop.count {
                        self.inventory.add_currency(currency_drop.id.to_string());
                        self.log(format!("Dropped: {}", currency_drop.id));
                    }
                }

                // Add uniques
                for unique_drop in drops.get_uniques() {
                    let seed: u64 = self.rng.gen();
                    if let Ok(item) = self.config.generator.generate_unique(unique_drop.id, seed) {
                        self.log(format!("Dropped UNIQUE: {}", item.name));
                        self.inventory.add_item(item);
                    }
                }
            }
            Err(e) => {
                self.log(format!("Drop error: {}", e));
            }
        }

        // Heal after combat (50% HP, 30% mana)
        let max_life = self.player.max_life();
        let max_mana = self.player.max_mana();
        self.player.stats.current_life =
            (self.player.stats.current_life + max_life * 0.50).min(max_life);
        self.player.stats.current_mana =
            (self.player.stats.current_mana + max_mana * 0.30).min(max_mana);
        self.log("Combat ended. +50% HP, +30% Mana restored.".to_string());

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
        let len = self.inventory.len();
        if len == 0 {
            return;
        }
        let new_index = (self.selected_inventory_index as i32 + delta).rem_euclid(len as i32);
        self.selected_inventory_index = new_index as usize;
    }

    pub fn equip_selected(&mut self) {
        if let Some(entry) = self.inventory.get(self.selected_inventory_index) {
            if let Some(item) = entry.as_item() {
                let slot = item_to_slot(item);
                if let Some(slot) = slot {
                    if let Some(item) = self.inventory.remove_item(self.selected_inventory_index) {
                        if let Some(old_item) = self.player.equip(slot, item) {
                            self.inventory.add_item(old_item);
                        }
                        self.log("Item equipped!".to_string());
                        if self.selected_inventory_index > 0
                            && self.selected_inventory_index >= self.inventory.len()
                        {
                            self.selected_inventory_index = self.inventory.len().saturating_sub(1);
                        }
                    }
                } else {
                    self.log("Cannot equip this item type.".to_string());
                }
            } else {
                self.log("Cannot equip currencies.".to_string());
            }
        }
    }

    pub fn start_crafting(&mut self) {
        if let Some(entry) = self.inventory.get(self.selected_inventory_index) {
            if entry.as_item().is_some() {
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
        let currencies = self.inventory.get_currency_indices();
        let item_index = self.selected_inventory_index;

        if let Some(&currency_actual_index) = currencies.get(currency_index) {
            // Get the item first (without removing) to apply currency
            let item_opt = self.inventory.get(item_index).and_then(|e| e.as_item()).cloned();

            if let Some(item) = item_opt {
                // Get currency info before removing
                let currency_id_opt = self.inventory.get(currency_actual_index).and_then(|e| {
                    if let crate::game::inventory::InventoryEntry::Currency { id, .. } = e {
                        Some(id.clone())
                    } else {
                        None
                    }
                });

                if let Some(currency_id) = currency_id_opt {
                    let old_rarity = item.rarity.clone();

                    match self.config.generator.apply_currency(&item, &currency_id) {
                        Ok(new_item) => {
                            // Success - consume the currency and replace the item in place
                            self.inventory.remove_currency(currency_actual_index);

                            // Adjust item index if currency was removed and was before the item
                            let currency_was_removed = !self.inventory.get(currency_actual_index)
                                .map(|e| matches!(e, crate::game::inventory::InventoryEntry::Currency { id, .. } if id == &currency_id))
                                .unwrap_or(false);

                            let final_item_index = if currency_was_removed && currency_actual_index < item_index {
                                item_index - 1
                            } else {
                                item_index
                            };

                            self.log(format!(
                                "SUCCESS: Applied {} to {}",
                                currency_id, item.name
                            ));

                            // Log what changed
                            if new_item.rarity != old_rarity {
                                self.log(format!(
                                    "  → Rarity: {:?} → {:?}",
                                    old_rarity, new_item.rarity
                                ));
                            }

                            let new_prefix_count = new_item.prefixes.len();
                            let new_suffix_count = new_item.suffixes.len();

                            self.log(format!(
                                "  → Mods: {} prefix, {} suffix",
                                new_prefix_count, new_suffix_count
                            ));

                            // Log each affix with its rolled value
                            for prefix in &new_item.prefixes {
                                if let Some(max_val) = prefix.value_max {
                                    self.log(format!(
                                        "  + [P] {}: {}-{} (T{})",
                                        prefix.name, prefix.value, max_val, prefix.tier
                                    ));
                                } else {
                                    self.log(format!(
                                        "  + [P] {}: {} (T{})",
                                        prefix.name, prefix.value, prefix.tier
                                    ));
                                }
                            }
                            for suffix in &new_item.suffixes {
                                if let Some(max_val) = suffix.value_max {
                                    self.log(format!(
                                        "  + [S] {}: {}-{} (T{})",
                                        suffix.name, suffix.value, max_val, suffix.tier
                                    ));
                                } else {
                                    self.log(format!(
                                        "  + [S] {}: {} (T{})",
                                        suffix.name, suffix.value, suffix.tier
                                    ));
                                }
                            }

                            // Replace item in place
                            self.inventory.replace_item(final_item_index, new_item);
                            self.selected_inventory_index = final_item_index;
                        }
                        Err(e) => {
                            self.log(format!("FAILED: Cannot apply {} - {}", currency_id, e));
                            // Don't consume the currency on failure
                        }
                    }
                }
            }
        }

        self.input_mode = InputMode::Inventory;
    }
}

fn item_to_slot(item: &LootItem) -> Option<stat_core::EquipmentSlot> {
    use loot_core::ItemClass;
    use stat_core::EquipmentSlot;

    match item.class {
        ItemClass::OneHandSword
        | ItemClass::OneHandAxe
        | ItemClass::OneHandMace
        | ItemClass::Dagger
        | ItemClass::Claw
        | ItemClass::Wand => Some(EquipmentSlot::MainHand),
        ItemClass::TwoHandSword
        | ItemClass::TwoHandAxe
        | ItemClass::TwoHandMace
        | ItemClass::Bow
        | ItemClass::Staff => Some(EquipmentSlot::MainHand),
        ItemClass::Shield => Some(EquipmentSlot::OffHand),
        ItemClass::Helmet => Some(EquipmentSlot::Helmet),
        ItemClass::BodyArmour => Some(EquipmentSlot::BodyArmour),
        ItemClass::Gloves => Some(EquipmentSlot::Gloves),
        ItemClass::Boots => Some(EquipmentSlot::Boots),
        ItemClass::Ring => Some(EquipmentSlot::Ring1),
        ItemClass::Amulet => Some(EquipmentSlot::Amulet),
        ItemClass::Belt => Some(EquipmentSlot::Belt),
    }
}
