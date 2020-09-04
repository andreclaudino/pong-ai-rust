use crate::{entity::Coordinates, action_state::{Direction, ActionState}};
use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportState {
    player1: Vec<f32>,
    player2: Vec<f32>,
    ball: Vec<f32>,
    action1: Vec<f32>,
    action2: Vec<f32>
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
            player1: Vec::from([player1.position.x, player1.position.y, player1.score as f32]),
            player2: Vec::from([player2.position.x, player2.position.y, player2.score as f32]),
            ball: Vec::from([ball.position.x, ball.position.y, ball.score as f32]),
            action1: action1,
            action2: action2
        }
    }
}


pub fn finish(base: &str, reported_state: ReportState) -> () { 

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/{}", base, "/buffer/finish");

    client.post(url.as_str())
        .json(&reported_state)
        .send()
        .unwrap();
    
    ()
}

pub fn report_buffer(base: &str, reported_state: ReportState) -> () { 

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/{}", base, "/buffer");

    client.post(url.as_str())
        .json(&reported_state)
        .send()
        .unwrap();
    
    ()
}