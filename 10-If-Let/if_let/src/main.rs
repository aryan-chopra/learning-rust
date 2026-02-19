use std::fmt;

enum MapMode {
    Normal,
    Night,
    ElectromagneticStorm
}

impl fmt::Display for MapMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self{
            MapMode::ElectromagneticStorm => format!("ElectromagneticStorm"),
            MapMode::Normal => format!("Normal"),
            MapMode::Night => format!("Night")
        };
        
        write!(f, "{}", repr)
    }
}

impl MapMode {
    fn can_view_at_distance(&self, distance_from_raider: u32) -> bool {
        match self {
            MapMode::Night => distance_from_raider <= 100,
            _ => true
        }
    }
}

/**
* For the sake of our example, let's assume only DamBattlegrounds can have a mode
*/
enum Map {
    DamBattlegrounds(MapMode),
    StellaMontis
}

fn is_dark_message(map: &Map) -> Option<String> {
    let mode = if let Map::DamBattlegrounds(mode) = map {
        mode
    } else {
        return None;
    };

    if mode.can_view_at_distance(150) {
        Some(format!("Ehh, it's pretty dark outside in {mode} mode"))
    } else {
        Some(format!("It's pretty bright in {mode} mode"))
    }
}

#[allow(dead_code)]
fn is_dark_message_v2(map: &Map) -> Option<String> {
    let Map::DamBattlegrounds(mode) = map else {
        return None
    };
    
    if mode.can_view_at_distance(150) {
        Some(format!("Ehh, it's pretty dark outside in {mode} mode"))
    } else {
        Some(format!("It's pretty bright in {mode} mode"))
    }
}

fn main() {
    let stella = Map::StellaMontis;
    let dam = Map::DamBattlegrounds(MapMode::Normal);
    let dam_dark = Map::DamBattlegrounds(MapMode::Night);
    let dam_electric = Map::DamBattlegrounds(MapMode::ElectromagneticStorm);

    let stella_message = is_dark_message(&stella);
    let dam_message = is_dark_message(&dam);
    let dam_dark_message = is_dark_message(&dam_dark);
    let dam_electric_message = is_dark_message(&dam_electric);

    if let Some(stella_msg) = stella_message {
        println!("Stella message: {stella_msg}");
    }
    if let Some(dam_msg) = dam_message {
        println!("Dam message: {dam_msg}");
    }
    if let Some(dam_dark_msg) = dam_dark_message{
        println!("Dam message: {dam_dark_msg}");
    }
    if let Some(dam_elec_msg) = dam_electric_message{
        println!("Dam message: {dam_elec_msg}");
    }
}
