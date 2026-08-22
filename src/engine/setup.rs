#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CivilizationChoice {
    Egypt,       // Pueblos del Nilo (Egipto)
    Greece,      // Ciudades-Estado del Egeo (Grecia)
    Rome,        // Legiones del Tíber (Roma/Imperium)
    Babylon,     // Creciente Fértil (Babilonia)
    Dynastic,    // Dinastías del Dragón
    Norse,       // Clanes Nórdicos
}

impl CivilizationChoice {
    pub const ALL: [CivilizationChoice; 6] = [
        CivilizationChoice::Egypt,
        CivilizationChoice::Greece,
        CivilizationChoice::Rome,
        CivilizationChoice::Babylon,
        CivilizationChoice::Dynastic,
        CivilizationChoice::Norse,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            CivilizationChoice::Egypt => "🏺 Pueblos del Nilo (Egipto)",
            CivilizationChoice::Greece => "🏛️ Ciudades del Egeo (Grecia)",
            CivilizationChoice::Rome => "🦅 Legiones del Tíber (Roma)",
            CivilizationChoice::Babylon => "📜 Creciente Fértil (Babilonia)",
            CivilizationChoice::Dynastic => "🐉 Dinastías del Este",
            CivilizationChoice::Norse => "⚔️ Clanes del Norte (Nórdicos)",
        }
    }

    pub fn bonus_desc(&self) -> &'static str {
        match self {
            CivilizationChoice::Egypt => "+30% Comida en ríos, +25% Velocidad de Maravillas.",
            CivilizationChoice::Greece => "+35% Generación de Filosofía y Astronomía, +20% Comercio.",
            CivilizationChoice::Rome => "+40% Disciplina militar, +30% Calzadas y Construcción.",
            CivilizationChoice::Babylon => "+40% Código de Leyes, Agricultura fluvial y Zodíacos.",
            CivilizationChoice::Dynastic => "+30% Crecimiento de Población y +25% Invenciones.",
            CivilizationChoice::Norse => "+35% Saqueo en expediciones y moral de combate.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaderTrait {
    Strategist,   // El Estratega Militar
    Philosopher,  // El Filósofo
    Merchant,     // El Gran Mercader
    HighPriest,   // El Sumo Sacerdote
    Astronomer,   // El Astrónomo / Científico
    MasterBuilder,// El Maestro Constructor
}

impl LeaderTrait {
    pub const ALL: [LeaderTrait; 6] = [
        LeaderTrait::Strategist,
        LeaderTrait::Philosopher,
        LeaderTrait::Merchant,
        LeaderTrait::HighPriest,
        LeaderTrait::Astronomer,
        LeaderTrait::MasterBuilder,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            LeaderTrait::Strategist => "⚔️ El Estratega Bélico",
            LeaderTrait::Philosopher => "📜 El Sabio Filósofo",
            LeaderTrait::Merchant => "🪙 El Gran Mercader",
            LeaderTrait::HighPriest => "🕯️ El Sumo Sacerdote",
            LeaderTrait::Astronomer => "🔭 El Astrónomo Pionero",
            LeaderTrait::MasterBuilder => "🔨 El Maestro Constructor",
        }
    }

    pub fn bonus_desc(&self) -> &'static str {
        match self {
            LeaderTrait::Strategist => "+25% Fuerza de combate, -15% Desgaste militar.",
            LeaderTrait::Philosopher => "+30% Generación de Ciencia y Puntos de Filosofía.",
            LeaderTrait::Merchant => "+35% Ingreso de Oro y -20% Coste en mercados.",
            LeaderTrait::HighPriest => "+40% Fe y Cohesión social contra revueltas.",
            LeaderTrait::Astronomer => "+35% Velocidad de investigación astronómica y satelital.",
            LeaderTrait::MasterBuilder => "+30% Velocidad de construcción de edificios urbanos.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameSpeed {
    Blitz,   // Rápido para pruebas (5x)
    Normal,  // Equilibrado (2x)
    Epic,    // Idle Histórico Realista (1x)
}

impl GameSpeed {
    pub fn name(&self) -> &'static str {
        match self {
            GameSpeed::Blitz => "⚡ Rápido (Blitz - 5x)",
            GameSpeed::Normal => "⏱️ Normal (4X Equilibrado - 2x)",
            GameSpeed::Epic => "⏳ Épico (Simulación Idle - 1x)",
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            GameSpeed::Blitz => 5.0,
            GameSpeed::Normal => 2.0,
            GameSpeed::Epic => 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub civ: CivilizationChoice,
    pub leader: LeaderTrait,
    pub speed: GameSpeed,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            civ: CivilizationChoice::Rome,
            leader: LeaderTrait::Strategist,
            speed: GameSpeed::Normal,
        }
    }
}
