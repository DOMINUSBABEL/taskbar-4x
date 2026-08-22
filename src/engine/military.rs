use super::eras::EraId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    // Eras 1-3
    PaleoHunter,         // Cazador con Lanza (Piedra)
    NeolithicSlinger,    // Hondero Neolítico
    CopperSpearman,      // Lancero del Cobre

    // Eras 4-6
    BronzeChariot,       // Carro de Guerra de Bronce
    BronzePhalanx,       // Falange Imperial
    RomanLegionary,      // Legionario Clásico
    Cataphract,          // Catafracta Blindado

    // Eras 7-8
    FeudalKnight,        // Caballero con Cota y Lanza
    GothicCrossbowman,   // Ballestero de Gremios

    // Eras 9-11
    RenaissanceCaravel,  // Carabela Armada
    LineMusketeer,       // Fusilero de Línea Ilustrado
    SteamIronclad,       // Acorazado a Vapor
    FieldArtillery,      // Batería de Artillería

    // Eras 12-15
    BattleTank,          // Tanque de Batalla Blindado
    SupersonicJet,       // Caza Supersónico
    OrbitalFrigate,      // Fragata Orbital
    DysonCruiser,        // Crucero de Antimateria
    QuantumAvatar,       // Avatar de Conciencia Cuántica
}

#[derive(Debug, Clone)]
pub struct UnitDef {
    pub unit_type: UnitType,
    pub name: &'static str,
    pub era_required: EraId,
    pub attack_power: u32,
    pub defense_power: u32,
    pub speed: f32, // Velocidad de marcha en nodos/segundo
    pub food_upkeep: u32,
    pub gold_upkeep: u32,
    pub production_cost: u32,
}

pub fn get_unit_definition(u_type: UnitType) -> UnitDef {
    match u_type {
        UnitType::PaleoHunter => UnitDef {
            unit_type: u_type,
            name: "Cazador Tribal con Lanza",
            era_required: EraId::Paleolithic,
            attack_power: 5,
            defense_power: 3,
            speed: 1.0,
            food_upkeep: 1,
            gold_upkeep: 0,
            production_cost: 15,
        },
        UnitType::NeolithicSlinger => UnitDef {
            unit_type: u_type,
            name: "Hondero Neolítico",
            era_required: EraId::Neolithic,
            attack_power: 8,
            defense_power: 4,
            speed: 1.2,
            food_upkeep: 1,
            gold_upkeep: 0,
            production_cost: 20,
        },
        UnitType::CopperSpearman => UnitDef {
            unit_type: u_type,
            name: "Lancero del Calcolítico",
            era_required: EraId::Chalcolithic,
            attack_power: 12,
            defense_power: 10,
            speed: 1.0,
            food_upkeep: 2,
            gold_upkeep: 0,
            production_cost: 35,
        },
        UnitType::BronzeChariot => UnitDef {
            unit_type: u_type,
            name: "Carro de Guerra de Bronce",
            era_required: EraId::BronzeAge,
            attack_power: 24,
            defense_power: 12,
            speed: 2.2,
            food_upkeep: 3,
            gold_upkeep: 2,
            production_cost: 60,
        },
        UnitType::BronzePhalanx => UnitDef {
            unit_type: u_type,
            name: "Falange de Bronce",
            era_required: EraId::BronzeAge,
            attack_power: 18,
            defense_power: 25,
            speed: 0.8,
            food_upkeep: 3,
            gold_upkeep: 1,
            production_cost: 55,
        },
        UnitType::RomanLegionary => UnitDef {
            unit_type: u_type,
            name: "Legionario del Hierro",
            era_required: EraId::IronAge,
            attack_power: 32,
            defense_power: 30,
            speed: 1.2,
            food_upkeep: 4,
            gold_upkeep: 3,
            production_cost: 80,
        },
        UnitType::Cataphract => UnitDef {
            unit_type: u_type,
            name: "Catafracta de Élite",
            era_required: EraId::LateAntiquity,
            attack_power: 45,
            defense_power: 38,
            speed: 1.8,
            food_upkeep: 5,
            gold_upkeep: 5,
            production_cost: 110,
        },
        UnitType::FeudalKnight => UnitDef {
            unit_type: u_type,
            name: "Caballero Medieval de Cota",
            era_required: EraId::EarlyMiddleAges,
            attack_power: 58,
            defense_power: 50,
            speed: 1.6,
            food_upkeep: 6,
            gold_upkeep: 6,
            production_cost: 140,
        },
        UnitType::GothicCrossbowman => UnitDef {
            unit_type: u_type,
            name: "Ballestero de Gremios",
            era_required: EraId::LateMiddleAges,
            attack_power: 50,
            defense_power: 35,
            speed: 1.0,
            food_upkeep: 4,
            gold_upkeep: 4,
            production_cost: 120,
        },
        UnitType::RenaissanceCaravel => UnitDef {
            unit_type: u_type,
            name: "Carabela de Cañones",
            era_required: EraId::Renaissance,
            attack_power: 80,
            defense_power: 65,
            speed: 2.0,
            food_upkeep: 6,
            gold_upkeep: 10,
            production_cost: 200,
        },
        UnitType::LineMusketeer => UnitDef {
            unit_type: u_type,
            name: "Fusilero de Línea",
            era_required: EraId::Enlightenment,
            attack_power: 95,
            defense_power: 70,
            speed: 1.3,
            food_upkeep: 6,
            gold_upkeep: 8,
            production_cost: 220,
        },
        UnitType::SteamIronclad => UnitDef {
            unit_type: u_type,
            name: "Acorazado Blindado a Vapor",
            era_required: EraId::Industrial,
            attack_power: 160,
            defense_power: 180,
            speed: 1.8,
            food_upkeep: 8,
            gold_upkeep: 20,
            production_cost: 450,
        },
        UnitType::FieldArtillery => UnitDef {
            unit_type: u_type,
            name: "Artillería Pesada Industrial",
            era_required: EraId::Industrial,
            attack_power: 200,
            defense_power: 80,
            speed: 0.9,
            food_upkeep: 8,
            gold_upkeep: 18,
            production_cost: 400,
        },
        UnitType::BattleTank => UnitDef {
            unit_type: u_type,
            name: "División Acorazada de Tanques",
            era_required: EraId::Atomic,
            attack_power: 320,
            defense_power: 300,
            speed: 2.5,
            food_upkeep: 10,
            gold_upkeep: 35,
            production_cost: 750,
        },
        UnitType::SupersonicJet => UnitDef {
            unit_type: u_type,
            name: "Escuadrón Caza Supersónico",
            era_required: EraId::Atomic,
            attack_power: 450,
            defense_power: 150,
            speed: 5.0,
            food_upkeep: 10,
            gold_upkeep: 50,
            production_cost: 900,
        },
        UnitType::OrbitalFrigate => UnitDef {
            unit_type: u_type,
            name: "Fragata de Asalto Orbital",
            era_required: EraId::SolarExpansion,
            attack_power: 700,
            defense_power: 600,
            speed: 4.0,
            food_upkeep: 15,
            gold_upkeep: 100,
            production_cost: 1600,
        },
        UnitType::DysonCruiser => UnitDef {
            unit_type: u_type,
            name: "Crucero Estelar de Antimateria",
            era_required: EraId::Interstellar,
            attack_power: 1800,
            defense_power: 1500,
            speed: 8.0,
            food_upkeep: 25,
            gold_upkeep: 300,
            production_cost: 4000,
        },
        UnitType::QuantumAvatar => UnitDef {
            unit_type: u_type,
            name: "Avatar Cuántico Trascendente",
            era_required: EraId::Singularity,
            attack_power: 5000,
            defense_power: 5000,
            speed: 10.0,
            food_upkeep: 0,
            gold_upkeep: 0,
            production_cost: 10000,
        },
    }
}

#[derive(Debug, Clone)]
pub struct Army {
    pub id: u32,
    pub name: String,
    pub unit_type: UnitType,
    pub count: u32,
    pub province_id: usize,
    pub target_province_id: Option<usize>,
    pub march_progress: f32, // 0.0 a 1.0
}

impl Army {
    pub fn combat_power(&self) -> u32 {
        let def = get_unit_definition(self.unit_type);
        (def.attack_power + def.defense_power) * self.count
    }
}
