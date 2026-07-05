use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::object::Draw;

/// Табло набранных очков
pub struct Scope {
    digit: [Image; 10],
    value: u16,
}

/// Управляет таблом набранных очков
impl Scope {
    /// Конструктор
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let digit = [
            Image::from_path(ctx, "/image/clock/digit0.png")?,
            Image::from_path(ctx, "/image/clock/digit1.png")?,
            Image::from_path(ctx, "/image/clock/digit2.png")?,
            Image::from_path(ctx, "/image/clock/digit3.png")?,
            Image::from_path(ctx, "/image/clock/digit4.png")?,
            Image::from_path(ctx, "/image/clock/digit5.png")?,
            Image::from_path(ctx, "/image/clock/digit6.png")?,
            Image::from_path(ctx, "/image/clock/digit7.png")?,
            Image::from_path(ctx, "/image/clock/digit8.png")?,
            Image::from_path(ctx, "/image/clock/digit9.png")?,
        ];
        let value: u16 = 0;
        Ok(Self { digit, value })
    }

    /// Возвращает количество набранных очков
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Устанавливает количество набранных очков
    pub fn set_value(&mut self, value: u16) {
        self.value = if value > 1000 { value % 1000 } else { value };
    }
}

/// Отрисовка табло набранных очков
impl Draw for Scope {
    /// Проверяет необходимость отрисовки
    fn is_draw(&mut self) -> bool {
        true
    }

    /// Рисует
    fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Сотни
        let hundreds = self.value / 100;
        if hundreds > 0 {
            let draw_param = DrawParam::default().dest(Point2 { x: 854.0, y: 37.0 });
            canvas.draw(&self.digit[hundreds as usize], draw_param);
        }

        // Десятки
        let dozens = (self.value - hundreds * 100) / 10;
        if dozens > 0 || self.value >= 100 {
            let draw_param = DrawParam::default().dest(Point2 { x: 924.0, y: 37.0 });
            canvas.draw(&self.digit[dozens as usize], draw_param);
        }

        // Единицы
        let units = self.value - hundreds * 100 - dozens * 10;
        let draw_param = DrawParam::default().dest(Point2 { x: 994.0, y: 37.0 });
        canvas.draw(&self.digit[units as usize], draw_param);
        Ok(())
    }
}
