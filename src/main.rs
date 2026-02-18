use ggez::event::{self, EventHandler};
use ggez::graphics::{Canvas, Color, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};
use ggez::conf::FullscreenType;
use rust_i18n::t;
use std::time::{Duration, Instant};
use sys_locale::get_locale;

mod anim;
mod boatdiver;
mod background;
mod diver;
mod game;
mod object;
mod octopus;
mod scope;

use crate::background::Background;
use crate::boatdiver::BoatDiver;
use crate::diver::Diver;
use crate::game::Game;
use crate::object::Draw;
use crate::octopus::Octopus;
use crate::scope::Scope;

pub const DESIRED_FPS: f64 = 24.0;
rust_i18n::i18n!("locales");

/// Параметры игры
struct GameState {
    last_frame: Instant,
    background: Background,
    game: Game,
    scope: Scope,
    boatdiver: BoatDiver,
    diver: Diver,
    octopus: Octopus,
    respawn: Option<Instant>,
}

/// Типаж состояния игры
impl GameState {
    /// Конструктор
    fn new(ctx: &Context) -> GameState {
        GameState {
            last_frame: Instant::now(),
            background: Background::new(ctx).unwrap(),
            game: Game::new(ctx).unwrap(),
            scope: Scope::new(ctx).unwrap(),
            boatdiver: BoatDiver::new(ctx).unwrap(),
            diver: Diver::new(ctx).unwrap(),
            octopus: Octopus::new(ctx).unwrap(),
            respawn: None,
        }
    }
}

/// Обработка игровых событий
impl EventHandler for GameState {

    /// Обновляет логику (вызывается каждый кадр)
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Контролирует частоту (FPS) кадров
        let now = Instant::now();
        let frame_time = now - self.last_frame;
        let target_time = Duration::from_secs_f64(1.0 / DESIRED_FPS);
        if frame_time < target_time {
            std::thread::sleep(target_time - frame_time);
        }
        self.last_frame = Instant::now();

        // Устаналивает количество очков на табло
        if self.scope.value() != self.diver.score() {
            self.scope.set_value(self.diver.score());
        }

        // Проверка пересечения щупальцы и водолаза
        if let None = self.respawn {
            let pos = self.diver.pos();
            if  pos > 0 && self.octopus.is_tentacles_max(pos - 1) {
                self.octopus.set_diver(true);
                self.diver.set_pos(6);
                self.respawn = Some(Instant::now());
            }
        }

        // Возобновление игры
        if let Some(respawn) = &self.respawn {
            if Instant::now() > *respawn + Duration::from_millis(1500)  {
                let mut count = self.boatdiver.count() - 1;
                if count < 0 {
                   count = 2;
                   self.diver.set_score(0);
                }
                self.diver.set_bag_value(0);
                self.octopus.set_diver(false);
                self.diver.set_pos(0);
                self.boatdiver.set_count(count);
                self.respawn = None;
            }
        }

        Ok(())
    }

    /// Отрисовывает сцену (вызывается каждый кадр)
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(204, 206, 199));
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 1415.0, 904.0));

        let mut redraw = false;
        redraw = self.game.is_draw() || redraw;
        redraw = self.scope.is_draw() || redraw;
        redraw = self.boatdiver.is_draw() || redraw;
        redraw = self.diver.is_draw() || redraw;
        redraw = self.octopus.is_draw() || redraw;

        if redraw {
            self.background.draw(ctx, &mut canvas)?;
            self.game.draw(ctx, &mut canvas)?;
            self.scope.draw(ctx, &mut canvas)?;
            self.boatdiver.draw(ctx, &mut canvas)?;
            self.diver.draw(ctx, &mut canvas)?;
            self.octopus.draw(ctx, &mut canvas)?;
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    /// Обработка нажатия кнопок клавиатуры
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Escape) | Some(KeyCode::Q) => {
                ctx.request_quit();
            },
            Some(KeyCode::A) => {
                if *self.game.mode() != game::GameMode::A {
                    self.octopus.set_speed(self.octopus.speed() * 2);
                    self.game.set_mode(game::GameMode::A);
                }
            },
            Some(KeyCode::B) => {
                if *self.game.mode() != game::GameMode::B {
                    self.octopus.set_speed(self.octopus.speed() / 2);
                    self.game.set_mode(game::GameMode::B);
                }
            },
            Some(KeyCode::G) => {
                if *self.game.mode() != game::GameMode::A {
                    self.octopus.set_speed(self.octopus.speed() * 2);
                    self.game.set_mode(game::GameMode::A);
                } else {
                    self.octopus.set_speed(self.octopus.speed() / 2);
                    self.game.set_mode(game::GameMode::B);
                }
            },
            Some(KeyCode::Down) | Some(KeyCode::Right) => {
                self.diver.next();
            }
            Some(KeyCode::Up) | Some(KeyCode::Left) => {
                self.diver.prev();
            }
            Some(KeyCode::F) => {
                ctx.gfx.set_fullscreen(FullscreenType::Desktop)?
            },
            Some(KeyCode::W) => {
                ctx.gfx.set_fullscreen(FullscreenType::Windowed)?
            },
            _ => {},
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let locale = get_locale().unwrap_or_else(|| String::from("en"));
    rust_i18n::set_locale(&locale);

    let window_mode = ggez::conf::WindowMode::default()
        .dimensions(1415.0, 904.0)
        .resizable(true);

    let window_setup = ggez::conf::WindowSetup::default()
        .title(&t!("Mystery of the Ocean"))
        .vsync(true);

    let (ctx, event_loop) = ggez::ContextBuilder::new("ocean", "Andrey Rodionov")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .add_resource_path("./resources")
        .build()?;

    let state = GameState::new(&ctx);
    event::run(ctx, event_loop, state)
}
