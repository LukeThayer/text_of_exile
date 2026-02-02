use super::player::EquipSlot;
use rand::rngs::StdRng;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct ItemStats {
    pub damage: u32,
    pub defense: u32,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub slot: EquipSlot,
    pub stats: ItemStats,
    pub rarity: Rarity,
    pub affixes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Magic,
    Rare,
}

impl Item {
    pub fn is_equipment(&self) -> bool {
        self.slot != EquipSlot::None
    }

    pub fn is_currency(&self) -> bool {
        self.name.contains("Orb")
    }

    pub fn apply_currency(&mut self, currency: &Item) {
        // Simplified currency application
        // In full implementation, this would use loot_core
        match currency.name.as_str() {
            "Orb of Enhancement" => {
                self.stats.damage += 2;
                self.stats.defense += 1;
                self.affixes.push("+2 damage, +1 defense".to_string());
            }
            "Orb of Chaos" => {
                self.stats.damage = rand::thread_rng().gen_range(5..20);
                self.stats.defense = rand::thread_rng().gen_range(0..10);
                self.affixes.clear();
                self.affixes.push("Rerolled stats".to_string());
                self.rarity = Rarity::Rare;
            }
            "Orb of Augmentation" => {
                if self.rarity == Rarity::Magic && self.affixes.len() < 2 {
                    self.affixes.push("Added modifier".to_string());
                    self.stats.damage += 1;
                }
            }
            _ => {}
        }
    }

    pub fn random_weapon(rng: &mut StdRng) -> Self {
        let weapons = ["Iron Sword", "Steel Axe", "Bronze Mace", "Silver Dagger"];
        let name = weapons[rng.gen_range(0..weapons.len())];
        let damage = rng.gen_range(5..15);

        Self {
            name: name.to_string(),
            slot: EquipSlot::Weapon,
            stats: ItemStats { damage, defense: 0 },
            rarity: if damage > 10 {
                Rarity::Magic
            } else {
                Rarity::Common
            },
            affixes: Vec::new(),
        }
    }

    pub fn random_armor(rng: &mut StdRng) -> Self {
        let armors = ["Leather Vest", "Chain Mail", "Iron Plate", "Cloth Robe"];
        let name = armors[rng.gen_range(0..armors.len())];
        let defense = rng.gen_range(2..8);

        Self {
            name: name.to_string(),
            slot: EquipSlot::Armor,
            stats: ItemStats {
                damage: 0,
                defense,
            },
            rarity: if defense > 5 {
                Rarity::Magic
            } else {
                Rarity::Common
            },
            affixes: Vec::new(),
        }
    }

    pub fn random_currency(rng: &mut StdRng) -> Self {
        let currencies = ["Orb of Enhancement", "Orb of Chaos", "Orb of Augmentation"];
        let name = currencies[rng.gen_range(0..currencies.len())];

        Self {
            name: name.to_string(),
            slot: EquipSlot::None,
            stats: ItemStats {
                damage: 0,
                defense: 0,
            },
            rarity: Rarity::Common,
            affixes: Vec::new(),
        }
    }

    pub fn rarity_symbol(&self) -> &'static str {
        match self.rarity {
            Rarity::Common => "",
            Rarity::Magic => "★",
            Rarity::Rare => "★★",
        }
    }
}

#[derive(Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn has_currency(&self) -> bool {
        self.items.iter().any(|item| item.is_currency())
    }

    pub fn get_currencies(&self) -> Vec<(usize, &Item)> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.is_currency())
            .collect()
    }
}
