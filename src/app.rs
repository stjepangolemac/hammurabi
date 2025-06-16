use crate::game::{GameState, GamePhase, GameAction, ActionResult, evaluate_performance};
use anyhow::Result;

pub struct App {
    pub game: GameState,
    pub input_buffer: String,
    pub message: String,
    pub event_messages: Vec<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new(seed: Option<u64>) -> Self {
        Self {
            game: GameState::new(seed),
            input_buffer: String::new(),
            message: String::new(),
            event_messages: Vec::new(),
            should_quit: false,
        }
    }

    pub fn handle_input(&mut self, c: char) {
        if matches!(self.game.current_phase, GamePhase::YearEnd | GamePhase::GameOver) {
            return;
        }

        match c {
            '0'..='9' | '-' => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    pub fn handle_backspace(&mut self) {
        self.input_buffer.pop();
    }

    pub fn handle_enter(&mut self) -> Result<()> {
        match self.game.current_phase {
            GamePhase::LandTransaction => self.process_land_transaction(),
            GamePhase::Planting => self.process_planting(),
            GamePhase::Feeding => self.process_feeding(),
            GamePhase::YearEnd => self.advance_to_next_year(),
            GamePhase::GameOver => self.should_quit = true,
        }
        Ok(())
    }

    fn process_land_transaction(&mut self) {
        if let Ok(amount) = self.input_buffer.trim().parse::<i32>() {
            let action = if amount > 0 {
                GameAction::BuyLand(amount as u32)
            } else if amount < 0 {
                GameAction::SellLand((-amount) as u32)
            } else {
                self.game.current_phase = GamePhase::Planting;
                self.input_buffer.clear();
                return;
            };

            match self.game.execute_action(action) {
                ActionResult::Success => {
                    self.game.current_phase = GamePhase::Planting;
                    self.input_buffer.clear();
                    self.message.clear();
                }
                ActionResult::InsufficientGrain => {
                    self.message = "Not enough grain to buy that much land!".to_string();
                }
                ActionResult::InsufficientLand => {
                    self.message = "You don't have that much land to sell!".to_string();
                }
                _ => {}
            }
        } else if self.input_buffer.is_empty() {
            self.game.current_phase = GamePhase::Planting;
        }
    }

    fn process_planting(&mut self) {
        if let Ok(acres) = self.input_buffer.trim().parse::<u32>() {
            match self.game.execute_action(GameAction::PlantAcres(acres)) {
                ActionResult::Success => {
                    self.game.current_phase = GamePhase::Feeding;
                    self.input_buffer.clear();
                    self.message.clear();
                }
                ActionResult::InsufficientGrain => {
                    self.message = "Not enough grain for seed!".to_string();
                }
                ActionResult::InsufficientLand => {
                    self.message = "You don't have that much land!".to_string();
                }
                ActionResult::InsufficientPopulation => {
                    self.message = "Not enough people to work the fields!".to_string();
                }
                _ => {}
            }
        }
    }

    fn process_feeding(&mut self) {
        if let Ok(bushels) = self.input_buffer.trim().parse::<u32>() {
            match self.game.execute_action(GameAction::FeedPopulation(bushels)) {
                ActionResult::Success => {
                    self.process_year_end();
                    self.input_buffer.clear();
                    self.message.clear();
                }
                ActionResult::InsufficientGrain => {
                    self.message = "You don't have that much grain!".to_string();
                }
                _ => {}
            }
        }
    }

    fn process_year_end(&mut self) {
        // Process random events
        let outcomes = self.game.process_year_events();
        self.event_messages.clear();
        
        for outcome in outcomes {
            self.event_messages.push(outcome.description);
        }

        // Save year summary
        self.game.save_year_summary();

        // Check game over conditions
        if self.game.is_game_over() {
            self.game.current_phase = GamePhase::GameOver;
            self.calculate_final_score();
        } else {
            self.game.current_phase = GamePhase::YearEnd;
        }
    }

    fn advance_to_next_year(&mut self) {
        self.game.advance_year();
        self.event_messages.clear();
    }

    fn calculate_final_score(&mut self) {
        let score = evaluate_performance(
            100,
            self.game.population,
            self.game.total_deaths,
            self.game.land,
        );

        self.event_messages.clear();
        self.event_messages.push(format!("Final Population: {}", self.game.population));
        self.event_messages.push(format!("Final Land: {} acres", self.game.land));
        self.event_messages.push(format!("Total Deaths: {}", score.total_deaths));
        self.event_messages.push(format!("Death Rate: {:.1}%", score.death_rate));
        self.event_messages.push(format!("Acres per Person: {:.1}", score.acres_per_person));
        self.event_messages.push("".to_string());
        self.event_messages.push(score.get_rating_message().to_string());
    }
}