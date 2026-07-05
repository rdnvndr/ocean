use ggez::graphics::Canvas;
use ggez::{Context, GameResult};

/// Отрисовывает объекты
pub trait Draw {
    /// Проверяет необходимость отрисовки
    fn is_draw(&mut self) -> bool;

    /// Рисует
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
}
