use super::eras::{EraId, BranchTech, generate_era_technologies};
use super::buildings::{BuildingType, get_building_definition};
use super::military::{Army, UnitType, get_unit_definition};

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
            BiomeType::Plains => "Llanuras",
            BiomeType::Forest => "Bosques",
            BiomeType::Mountains => "Montañas",
            BiomeType::River => "Valle Fluvial",
            BiomeType::Coast => "Costa Marítima",
            BiomeType::Desert => "Desierto",
            BiomeType::Tundra => "Tundra",
            BiomeType::Orbit => "Órbita Planetaria",
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
    pub development_level: u32,
    pub x: f32, // Coordenadas normalizadas 0.0 - 1.0 para el mapa 2.5D
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct City {
    pub id: usize,
    pub name: String,
    pub province_id: usize,
    pub population: u32,
    pub buildings: Vec<BuildingType>,
    pub current_building: Option<(BuildingType, f32, f32)>, // (Type, progress, total_cost)
}

#[derive(Debug, Clone)]
pub struct WonderProgress {
    pub era: EraId,
    pub name: &'static str,
    pub progress: f32, // 0.0 a 100.0%
    pub is_completed: bool,
}

#[derive(Debug, Clone)]
pub struct HistoricalCrisis {
    pub title: &'static str,
    pub description: &'static str,
    pub severity: u32, // 1-5
    pub time_remaining: f32,
    pub option_a: &'static str,
    pub option_b: &'static str,
}

#[derive(Debug, Clone)]
pub struct GameState {
    // Época y Tiempo
    pub current_era: EraId,
    pub year: u32,
    pub epoch_time: f32,

    // Recursos Principales
    pub food: f32,
    pub materials: f32,
    pub gold: f32,
    pub faith: f32,
    pub philosophy: f32,
    pub culture: f32,
    pub science: f32,
    pub military_power: f32,

    // Tasas por segundo (para UI)
    pub food_rate: f32,
    pub materials_rate: f32,
    pub gold_rate: f32,
    pub faith_rate: f32,
    pub philosophy_rate: f32,
    pub culture_rate: f32,
    pub science_rate: f32,

    // Métricas Sociales
    pub population: u32,
    pub stability: f32, // 0.0 a 100.0%

    // Árbol de Tecnologías (por Era)
    pub era_technologies: Vec<BranchTech>,

    // Provincias y Territorio
    pub provinces: Vec<Province>,
    pub cities: Vec<City>,
    pub selected_province: usize,
    pub selected_city: usize,

    // Ejércitos y Frentes
    pub armies: Vec<Army>,
    pub next_army_id: u32,

    // Maravillas
    pub wonders: Vec<WonderProgress>,

    // Expedición 1D (Barra)
    pub expedition_progress: f32, // 0.0 a 1.0
    pub current_frontier_node: usize,

    // Crisis Activa
    pub active_crisis: Option<HistoricalCrisis>,

    // Mensajes y Bitácora de Eventos
    pub event_log: Vec<String>,

    // Meta-Progreso (Singularidad)
    pub singularity_dust: u32,
}

impl GameState {
    pub fn new() -> Self {
        let initial_era = EraId::Paleolithic;
        let era_techs = generate_era_technologies(initial_era);

        let provinces = vec![
            Province { id: 0, name: "Valle del Fuego Central".to_string(), biome: BiomeType::River, is_colonized: true, is_hostile: false, garrison_strength: 10, development_level: 1, x: 0.25, y: 0.45 },
            Province { id: 1, name: "Bosques de Sílex".to_string(), biome: BiomeType::Forest, is_colonized: true, is_hostile: false, garrison_strength: 5, development_level: 1, x: 0.38, y: 0.35 },
            Province { id: 2, name: "Llanura de Mamuts".to_string(), biome: BiomeType::Plains, is_colonized: false, is_hostile: false, garrison_strength: 0, development_level: 0, x: 0.52, y: 0.40 },
            Province { id: 3, name: "Cantera de Roca Negra".to_string(), biome: BiomeType::Mountains, is_colonized: false, is_hostile: false, garrison_strength: 0, development_level: 0, x: 0.65, y: 0.30 },
            Province { id: 4, name: "Costa de Conchas".to_string(), biome: BiomeType::Coast, is_colonized: false, is_hostile: false, garrison_strength: 0, development_level: 0, x: 0.20, y: 0.65 },
            Province { id: 5, name: "Tierras Bárbaras del Norte".to_string(), biome: BiomeType::Tundra, is_colonized: false, is_hostile: true, garrison_strength: 25, development_level: 0, x: 0.45, y: 0.18 },
            Province { id: 6, name: "Meseta de los Monolitos".to_string(), biome: BiomeType::Desert, is_colonized: false, is_hostile: false, garrison_strength: 0, development_level: 0, x: 0.75, y: 0.50 },
            Province { id: 7, name: "Cráter Estelar".to_string(), biome: BiomeType::Orbit, is_colonized: false, is_hostile: false, garrison_strength: 0, development_level: 0, x: 0.85, y: 0.25 },
        ];

        let capital_city = City {
            id: 0,
            name: "Asentamiento Primigenio".to_string(),
            province_id: 0,
            population: 15,
            buildings: vec![BuildingType::Hearth],
            current_building: None,
        };

        let initial_army = Army {
            id: 1,
            name: "Patrulla de Exploradores".to_string(),
            unit_type: UnitType::PaleoHunter,
            count: 3,
            province_id: 0,
            target_province_id: None,
            march_progress: 0.0,
        };

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
            year: 1,
            epoch_time: 0.0,
            food: 40.0,
            materials: 30.0,
            gold: 10.0,
            faith: 5.0,
            philosophy: 2.0,
            culture: 1.0,
            science: 0.0,
            military_power: 15.0,

            food_rate: 2.5,
            materials_rate: 2.0,
            gold_rate: 0.5,
            faith_rate: 1.0,
            philosophy_rate: 0.5,
            culture_rate: 0.3,
            science_rate: 0.8,

            population: 15,
            stability: 95.0,

            era_technologies: era_techs,
            provinces,
            cities: vec![capital_city],
            selected_province: 0,
            selected_city: 0,

            armies: vec![initial_army],
            next_army_id: 2,

            wonders,
            expedition_progress: 0.0,
            current_frontier_node: 0,
            active_crisis: None,
            event_log: vec!["El Clan Primigenio enciende la primera hoguera.".to_string()],
            singularity_dust: 0,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.epoch_time += dt;
        self.year += (dt * 1.5) as u32;

        // Calcular tasas de producción
        let mut f_rate = 1.0 + (self.population as f32 * 0.15);
        let mut m_rate = 1.0;
        let mut g_rate = 0.2;
        let mut fa_rate = 0.5;
        let mut ph_rate = 0.3;
        let mut cu_rate = 0.2;
        let mut sc_rate = 0.5;

        // Sumar producción de edificios de todas las ciudades
        for city in &mut self.cities {
            for b in &city.buildings {
                match b {
                    BuildingType::Hearth => { f_rate += 2.0; fa_rate += 1.0; }
                    BuildingType::GrainPit => { f_rate += 1.0; }
                    BuildingType::StoneQuarry => { m_rate += 3.0; }
                    BuildingType::ShamanHut => { fa_rate += 2.0; cu_rate += 1.0; }
                    BuildingType::MegalithCircle => { sc_rate += 3.0; ph_rate += 2.0; }
                    BuildingType::BronzeForge => { m_rate += 5.0; }
                    BuildingType::MudbrickGranary => { f_rate += 6.0; }
                    BuildingType::Forum => { g_rate += 4.0; }
                    BuildingType::Aqueduct => { f_rate += 5.0; }
                    BuildingType::Watermill => { m_rate += 15.0; }
                    BuildingType::Monastery => { fa_rate += 8.0; ph_rate += 5.0; cu_rate += 4.0; }
                    BuildingType::Castle => { m_rate += 8.0; }
                    BuildingType::Guildhall => { g_rate += 18.0; m_rate += 8.0; }
                    BuildingType::PrintingHouse => { sc_rate += 20.0; cu_rate += 12.0; }
                    BuildingType::ScientificAcademy => { sc_rate += 35.0; ph_rate += 20.0; }
                    BuildingType::SteamMill => { m_rate += 60.0; }
                    BuildingType::NuclearReactor => { sc_rate += 80.0; m_rate += 100.0; }
                    BuildingType::SupercomputerLab => { sc_rate += 200.0; }
                    BuildingType::Spaceport => { m_rate += 300.0; sc_rate += 300.0; }
                    BuildingType::DysonCollectorNode => { sc_rate += 1000.0; m_rate += 1000.0; }
                    BuildingType::QuantumMatrioshka => { sc_rate += 5000.0; ph_rate += 5000.0; }
                    _ => {}
                }
            }

            // Procesar construcción activa de la ciudad
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

        // Acumular recursos
        self.food += f_rate * dt;
        self.materials += m_rate * dt;
        self.gold += g_rate * dt;
        self.faith += fa_rate * dt;
        self.philosophy += ph_rate * dt;
        self.culture += cu_rate * dt;
        self.science += sc_rate * dt;

        // Crecimiento demográfico
        if self.food > 100.0 {
            self.food -= 50.0;
            self.population += 1;
            if let Some(c) = self.cities.first_mut() {
                c.population += 1;
            }
        }

        // Progreso de expedición 1D
        self.expedition_progress += dt * 0.05;
        if self.expedition_progress >= 1.0 {
            self.expedition_progress = 0.0;
            self.current_frontier_node = (self.current_frontier_node + 1) % self.provinces.len();
            let prov_name = self.provinces[self.current_frontier_node].name.clone();
            self.event_log.push(format!("Expedición alcanzó: {}", prov_name));
            if self.event_log.len() > 10 {
                self.event_log.remove(0);
            }
        }

        // Progreso de Maravilla de la Era actual
        let era_idx = self.current_era.index();
        if let Some(wonder) = self.wonders.get_mut(era_idx) {
            if !wonder.is_completed && self.materials > 50.0 {
                wonder.progress += dt * 1.5;
                if wonder.progress >= 100.0 {
                    wonder.progress = 100.0;
                    wonder.is_completed = true;
                    self.event_log.push(format!("¡Gran Maravilla erigida: {}!", wonder.name));
                }
            }
        }

        // Actualizar ejércitos en marcha
        for army in &mut self.armies {
            if let Some(target) = army.target_province_id {
                let speed = get_unit_definition(army.unit_type).speed;
                army.march_progress += dt * speed * 0.2;
                if army.march_progress >= 1.0 {
                    army.province_id = target;
                    army.target_province_id = None;
                    army.march_progress = 0.0;
                }
            }
        }
    }

    pub fn advance_era(&mut self) -> bool {
        if let Some(next_era) = self.current_era.next() {
            self.current_era = next_era;
            self.era_technologies = generate_era_technologies(next_era);
            self.event_log.push(format!("¡La civilización entra en: {}!", next_era.name()));
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

        // Test singluarity is the final era
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
    fn test_building_construction_and_yields() {
        let mut state = GameState::new();
        state.materials = 500.0;
        state.food = 500.0;

        let success = state.start_building_construction(0, BuildingType::StoneQuarry);
        assert!(success);

        // Tick simulation until completed
        for _ in 0..100 {
            state.tick(1.0);
        }

        assert!(state.cities[0].buildings.contains(&BuildingType::StoneQuarry));
        assert!(state.materials_rate > 2.0);
    }

    #[test]
    fn test_army_combat_power_calculation() {
        let state = GameState::new();
        assert!(!state.armies.is_empty());
        let power = state.armies[0].combat_power();
        assert!(power > 0);
    }
}
