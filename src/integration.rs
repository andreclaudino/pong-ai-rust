use crate::{entity::Coordinates, action_state::{Direction, ActionState}};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportState {
    player1: f32,
    player2: f32,
    ball1: Vec<f32>,
    ball2: Vec<f32>,
    ball1_velocity: Vec<f32>,
    ball2_velocity: Vec<f32>,
    action1: Vec<f32>,
    action2: Vec<f32>,
    score: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Player {
    Player1,
    Player2
}

#[derive(Deserialize)]
struct ResponseAction {
    action: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PlayerState {
    observation: Vec<f32>,
    action: Vec<f32>,
    score: f32,
    player: Player
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
            player1: player1.position.y/WINDOW_HEIGHT,
            player2: player2.position.y/WINDOW_HEIGHT,
            ball1: Vec::from([ball.position.x/WINDOW_WIDTH, ball.position.y/WINDOW_HEIGHT]),
            ball2: Vec::from([1.0 - ball.position.x/WINDOW_WIDTH, ball.position.y/WINDOW_HEIGHT]),
            ball1_velocity: Vec::from([ball.velocity.x, ball.velocity.y]),
            ball2_velocity: Vec::from([-ball.velocity.x, ball.velocity.y]),
            action1,
            action2,
            score: 0.0
        }
    }

    pub fn with_score(mut self, score: f32) -> ReportState {
        self.score = score;
        self
    }

    pub fn to_player1(self) -> PlayerState {
        PlayerState {
            observation: [vec![self.player1, self.player2], self.ball1, self.ball1_velocity].concat(),
            action: self.action1,
            score: self.score,
            player: Player::Player1
        }
    }


    pub fn to_player2(self) -> PlayerState {
        PlayerState {
            observation: [vec![self.player1, self.player2], self.ball1, self.ball1_velocity].concat(),
            action: self.action1,
            score: self.score,
            player: Player::Player2
        }
    }
}


pub fn finish(base: &String, reported_state: ReportState, player: Player, score: f32) -> () {

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/{}", base, "buffer/finish");

    let player_state = match player {
        Player::Player1 => reported_state.with_score(score).to_player1(),
        Player::Player2 => reported_state.with_score(score).to_player2()
    };

    client.post(url.as_str())
        .json(&player_state)
        .send()
        .unwrap();
    ()
}

pub fn infer_next_state(base: &String, reported_state: ReportState, player: Player, score: f32)
    -> Option<Direction> {

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/{}", base, "buffer");

    let player_state = match player {
        Player::Player1 => reported_state.with_score(score).to_player1(),
        Player::Player2 => reported_state.with_score(score).to_player2()
    };

    let response = 
        client.post(url.as_str())
            .json(&player_state)
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