use super::eras::EraId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildingType {
    // Era 1-3
    Hearth,             // Fogata Comunal (+Comida, +Fe)
    GrainPit,           // Foso de Grano / Silo (+Almacén comida)
    StoneQuarry,        // Cantera Lítica (+Materiales)
    ShamanHut,          // Choza del Chamán (+Fe, +Cultura)
    MegalithCircle,     // Círculo Megalítico (+Astronomía, +Filosofía)

    // Era 4-6
    BronzeForge,        // Forja de Bronce (+Poder militar, +Herramientas)
    MudbrickGranary,    // Granero de Adobe (+Comida x2)
    Forum,              // Foro / Ágora (+Política, +Oro)
    Aqueduct,           // Acueducto (+Población máxima, +Salud)
    Barracks,           // Cuartel de Infantería (+Reclutamiento)
    PtolemaicObservatory,// Observatorio Ptolemaico (+Astronomía/Astrología)

    // Era 7-8
    Watermill,          // Molino Hidráulico (+Producción x3)
    Monastery,          // Monasterio (+Fe, +Filosofía, +Copistas)
    Castle,             // Castillo Feudal (+Defensa, +Control provincial)
    Guildhall,          // Casa de Gremios (+Oro comercial, +Bienes)

    // Era 9-11
    PrintingHouse,      // Imprenta (+Ciencia x2, +Cultura)
    ScientificAcademy,  // Real Academia de Ciencias (+Ciencia x3)
    SteamMill,          // Fábrica de Vapor (+Producción masiva)
    RailwayStation,     // Estación Ferroviaria (+Logística x4)

    // Era 12-15
    NuclearReactor,     // Reactor de Fisión (+Energía pura)
    SupercomputerLab,   // Laboratorio de Silicio (+Cálculo)
    Spaceport,          // Puerto Espacial (+Despliegue orbital)
    DysonCollectorNode, // Colector Solar Estelar (+Energía x10)
    QuantumMatrioshka,  // Nodo Cuántico de Trascendencia (+Omnisciencia)
}

#[derive(Debug, Clone)]
pub struct BuildingDef {
    pub building_type: BuildingType,
    pub name: &'static str,
    pub era_required: EraId,
    pub food_cost: u32,
    pub material_cost: u32,
    pub gold_cost: u32,
    pub production_bonus_desc: &'static str,
}

pub fn get_building_definition(b_type: BuildingType) -> BuildingDef {
    match b_type {
        BuildingType::Hearth => BuildingDef {
            building_type: b_type,
            name: "Hogaza Comunal",
            era_required: EraId::Paleolithic,
            food_cost: 10,
            material_cost: 15,
            gold_cost: 0,
            production_bonus_desc: "+2 Comida/s, +1 Fe/s",
        },
        BuildingType::GrainPit => BuildingDef {
            building_type: b_type,
            name: "Foso de Almacenamiento",
            era_required: EraId::Paleolithic,
            food_cost: 5,
            material_cost: 25,
            gold_cost: 0,
            production_bonus_desc: "+100 Capacidad de Comida",
        },
        BuildingType::StoneQuarry => BuildingDef {
            building_type: b_type,
            name: "Cantera de Sílex y Roca",
            era_required: EraId::Neolithic,
            food_cost: 15,
            material_cost: 30,
            gold_cost: 0,
            production_bonus_desc: "+3 Materiales/s",
        },
        BuildingType::ShamanHut => BuildingDef {
            building_type: b_type,
            name: "Choza del Chamán",
            era_required: EraId::Paleolithic,
            food_cost: 10,
            material_cost: 20,
            gold_cost: 0,
            production_bonus_desc: "+2 Fe/s, +1 Cultura/s",
        },
        BuildingType::MegalithCircle => BuildingDef {
            building_type: b_type,
            name: "Círculo Megalítico Solar",
            era_required: EraId::Neolithic,
            food_cost: 25,
            material_cost: 60,
            gold_cost: 0,
            production_bonus_desc: "+3 Astronomía/s, +2 Filosofía/s",
        },
        BuildingType::BronzeForge => BuildingDef {
            building_type: b_type,
            name: "Forja de Bronce",
            era_required: EraId::BronzeAge,
            food_cost: 30,
            material_cost: 80,
            gold_cost: 20,
            production_bonus_desc: "+5 Materiales/s, +3 Poder Militar/s",
        },
        BuildingType::MudbrickGranary => BuildingDef {
            building_type: b_type,
            name: "Granero de Adobe",
            era_required: EraId::BronzeAge,
            food_cost: 20,
            material_cost: 50,
            gold_cost: 10,
            production_bonus_desc: "+6 Comida/s, +300 Capacidad",
        },
        BuildingType::Forum => BuildingDef {
            building_type: b_type,
            name: "Foro Clásico",
            era_required: EraId::IronAge,
            food_cost: 40,
            material_cost: 120,
            gold_cost: 50,
            production_bonus_desc: "+4 Oro/s, +3 Política/s, +10% Cohesión",
        },
        BuildingType::Aqueduct => BuildingDef {
            building_type: b_type,
            name: "Gran Acueducto",
            era_required: EraId::IronAge,
            food_cost: 50,
            material_cost: 150,
            gold_cost: 60,
            production_bonus_desc: "+5 Crecimiento Urbano, +200 Población Máx",
        },
        BuildingType::Barracks => BuildingDef {
            building_type: b_type,
            name: "Cuartel Legionario",
            era_required: EraId::IronAge,
            food_cost: 40,
            material_cost: 100,
            gold_cost: 40,
            production_bonus_desc: "+8 Poder Militar/s, Recluta tropas x2",
        },
        BuildingType::PtolemaicObservatory => BuildingDef {
            building_type: b_type,
            name: "Observatorio Astrológico",
            era_required: EraId::IronAge,
            food_cost: 30,
            material_cost: 90,
            gold_cost: 50,
            production_bonus_desc: "+6 Astronomía/s, +3 Filosofía/s",
        },
        BuildingType::Watermill => BuildingDef {
            building_type: b_type,
            name: "Molino Hidráulico",
            era_required: EraId::EarlyMiddleAges,
            food_cost: 60,
            material_cost: 180,
            gold_cost: 80,
            production_bonus_desc: "+15 Producción/s sin mano de obra",
        },
        BuildingType::Monastery => BuildingDef {
            building_type: b_type,
            name: "Monasterio de Copistas",
            era_required: EraId::EarlyMiddleAges,
            food_cost: 50,
            material_cost: 140,
            gold_cost: 60,
            production_bonus_desc: "+8 Fe/s, +5 Filosofía/s, +4 Cultura/s",
        },
        BuildingType::Castle => BuildingDef {
            building_type: b_type,
            name: "Castillo Feudal",
            era_required: EraId::EarlyMiddleAges,
            food_cost: 80,
            material_cost: 250,
            gold_cost: 120,
            production_bonus_desc: "+50 Defensa Provincial, +12 Militar/s",
        },
        BuildingType::Guildhall => BuildingDef {
            building_type: b_type,
            name: "Lonja de Gremios",
            era_required: EraId::LateMiddleAges,
            food_cost: 70,
            material_cost: 200,
            gold_cost: 150,
            production_bonus_desc: "+18 Oro/s, +8 Materiales/s",
        },
        BuildingType::PrintingHouse => BuildingDef {
            building_type: b_type,
            name: "Imprenta de Tipos Móviles",
            era_required: EraId::Renaissance,
            food_cost: 90,
            material_cost: 280,
            gold_cost: 200,
            production_bonus_desc: "+20 Ciencia/s, +12 Cultura/s",
        },
        BuildingType::ScientificAcademy => BuildingDef {
            building_type: b_type,
            name: "Real Academia de Ciencias",
            era_required: EraId::Enlightenment,
            food_cost: 120,
            material_cost: 350,
            gold_cost: 300,
            production_bonus_desc: "+35 Ciencia/s, +20 Filosofía/s",
        },
        BuildingType::SteamMill => BuildingDef {
            building_type: b_type,
            name: "Fábrica a Vapor y Fundición",
            era_required: EraId::Industrial,
            food_cost: 150,
            material_cost: 500,
            gold_cost: 400,
            production_bonus_desc: "+60 Materiales/s, +30 Maquinaria/s",
        },
        BuildingType::RailwayStation => BuildingDef {
            building_type: b_type,
            name: "Terminal de Ferrocarril",
            era_required: EraId::Industrial,
            food_cost: 180,
            material_cost: 600,
            gold_cost: 500,
            production_bonus_desc: "+100% Velocidad de Transporte Logístico",
        },
        BuildingType::NuclearReactor => BuildingDef {
            building_type: b_type,
            name: "Reactor Nuclear de Fisión",
            era_required: EraId::Atomic,
            food_cost: 250,
            material_cost: 900,
            gold_cost: 800,
            production_bonus_desc: "+150 Energía/s, +80 Ciencia/s",
        },
        BuildingType::SupercomputerLab => BuildingDef {
            building_type: b_type,
            name: "Centro de Supercómputo",
            era_required: EraId::Atomic,
            food_cost: 200,
            material_cost: 800,
            gold_cost: 900,
            production_bonus_desc: "+200 Ciencia/s, +100 Cómputo/s",
        },
        BuildingType::Spaceport => BuildingDef {
            building_type: b_type,
            name: "Puerto de Lanzamiento Orbital",
            era_required: EraId::SolarExpansion,
            food_cost: 350,
            material_cost: 1500,
            gold_cost: 1500,
            production_bonus_desc: "+Despliegue aéreo y satelital masivo",
        },
        BuildingType::DysonCollectorNode => BuildingDef {
            building_type: b_type,
            name: "Nodo Colector Estelar",
            era_required: EraId::Interstellar,
            food_cost: 500,
            material_cost: 3000,
            gold_cost: 3000,
            production_bonus_desc: "+1000 Energía Solar/s",
        },
        BuildingType::QuantumMatrioshka => BuildingDef {
            building_type: b_type,
            name: "Matriz Cuántica Planetaria",
            era_required: EraId::Singularity,
            food_cost: 1000,
            material_cost: 8000,
            gold_cost: 8000,
            production_bonus_desc: "+Trascendencia y Cómputo Infinito",
        },
    }
}
