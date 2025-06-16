use crate::game::{evaluate_performance, ActionResult, GameAction, GamePhase, GameState};
use crate::messages::MessageTemplates;
use anyhow::Result;
use std::time::Instant;

pub struct App {
    pub game: GameState,
    pub input_buffer: String,
    pub message: String,
    pub event_messages: Vec<String>,
    pub should_quit: bool,
    pub splash_start: Option<Instant>,
    pub messages: MessageTemplates,
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
            messages: MessageTemplates::new(seed),
        }
    }

    pub fn handle_input(&mut self, c: char) {
        // Allow any key to skip splash
        if matches!(self.game.current_phase, GamePhase::Splash) {
            self.game.current_phase = GamePhase::Instructions;
            self.splash_start = None;
            return;
        }

        if matches!(
            self.game.current_phase,
            GamePhase::Instructions | GamePhase::YearEnd | GamePhase::GameOver
        ) {
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
                    self.message = self.messages.insufficient_grain_land_message();
                }
                ActionResult::InsufficientLand => {
                    self.message = self.messages.insufficient_land_message();
                }
                ActionResult::InsufficientPopulation => {
                    // This should never happen for land transactions, but handle it to be exhaustive
                    unreachable!()
                }
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
                    self.message = self.messages.insufficient_grain_seed_message();
                }
                ActionResult::InsufficientLand => {
                    self.message = self.messages.insufficient_land_planting_message();
                }
                ActionResult::InsufficientPopulation => {
                    self.message = self.messages.insufficient_workers_message();
                }
            }
        }
    }

    fn process_feeding(&mut self) {
        if let Ok(bushels) = self.input_buffer.trim().parse::<u32>() {
            match self
                .game
                .execute_action(GameAction::FeedPopulation(bushels))
            {
                ActionResult::Success => {
                    self.process_year_end();
                    self.input_buffer.clear();
                    self.message.clear();
                }
                ActionResult::InsufficientGrain => {
                    self.message = self.messages.insufficient_grain_feeding_message();
                }
                ActionResult::InsufficientLand | ActionResult::InsufficientPopulation => {
                    // These should never happen for feeding, but handle them to be exhaustive
                    unreachable!()
                }
            }
        }
    }

    fn process_year_end(&mut self) {
        // Process random events
        let outcomes = self.game.process_year_events(&mut self.messages);
        self.event_messages.clear();

        // Add starvation report first if any
        if self.game.deaths_starvation > 0 {
            self.event_messages.push(
                self.messages
                    .starvation_message(self.game.deaths_starvation),
            );
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

        if self.game.deaths_starvation > 0
            && self.game.deaths_starvation * 100
                / (self.game.population + self.game.deaths_starvation)
                > 45
        {
            self.event_messages.push(
                "THOU HAST STARVED MORE THAN HALF THY SUBJECTS IN A SINGLE YEAR!".to_string(),
            );
            self.event_messages
                .push("FOR THIS MOST GRIEVOUS SIN, THOU ART NOT ONLY".to_string());
            self.event_messages
                .push("CAST FROM THY THRONE, BUT SHALL BE REMEMBERED".to_string());
            self.event_messages
                .push("AS THE GREATEST FOOL TO EVER WEAR A CROWN!!!!".to_string());
        } else {
            self.event_messages
                .push("IN THY TEN-YEAR REIGN OVER BABYLON:".to_string());
            self.event_messages.push(format!(
                "{:.1} PERCENT OF THY SUBJECTS STARVED EACH YEAR",
                score.death_rate / 10.0
            ));
            self.event_messages.push(format!(
                "A TOTAL OF {} SOULS PERISHED UNDER THY RULE!",
                score.total_deaths
            ));
            self.event_messages.push(format!(
                "THOU BEGAN WITH 10 ACRES PER SUBJECT AND ENDED WITH {:.1}",
                score.acres_per_person
            ));
            self.event_messages.push("".to_string());
            self.event_messages
                .push(score.get_rating_message().to_string());
        }
    }
}
