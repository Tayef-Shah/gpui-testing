pub const WIDTH: f32 = 720.0;
pub const HEIGHT: f32 = 400.0;
pub const PADDLE_HEIGHT: f32 = 80.0;
pub const PADDLE_WIDTH: f32 = 12.0;
pub const INSET: f32 = 24.0;
pub const BALL_RADIUS: f32 = 7.0;
const PADDLE_SPEED: f32 = 340.0;
const BALL_SPEED: f32 = 310.0;

#[derive(Default)]
pub struct Controls {
    pub left_up: bool,
    pub left_down: bool,
    pub right_up: bool,
    pub right_down: bool,
}

pub struct Game {
    pub paddles: [f32; 2],
    pub ball: [f32; 2],
    pub velocity: [f32; 2],
    pub scores: [u32; 2],
    pub serve_delay: f32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            paddles: [(HEIGHT - PADDLE_HEIGHT) / 2.0; 2],
            ball: [WIDTH / 2.0, HEIGHT / 2.0],
            velocity: [BALL_SPEED, BALL_SPEED * 0.35],
            scores: [0; 2],
            serve_delay: 0.8,
        }
    }
}

impl Game {
    // Called at 120 Hz, independent of display refresh rate and keyboard repeat.
    pub fn step(&mut self, dt: f32, controls: &Controls) {
        for (index, (up, down)) in [
            (controls.left_up, controls.left_down),
            (controls.right_up, controls.right_down),
        ]
        .into_iter()
        .enumerate()
        {
            let direction = i32::from(down) - i32::from(up);
            self.paddles[index] = (self.paddles[index] + direction as f32 * PADDLE_SPEED * dt)
                .clamp(0.0, HEIGHT - PADDLE_HEIGHT);
        }
        if self.serve_delay > 0.0 {
            self.serve_delay = (self.serve_delay - dt).max(0.0);
            return;
        }

        let previous_x = self.ball[0];
        self.ball[0] += self.velocity[0] * dt;
        self.ball[1] += self.velocity[1] * dt;
        if self.ball[1] < BALL_RADIUS {
            self.ball[1] = 2.0 * BALL_RADIUS - self.ball[1];
            self.velocity[1] = self.velocity[1].abs();
        } else if self.ball[1] > HEIGHT - BALL_RADIUS {
            self.ball[1] = 2.0 * (HEIGHT - BALL_RADIUS) - self.ball[1];
            self.velocity[1] = -self.velocity[1].abs();
        }

        let left_face = INSET + PADDLE_WIDTH + BALL_RADIUS;
        let right_face = WIDTH - INSET - PADDLE_WIDTH - BALL_RADIUS;
        let hit = if self.velocity[0] < 0.0 && previous_x >= left_face && self.ball[0] <= left_face
        {
            Some((0, left_face, 1.0))
        } else if self.velocity[0] > 0.0 && previous_x <= right_face && self.ball[0] >= right_face {
            Some((1, right_face, -1.0))
        } else {
            None
        };
        if let Some((index, face, direction)) = hit {
            let paddle = self.paddles[index];
            if self.ball[1] + BALL_RADIUS >= paddle
                && self.ball[1] - BALL_RADIUS <= paddle + PADDLE_HEIGHT
            {
                let offset = ((self.ball[1] - paddle - PADDLE_HEIGHT / 2.0)
                    / (PADDLE_HEIGHT / 2.0))
                    .clamp(-1.0, 1.0);
                let speed = self.velocity[0].hypot(self.velocity[1]) * 1.04;
                let speed = speed.min(650.0);
                let angle = offset * 0.95;
                self.ball[0] = face;
                self.velocity = [direction * speed * angle.cos(), speed * angle.sin()];
            }
        }
        if self.ball[0] < -BALL_RADIUS || self.ball[0] > WIDTH + BALL_RADIUS {
            let scorer = usize::from(self.ball[0] < 0.0);
            self.scores[scorer] = self.scores[scorer].saturating_add(1);
            self.ball = [WIDTH / 2.0, HEIGHT / 2.0];
            let direction = if scorer == 0 { 1.0 } else { -1.0 };
            let vertical = if (self.scores[0] + self.scores[1]).is_multiple_of(2) {
                1.0
            } else {
                -1.0
            };
            self.velocity = [direction * BALL_SPEED, vertical * BALL_SPEED * 0.35];
            self.serve_delay = 0.8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controls_move_both_paddles_and_clamp_at_edges() {
        let mut game = Game::default();
        let controls = Controls {
            left_up: true,
            right_down: true,
            ..Default::default()
        };
        game.step(1.0, &controls);
        assert_eq!(game.paddles, [0.0, HEIGHT - PADDLE_HEIGHT]);
        let controls = Controls {
            left_up: true,
            left_down: true,
            ..Default::default()
        };
        game.step(0.1, &controls);
        assert_eq!(game.paddles[0], 0.0);
    }

    #[test]
    fn ball_reflects_at_wall_and_paddle() {
        let mut game = Game {
            serve_delay: 0.0,
            ..Default::default()
        };
        game.ball = [WIDTH / 2.0, BALL_RADIUS + 1.0];
        game.velocity = [100.0, -200.0];
        game.step(1.0 / 120.0, &Controls::default());
        assert!(game.velocity[1] > 0.0);
        game.ball = [INSET + PADDLE_WIDTH + BALL_RADIUS + 1.0, HEIGHT / 2.0];
        game.velocity = [-BALL_SPEED, 0.0];
        game.step(1.0 / 120.0, &Controls::default());
        assert!(game.velocity[0] > 0.0);
        assert_eq!(game.scores, [0, 0]);
    }

    #[test]
    fn missing_paddle_scores_once_and_resets_serve() {
        let mut game = Game {
            serve_delay: 0.0,
            ..Default::default()
        };
        game.ball = [WIDTH + BALL_RADIUS - 1.0, 20.0];
        game.step(1.0 / 120.0, &Controls::default());
        assert_eq!(game.scores, [1, 0]);
        assert_eq!(game.ball, [WIDTH / 2.0, HEIGHT / 2.0]);
        game.step(1.0 / 120.0, &Controls::default());
        assert_eq!(game.scores, [1, 0]);
        game.serve_delay = 0.0;
        game.ball = [-BALL_RADIUS + 1.0, 20.0];
        game.velocity[0] = -BALL_SPEED;
        game.step(1.0 / 120.0, &Controls::default());
        assert_eq!(game.scores, [1, 1]);
    }
}
