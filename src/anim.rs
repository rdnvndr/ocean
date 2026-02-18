use std::time::{Instant, Duration};
use ggez::audio::{ SoundSource, Source };
use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::mint::Point2;

use crate::object::Draw;

/// Параметры отображения объекта анимации
pub struct AnimData {
    pub image: Option<Image>,
    pub x: f32,
    pub y: f32,
    pub pause: u64,
    pub sound: Option<Source>,
}

/// Параметры отображения анимации
pub struct Anim<const COUNT: usize> {
    ticks: Option<Instant>,
    current: i8,
    objs: [AnimData; COUNT],
    count: i8,
    draw_curr: i32,
    sound_playing: bool,
}

/// Выполняет анимацию
impl<const COUNT: usize> Anim<COUNT> {
    /// Конструктор
    pub fn new(objs: [AnimData; COUNT], repeat: i8) -> GameResult<Self> {
        let ticks = None;
        let current = 0;
        let count = if repeat >= 0 { repeat - 1 } else { -1 };
        let draw_curr = 0;
        let sound_playing = false;
        Ok(Self { ticks, current, objs, count, draw_curr, sound_playing})
    }

    /// Начинает анимацию
    pub fn play(&mut self) {
        self.ticks = Some(Instant::now());
        self.current = 0;
        self.draw_curr = 0;
        self.sound_playing = false
    }

    /// Проверяет запущена ли анимация
    pub fn playing(&self) -> bool {
        if let Some(_) = &self.ticks { true } else { false }
    }

    /// Останавливает анимацию
    pub fn stop(&mut self) {
        self.ticks = None;
    }
}

/// Отрисовывает объекты анимации
impl<const COUNT: usize> Draw for Anim<COUNT> {
    /// Проверяет необходимость отрисовки объекта
    fn is_draw(&mut self) -> bool {
        if let Some(ticks) = &self.ticks {
            let obj = &self.objs[self.draw_curr as usize];
            if Instant::now() <= *ticks + Duration::from_millis(obj.pause)  {
                return false
            }
            let mut value = self.draw_curr + 1;
            self.sound_playing = false;

            // Обработка количества повторов
            if value as usize == self.objs.len() {
                if self.count == -1 {
                    value = 0;
                }
                if self.current < self.count {
                    self.current = self.current + 1;
                    value = 0;
                }
            }

            if value as usize != self.objs.len() {
                self.draw_curr = value;
                self.ticks = Some(Instant::now());
            } else {
                self.ticks = None;
            }
        }
        true
    }

    /// Рисует объект
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let obj = &mut self.objs[self.draw_curr as usize];

        if let Some(sound) = &mut obj.sound {
            if sound.stopped() && !self.sound_playing {
                sound.play(ctx)?;
                self.sound_playing = true
            }
        }

        if let Some(image) = &mut obj.image {
            let draw_param = DrawParam::default().dest(Point2{ x: obj.x, y: obj.y });
            canvas.draw(image, draw_param);
        }
        Ok(())
    }
}
