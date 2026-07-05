use ggez::audio::{SoundSource, Source};
use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::{Context, GameResult};

use crate::object::Draw;

/// Параметры фона и фонового звука
pub struct Background {
    background: Image,
    sound: Source,
}

/// Типаж фона и фонового звука
impl Background {
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let background = Image::from_path(ctx, "/image/main/background.png")?;
        let mut sound = Source::new(ctx, "/sound/background.wav")?;
        sound.set_repeat(true);
        Ok(Self { background, sound })
    }
}

/// Отрисовка фона
impl Draw for Background {
    /// Проверяет необходимость отрисовки
    fn is_draw(&mut self) -> bool {
        true
    }

    /// Рисует
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let draw_param = DrawParam::default();
        canvas.draw(&self.background, draw_param);

        if self.sound.stopped() {
            self.sound.play(ctx)?;
        }

        Ok(())
    }
}
