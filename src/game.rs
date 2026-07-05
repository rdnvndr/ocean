use ggez::audio::{SoundSource, Source};
use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::object::Draw;

/// Режим игры
#[derive(PartialEq)]
pub enum GameMode {
    A,
    B,
}

/// Параметры игры
pub struct Game {
    game_a: Image,
    game_b: Image,
    sound_a: Source,
    sound_b: Source,
    mode: GameMode,
    playing: bool,
}

/// Управление режимом игры
impl Game {
    /// Конструктор
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let game_a = Image::from_path(ctx, "/image/game/gamea.png")?;
        let game_b = Image::from_path(ctx, "/image/game/gameb.png")?;
        let sound_a = Source::new(ctx, "/sound/gamea.wav")?;
        let sound_b = Source::new(ctx, "/sound/gameb.wav")?;
        let mode = GameMode::A;
        let playing = false;
        Ok(Self {
            game_a,
            game_b,
            sound_a,
            sound_b,
            mode,
            playing,
        })
    }

    /// Возвращает режим игры
    pub fn mode(&self) -> &GameMode {
        &self.mode
    }

    /// Устанавливает режим игры
    pub fn set_mode(&mut self, mode: GameMode) {
        if self.mode != mode {
            self.playing = true;
            self.mode = mode
        }
    }
}

/// Отрисовывает режим игры
impl Draw for Game {
    /// Проверяет необходимость отрисовки
    fn is_draw(&mut self) -> bool {
        true
    }

    /// Рисует
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        match self.mode {
            GameMode::A => {
                if self.playing && self.sound_a.stopped() {
                    self.sound_a.play(ctx)?;
                    self.playing = false;
                }
                let draw_param = DrawParam::default().dest(Point2 { x: 26.0, y: 781.0 });
                canvas.draw(&self.game_a, draw_param);
            }
            GameMode::B => {
                if self.playing && self.sound_a.stopped() {
                    self.sound_b.play(ctx)?;
                    self.playing = false;
                }
                let draw_param = DrawParam::default().dest(Point2 { x: 26.0, y: 843.0 });
                canvas.draw(&self.game_b, draw_param);
            }
        };
        Ok(())
    }
}
