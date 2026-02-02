use super::enemy::Enemy;
use super::player::Player;
use stat_core::{CombatResult, DamagePacketGenerator, DamageType};

/// Combat system using stat_core's damage calculation
pub struct Combat;

impl Combat {
    /// Calculate and resolve a basic attack from player to enemy
    pub fn player_basic_attack(player: &Player, enemy: &mut Enemy) -> CombatResult {
        let basic_attack = DamagePacketGenerator::basic_attack();
        let packet = player.stats.attack(&basic_attack);
        let (new_stats, result) = enemy.stats.receive_damage(&packet);
        enemy.stats = new_stats;
        result
    }

    /// Calculate and resolve a skill attack from player to enemy
    pub fn player_skill_attack(
        player: &Player,
        enemy: &mut Enemy,
        skill_index: usize,
    ) -> CombatResult {
        let skill = &player.skills[skill_index];
        let packet = player.stats.attack(skill);
        let (new_stats, result) = enemy.stats.receive_damage(&packet);
        enemy.stats = new_stats;
        result
    }

    /// Calculate and resolve an enemy attack against player
    pub fn enemy_attack(enemy: &Enemy, player: &mut Player) -> CombatResult {
        let basic_attack = DamagePacketGenerator::basic_attack();
        let packet = enemy.stats.attack(&basic_attack);
        let (new_stats, result) = player.stats.receive_damage(&packet);
        player.stats = new_stats;
        result
    }
}

/// Format a CombatResult for the combat log
pub fn format_combat_result(
    result: &CombatResult,
    attacker_name: &str,
    is_crit: bool,
) -> Vec<String> {
    let mut lines = Vec::new();

    // Main damage line
    let crit_text = if is_crit { " (CRITICAL!)" } else { "" };
    lines.push(format!(
        "{} deals {} damage!{}",
        attacker_name, result.total_damage as u32, crit_text
    ));

    // Damage breakdown by type
    let breakdown: Vec<String> = result
        .damage_taken
        .iter()
        .filter(|d| d.final_amount > 0.0)
        .map(|d| format!("{:.0} {}", d.final_amount, format_damage_type(d.damage_type)))
        .collect();

    if !breakdown.is_empty() {
        lines.push(format!("  ├─ {}", breakdown.join(" + ")));
    }

    // Mitigation info
    let mut mitigations = Vec::new();
    if result.damage_reduced_by_armour > 0.0 {
        mitigations.push(format!("{:.0} blocked", result.damage_reduced_by_armour));
    }
    if result.damage_reduced_by_resists > 0.0 {
        mitigations.push(format!("{:.0} resisted", result.damage_reduced_by_resists));
    }
    if result.damage_prevented_by_evasion > 0.0 {
        mitigations.push(format!("{:.0} evaded", result.damage_prevented_by_evasion));
    }
    if result.damage_blocked_by_es > 0.0 {
        mitigations.push(format!("{:.0} absorbed by ES", result.damage_blocked_by_es));
    }

    if !mitigations.is_empty() {
        lines.push(format!("  └─ {}", mitigations.join(", ")));
    }

    // Status effects applied
    for effect in &result.effects_applied {
        lines.push(format!("  └─ Applied: {}", effect.name));
    }

    lines
}

fn format_damage_type(dt: DamageType) -> &'static str {
    match dt {
        DamageType::Physical => "phys",
        DamageType::Fire => "fire",
        DamageType::Cold => "cold",
        DamageType::Lightning => "light",
        DamageType::Chaos => "chaos",
    }
}
