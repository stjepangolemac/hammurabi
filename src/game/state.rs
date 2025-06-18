use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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
    pub current_phase: GamePhase,
    pub unlimited_mode: bool,
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

impl GameState {
    pub fn new(seed: Option<u64>, unlimited: bool) -> Self {
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
            current_phase: GamePhase::Splash,
            unlimited_mode: unlimited,
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
        if !self.unlimited_mode && self.year > 10 {
            return true;
        }

        if self.population == 0 {
            return true;
        }

        if self.deaths_starvation > 0
            && self.deaths_starvation * 100 / (self.population + self.deaths_starvation) > 45
        {
            return true;
        }

        false
    }

    pub fn save_year_summary(&mut self) {
        // This method is kept for compatibility but no longer stores history
    }
}
