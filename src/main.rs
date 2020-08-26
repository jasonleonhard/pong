use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 600.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        // player1
        // W = down
        // Q = up
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player_1_pos.y -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Q) {
            self.player_1_pos.y += PLAYER_SPEED * dt;
        }

        // player2
        // A = down
        // S = up
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.player_2_pos.y -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player_2_pos.y += PLAYER_SPEED * dt;
        }

        // ball
        // O = down
        // P = up
        if keyboard::is_key_pressed(ctx, KeyCode::O) {
            self.ball_pos.y -= BALL_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::P) {
            self.ball_pos.y += BALL_SPEED * dt;
        }
        // ball
        // K = down
        // L = up
        if keyboard::is_key_pressed(ctx, KeyCode::K) {
            self.ball_pos.x -= BALL_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::L) {
            self.ball_pos.x += BALL_SPEED * dt;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::WHITE,
        )?;

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE,
        )?;

        let mut draw_param = graphics::DrawParam::default();

        // draw left right players and ball first time
        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Snake_0", "TanTan");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "SNAKE");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state);

    Ok(())
}
