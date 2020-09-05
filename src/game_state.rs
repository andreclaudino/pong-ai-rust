/// TODO: aumentar o score sempre que bater na bola, e remover sempre que perder
use tetra::graphics::{self, Color, Texture, DrawParams};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use tetra::{Context, State};
use tetra::input::{self, Key};
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH, BALL_SPEED, PADDLE_SPEED, PADDLE_SPIN, BALL_ACC, PADDLE_X_POSITION, MAX_CICLES_BEFORE_SEND, POINTS_ON_WIN, POINTS_ON_LOOSE};
use crate::entity::Entity;
use crate::integration::{infer_next_state, ReportState, finish};
use crate::action_state::{ActionState, Direction};
use tetra;

#[derive(Debug, Clone)]
pub struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,

    score1: Text,
    score2: Text,

    score1_position: Vec2<f32>,
    score2_position: Vec2<f32>,

    pub action_state: ActionState,
    sents: u32,

    p1_remote: bool,
    p2_remote: bool,

    p1_bot_url: String,
    p2_bot_url: String,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            PADDLE_X_POSITION,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - PADDLE_X_POSITION,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        let ball_position = Vec2::new(
            (WINDOW_WIDTH - ball_texture.width() as f32)/2.0,
            (WINDOW_HEIGHT - ball_texture.height() as f32)/2.0
        );

        let score1 = graphics::text::Text::new(format!("{}", 0),
            Font::vector(ctx, "./resources/joystix.ttf", 100.0)?
        );
        let score1_position = Vec2::new(16.0, 0.0);
        
        let score2 = graphics::text::Text::new(format!("{}", 0),
            Font::vector(ctx, "./resources/joystix.ttf", 100.0)?
        );
        let score2_position = Vec2::new(WINDOW_WIDTH - 84.0, 0.0);

        let (p1_remote, p2_remote, p1_bot_url, p2_bot_url) = GameState::configure_remotes();

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            
            score1, score2,
            score1_position,
            score2_position,

            sents: 0,
            action_state: ActionState::empty(),

            p1_remote,
            p2_remote,

            p1_bot_url,
            p2_bot_url
        })
    }

    fn configure_remotes() -> (bool, bool, String, String) {
        let p1_remote = is_player1_remote();
        let p2_remote = is_player2_remote();

        let p1_bot_url: String = env_or_default("P1_BOT_URL", "http://0.0.0.0:8080");
        let p2_bot_url: String = env_or_default("P2_BOT_URL", "http://0.0.0.0:8081");
        (p1_remote, p2_remote, p1_bot_url, p2_bot_url)
    }

    pub fn reset(&mut self, direction: i32){
        let player1_position = Vec2::new(
            PADDLE_X_POSITION,
            (WINDOW_HEIGHT - self.player1.texture.height() as f32) / 2.0,
        );
        self.player1.coordinates.position = player1_position;

        let player2_position = Vec2::new(
            WINDOW_WIDTH - self.player2.texture.width() as f32 - PADDLE_X_POSITION,
            (WINDOW_HEIGHT - self.player2.texture.height() as f32) / 2.0,
        );
        self.player2.coordinates.position = player2_position;

        let ball_position = Vec2::new(
            (WINDOW_WIDTH - self.ball.texture.width() as f32)/2.0,
            (WINDOW_HEIGHT - self.ball.texture.height() as f32)/2.0
        );
        self.ball.coordinates.position = ball_position;
        self.ball.coordinates.velocity = Vec2::new(direction as f32 * BALL_SPEED, 0.0);
    }

    pub fn coordinates(&self) -> ReportState {
        let player1 = self.player1.coordinates;
        let player2 = self.player2.coordinates;
        let ball = self.ball.coordinates;
        let state = self.action_state;

        ReportState::new(player1, player2, ball, state)
        
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.check_action(ctx);
        self.check_move();
        self.check_colision(ctx);
        self.report_state();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0., 0., 0.));

        graphics::draw(ctx, &self.player1.texture, self.player1.coordinates.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.coordinates.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.coordinates.position);

        let draw_params1 = DrawParams::new()
            .color(Color::rgb(1.0, 1.0, 1.0))
            .position(self.score1_position);
        graphics::draw(ctx, &self.score1, draw_params1);

        let draw_params2 = DrawParams::new()
            .color(Color::rgb(1.0, 1.0, 1.0))
            .position(self.score2_position);
        graphics::draw(ctx, &self.score2, draw_params2);

        self.ball.coordinates.position += self.ball.coordinates.velocity;

        Ok(())
    }
}

impl GameState {
    fn check_colision(&mut self, ctx: &mut Context) {
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.ball.coordinates.velocity.x = -(self.ball.coordinates.velocity.x + (BALL_ACC * self.ball.coordinates.velocity.x.signum()));
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

            self.ball.coordinates.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.coordinates.position.y <= 0.0 || self.ball.coordinates.position.y + self.ball.height() >= WINDOW_HEIGHT {
            self.ball.coordinates.velocity.y = -self.ball.coordinates.velocity.y;
        }

        if self.ball.coordinates.position.x < 0.0 {
            self.reset(-1);
            self.player2.coordinates.score += POINTS_ON_WIN;
            self.score2.set_content(format!("{}", self.player2.coordinates.score));
            if let Some(optional_score2) = self.score2.get_bounds(ctx) {
                self.score2_position.x = WINDOW_WIDTH - (16.0 + optional_score2.width);
            }
            println!("Player 2 wins!");

            // Interact with remote server if controller is remote
            if self.p1_remote {
                finish(&self.p1_bot_url, self.coordinates(), POINTS_ON_WIN);
            }
            if self.p2_remote {
                finish(&self.p2_bot_url, self.coordinates(), POINTS_ON_LOOSE);
            }

        }

        if self.ball.coordinates.position.x > WINDOW_WIDTH {
            self.reset(1);
            self.player1.coordinates.score += POINTS_ON_WIN;
            self.score1.set_content(format!("{}", self.player1.coordinates.score));
            println!("Player 1 wins!");

            // Interact with remote server if controller is remote
            if self.p1_remote {
                finish(&self.p1_bot_url, self.coordinates(), POINTS_ON_WIN);
            }

            if self.p2_remote {
                finish(&self.p2_bot_url, self.coordinates(), POINTS_ON_LOOSE);
            }
        }
    }
}

// Deal with replay buffer and event collection from API result
impl GameState {
    fn report_state(&mut self) {
        if self.sents >= MAX_CICLES_BEFORE_SEND {
            // P1 Reports
            if self.p1_remote {
                let p1_direction = infer_next_state(&self.p1_bot_url, self.coordinates());
                self.action_state.move_p1(p1_direction);
            }

            //P2 Reports
            if self.p2_remote {
                let p1_direction = infer_next_state(&self.p1_bot_url, self.coordinates());
                self.action_state.move_p2(p1_direction);
            }

            self.sents = 0;
        } else {
            self.sents += 1;
        }
    }
}

// Deal with events
impl GameState {
    fn check_action(&mut self, ctx: &mut Context) {
        if input::is_key_down(ctx, Key::W) && !self.p1_remote {
            self.action_state.move_p1(Some(Direction::Up));
        } else if input::is_key_down(ctx, Key::S) && !self.p1_remote {
            self.action_state.move_p1(Some(Direction::Down));
        } else if !self.p1_remote {
            self.action_state.move_p1(None);
        }

        if input::is_key_down(ctx, Key::Up) && !self.p2_remote {
            self.action_state.move_p2(Some(Direction::Up));
        } else if input::is_key_down(ctx, Key::Down) && !self.p2_remote {
            self.action_state.move_p2(Some(Direction::Down));
        } else if !self.p2_remote {
            self.action_state.move_p2(None);
        }
    }
}

// Deal with moviments
impl GameState {
    fn check_move(&mut self) {
        match self.action_state.player1 {
            Some(Direction::Up) => self.player1.coordinates.position.y += PADDLE_SPEED,
            Some(Direction::Down) => self.player1.coordinates.position.y -= PADDLE_SPEED,
            None => ()
        }

        match self.action_state.player2 {
            Some(Direction::Up) => self.player2.coordinates.position.y += PADDLE_SPEED,
            Some(Direction::Down) => self.player2.coordinates.position.y -= PADDLE_SPEED,
            None => ()
        }
    }
}


fn is_player1_remote() -> bool {is_player_remote("P1_IS_REMOTE")}

fn is_player2_remote() -> bool {
    is_player_remote("P2_IS_REMOTE")
}

fn is_player_remote(environment_variable: &str) -> bool {
    match std::env::var(environment_variable)
        .unwrap_or("FALSE".to_string())
        .to_uppercase()
        .as_str()
    {
        "TRUE" => true,
        _ => false
    }
}

fn env_or_default(env_name: &str, default: &str) -> String {
    std::env::var(env_name)
        .unwrap_or(default.to_string())
}