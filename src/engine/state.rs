use super::eras::{EraId, BranchTech, generate_era_technologies};
use super::buildings::{BuildingType, get_building_definition};
use super::military::{Army, UnitType, get_unit_definition};
use super::setup::{GameConfig, CivilizationChoice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiomeType {
    Plains,    // Llanuras fértiles (Comida)
    Forest,    // Bosques antiguos (Madera, Fe)
    Mountains, // Montañas rocosas (Materiales, Minerales)
    River,     // Valle fluvial (Comida x2, Comercio)
    Coast,     // Costa marítima (Comercio, Exploración)
    Desert,    // Desierto árido (Astronomía, Especias)
    Tundra,    // Tundra glacial (Supervivencia)
    Orbit,     // Órbita planetaria / Espacio (Ciencia, Cómputo)
}

impl BiomeType {
    pub fn name(&self) -> &'static str {
        match self {
            BiomeType::Plains => "Llanuras Fértiles",
            BiomeType::Forest => "Bosques Ancestrales",
            BiomeType::Mountains => "Cordillera Mineral",
            BiomeType::River => "Valle Fluvial",
            BiomeType::Coast => "Costa Marítima",
            BiomeType::Desert => "Gran Desierto de Dunas",
            BiomeType::Tundra => "Tundra Boreal",
            BiomeType::Orbit => "Sector Orbital",
        }
    }
}

// Distritos Regionales estilo Dune: Spice Wars (D4X)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionalDistrict {
    WaterCatchment,   // Trampa de Viento / Acueducto (+15 Comida)
    PlastacreteMine,  // Cantera de Materiales (+20 Producción)
    TradingPost,      // Centro de Comercio & Bazar (+18 Oro)
    MilitaryPost,     // Torreón de Guarnición (+30 Defensa, +10 Suministro)
    ResearchOutpost,  // Laboratorio & Observatorio (+25 Ciencia)
}

impl RegionalDistrict {
    pub fn name(&self) -> &'static str {
        match self {
            RegionalDistrict::WaterCatchment => "💧 Captador Fluvial",
            RegionalDistrict::PlastacreteMine => "⛏️ Mina de Minerales",
            RegionalDistrict::TradingPost => "🪙 Bazar de Comercio",
            RegionalDistrict::MilitaryPost => "🛡️ Torreón Defensivo",
            RegionalDistrict::ResearchOutpost => "🔬 Puesto Científico",
        }
    }

    pub fn cost(&self) -> (u32, u32) {
        match self {
            RegionalDistrict::WaterCatchment => (20, 30),
            RegionalDistrict::PlastacreteMine => (15, 45),
            RegionalDistrict::TradingPost => (25, 40),
            RegionalDistrict::MilitaryPost => (30, 60),
            RegionalDistrict::ResearchOutpost => (20, 70),
        }
    }
}

// Resoluciones del Consejo Imperial (Landsraad estilo D4X)
#[derive(Debug, Clone)]
pub struct ImperialDecree {
    pub title: &'static str,
    pub description: &'static str,
    pub votes_favor: u32,
    pub votes_against: u32,
    pub is_enacted: bool,
}

#[derive(Debug, Clone)]
pub struct Province {
    pub id: usize,
    pub name: String,
    pub biome: BiomeType,
    pub is_colonized: bool,
    pub is_hostile: bool,
    pub garrison_strength: u32,
    pub garrison_hp: f32,
    pub max_garrison_hp: f32,
    pub development_level: u32,
    pub x: f32,
    pub y: f32,
    pub buildings: Vec<BuildingType>,
    pub districts: Vec<RegionalDistrict>,
    pub max_districts: usize,
}

#[derive(Debug, Clone)]
pub struct City {
    pub id: usize,
    pub name: String,
    pub province_id: usize,
    pub population: u32,
    pub buildings: Vec<BuildingType>,
    pub current_building: Option<(BuildingType, f32, f32)>,
}

#[derive(Debug, Clone)]
pub struct WonderProgress {
    pub era: EraId,
    pub name: &'static str,
    pub progress: f32,
    pub is_completed: bool,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub current_era: EraId,
    pub year: u32,
    pub epoch_time: f32,

    // Recursos Clave D4X
    pub food: f32,        // Comida / Agua
    pub materials: f32,   // Materiales / Plastacreto
    pub gold: f32,        // Oro / Solari
    pub faith: f32,       // Fe / Cohesión
    pub philosophy: f32,  // Filosofía / Autoridad
    pub culture: f32,     // Cultura / Hegemonía
    pub science: f32,     // Ciencia / Conocimiento
    pub military_power: f32,

    pub food_rate: f32,
    pub materials_rate: f32,
    pub gold_rate: f32,
    pub faith_rate: f32,
    pub philosophy_rate: f32,
    pub culture_rate: f32,
    pub science_rate: f32,

    pub population: u32,
    pub stability: f32,     // 0-100%
    pub hegemony_points: u32, // Puntos de Hegemonía D4X (Victoria a los 10,000)

    pub era_technologies: Vec<BranchTech>,
    pub provinces: Vec<Province>,
    pub cities: Vec<City>,
    pub selected_province: usize,
    pub selected_army_id: Option<u32>,
    pub selected_city: usize,

    pub armies: Vec<Army>,
    pub next_army_id: u32,
    pub wonders: Vec<WonderProgress>,
    pub active_decrees: Vec<ImperialDecree>,

    pub event_log: Vec<String>,
    pub singularity_dust: u32,
    pub config: GameConfig,

    pub radial_menu_open: bool,
    pub radial_pos: (f32, f32),
}

impl GameState {
    pub fn new() -> Self {
        Self::new_with_config(GameConfig::default())
    }

    pub fn new_with_config(config: GameConfig) -> Self {
        let initial_era = EraId::Paleolithic;
        let era_techs = generate_era_technologies(initial_era);

        let (civ_name, init_food, init_materials, init_gold) = match config.civ {
            CivilizationChoice::Egypt => ("Hegemonía del Nilo", 90.0, 45.0, 30.0),
            CivilizationChoice::Greece => ("Liga del Egeo", 60.0, 40.0, 45.0),
            CivilizationChoice::Rome => ("Imperio Romano (Atreides Style)", 70.0, 55.0, 35.0),
            CivilizationChoice::Babylon => ("Dinastía de Eufrates", 80.0, 50.0, 35.0),
            CivilizationChoice::Dynastic => ("Imperio Celestial (Harkonnen Style)", 85.0, 60.0, 30.0),
            CivilizationChoice::Norse => ("Clanes del Norte (Fremen Style)", 65.0, 65.0, 20.0),
        };

        let provinces = vec![
            Province { id: 0, name: "Región Capital Central".to_string(), biome: BiomeType::River, is_colonized: true, is_hostile: false, garrison_strength: 20, garrison_hp: 250.0, max_garrison_hp: 250.0, development_level: 3, x: 0.28, y: 0.48, buildings: vec![BuildingType::Hearth, BuildingType::GrainPit], districts: vec![RegionalDistrict::WaterCatchment, RegionalDistrict::TradingPost], max_districts: 4 },
            Province { id: 1, name: "Valle Verde del Norte".to_string(), biome: BiomeType::Forest, is_colonized: true, is_hostile: false, garrison_strength: 10, garrison_hp: 120.0, max_garrison_hp: 120.0, development_level: 1, x: 0.42, y: 0.32, buildings: vec![BuildingType::StoneQuarry], districts: vec![RegionalDistrict::PlastacreteMine], max_districts: 3 },
            Province { id: 2, name: "Llanuras de la Cuenca".to_string(), biome: BiomeType::Plains, is_colonized: true, is_hostile: false, garrison_strength: 12, garrison_hp: 140.0, max_garrison_hp: 140.0, development_level: 1, x: 0.55, y: 0.42, buildings: vec![], districts: vec![], max_districts: 3 },
            Province { id: 3, name: "Cordillera Mineral".to_string(), biome: BiomeType::Mountains, is_colonized: false, is_hostile: false, garrison_strength: 0, garrison_hp: 0.0, max_garrison_hp: 0.0, development_level: 0, x: 0.68, y: 0.30, buildings: vec![], districts: vec![], max_districts: 3 },
            Province { id: 4, name: "Sector Costero de Arrecifes".to_string(), biome: BiomeType::Coast, is_colonized: false, is_hostile: false, garrison_strength: 0, garrison_hp: 0.0, max_garrison_hp: 0.0, development_level: 0, x: 0.22, y: 0.70, buildings: vec![], districts: vec![], max_districts: 2 },
            Province { id: 5, name: "Sietch Rebelde Hostil".to_string(), biome: BiomeType::Tundra, is_colonized: false, is_hostile: true, garrison_strength: 40, garrison_hp: 450.0, max_garrison_hp: 450.0, development_level: 0, x: 0.48, y: 0.16, buildings: vec![], districts: vec![], max_districts: 3 },
            Province { id: 6, name: "Yacimiento del Gran Desierto".to_string(), biome: BiomeType::Desert, is_colonized: false, is_hostile: false, garrison_strength: 0, garrison_hp: 0.0, max_garrison_hp: 0.0, development_level: 0, x: 0.78, y: 0.52, buildings: vec![], districts: vec![], max_districts: 4 },
            Province { id: 7, name: "Fortaleza Fronteriza Enemiga".to_string(), biome: BiomeType::Plains, is_colonized: false, is_hostile: true, garrison_strength: 60, garrison_hp: 700.0, max_garrison_hp: 700.0, development_level: 0, x: 0.88, y: 0.28, buildings: vec![], districts: vec![], max_districts: 3 },
        ];

        let capital_city = City {
            id: 0,
            name: civ_name.to_string(),
            province_id: 0,
            population: 25,
            buildings: vec![BuildingType::Hearth, BuildingType::GrainPit],
            current_building: None,
        };

        let initial_army = Army::new(
            1,
            "1.ª División de Asalto".to_string(),
            UnitType::Musketeer,
            36,
            0,
            0.28,
            0.48,
        );

        let mut wonders = Vec::new();
        for era in EraId::ALL.iter() {
            wonders.push(WonderProgress {
                era: *era,
                name: era.wonder_name(),
                progress: 0.0,
                is_completed: false,
            });
        }

        let decrees = vec![
            ImperialDecree { title: "Carta de Comercio Monopolístico", description: "+25% Producción de Oro, -10% Cohesión", votes_favor: 240, votes_against: 110, is_enacted: true },
            ImperialDecree { title: "Leva de Reclutamiento Forzoso", description: "+35% Velocidad de Reclutamiento, -5% Felicidad", votes_favor: 180, votes_against: 190, is_enacted: false },
            ImperialDecree { title: "Subsidio de Irrigación e Hidráulica", description: "+30% Rendimiento de Comida en ríos", votes_favor: 310, votes_against: 40, is_enacted: true },
        ];

        Self {
            current_era: initial_era,
            year: 337,
            epoch_time: 0.0,
            food: init_food,
            materials: init_materials,
            gold: init_gold,
            faith: 20.0,
            philosophy: 15.0,
            culture: 10.0,
            science: 8.0,
            military_power: 50.0,

            food_rate: 4.5,
            materials_rate: 3.8,
            gold_rate: 2.0,
            faith_rate: 1.8,
            philosophy_rate: 1.2,
            culture_rate: 1.0,
            science_rate: 1.8,

            population: 25,
            stability: 85.0,
            hegemony_points: 1250,

            era_technologies: era_techs,
            provinces,
            cities: vec![capital_city],
            selected_province: 0,
            selected_army_id: Some(1),
            selected_city: 0,

            armies: vec![initial_army],
            next_army_id: 2,
            wonders,
            active_decrees: decrees,

            event_log: vec![format!("¡Fundada {}! Consejo Imperial Landsraad activo.", civ_name)],
            singularity_dust: 0,
            config,

            radial_menu_open: false,
            radial_pos: (0.0, 0.0),
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.epoch_time += dt;
        self.year = 337 + (self.epoch_time * 0.5) as u32;

        let mut f_rate = 2.0 + (self.population as f32 * 0.15);
        let mut m_rate = 1.8;
        let mut g_rate = 0.8;
        let mut fa_rate = 1.0;
        let mut ph_rate = 0.8;
        let mut cu_rate = 0.6;
        let mut sc_rate = 1.0;

        // Sumar producción de Distritos Regionales (D4X style)
        for prov in &self.provinces {
            if prov.is_colonized {
                for dist in &prov.districts {
                    match dist {
                        RegionalDistrict::WaterCatchment => f_rate += 4.0,
                        RegionalDistrict::PlastacreteMine => m_rate += 5.0,
                        RegionalDistrict::TradingPost => g_rate += 4.5,
                        RegionalDistrict::MilitaryPost => m_rate += 2.0,
                        RegionalDistrict::ResearchOutpost => sc_rate += 6.0,
                    }
                }
            }
        }

        // Sumar producción de edificios de ciudades
        for city in &mut self.cities {
            for b in &city.buildings {
                match b {
                    BuildingType::Hearth => { f_rate += 2.0; fa_rate += 1.0; }
                    BuildingType::GrainPit => { f_rate += 3.0; }
                    BuildingType::StoneQuarry => { m_rate += 4.0; }
                    BuildingType::ShamanHut => { fa_rate += 3.0; cu_rate += 1.5; }
                    BuildingType::MegalithCircle => { sc_rate += 4.0; ph_rate += 2.5; }
                    BuildingType::BronzeForge => { m_rate += 6.0; }
                    BuildingType::Forum => { g_rate += 5.0; }
                    BuildingType::Watermill => { m_rate += 15.0; f_rate += 5.0; }
                    _ => {}
                }
            }

            if let Some((b_type, ref mut prog, total)) = city.current_building {
                *prog += dt * f32::max(m_rate * 0.5, 1.0);
                if *prog >= total {
                    city.buildings.push(b_type);
                    city.current_building = None;
                }
            }
        }

        self.food_rate = f_rate;
        self.materials_rate = m_rate;
        self.gold_rate = g_rate;
        self.faith_rate = fa_rate;
        self.philosophy_rate = ph_rate;
        self.culture_rate = cu_rate;
        self.science_rate = sc_rate;

        self.food += f_rate * dt;
        self.materials += m_rate * dt;
        self.gold += g_rate * dt;
        self.faith += fa_rate * dt;
        self.philosophy += ph_rate * dt;
        self.culture += cu_rate * dt;
        self.science += sc_rate * dt;

        // Puntos de Hegemonía pasivos
        self.hegemony_points += (dt * 2.0) as u32;

        // Crecimiento de población
        if self.food > 120.0 {
            self.food -= 60.0;
            self.population += 1;
            if let Some(c) = self.cities.first_mut() {
                c.population += 1;
            }
        }

        // Movimiento y Combate RTS de Ejércitos
        for i in 0..self.armies.len() {
            if self.armies[i].is_moving {
                let speed = get_unit_definition(self.armies[i].unit_type).speed * 0.18 * dt;
                let dx = self.armies[i].target_x - self.armies[i].world_x;
                let dy = self.armies[i].target_y - self.armies[i].world_y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < speed || dist < 0.01 {
                    self.armies[i].world_x = self.armies[i].target_x;
                    self.armies[i].world_y = self.armies[i].target_y;
                    self.armies[i].is_moving = false;

                    if let Some(target_prov_id) = self.armies[i].target_province_id {
                        self.armies[i].current_province_id = target_prov_id;
                        let target_prov = &mut self.provinces[target_prov_id];

                        if target_prov.is_hostile {
                            self.armies[i].in_combat = true;
                            self.event_log.push(format!("⚔️ ¡{} asalta {}!", self.armies[i].name, target_prov.name));
                        } else if !target_prov.is_colonized {
                            target_prov.is_colonized = true;
                            self.hegemony_points += 500;
                            self.event_log.push(format!("🚩 ¡Región {} anexionada! (+500 Hegemonía)", target_prov.name));
                        }
                    }
                } else {
                    self.armies[i].world_x += (dx / dist) * speed;
                    self.armies[i].world_y += (dy / dist) * speed;
                }
            }

            if self.armies[i].in_combat {
                let target_prov_id = self.armies[i].current_province_id;
                let army_pow = self.armies[i].combat_power() as f32;
                let prov = &mut self.provinces[target_prov_id];

                prov.garrison_hp -= army_pow * 0.5 * dt;
                self.armies[i].hp -= (prov.garrison_strength as f32 * 2.5) * dt;

                if prov.garrison_hp <= 0.0 {
                    prov.garrison_hp = 0.0;
                    prov.is_hostile = false;
                    prov.is_colonized = true;
                    prov.garrison_strength = 15;
                    self.armies[i].in_combat = false;
                    self.hegemony_points += 1000;
                    self.event_log.push(format!("🏆 ¡Victoria! {} conquistada. (+1000 Hegemonía)", prov.name));
                } else if self.armies[i].hp <= 0.0 {
                    self.armies[i].hp = 0.0;
                    self.armies[i].in_combat = false;
                    self.event_log.push(format!("💀 ¡{} fue destruida en combate!", self.armies[i].name));
                }
            }
        }
    }

    pub fn order_army_to_province(&mut self, army_id: u32, target_prov_id: usize) {
        if let Some(army) = self.armies.iter_mut().find(|a| a.id == army_id) {
            if target_prov_id < self.provinces.len() {
                let prov = &self.provinces[target_prov_id];
                army.target_x = prov.x;
                army.target_y = prov.y;
                army.target_province_id = Some(target_prov_id);
                army.is_moving = true;
                army.in_combat = false;
                self.event_log.push(format!("🚩 {} marcha hacia {}.", army.name, prov.name));
            }
        }
    }

    pub fn build_district_in_province(&mut self, province_id: usize, district: RegionalDistrict) -> bool {
        if province_id < self.provinces.len() {
            let (mat_cost, gold_cost) = district.cost();
            if self.materials >= mat_cost as f32 && self.gold >= gold_cost as f32 {
                let prov = &mut self.provinces[province_id];
                if prov.is_colonized && prov.districts.len() < prov.max_districts {
                    self.materials -= mat_cost as f32;
                    self.gold -= gold_cost as f32;
                    prov.districts.push(district);
                    self.hegemony_points += 150;
                    self.event_log.push(format!("🏗️ Construido {} en {}.", district.name(), prov.name));
                    return true;
                }
            }
        }
        false
    }

    pub fn advance_era(&mut self) -> bool {
        if let Some(next_era) = self.current_era.next() {
            self.current_era = next_era;
            self.era_technologies = generate_era_technologies(next_era);
            self.hegemony_points += 2000;
            self.event_log.push(format!("¡La civilización avanza a: {}! (+2000 Hegemonía)", next_era.name()));
            true
        } else {
            false
        }
    }

    pub fn select_technology_choice(&mut self, tech_index: usize, choice_index: u8) {
        if let Some(tech) = self.era_technologies.get_mut(tech_index) {
            tech.selected_choice = Some(choice_index);
            self.event_log.push(format!("Adoptada doctrina: {}", if choice_index == 0 { tech.choice_a.name } else { tech.choice_b.name }));
        }
    }

    pub fn start_building_construction(&mut self, city_index: usize, building: BuildingType) -> bool {
        let def = get_building_definition(building);
        if self.materials >= def.material_cost as f32 && self.food >= def.food_cost as f32 {
            self.materials -= def.material_cost as f32;
            self.food -= def.food_cost as f32;
            if let Some(city) = self.cities.get_mut(city_index) {
                if city.current_building.is_none() {
                    let total_cost = (def.material_cost + def.food_cost) as f32;
                    city.current_building = Some((building, 0.0, total_cost));
                    self.event_log.push(format!("Iniciada construcción de {} en {}", def.name, city.name));
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::*;

    #[test]
    fn test_all_15_eras_progression() {
        let mut state = GameState::new();
        assert_eq!(state.current_era, EraId::Paleolithic);

        for era in EraId::ALL.iter().skip(1) {
            let advanced = state.advance_era();
            assert!(advanced);
            assert_eq!(state.current_era, *era);
            assert_eq!(state.era_technologies.len(), 8);
        }

        assert_eq!(state.current_era, EraId::Singularity);
        assert!(!state.advance_era());
    }

    #[test]
    fn test_d4x_district_construction() {
        let mut state = GameState::new();
        state.materials = 500.0;
        state.gold = 500.0;

        let success = state.build_district_in_province(0, RegionalDistrict::PlastacreteMine);
        assert!(success);
        assert!(state.provinces[0].districts.contains(&RegionalDistrict::PlastacreteMine));
    }

    #[test]
    fn test_rts_army_movement_order() {
        let mut state = GameState::new();
        state.order_army_to_province(1, 1);
        assert!(state.armies[0].is_moving);
        assert_eq!(state.armies[0].target_province_id, Some(1));

        for _ in 0..50 {
            state.tick(1.0);
        }

        assert_eq!(state.armies[0].current_province_id, 1);
    }
}
