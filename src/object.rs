use ggez::{ Context, GameResult };
use ggez::graphics::Canvas;

/// Отрисовывает объекты
pub trait Draw {
    /// Проверяет необходимость отрисовки
    fn is_draw(&mut self) -> bool;

    /// Рисует
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
}
