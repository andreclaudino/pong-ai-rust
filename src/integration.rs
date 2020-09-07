use crate::{entity::Coordinates, action_state::{Direction, ActionState}};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::constants::{WINDOW_WIDTH, WINDOW_HEIGHT, DISTANCE_FACTOR, TIMEOUT, PLAY_SUFFIX, ACT_SUFFIX, FINISH_SUFFIX};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportState {
    player1: f32,
    player2: f32,
    ball1: Vec<f32>,
    ball2: Vec<f32>,
    ball1_velocity: Vec<f32>,
    ball2_velocity: Vec<f32>,
    action1: i8,
    action2: i8,
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
pub struct PlayerState {
    observation: Vec<f32>,
    action: i8,
    score: f32,
    player: Player,
}

impl ReportState {
    pub fn new(player1: Coordinates, player2: Coordinates, ball: Coordinates, actions: ActionState) -> ReportState {

        let action1 = match actions.player1 {
            None => 0,
            Some(Direction::Up) => 1,
            Some(Direction::Down) => 2,
        };

        let action2 = match actions.player2 {
            None => 0,
            Some(Direction::Up) => 1,
            Some(Direction::Down) => 2,
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
        let ball_x = self.ball1[0];
        let ball_y = self.ball1[1];

        PlayerState {
            observation: [vec![self.player1, self.player2], vec![ball_x, ball_y], self.ball1_velocity].concat(),
            action: self.action1,
            score: self.score - 3.0*(ball_y -self.player1).abs(),
            player: Player::Player1
        }
    }


    pub fn to_player2(self) -> PlayerState {
        let ball_x = self.ball2[0];
        let ball_y = self.ball2[1];

        PlayerState {
            observation: [vec![self.player1, self.player2], vec![ball_x, ball_y], self.ball1_velocity].concat(),
            action: self.action1,
            score: self.score - DISTANCE_FACTOR*(ball_y -self.player1).abs(),
            player: Player::Player2
        }
    }
}


pub fn finish(base: &String, reported_state: ReportState, player: Player, score: f32) -> () {

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/{}/{}", base, PLAY_SUFFIX, FINISH_SUFFIX);

    let player_state = match player {
        Player::Player1 => reported_state.with_score(score).to_player1(),
        Player::Player2 => reported_state.with_score(score).to_player2()
    };

    client.post(url.as_str())
        .json(&player_state)
        .timeout(std::time::Duration::from_secs(TIMEOUT))
        .send()
        .unwrap();
    ()
}

pub fn infer_next_state(base: &String, reported_state: ReportState, player: Player, score: f32)
    -> Option<Direction> {

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/{}/{}", base, PLAY_SUFFIX, ACT_SUFFIX);

    let player_state = match player {
        Player::Player1 => reported_state.with_score(score).to_player1(),
        Player::Player2 => reported_state.with_score(score).to_player2()
    };

    let response = 
        client.post(url.as_str())
            .json(&player_state)
            .timeout(std::time::Duration::from_secs(TIMEOUT))
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