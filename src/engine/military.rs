#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    // Eras Primitivas
    PaleoHunter,
    Spearman,
    Chariot,
    
    // Eras Clásicas y Medievales (Imperium / Cossacks)
    Legionary,
    PhalanxHoplite,
    Knight,
    Longbowman,
    
    // Eras Renacimiento e Ilustración (Cossacks 3 style)
    Musketeer,
    Pikeman,
    HussarCavalry,
    FieldCannon,
    
    // Eras Modernas y Futuras (Empire Earth / Dune)
    RiflemanInfantry,
    SteamIronclad,
    BattleTank,
    FighterJet,
    DreadnoughtShip,
    OrbitalCruiser,
    QuantumMech,
}

#[derive(Debug, Clone)]
pub struct UnitDef {
    pub name: &'static str,
    pub food_cost: u32,
    pub material_cost: u32,
    pub attack: u32,
    pub defense: u32,
    pub max_hp: f32,
    pub speed: f32,
    pub era_index: usize,
}

pub fn get_unit_definition(unit: UnitType) -> UnitDef {
    match unit {
        UnitType::PaleoHunter => UnitDef { name: "Cazador con Lanza", food_cost: 15, material_cost: 10, attack: 4, defense: 2, max_hp: 50.0, speed: 1.2, era_index: 0 },
        UnitType::Spearman => UnitDef { name: "Lancero Tribal", food_cost: 20, material_cost: 15, attack: 6, defense: 5, max_hp: 70.0, speed: 1.0, era_index: 1 },
        UnitType::Chariot => UnitDef { name: "Carro de Guerra de Bronce", food_cost: 40, material_cost: 35, attack: 14, defense: 8, max_hp: 120.0, speed: 1.8, era_index: 3 },
        UnitType::Legionary => UnitDef { name: "Legionario Romano", food_cost: 35, material_cost: 40, attack: 18, defense: 16, max_hp: 150.0, speed: 1.1, era_index: 4 },
        UnitType::PhalanxHoplite => UnitDef { name: "Falange Hoplita", food_cost: 30, material_cost: 45, attack: 16, defense: 20, max_hp: 160.0, speed: 0.9, era_index: 4 },
        UnitType::Knight => UnitDef { name: "Caballero Feudal Montado", food_cost: 60, material_cost: 70, attack: 28, defense: 22, max_hp: 220.0, speed: 1.6, era_index: 7 },
        UnitType::Longbowman => UnitDef { name: "Arquero de Arco Largo", food_cost: 25, material_cost: 35, attack: 22, defense: 10, max_hp: 90.0, speed: 1.2, era_index: 7 },
        UnitType::Musketeer => UnitDef { name: "Mosquetero de Línea", food_cost: 40, material_cost: 60, attack: 38, defense: 18, max_hp: 140.0, speed: 1.1, era_index: 9 },
        UnitType::Pikeman => UnitDef { name: "Piquero de Formación", food_cost: 30, material_cost: 50, attack: 24, defense: 30, max_hp: 180.0, speed: 1.0, era_index: 9 },
        UnitType::HussarCavalry => UnitDef { name: "Húsar Alado", food_cost: 65, material_cost: 80, attack: 45, defense: 25, max_hp: 240.0, speed: 2.0, era_index: 9 },
        UnitType::FieldCannon => UnitDef { name: "Cañón de Asedio", food_cost: 30, material_cost: 120, attack: 65, defense: 15, max_hp: 160.0, speed: 0.7, era_index: 10 },
        UnitType::RiflemanInfantry => UnitDef { name: "Infantería con Fusil", food_cost: 50, material_cost: 80, attack: 55, defense: 35, max_hp: 200.0, speed: 1.3, era_index: 10 },
        UnitType::SteamIronclad => UnitDef { name: "Acorazado de Vapor", food_cost: 80, material_cost: 250, attack: 110, defense: 85, max_hp: 500.0, speed: 1.2, era_index: 10 },
        UnitType::BattleTank => UnitDef { name: "Tanque Blindado", food_cost: 80, material_cost: 220, attack: 130, defense: 95, max_hp: 450.0, speed: 1.5, era_index: 11 },
        UnitType::FighterJet => UnitDef { name: "Caza Supersónico", food_cost: 60, material_cost: 300, attack: 160, defense: 60, max_hp: 280.0, speed: 3.5, era_index: 11 },
        UnitType::DreadnoughtShip => UnitDef { name: "Dreadnought Nuclear", food_cost: 100, material_cost: 500, attack: 280, defense: 200, max_hp: 900.0, speed: 1.4, era_index: 12 },
        UnitType::OrbitalCruiser => UnitDef { name: "Crucero Orbital", food_cost: 200, material_cost: 1200, attack: 600, defense: 450, max_hp: 2000.0, speed: 2.5, era_index: 13 },
        UnitType::QuantumMech => UnitDef { name: "Titán Cuántico", food_cost: 500, material_cost: 3000, attack: 1800, defense: 1400, max_hp: 6000.0, speed: 3.0, era_index: 14 },
    }
}

#[derive(Debug, Clone)]
pub struct Army {
    pub id: u32,
    pub name: String,
    pub unit_type: UnitType,
    pub count: u32,
    pub hp: f32,
    pub max_hp: f32,
    pub current_province_id: usize,
    pub target_province_id: Option<usize>,
    pub world_x: f32,
    pub world_y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub is_moving: bool,
    pub in_combat: bool,
}

impl Army {
    pub fn new(id: u32, name: String, unit_type: UnitType, count: u32, province_id: usize, x: f32, y: f32) -> Self {
        let def = get_unit_definition(unit_type);
        let total_hp = def.max_hp * count as f32;
        Self {
            id,
            name,
            unit_type,
            count,
            hp: total_hp,
            max_hp: total_hp,
            current_province_id: province_id,
            target_province_id: None,
            world_x: x,
            world_y: y,
            target_x: x,
            target_y: y,
            is_moving: false,
            in_combat: false,
        }
    }

    pub fn combat_power(&self) -> u32 {
        let def = get_unit_definition(self.unit_type);
        let hp_ratio = (self.hp / self.max_hp.max(1.0)).clamp(0.1, 1.0);
        ((def.attack + def.defense) as f32 * self.count as f32 * hp_ratio) as u32
    }
}
