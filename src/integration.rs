use crate::{entity::Coordinates, action_state::{Direction, ActionState}};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::constants::WINDOW_WIDTH;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportState {
    player1: f32,
    player2: f32,
    ball1: Vec<f32>,
    ball2: Vec<f32>,
    action1: Vec<f32>,
    action2: Vec<f32>,
    score: f32
}

#[derive(Deserialize)]
struct ResponseAction {
    action: Option<String>
}

impl ReportState {
    pub fn new(player1: Coordinates, player2: Coordinates, ball: Coordinates, actions: ActionState) -> ReportState {

        let action1 = match actions.player1 {
            Some(Direction::Up) => Vec::from([1., 0., 0.]),
            Some(Direction::Down) => Vec::from([0., 1., 0.]),
            None => Vec::from([0., 0., 1.])
        };

        let action2 = match actions.player2 {
            Some(Direction::Up) => Vec::from([1., 0., 0.]),
            Some(Direction::Down) => Vec::from([0., 1., 0.]),
            None => Vec::from([0., 0., 1.])
        };

        ReportState{
            player1: player1.position.y,
            player2: player2.position.y,
            ball1: Vec::from([ball.position.x, ball.position.y]),
            ball2: Vec::from([WINDOW_WIDTH - ball.position.x, ball.position.y]),
            action1,
            action2,
            score: 0.0
        }
    }

    pub fn with_score(mut self, score: f32) -> ReportState {
        self.score = score;
        self
    }
}


pub fn finish(base: &String, reported_state: ReportState, current_score: f32) -> () {

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/{}", base, "buffer/finish");
    let state_to_send = reported_state.with_score(current_score);

    client.post(url.as_str())
        .json(&state_to_send)
        .send()
        .unwrap();
    ()
}

pub fn infer_next_state(base: &String, reported_state: ReportState) -> Option<Direction> {

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/{}", base, "buffer");

    let response = 
        client.post(url.as_str())
            .json(&reported_state)
            .send()
            .unwrap();
    
    match response.json::<ResponseAction>() {
        Ok(response_acton) =>
            {
                if response_acton.action == Some("Up".to_string()) {
                    Some(Direction::Up)
                } else if response_acton.action == Some("Down".to_string()) {
                    Some(Direction::Down)
                } else {
                    None
                }
            },
        _ => None
    }
}