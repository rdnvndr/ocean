use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::anim::{Anim, AnimData};
use crate::object::Draw;

/// Параметры ожидающих водолазов в лодке
pub struct BoatDiver {
    boatdiver: Image,
    onediver: Anim<2>,
    count: i8,
}

/// Управление ожидающими водолазами в лодке
impl BoatDiver {
    /// Конструктор
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let boatdiver = Image::from_path(ctx, "/image/diver/boat_diver.png")?;
        let anim_data = [
            AnimData{ image: Some(boatdiver.clone()), x: 400.0, y: 26.0, pause: 250, sound: None },
            AnimData{ image: Some(boatdiver.clone()), x: 288.0, y: 26.0, pause: 0, sound: None },
        ];
        let onediver = Anim::new(anim_data, 1)?;
        let count: i8 = 2;
        Ok(Self { boatdiver, onediver, count })
    }

    /// Устанавливает количество ожидающих водолазов в лодке
    pub fn set_count(&mut self, count: i8) {
        if count >= 0 && count <= 2 {
            if count == 1 {
                self.onediver.play();
            }
            self.count = count;
        }
    }

    /// Возвращает количество ожидающих водолазов в лодке
    pub fn count(&self) -> i8 {
        self.count
    }
}

/// Отрисовка ожидающих водолазов в лодке
impl Draw for BoatDiver {
    /// Проверяет необходимость отрисовки
    fn is_draw(&mut self) -> bool {
        self.onediver.is_draw()
    }

    /// Рисует
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        if self.count == 1 {
            self.onediver.draw(ctx, canvas)?;
        }

        if self.count == 2 {
            let draw_param = DrawParam::default().dest(Point2{ x: 400.0, y: 26.0 });
            canvas.draw(&self.boatdiver, draw_param);
            let draw_param = DrawParam::default().dest(Point2{ x: 288.0, y: 26.0 });
            canvas.draw(&self.boatdiver, draw_param);
        }
        Ok(())
    }
}
