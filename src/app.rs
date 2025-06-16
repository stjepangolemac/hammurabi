use crate::game::{GameState, GamePhase, GameAction, ActionResult, evaluate_performance};
use anyhow::Result;
use std::time::Instant;

pub struct App {
    pub game: GameState,
    pub input_buffer: String,
    pub message: String,
    pub event_messages: Vec<String>,
    pub should_quit: bool,
    pub splash_start: Option<Instant>,
}

impl App {
    pub fn new(seed: Option<u64>) -> Self {
        Self {
            game: GameState::new(seed),
            input_buffer: String::new(),
            message: String::new(),
            event_messages: Vec::new(),
            should_quit: false,
            splash_start: Some(Instant::now()),
        }
    }

    pub fn handle_input(&mut self, c: char) {
        // Allow any key to skip splash
        if matches!(self.game.current_phase, GamePhase::Splash) {
            self.game.current_phase = GamePhase::Instructions;
            self.splash_start = None;
            return;
        }
        
        if matches!(self.game.current_phase, GamePhase::Instructions | GamePhase::YearEnd | GamePhase::GameOver) {
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
        // Allow any key to skip splash
        if matches!(self.game.current_phase, GamePhase::Splash) {
            self.game.current_phase = GamePhase::Instructions;
            self.splash_start = None;
            return;
        }
        
        self.input_buffer.pop();
    }

    pub fn handle_enter(&mut self) -> Result<()> {
        match self.game.current_phase {
            GamePhase::Splash => {
                self.game.current_phase = GamePhase::Instructions;
                self.splash_start = None;
            }
            GamePhase::Instructions => {
                self.game.current_phase = GamePhase::LandTransaction;
            }
            GamePhase::LandTransaction => self.process_land_transaction(),
            GamePhase::Planting => self.process_planting(),
            GamePhase::Feeding => self.process_feeding(),
            GamePhase::YearEnd => self.advance_to_next_year(),
            GamePhase::GameOver => self.should_quit = true,
        }
        Ok(())
    }
    
    pub fn check_splash_timeout(&mut self) {
        if let Some(start) = self.splash_start {
            if start.elapsed().as_secs() >= 5 && self.game.current_phase == GamePhase::Splash {
                self.game.current_phase = GamePhase::Instructions;
                self.splash_start = None;
            }
        }
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
        
        // Add starvation report first if any
        if self.game.deaths_starvation > 0 {
            self.event_messages.push(format!("YOU STARVED {} PEOPLE", self.game.deaths_starvation));
        }
        
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
        
        if self.game.deaths_starvation > 0 && 
           self.game.deaths_starvation * 100 / (self.game.population + self.game.deaths_starvation) > 45 {
            self.event_messages.push("YOU STARVED OVER 45% OF THE POPULATION IN ONE YEAR!".to_string());
            self.event_messages.push("DUE TO THIS EXTREME MISMANAGEMENT YOU HAVE NOT ONLY".to_string());
            self.event_messages.push("BEEN IMPEACHED AND THROWN OUT OF OFFICE BUT YOU HAVE".to_string());
            self.event_messages.push("ALSO BEEN DECLARED NATIONAL FINK!!!!".to_string());
        } else {
            self.event_messages.push("IN YOUR 10-YEAR TERM OF OFFICE:".to_string());
            self.event_messages.push(format!("{:.1} PERCENT OF THE POPULATION STARVED PER YEAR ON AVERAGE", score.death_rate / 10.0));
            self.event_messages.push(format!("A TOTAL OF {} PEOPLE DIED!!", score.total_deaths));
            self.event_messages.push(format!("YOU STARTED WITH 10 ACRES PER PERSON AND ENDED WITH {:.1}", score.acres_per_person));
            self.event_messages.push("".to_string());
            self.event_messages.push(score.get_rating_message().to_string());
        }
    }
}