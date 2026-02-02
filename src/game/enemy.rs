use super::inventory::Item;
use rand::rngs::StdRng;
use rand::Rng;

#[derive(Debug)]
pub struct Enemy {
    pub name: String,
    pub max_hp: u32,
    pub current_hp: u32,
    pub attack: u32,
    pub defense: u32,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            name: "Goblin".to_string(),
            max_hp: 50,
            current_hp: 50,
            attack: 8,
            defense: 2,
        }
    }

    pub fn take_damage(&mut self, damage: u32) -> bool {
        let actual_damage = damage.saturating_sub(self.defense);
        self.current_hp = self.current_hp.saturating_sub(actual_damage);
        self.current_hp == 0
    }

    pub fn roll_drops(&self, rng: &mut StdRng) -> Vec<Item> {
        let mut drops = Vec::new();

        // Simple drop table simulation
        // In full implementation, this would use tables_core
        let roll: u32 = rng.gen_range(0..100);

        if roll < 30 {
            // 30% chance for weapon
            drops.push(Item::random_weapon(rng));
        }
        if roll < 50 {
            // 50% chance for armor (can stack with weapon)
            drops.push(Item::random_armor(rng));
        }
        if roll < 80 {
            // 80% chance for currency
            drops.push(Item::random_currency(rng));
        }

        if drops.is_empty() {
            // Always drop at least one currency
            drops.push(Item::random_currency(rng));
        }

        drops
    }
}
