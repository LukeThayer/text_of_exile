use super::GameConfig;
use loot_core::Item;
use stat_core::{
    types::SkillTag, BaseDamage, DamagePacketGenerator, DamageType, EquipmentSlot, StatBlock,
};

/// Create the player's skill set as DamagePacketGenerators
pub fn create_skills() -> Vec<DamagePacketGenerator> {
    vec![
        // Power Strike - high damage melee attack
        DamagePacketGenerator {
            id: "power_strike".to_string(),
            name: "Power Strike".to_string(),
            base_damages: vec![BaseDamage::new(DamageType::Physical, 5.0, 10.0)],
            weapon_effectiveness: 1.5,
            damage_effectiveness: 1.0,
            base_crit_chance: 5.0,
            tags: vec![SkillTag::Attack, SkillTag::Physical, SkillTag::Melee],
            ..Default::default()
        },
        // Fireball - fire spell
        DamagePacketGenerator {
            id: "fireball".to_string(),
            name: "Fireball".to_string(),
            base_damages: vec![BaseDamage::new(DamageType::Fire, 20.0, 35.0)],
            weapon_effectiveness: 0.0,
            damage_effectiveness: 1.0,
            base_crit_chance: 6.0,
            tags: vec![SkillTag::Spell, SkillTag::Fire, SkillTag::Projectile],
            ..Default::default()
        },
        // Quick Slash - fast but weaker attack
        DamagePacketGenerator {
            id: "quick_slash".to_string(),
            name: "Quick Slash".to_string(),
            base_damages: vec![BaseDamage::new(DamageType::Physical, 3.0, 6.0)],
            weapon_effectiveness: 0.8,
            damage_effectiveness: 0.8,
            base_crit_chance: 8.0,
            attack_speed_modifier: 1.3,
            tags: vec![SkillTag::Attack, SkillTag::Physical, SkillTag::Melee],
            ..Default::default()
        },
        // Heavy Blow - slow but massive damage
        DamagePacketGenerator {
            id: "heavy_blow".to_string(),
            name: "Heavy Blow".to_string(),
            base_damages: vec![BaseDamage::new(DamageType::Physical, 15.0, 25.0)],
            weapon_effectiveness: 2.0,
            damage_effectiveness: 1.5,
            base_crit_chance: 5.0,
            attack_speed_modifier: 0.7,
            tags: vec![SkillTag::Attack, SkillTag::Physical, SkillTag::Melee],
            ..Default::default()
        },
    ]
}

/// Mana costs for skills (indexed by skill position)
pub fn skill_mana_costs() -> Vec<u32> {
    vec![10, 20, 5, 25]
}

pub struct Player {
    pub stats: StatBlock,
    pub skills: Vec<DamagePacketGenerator>,
    pub skill_mana_costs: Vec<u32>,
}

impl Player {
    pub fn new(_config: &GameConfig) -> Self {
        let mut stats = StatBlock::with_id("player");

        // Set base stats
        stats.max_life.base = 100.0;
        stats.current_life = 100.0;
        stats.max_mana.base = 50.0;
        stats.current_mana = 50.0;

        // Attributes
        stats.strength.base = 10.0;
        stats.dexterity.base = 8.0;
        stats.intelligence.base = 6.0;

        // Base weapon damage (unarmed)
        stats.weapon_physical_min = 5.0;
        stats.weapon_physical_max = 8.0;
        stats.weapon_attack_speed = 1.0;
        stats.weapon_crit_chance = 5.0;

        // Base defenses
        stats.armour.base = 10.0;
        stats.evasion.base = 10.0;

        // Base accuracy for hit calculations
        stats.accuracy.base = 100.0;

        Self {
            stats,
            skills: create_skills(),
            skill_mana_costs: skill_mana_costs(),
        }
    }

    pub fn equip(&mut self, slot: EquipmentSlot, item: Item) -> Option<Item> {
        // Save current resources before equipping (rebuild() resets everything)
        let current_life = self.stats.current_life;
        let current_mana = self.stats.current_mana;

        // Save base stats that rebuild() will wipe
        let base_max_life = self.stats.max_life.base;
        let base_max_mana = self.stats.max_mana.base;
        let base_strength = self.stats.strength.base;
        let base_dexterity = self.stats.dexterity.base;
        let base_intelligence = self.stats.intelligence.base;
        let base_armour = self.stats.armour.base;
        let base_evasion = self.stats.evasion.base;
        let base_accuracy = self.stats.accuracy.base;

        // First unequip any existing item
        let old_item = self.stats.unequip(slot);

        // Equip the new item (this calls rebuild() internally)
        self.stats.equip(slot, item);

        // Restore base stats after rebuild
        self.stats.max_life.base = base_max_life;
        self.stats.max_mana.base = base_max_mana;
        self.stats.strength.base = base_strength;
        self.stats.dexterity.base = base_dexterity;
        self.stats.intelligence.base = base_intelligence;
        self.stats.armour.base = base_armour;
        self.stats.evasion.base = base_evasion;
        self.stats.accuracy.base = base_accuracy;

        // If no weapon equipped, restore unarmed damage
        if self.stats.equipped(EquipmentSlot::MainHand).is_none() {
            self.stats.weapon_physical_min = 5.0;
            self.stats.weapon_physical_max = 8.0;
            self.stats.weapon_attack_speed = 1.0;
            self.stats.weapon_crit_chance = 5.0;
        }

        // Restore current HP/mana, clamped to new max values
        let max_life = self.stats.max_life.compute();
        let max_mana = self.stats.max_mana.compute();
        self.stats.current_life = current_life.min(max_life);
        self.stats.current_mana = current_mana.min(max_mana);

        old_item
    }

    pub fn max_life(&self) -> f64 {
        self.stats.max_life.compute()
    }

    pub fn max_mana(&self) -> f64 {
        self.stats.max_mana.compute()
    }

    pub fn get_equipped(&self, slot: EquipmentSlot) -> Option<&Item> {
        self.stats.equipped(slot)
    }
}
