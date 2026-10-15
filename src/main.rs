mod game;

use game::{BALL_RADIUS, Controls, Game, HEIGHT, INSET, PADDLE_HEIGHT, PADDLE_WIDTH, WIDTH};
use gpui::{
    App, Application, Bounds, Context, FocusHandle, IntoElement, KeyBinding, Render,
    TitlebarOptions, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size,
};
use std::time::Instant;

actions!(gpui_testing, [Quit]);
const STEP: f32 = 1.0 / 120.0;

struct Pong {
    game: Game,
    controls: Controls,
    focus: FocusHandle,
    last_frame: Instant,
    accumulator: f32,
    paused: bool,
}

impl Pong {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        window.focus(&focus);
        Self {
            game: Game::default(),
            controls: Controls::default(),
            focus,
            last_frame: Instant::now(),
            accumulator: 0.0,
            paused: false,
        }
    }

    fn set_key(&mut self, key: &str, pressed: bool) -> bool {
        match key {
            "w" | "W" => self.controls.left_up = pressed,
            "s" | "S" => self.controls.left_down = pressed,
            "up" => self.controls.right_up = pressed,
            "down" => self.controls.right_down = pressed,
            _ => return false,
        }
        true
    }

    fn restart(&mut self) {
        self.game = Game::default();
        self.controls = Controls::default();
        self.paused = false;
        self.accumulator = 0.0;
        self.last_frame = Instant::now();
    }
}

impl Render for Pong {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame).as_secs_f32().min(0.05);
        self.last_frame = now;
        let active = window.is_window_active() && self.focus.is_focused(window);
        if active && !self.paused {
            self.accumulator += elapsed;
            while self.accumulator >= STEP {
                self.game.step(STEP, &self.controls);
                self.accumulator -= STEP;
            }
        } else {
            // Losing focus must not leave a held key moving a paddle indefinitely.
            self.controls = Controls::default();
            self.accumulator = 0.0;
        }
        window.request_animation_frame();
        let status = if !active {
            "Click the court to resume"
        } else if self.paused {
            "Paused · Space to resume"
        } else if self.game.serve_delay > 0.0 {
            "Get ready…"
        } else {
            "Space to pause · R to restart"
        };

        div()
            .id("pong")
            .track_focus(&self.focus)
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0x0b1020))
            .text_color(rgb(0xe2e8f0))
            .font_family(".SystemUIFont")
            .on_click(cx.listener(|view, _, window, _| window.focus(&view.focus)))
            .on_key_down(cx.listener(|view, event: &gpui::KeyDownEvent, _, cx| {
                let key = event.keystroke.key.as_str();
                if view.set_key(key, true) {
                    cx.stop_propagation();
                } else if !event.is_held && key == "space" {
                    view.paused = !view.paused;
                    view.controls = Controls::default();
                    cx.stop_propagation();
                } else if !event.is_held && key == "r" {
                    view.restart();
                    cx.stop_propagation();
                }
                cx.notify();
            }))
            .on_key_up(cx.listener(|view, event: &gpui::KeyUpEvent, _, cx| {
                if view.set_key(event.keystroke.key.as_str(), false) {
                    cx.stop_propagation();
                }
            }))
            .child(div().text_3xl().child("PONG"))
            .child(
                div()
                    .w(px(WIDTH))
                    .flex()
                    .justify_between()
                    .child(format!("PLAYER 1   {}", self.game.scores[0]))
                    .child(format!("{}   PLAYER 2", self.game.scores[1])),
            )
            .child(
                div()
                    .relative()
                    .w(px(WIDTH))
                    .h(px(HEIGHT))
                    .flex_shrink_0()
                    .overflow_hidden()
                    .rounded_lg()
                    .bg(rgb(0x111c30))
                    .children((0..12).map(|i| {
                        div()
                            .absolute()
                            .left(px(WIDTH / 2.0 - 1.0))
                            .top(px(i as f32 * HEIGHT / 12.0 + 8.0))
                            .w(px(2.0))
                            .h(px(16.0))
                            .bg(rgb(0x334155))
                    }))
                    .child(
                        div()
                            .absolute()
                            .left(px(INSET))
                            .top(px(self.game.paddles[0]))
                            .w(px(PADDLE_WIDTH))
                            .h(px(PADDLE_HEIGHT))
                            .rounded_md()
                            .bg(rgb(0x38bdf8)),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(px(WIDTH - INSET - PADDLE_WIDTH))
                            .top(px(self.game.paddles[1]))
                            .w(px(PADDLE_WIDTH))
                            .h(px(PADDLE_HEIGHT))
                            .rounded_md()
                            .bg(rgb(0xa78bfa)),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(px(self.game.ball[0] - BALL_RADIUS))
                            .top(px(self.game.ball[1] - BALL_RADIUS))
                            .size(px(BALL_RADIUS * 2.0))
                            .rounded_full()
                            .bg(rgb(0xf8fafc)),
                    ),
            )
            .child(
                div()
                    .w(px(WIDTH))
                    .flex()
                    .justify_between()
                    .child("W / S · Player 1")
                    .child("↑ / ↓ · Player 2"),
            )
            .child(div().text_sm().text_color(rgb(0x94a3b8)).child(status))
            .child(
                div()
                    .id("restart")
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x24324a))
                    .cursor_pointer()
                    .hover(|style| style.bg(rgb(0x334155)))
                    .on_click(cx.listener(|view, _, window, cx| {
                        view.restart();
                        window.focus(&view.focus);
                        cx.notify();
                    }))
                    .child("Restart game"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        let bounds = Bounds::centered(None, size(px(820.0), px(660.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(780.0), px(640.0))),
                titlebar: Some(TitlebarOptions {
                    title: Some("GPUI Pong".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Pong::new(window, cx)),
        )
        .expect("Could not open the Pong window");
        cx.activate(true);
    });
}
