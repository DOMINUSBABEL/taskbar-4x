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
    Desert,    // Desierto árido (Astronomía, Ruinas)
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
            BiomeType::Desert => "Desierto de Dunas",
            BiomeType::Tundra => "Tundra Boreal",
            BiomeType::Orbit => "Sector Orbital",
        }
    }
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
    pub x: f32, // Coordenadas en mapa 0.0 - 1.0
    pub y: f32,
    pub buildings: Vec<BuildingType>,
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

    // 8 Recursos Clave
    pub food: f32,
    pub materials: f32,
    pub gold: f32,
    pub faith: f32,
    pub philosophy: f32,
    pub culture: f32,
    pub science: f32,
    pub military_power: f32,

    pub food_rate: f32,
    pub materials_rate: f32,
    pub gold_rate: f32,
    pub faith_rate: f32,
    pub philosophy_rate: f32,
    pub culture_rate: f32,
    pub science_rate: f32,

    pub population: u32,
    pub stability: f32, // 0-100%

    pub era_technologies: Vec<BranchTech>,
    pub provinces: Vec<Province>,
    pub cities: Vec<City>,
    pub selected_province: usize,
    pub selected_army_id: Option<u32>,
    pub selected_city: usize,

    pub armies: Vec<Army>,
    pub next_army_id: u32,
    pub wonders: Vec<WonderProgress>,

    pub event_log: Vec<String>,
    pub singularity_dust: u32,
    pub config: GameConfig,

    // UI Interactiva RTS
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
            CivilizationChoice::Egypt => ("Persia / Valle del Nilo", 80.0, 40.0, 25.0),
            CivilizationChoice::Greece => ("Atenas / Acrópolis", 50.0, 35.0, 40.0),
            CivilizationChoice::Rome => ("Roma Imperial", 60.0, 50.0, 30.0),
            CivilizationChoice::Babylon => ("Babilonia / Eufrates", 70.0, 45.0, 30.0),
            CivilizationChoice::Dynastic => ("Chang'an / Valle Amarillo", 75.0, 50.0, 25.0),
            CivilizationChoice::Norse => ("Midgard / Fiordo", 55.0, 60.0, 15.0),
        };

        let provinces = vec![
            Province { id: 0, name: "Persia Central (Capital)".to_string(), biome: BiomeType::River, is_colonized: true, is_hostile: false, garrison_strength: 15, garrison_hp: 200.0, max_garrison_hp: 200.0, development_level: 2, x: 0.28, y: 0.48, buildings: vec![BuildingType::Hearth, BuildingType::GrainPit] },
            Province { id: 1, name: "Bosques del Norte".to_string(), biome: BiomeType::Forest, is_colonized: true, is_hostile: false, garrison_strength: 8, garrison_hp: 100.0, max_garrison_hp: 100.0, development_level: 1, x: 0.42, y: 0.32, buildings: vec![BuildingType::StoneQuarry] },
            Province { id: 2, name: "Llanuras de Media".to_string(), biome: BiomeType::Plains, is_colonized: true, is_hostile: false, garrison_strength: 10, garrison_hp: 120.0, max_garrison_hp: 120.0, development_level: 1, x: 0.55, y: 0.42, buildings: vec![] },
            Province { id: 3, name: "Cordillera Zagros".to_string(), biome: BiomeType::Mountains, is_colonized: false, is_hostile: false, garrison_strength: 0, garrison_hp: 0.0, max_garrison_hp: 0.0, development_level: 0, x: 0.68, y: 0.30, buildings: vec![] },
            Province { id: 4, name: "Costa del Golfo".to_string(), biome: BiomeType::Coast, is_colonized: false, is_hostile: false, garrison_strength: 0, garrison_hp: 0.0, max_garrison_hp: 0.0, development_level: 0, x: 0.22, y: 0.70, buildings: vec![] },
            Province { id: 5, name: "Tierras Bárbaras Hostiles".to_string(), biome: BiomeType::Tundra, is_colonized: false, is_hostile: true, garrison_strength: 35, garrison_hp: 400.0, max_garrison_hp: 400.0, development_level: 0, x: 0.48, y: 0.16, buildings: vec![] },
            Province { id: 6, name: "Meseta de Susa".to_string(), biome: BiomeType::Desert, is_colonized: false, is_hostile: false, garrison_strength: 0, garrison_hp: 0.0, max_garrison_hp: 0.0, development_level: 0, x: 0.78, y: 0.52, buildings: vec![] },
            Province { id: 7, name: "Dominio de Bactria".to_string(), biome: BiomeType::Plains, is_colonized: false, is_hostile: true, garrison_strength: 50, garrison_hp: 600.0, max_garrison_hp: 600.0, development_level: 0, x: 0.88, y: 0.28, buildings: vec![] },
        ];

        let capital_city = City {
            id: 0,
            name: civ_name.to_string(),
            province_id: 0,
            population: 20,
            buildings: vec![BuildingType::Hearth, BuildingType::GrainPit],
            current_building: None,
        };

        let initial_army = Army::new(
            1,
            "1.ª Legión Imperial".to_string(),
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

        Self {
            current_era: initial_era,
            year: 337, // Estilo Demise of Nations (337 BC)
            epoch_time: 0.0,
            food: init_food,
            materials: init_materials,
            gold: init_gold,
            faith: 15.0,
            philosophy: 10.0,
            culture: 8.0,
            science: 5.0,
            military_power: 45.0,

            food_rate: 3.5,
            materials_rate: 2.8,
            gold_rate: 1.2,
            faith_rate: 1.5,
            philosophy_rate: 0.8,
            culture_rate: 0.6,
            science_rate: 1.2,

            population: 20,
            stability: 81.0, // 81% Felicidad como en Demise of Nations

            era_technologies: era_techs,
            provinces,
            cities: vec![capital_city],
            selected_province: 0,
            selected_army_id: Some(1),
            selected_city: 0,

            armies: vec![initial_army],
            next_army_id: 2,
            wonders,

            event_log: vec![format!("¡Fundada {}! Felicidad inicial: 81%.", civ_name)],
            singularity_dust: 0,
            config,

            radial_menu_open: false,
            radial_pos: (0.0, 0.0),
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.epoch_time += dt;
        self.year = 337 + (self.epoch_time * 0.5) as u32;

        let mut f_rate = 1.5 + (self.population as f32 * 0.15);
        let mut m_rate = 1.2;
        let mut g_rate = 0.5;
        let mut fa_rate = 0.8;
        let mut ph_rate = 0.5;
        let mut cu_rate = 0.4;
        let mut sc_rate = 0.8;

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

        // Crecimiento de población
        if self.food > 120.0 {
            self.food -= 60.0;
            self.population += 1;
            if let Some(c) = self.cities.first_mut() {
                c.population += 1;
            }
        }

        // Simulación RTS de Movimiento y Combate de Ejércitos (Dune: Spice Wars style)
        for i in 0..self.armies.len() {
            if self.armies[i].is_moving {
                let speed = get_unit_definition(self.armies[i].unit_type).speed * 0.15 * dt;
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
                            // Iniciar combate RTS en la provincia enemiga
                            self.armies[i].in_combat = true;
                            self.event_log.push(format!("⚔️ ¡{} asalta {}!", self.armies[i].name, target_prov.name));
                        } else if !target_prov.is_colonized {
                            // Colonizar pacíficamente
                            target_prov.is_colonized = true;
                            self.event_log.push(format!("🚩 ¡Provincia {} anexionada al Imperio!", target_prov.name));
                        }
                    }
                } else {
                    self.armies[i].world_x += (dx / dist) * speed;
                    self.armies[i].world_y += (dy / dist) * speed;
                }
            }

            // Resolución de Combate en Tiempo Real
            if self.armies[i].in_combat {
                let target_prov_id = self.armies[i].current_province_id;
                let army_pow = self.armies[i].combat_power() as f32;
                let prov = &mut self.provinces[target_prov_id];

                // Daño recíproco
                prov.garrison_hp -= army_pow * 0.4 * dt;
                self.armies[i].hp -= (prov.garrison_strength as f32 * 3.0) * dt;

                if prov.garrison_hp <= 0.0 {
                    prov.garrison_hp = 0.0;
                    prov.is_hostile = false;
                    prov.is_colonized = true;
                    prov.garrison_strength = 10;
                    self.armies[i].in_combat = false;
                    self.event_log.push(format!("🏆 ¡Victoria! {} ha sido conquistada.", prov.name));
                } else if self.armies[i].hp <= 0.0 {
                    self.armies[i].hp = 0.0;
                    self.armies[i].in_combat = false;
                    self.event_log.push(format!("💀 ¡{} fue destruida en combate!", self.armies[i].name));
                }
            }
        }
    }

    // Ordenar movimiento RTS de ejército hacia una provincia
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

    pub fn advance_era(&mut self) -> bool {
        if let Some(next_era) = self.current_era.next() {
            self.current_era = next_era;
            self.era_technologies = generate_era_technologies(next_era);
            self.event_log.push(format!("¡La civilización avanza a: {}!", next_era.name()));
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
    fn test_8_developmental_branches() {
        let state = GameState::new();
        let branches: Vec<BranchType> = state.era_technologies.iter().map(|t| t.branch).collect();
        for b in BranchType::ALL.iter() {
            assert!(branches.contains(b), "Missing branch: {:?}", b);
        }
    }

    #[test]
    fn test_rts_army_movement_order() {
        let mut state = GameState::new();
        state.order_army_to_province(1, 1);
        assert!(state.armies[0].is_moving);
        assert_eq!(state.armies[0].target_province_id, Some(1));

        // Simular movimiento con varios ticks
        for _ in 0..50 {
            state.tick(1.0);
        }

        assert_eq!(state.armies[0].current_province_id, 1);
    }
}
