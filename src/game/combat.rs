use super::enemy::Enemy;
use super::player::Player;

pub struct Combat;

impl Combat {
    pub fn calculate_basic_attack(player: &Player) -> u32 {
        player.calculate_attack()
    }

    pub fn calculate_skill_damage(player: &Player, skill_index: usize) -> u32 {
        let base_attack = player.calculate_attack();
        let skill = &player.skills[skill_index];
        (base_attack as f32 * skill.damage_multiplier) as u32
    }

    pub fn calculate_enemy_attack(enemy: &Enemy) -> u32 {
        enemy.attack
    }
}
