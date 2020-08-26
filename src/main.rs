use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};

const PLAYER_SPEED: f32 = 600.0;

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;

const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_SPEED: f32 = 100.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut na::Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let _dt = ggez::timer::delta(ctx).as_secs_f32();
    let _screen_h = graphics::drawable_size(ctx).1;

    if keyboard::is_key_pressed(ctx, keycode) {
        pos.y -= y_dir * PLAYER_SPEED * _dt;
    }

    clamp(
        &mut pos.y,
        RACKET_HEIGHT_HALF,
        _screen_h - RACKET_HEIGHT_HALF,
    );
}

// spawn ball with random velocity, 50/50
fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (_screen_w, _screen_h) = graphics::drawable_size(ctx);
        let (_screen_w_half, screen_h_half) = (_screen_w * 0.5, _screen_h * 0.5);
        let mut ball_vel = na::Vector2::new(0.0, 0.0);
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos: na::Point2::new(_screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(_screen_w_half, screen_h_half),
            ball_vel: na::Vector2::new(BALL_SPEED, BALL_SPEED),
            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let _dt = ggez::timer::delta(ctx).as_secs_f32();
        let (_screen_h, _screen_w) = graphics::drawable_size(ctx);

        move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);

        self.ball_pos += self.ball_vel * _dt;

        // if ball goes off screen
        // ball back to center & randomize speed of ball
        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = _screen_w * 0.5;
            self.ball_pos.y = _screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        }
        if self.ball_pos.x > _screen_w {
            self.ball_pos.x = _screen_w * 0.5;
            self.ball_pos.y = _screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
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

        // draw left, right players and ball first time
        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        let score_text = graphics::Text::new(format!(
            "{}             {}",
            self.player_1_score, self.player_2_score
        ));
        // let (screen_w, screen_h) = graphics::drawable_size();
        let screen_w = graphics::drawable_size(ctx).0;
        let _screen_w_half = screen_w * 0.5;

        // let score_pos = na::Point2::new(_screen_w_half, 40.0);
        let score_pos = na::Point2::new(380.0, 40.0);
        draw_param.dest = score_pos.into();

        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Snake_0", "TanTan");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "SNAKE");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state)?;
    Ok(())
}
