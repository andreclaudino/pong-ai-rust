use tetra::graphics::{self, Color, Texture, DrawParams};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use tetra::{Context, State};
use tetra::input::{self, Key};
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH, BALL_SPEED, PADDLE_SPEED, PADDLE_SPIN, BALL_ACC, PADDLE_X_POSITION};
use crate::entity::Entity;


pub struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,

    score1: Text,
    score2: Text,

    score1_position: Vec2<f32>,
    score2_position: Vec2<f32>
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

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            
            score1, score2,
            score1_position,
            score2_position
        })
    }

    pub fn reset(&mut self, direction: i32){
        let player1_position = Vec2::new(
            PADDLE_X_POSITION,
            (WINDOW_HEIGHT - self.player1.texture.height() as f32) / 2.0,
        );
        self.player1.position = player1_position;

        let player2_position = Vec2::new(
            WINDOW_WIDTH - self.player2.texture.width() as f32 - PADDLE_X_POSITION,
            (WINDOW_HEIGHT - self.player2.texture.height() as f32) / 2.0,
        );
        self.player2.position = player2_position;

        let ball_position = Vec2::new(
            (WINDOW_WIDTH - self.ball.texture.width() as f32)/2.0,
            (WINDOW_HEIGHT - self.ball.texture.height() as f32)/2.0
        );
        self.ball.position = ball_position;
        self.ball.velocity = Vec2::new(direction as f32 * BALL_SPEED, 0.0);
    }
}

impl State for GameState {
    
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0., 0., 0.));
        
        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);

        let draw_params1 = DrawParams::new()
            .color(Color::rgb(1.0, 1.0, 1.0))
            .position(self.score1_position);
        graphics::draw(ctx, &self.score1, draw_params1);

        let draw_params2 = DrawParams::new()
            .color(Color::rgb(1.0, 1.0, 1.0))
            .position(self.score2_position);
        graphics::draw(ctx, &self.score2, draw_params2);
        
        self.ball.position += self.ball.velocity;

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }
    
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
            self.ball.velocity.x = -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        if self.ball.position.x < 0.0 {
            self.reset(-1);
            self.player2.score += 1;
            self.score2.set_content(format!("{}", self.player2.score));
            if let Some(optional_score2) = self.score2.get_bounds(ctx) {
                self.score2_position.x = WINDOW_WIDTH - (16.0 + optional_score2.width);
            }
            println!("Player 2 wins!");
        }

        if self.ball.position.x > WINDOW_WIDTH {
            self.reset(1);
            self.player1.score += 1;
            self.score1.set_content(format!("{}", self.player1.score));
            println!("Player 1 wins!");
        }

        Ok(())
    }
}