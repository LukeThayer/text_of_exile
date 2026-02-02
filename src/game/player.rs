use super::inventory::Item;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Skill {
    pub name: String,
    pub damage_multiplier: f32,
    pub mana_cost: u32,
    pub description: String,
}

#[derive(Debug)]
pub struct Player {
    pub max_hp: u32,
    pub current_hp: u32,
    pub max_mana: u32,
    pub current_mana: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub equipped_weapon: Option<Item>,
    pub equipped_armor: Option<Item>,
    pub skills: Vec<Skill>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            max_hp: 100,
            current_hp: 100,
            max_mana: 50,
            current_mana: 50,
            strength: 10,
            dexterity: 8,
            intelligence: 6,
            equipped_weapon: None,
            equipped_armor: None,
            skills: Self::load_default_skills(),
        }
    }

    fn load_default_skills() -> Vec<Skill> {
        vec![
            Skill {
                name: "Power Strike".to_string(),
                damage_multiplier: 1.5,
                mana_cost: 10,
                description: "A powerful melee attack".to_string(),
            },
            Skill {
                name: "Fireball".to_string(),
                damage_multiplier: 2.0,
                mana_cost: 20,
                description: "Hurls a ball of fire".to_string(),
            },
            Skill {
                name: "Quick Slash".to_string(),
                damage_multiplier: 0.8,
                mana_cost: 5,
                description: "A fast, weak attack".to_string(),
            },
            Skill {
                name: "Heavy Blow".to_string(),
                damage_multiplier: 2.5,
                mana_cost: 30,
                description: "A devastating strike".to_string(),
            },
        ]
    }

    pub fn take_damage(&mut self, damage: u32) -> bool {
        let defense = self.calculate_defense();
        let actual_damage = damage.saturating_sub(defense);
        self.current_hp = self.current_hp.saturating_sub(actual_damage);
        self.current_hp == 0
    }

    pub fn calculate_defense(&self) -> u32 {
        let base_defense = self.dexterity / 2;
        let armor_defense = self
            .equipped_armor
            .as_ref()
            .map(|a| a.stats.defense)
            .unwrap_or(0);
        base_defense + armor_defense
    }

    pub fn calculate_attack(&self) -> u32 {
        let base_attack = self.strength;
        let weapon_attack = self
            .equipped_weapon
            .as_ref()
            .map(|w| w.stats.damage)
            .unwrap_or(0);
        base_attack + weapon_attack
    }

    pub fn equip(&mut self, item: Item) -> Option<Item> {
        match item.slot {
            EquipSlot::Weapon => {
                let old = self.equipped_weapon.take();
                self.equipped_weapon = Some(item);
                old
            }
            EquipSlot::Armor => {
                let old = self.equipped_armor.take();
                self.equipped_armor = Some(item);
                old
            }
            EquipSlot::None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipSlot {
    Weapon,
    Armor,
    None,
}
