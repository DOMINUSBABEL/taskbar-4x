#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EraId {
    Paleolithic = 0,     // 1. Edad de Piedra (Paleolítico)
    Neolithic = 1,       // 2. Edad del Neolítico (Agricultura)
    Chalcolithic = 2,    // 3. Edad del Cobre (Calcolítico)
    BronzeAge = 3,       // 4. Edad del Bronce (Primeros Imperios)
    IronAge = 4,         // 5. Edad del Hierro (Imperios Clásicos)
    LateAntiquity = 5,   // 6. Antigüedad Tardía (Crisis & Limes)
    EarlyMiddleAges = 6, // 7. Alta Edad Media (Feudalismo)
    LateMiddleAges = 7,  // 8. Baja Edad Media (Renacimiento Urbano)
    Renaissance = 8,     // 9. Renacimiento y Descubrimientos
    Enlightenment = 9,   // 10. Era de la Ilustración y Revoluciones
    Industrial = 10,     // 11. Era Industrial (Carbón y Vapor)
    Atomic = 11,         // 12. Era Atómica e Informática
    SolarExpansion = 12, // 13. Era de la Expansión Solar
    Interstellar = 13,   // 14. Era Interestelar (Megadiseños)
    Singularity = 14,    // 15. Era de la Singularidad y Trascendencia
}

impl EraId {
    pub const ALL: [EraId; 15] = [
        EraId::Paleolithic,
        EraId::Neolithic,
        EraId::Chalcolithic,
        EraId::BronzeAge,
        EraId::IronAge,
        EraId::LateAntiquity,
        EraId::EarlyMiddleAges,
        EraId::LateMiddleAges,
        EraId::Renaissance,
        EraId::Enlightenment,
        EraId::Industrial,
        EraId::Atomic,
        EraId::SolarExpansion,
        EraId::Interstellar,
        EraId::Singularity,
    ];

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn from_index(index: usize) -> Self {
        if index < Self::ALL.len() {
            Self::ALL[index]
        } else {
            EraId::Singularity
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            EraId::Paleolithic => "1. Edad de Piedra (Paleolítico)",
            EraId::Neolithic => "2. Edad del Neolítico (Agricultura)",
            EraId::Chalcolithic => "3. Edad del Cobre (Calcolítico)",
            EraId::BronzeAge => "4. Edad del Bronce (Primeros Imperios)",
            EraId::IronAge => "5. Edad del Hierro (Imperios Clásicos)",
            EraId::LateAntiquity => "6. Antigüedad Tardía (Crisis & Limes)",
            EraId::EarlyMiddleAges => "7. Alta Edad Media (Feudalismo)",
            EraId::LateMiddleAges => "8. Baja Edad Media (Renacimiento Urbano)",
            EraId::Renaissance => "9. Renacimiento y Descubrimientos",
            EraId::Enlightenment => "10. Era de la Ilustración",
            EraId::Industrial => "11. Era Industrial (Vapor & Carbón)",
            EraId::Atomic => "12. Era Atómica & Silicio",
            EraId::SolarExpansion => "13. Era de la Expansión Solar",
            EraId::Interstellar => "14. Era Interestelar (Megadiseños)",
            EraId::Singularity => "15. Era de la Singularidad",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            EraId::Paleolithic => "Paleolítico",
            EraId::Neolithic => "Neolítico",
            EraId::Chalcolithic => "Cobre",
            EraId::BronzeAge => "Bronce",
            EraId::IronAge => "Hierro",
            EraId::LateAntiquity => "Antigüedad Tardía",
            EraId::EarlyMiddleAges => "Alta Edad Media",
            EraId::LateMiddleAges => "Baja Edad Media",
            EraId::Renaissance => "Renacimiento",
            EraId::Enlightenment => "Ilustración",
            EraId::Industrial => "Industrial",
            EraId::Atomic => "Atómica",
            EraId::SolarExpansion => "Expansión Solar",
            EraId::Interstellar => "Interestelar",
            EraId::Singularity => "Singularidad",
        }
    }

    pub fn wonder_name(&self) -> &'static str {
        match self {
            EraId::Paleolithic => "Pinturas de Altamira",
            EraId::Neolithic => "Santuario de Stonehenge",
            EraId::Chalcolithic => "Túmulos Funerarios Reales",
            EraId::BronzeAge => "Gran Pirámide de Giza",
            EraId::IronAge => "Coliseo & Vía Apia",
            EraId::LateAntiquity => "Muros Teodosianos",
            EraId::EarlyMiddleAges => "Abadía Mayor de Cluny",
            EraId::LateMiddleAges => "Catedral de Notre Dame",
            EraId::Renaissance => "Taller de Invención de Da Vinci",
            EraId::Enlightenment => "Observatorio Real & Gran Enciclopedia",
            EraId::Industrial => "Torre Eiffel & Red Ferroviaria",
            EraId::Atomic => "Gran Colisionador CERN & Misión Apollo",
            EraId::SolarExpansion => "Ascensor Espacial Ecuatorial",
            EraId::Interstellar => "Esfera de Dyson Parcial",
            EraId::Singularity => "Mente Matrioshka Cuántica",
        }
    }

    pub fn next(&self) -> Option<EraId> {
        let idx = self.index();
        if idx + 1 < Self::ALL.len() {
            Some(Self::ALL[idx + 1])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchType {
    Military,      // Militar
    Economic,      // Económico
    Political,     // Político
    Civil,         // Civil / Urbanismo
    Ecclesiastical,// Eclesiástico / Religión
    Philosophical, // Filosófico / Ético
    Cultural,      // Cultural / Artístico
    Astronomical,  // Astronómico & Astrológico ➔ Paradigma Científico
}

impl BranchType {
    pub const ALL: [BranchType; 8] = [
        BranchType::Military,
        BranchType::Economic,
        BranchType::Political,
        BranchType::Civil,
        BranchType::Ecclesiastical,
        BranchType::Philosophical,
        BranchType::Cultural,
        BranchType::Astronomical,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            BranchType::Military => "⚔️ Militar",
            BranchType::Economic => "🌾 Económico",
            BranchType::Political => "🏛️ Político",
            BranchType::Civil => "🏘️ Civil",
            BranchType::Ecclesiastical => "🕯️ Eclesiástico",
            BranchType::Philosophical => "📜 Filosófico",
            BranchType::Cultural => "🎨 Cultural",
            BranchType::Astronomical => "🔭 Astronómico / Astrológico",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            BranchType::Military => "⚔️",
            BranchType::Economic => "🌾",
            BranchType::Political => "🏛️",
            BranchType::Civil => "🏘️",
            BranchType::Ecclesiastical => "🕯️",
            BranchType::Philosophical => "📜",
            BranchType::Cultural => "🎨",
            BranchType::Astronomical => "🔭",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DecisionChoice {
    pub name: &'static str,
    pub description: &'static str,
    pub buff_desc: &'static str,
    pub challenge_desc: &'static str,
}

#[derive(Debug, Clone)]
pub struct BranchTech {
    pub id: u32,
    pub era: EraId,
    pub branch: BranchType,
    pub title: &'static str,
    pub cost: u32, // Costo en puntos de ciencia/filosofía
    pub choice_a: DecisionChoice,
    pub choice_b: DecisionChoice,
    pub selected_choice: Option<u8>, // None = no investigada, Some(0) = Choice A, Some(1) = Choice B
}

pub fn generate_era_technologies(era: EraId) -> Vec<BranchTech> {
    let mut techs = Vec::new();
    let base_cost = ((era.index() as u32) + 1) * 30;

    for (i, branch) in BranchType::ALL.iter().enumerate() {
        let tech_id = (era.index() as u32) * 10 + (i as u32);
        let (title, choice_a, choice_b) = match (era, branch) {
            // ERA 1: Paleolítico
            (EraId::Paleolithic, BranchType::Military) => (
                "Tácticas de Caza Mayor",
                DecisionChoice {
                    name: "Lanzas con Punta de Sílex",
                    description: "Armas ofensivas ligeras para cazar fauna peligrosa.",
                    buff_desc: "+25% Ataque de patrullas",
                    challenge_desc: "-10% Defensa en el campamento",
                },
                DecisionChoice {
                    name: "Fosos y Empalizadas",
                    description: "Trampas pasivas de defensa alrededor del asentamiento.",
                    buff_desc: "+30% Defensa territorial",
                    challenge_desc: "-15% Velocidad de expedición",
                },
            ),
            (EraId::Paleolithic, BranchType::Economic) => (
                "Sustento Primitivo",
                DecisionChoice {
                    name: "Cazadores de Megafauna",
                    description: "Enfoque en mamuts y bisontes para pieles y carne densa.",
                    buff_desc: "+35% Producción de comida cárnica",
                    challenge_desc: "Consume 2 de población fija",
                },
                DecisionChoice {
                    name: "Recolectores Nómadas",
                    description: "Búsqueda amplia de raíces, frutos y frutos secos.",
                    buff_desc: "+20% Estabilidad alimentaria",
                    challenge_desc: "Menor capacidad de almacenamiento",
                },
            ),
            (EraId::Paleolithic, BranchType::Political) => (
                "Liderazgo Tribal",
                DecisionChoice {
                    name: "Caudillo de Fuerza",
                    description: "El cazador más fuerte dirige la horda con mano firme.",
                    buff_desc: "+20% Velocidad de marcha y ataque",
                    challenge_desc: "Riesgo de disputa al morir el líder",
                },
                DecisionChoice {
                    name: "Consejo de Ancianos",
                    description: "Los miembros más sabios preservan la tradición y el consenso.",
                    buff_desc: "+25% Cohesión social y menos revueltas",
                    challenge_desc: "Decisiones de guerra más lentas",
                },
            ),
            (EraId::Paleolithic, BranchType::Civil) => (
                "Refugios Rupestres",
                DecisionChoice {
                    name: "Cuevas Profundas Acondicionadas",
                    description: "Aislamiento térmico natural contra las heladas glaciales.",
                    buff_desc: "+30% Protección contra el frío",
                    challenge_desc: "Límite estricto de población urbana",
                },
                DecisionChoice {
                    name: "Chozas de Pieles y Huesos",
                    description: "Campamentos modulares fáciles de desmontar y trasladar.",
                    buff_desc: "+25% Facilidad de colonización",
                    challenge_desc: "Vulnerables a tormentas y bestias",
                },
            ),
            (EraId::Paleolithic, BranchType::Ecclesiastical) => (
                "Espiritualidad Primitiva",
                DecisionChoice {
                    name: "Animismo Totémico",
                    description: "Veneración a los espíritus de los animales y las aguas.",
                    buff_desc: "+30% Fervor espiritual en bosques",
                    challenge_desc: "Prohibición de talar árboles sagrados",
                },
                DecisionChoice {
                    name: "Culto a los Ancestros",
                    description: "Rituales funerarios para invocar la guía de los muertos.",
                    buff_desc: "+20% Moral y memoria cultural",
                    challenge_desc: "Requiere ofrendas de carne periódicas",
                },
            ),
            (EraId::Paleolithic, BranchType::Philosophical) => (
                "Mitos de Creación",
                DecisionChoice {
                    name: "Tradición Oral del Fuego",
                    description: "Historias contadas en la hoguera sobre el origen del cosmos.",
                    buff_desc: "+25% Generación de Filosofía",
                    challenge_desc: "Conocimiento vulnerable al olvido",
                },
                DecisionChoice {
                    name: "Marcas Líticas y Simbolismo",
                    description: "Primeros grabados geométricos en bastones de mando.",
                    buff_desc: "+20% Ciencia e inventario mental",
                    challenge_desc: "Menor cohesión mística tribal",
                },
            ),
            (EraId::Paleolithic, BranchType::Cultural) => (
                "Arte Rupestre",
                DecisionChoice {
                    name: "Pigmentos de Ocre y Carbón",
                    description: "Pinturas de siluetas de manos y bisontes en cavernas.",
                    buff_desc: "+40% Cultura permanente",
                    challenge_desc: "Consume minerales y grasas",
                },
                DecisionChoice {
                    name: "Adornos de Dientes y Conchas",
                    description: "Collares de prestigio para distinguir a los exploradores.",
                    buff_desc: "+25% Influencia en tribus vecinas",
                    challenge_desc: "Fomenta celos y jerarquías internas",
                },
            ),
            (EraId::Paleolithic, BranchType::Astronomical) => (
                "Observación Celeste Primitiva",
                DecisionChoice {
                    name: "Astrología Lunar y Caza",
                    description: "Seguimiento de las fases de la luna para predecir migraciones.",
                    buff_desc: "+25% Precisión en eventos de caza",
                    challenge_desc: "Superstición ante eclipses",
                },
                DecisionChoice {
                    name: "Alineación con la Estrella Polar",
                    description: "Orientación nocturna para exploraciones a larga distancia.",
                    buff_desc: "+30% Rango de visión de la frontera",
                    challenge_desc: "Inútil en noches nubladas",
                },
            ),

            // ERA 4: Edad del Bronce (Ejemplo representativo)
            (EraId::BronzeAge, BranchType::Military) => (
                "Doctrina de Guerra de Bronce",
                DecisionChoice {
                    name: "Infantería en Falange Cerrada",
                    description: "Muro impenetrable de escudos de bronce y lanzas largas.",
                    buff_desc: "+35% Defensa en frentes abiertos",
                    challenge_desc: "-20% Movilidad en terreno montañoso",
                },
                DecisionChoice {
                    name: "Carros de Guerra Tirados por Caballos",
                    description: "Fuerza de choque rápida para asaltar flancos enemigos.",
                    buff_desc: "+40% Daño de asalto",
                    challenge_desc: "Elevado consumo de bronce y forraje",
                },
            ),
            (EraId::BronzeAge, BranchType::Astronomical) => (
                "Zodíacos Babilónicos y Calendarios",
                DecisionChoice {
                    name: "Tablillas de Efemérides Planetarias",
                    description: "Registro sistemático del tránsito de Venus y Marte.",
                    buff_desc: "+35% Avance hacia el paradigma científico",
                    challenge_desc: "Los escribas exigen diezmo de grano",
                },
                DecisionChoice {
                    name: "Horóscopos Reales de Estado",
                    description: "Uso de la astrología para legitimar las guerras del monarca.",
                    buff_desc: "+30% Estabilidad y fervor bélico",
                    challenge_desc: "Riesgo de parálisis ante malos augurios",
                },
            ),

            // Fallback genérico estructurado para las demás eras
            _ => {
                let _era_name = era.short_name();
                let _branch_name = branch.name();
                (
                    "Avance Fundamental de Época",
                    DecisionChoice {
                        name: "Doctrina de Vanguardia A",
                        description: "Enfoque en desarrollo intensivo y especializado.",
                        buff_desc: "+30% Eficiencia en la rama",
                        challenge_desc: "-10% Rendimiento secundario",
                    },
                    DecisionChoice {
                        name: "Doctrina de Adaptación B",
                        description: "Enfoque en estabilidad, resiliencia y cobertura amplia.",
                        buff_desc: "+25% Cohesión y defensa global",
                        challenge_desc: "-15% Velocidad de investigación",
                    },
                )
            }
        };

        techs.push(BranchTech {
            id: tech_id,
            era,
            branch: *branch,
            title,
            cost: base_cost,
            choice_a,
            choice_b,
            selected_choice: None,
        });
    }

    techs
}
