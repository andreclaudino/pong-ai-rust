#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up, Down
}

#[derive(Debug, Clone)]
pub struct ActionState {
    pub player1: Option<Direction>,
    pub player2: Option<Direction>
}

impl ActionState {
    pub fn empty() -> ActionState {
        ActionState{
            player1: None,
            player2: None
        }
    }

    pub fn move_p1(&mut self, direction: Option<Direction>) {
        self.player1 = direction;
    }

    pub fn move_p2(&mut self, direction: Option<Direction>) {
        self.player2 = direction;
    }
}