use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug, Clone)]
pub struct GameState {
    pub year: u32,
    pub population: u32,
    pub grain: u32,
    pub land: u32,
    pub land_price: u32,
    pub harvest_yield: u32,
    pub grain_eaten_by_rats: u32,
    pub new_citizens: u32,
    pub deaths_starvation: u32,
    pub deaths_plague: u32,
    pub total_deaths: u32,
    pub grain_harvested: u32,
    pub acres_planted: u32,
    pub rng: StdRng,
    pub history: Vec<YearSummary>,
    pub current_phase: GamePhase,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GamePhase {
    Splash,
    Instructions,
    LandTransaction,
    Planting,
    Feeding,
    YearEnd,
    GameOver,
}

#[derive(Debug, Clone)]
pub struct YearSummary {
    pub year: u32,
    pub population: u32,
    pub grain: u32,
    pub land: u32,
    pub starved: u32,
    pub new_citizens: u32,
    pub plague_deaths: u32,
    pub harvest_yield: u32,
    pub rats_damage: u32,
}

impl GameState {
    pub fn new(seed: Option<u64>) -> Self {
        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };

        let initial_land_price = rng.gen_range(17..=26);

        Self {
            year: 1,
            population: 100,
            grain: 2800,
            land: 1000,
            land_price: initial_land_price,
            harvest_yield: 3,
            grain_eaten_by_rats: 0,
            new_citizens: 0,
            deaths_starvation: 0,
            deaths_plague: 0,
            total_deaths: 0,
            grain_harvested: 0,
            acres_planted: 0,
            rng,
            history: Vec::new(),
            current_phase: GamePhase::Splash,
        }
    }

    pub fn advance_year(&mut self) {
        self.year += 1;
        self.land_price = self.rng.gen_range(17..=26);
        self.current_phase = GamePhase::LandTransaction;
        
        // Reset per-year tracking variables
        self.harvest_yield = 0;
        self.grain_eaten_by_rats = 0;
        self.new_citizens = 0;
        self.deaths_starvation = 0;
        self.deaths_plague = 0;
        self.grain_harvested = 0;
        self.acres_planted = 0;
    }

    pub fn can_afford_land(&self, acres: u32) -> bool {
        acres * self.land_price <= self.grain
    }

    pub fn max_plantable_acres(&self) -> u32 {
        let by_population = self.population * 10;
        let by_grain = self.grain;
        let by_land = self.land;
        
        by_population.min(by_grain).min(by_land)
    }

    pub fn grain_needed_for_feeding(&self) -> u32 {
        self.population * 20
    }

    pub fn is_game_over(&self) -> bool {
        if self.year > 10 {
            return true;
        }
        
        if self.population == 0 {
            return true;
        }
        
        if self.deaths_starvation > 0 && 
           self.deaths_starvation * 100 / (self.population + self.deaths_starvation) > 45 {
            return true;
        }
        
        false
    }

    pub fn save_year_summary(&mut self) {
        let summary = YearSummary {
            year: self.year,
            population: self.population,
            grain: self.grain,
            land: self.land,
            starved: self.deaths_starvation,
            new_citizens: self.new_citizens,
            plague_deaths: self.deaths_plague,
            harvest_yield: self.harvest_yield,
            rats_damage: self.grain_eaten_by_rats,
        };
        self.history.push(summary);
    }
}