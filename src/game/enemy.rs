use stat_core::StatBlock;

pub struct Enemy {
    pub name: String,
    pub stats: StatBlock,
}

impl Enemy {
    pub fn new() -> Self {
        let mut stats = StatBlock::with_id("goblin");

        stats.max_life.base = 50.0;
        stats.current_life = 50.0;

        // Enemy attack stats
        stats.weapon_physical_min = 5.0;
        stats.weapon_physical_max = 10.0;
        stats.weapon_attack_speed = 1.0;
        stats.weapon_crit_chance = 5.0;

        // Accuracy for evasion checks
        stats.accuracy.base = 80.0;

        // Defenses
        stats.armour.base = 20.0;
        stats.fire_resistance.base = 10.0;
        stats.cold_resistance.base = 10.0;
        stats.lightning_resistance.base = 5.0;

        Self {
            name: "Goblin".to_string(),
            stats,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.stats.current_life <= 0.0
    }

    pub fn max_life(&self) -> f64 {
        self.stats.max_life.compute()
    }

    pub fn current_life(&self) -> f64 {
        self.stats.current_life
    }
}
