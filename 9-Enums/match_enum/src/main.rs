#[derive(Debug)]
#[allow(dead_code)]
enum Mode {
    Normal,
    Night,
}

#[allow(dead_code)]
#[derive(Debug)]
enum DamBattlegroundMode {
    BaseMode(Mode),
    ElectromagneticStorm,
}

#[allow(dead_code)]
#[derive(Debug)]
enum StellaMontisMode{
    BaseMode(Mode),
}

#[allow(dead_code)]
#[derive(Debug)]
enum BurriedCityMode {
    BaseMode(Mode),
}

#[allow(dead_code)]
#[derive(Debug)]
enum BlueGateMode {
    BaseMode(Mode),
    LockedGate
}

#[allow(dead_code)]
#[derive(Debug)]
enum SpaceportMode {
    BaseMode(Mode),
    ElectromagneticStorm,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Map {
    StellaMontis(StellaMontisMode),
    DamBattlegrounds(DamBattlegroundMode),
    BurriedCity(BurriedCityMode),
    BlueGate(BlueGateMode),
    Spaceport(SpaceportMode),
}


#[allow(dead_code)]
fn difficulty(map: Map) -> u8 {
    match map {
        // Multi-line match
        Map::StellaMontis(_) => {
            println!("Ded T_T");
            100
        },
        Map::DamBattlegrounds(mode) => {
            println!("Mode: {mode:?} | Does it really matter though....");
            20
        },
        Map::BurriedCity(_) => 50,
        Map::BlueGate(_) => 40,
        other_map => {
            println!("Unhandled difficulty for the map: {other_map:?}");
            0
        }
    }
}

#[allow(dead_code)]
fn difficulty_with_option(map: Option<Map>) -> Option<u8> {
    match map {
        None => None,
        Some(map) => Some(difficulty(map))
    }
}

fn main() {
    let map = Some(Map::DamBattlegrounds(DamBattlegroundMode::ElectromagneticStorm));
    
    println!("Map: {:?}", map);
    
    let difficulty = difficulty_with_option(map);
    
    match difficulty {
        None => println!("Map wasn't provided!"),
        Some(diff) => println!("Difficulty is {diff}")
    }
    
    let map = None;
    
    println!("Map: {:?}", map);
    
    let difficulty = difficulty_with_option(map);
    
    match difficulty {
        None => println!("Map wasn't provided!"),
        Some(diff) => println!("Difficulty is {diff}")
    }
}
