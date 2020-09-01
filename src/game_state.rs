use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::{Context, State};
use tetra::input::{self, Key};
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH, BALL_SPEED, PADDLE_SPEED, PADDLE_SPIN, BALL_ACC, PADDLE_X_POSITION};
use crate::entity::Entity;


pub struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
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

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity)
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
        
        let text = graphics::text::Text::new(,
            Font::vector(ctx, "./examples/resources/DejaVuSansMono.ttf", 16.0)?);

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
            println!("Player 2 wins!");
        }

        if self.ball.position.x > WINDOW_WIDTH {
            self.reset(1);
            self.player1.score += 1;
            println!("Player 1 wins!");
        }

        Ok(())
    }
}