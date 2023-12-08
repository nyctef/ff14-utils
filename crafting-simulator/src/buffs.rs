use crate::model::PlayerStats;
use ff14_data::model::Food;

pub fn apply_buff_hq(player: &PlayerStats, buff: &Food) -> PlayerStats {
    let mut craftsmanship = player.craftsmanship;
    let mut control = player.control;
    let mut cp = player.cp;

    for bonus in &buff.bonuses {
        match bonus.name.as_str() {
            "CP" => cp += bonus.max_hq as u16,
            "Control" => control += bonus.max_hq as u16,
            "Craftsmanship" => craftsmanship += bonus.max_hq as u16,
            _ => panic!("Unknown bonus type: {}", bonus.name)
        };
    }

    PlayerStats { craftsmanship, control, cp, ..*player }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presets::Presets as p;

    #[test]
    fn when_player_stats_high_enough_buff_amount_reaches_cap() {
        let player = PlayerStats::level_90(9000, 9000, 9000);
        let buff = p::jhinga_biryani();

        assert_eq!(
            apply_buff_hq(&player, buff),
            PlayerStats::level_90(9000, 9000 + 90, 9000 + 86)
        )
    }
}
