use ggez::audio::{SoundSource, Source};
use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::anim::{Anim, AnimData};
use crate::object::Draw;

/// Параметры объектов
struct Data {
    image: Image,
    x: f32,
    y: f32,
}

/// Параметры водолаза
pub struct Diver {
    pos: u8,
    bag_value: u16,
    score: u16,
    diver: [Data; 7],
    bag: [Data; 6],
    lifting: Anim<4>,
    filling: Anim<5>,
    hand: Anim<2>,
    move_sound: Source,
    playing_sound: bool,
}

/// Управление водолазом
impl Diver {
    /// Конструктор
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let pos = 0;
        let bag_value = 0;
        let score = 0;
        let diver = [
            Data {
                image: Image::from_path(ctx, "/image/diver/diver1.png")?,
                x: 96.0,
                y: 28.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver2.png")?,
                x: 98.0,
                y: 260.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver3.png")?,
                x: 150.0,
                y: 515.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver4.png")?,
                x: 364.0,
                y: 672.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver5.png")?,
                x: 620.0,
                y: 680.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver6.png")?,
                x: 936.0,
                y: 699.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver7.png")?,
                x: 664.0,
                y: 400.0,
            },
        ];
        let bag = [
            Data {
                image: Image::from_path(ctx, "/image/diver/diver1_bag.png")?,
                x: 198.0,
                y: 137.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver2_bag.png")?,
                x: 260.0,
                y: 372.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver3_bag.png")?,
                x: 261.0,
                y: 669.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver4_bag.png")?,
                x: 508.0,
                y: 803.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver5_bag.png")?,
                x: 766.0,
                y: 798.0,
            },
            Data {
                image: Image::from_path(ctx, "/image/diver/diver6_bag.png")?,
                x: 875.0,
                y: 762.0,
            },
        ];
        let lifting_data = [
            AnimData {
                image: None,
                x: 0.0,
                y: 0.0,
                pause: 0,
                sound: None,
            },
            AnimData {
                image: Some(bag[0].image.clone()),
                x: bag[0].x,
                y: bag[0].y,
                pause: 100,
                sound: Some(Source::new(ctx, "/sound/filling.wav")?),
            },
            AnimData {
                image: Some(Image::from_path(ctx, "/image/diver/diver1_upbag.png")?),
                x: 196.0,
                y: 54.0,
                pause: 100,
                sound: None,
            },
            AnimData {
                image: None,
                x: 0.0,
                y: 0.0,
                pause: 0,
                sound: None,
            },
        ];
        let lifting = Anim::new(lifting_data, 3)?;
        let filling_data = [
            AnimData {
                image: None,
                x: 0.0,
                y: 0.0,
                pause: 0,
                sound: None,
            },
            AnimData {
                image: Some(Image::from_path(ctx, "/image/diver/diver6_hand1.png")?),
                x: 1040.0,
                y: 802.0,
                pause: 100,
                sound: Some(Source::new(ctx, "/sound/filling.wav")?),
            },
            AnimData {
                image: Some(Image::from_path(ctx, "/image/diver/diver6_hand2.png")?),
                x: 1061.0,
                y: 789.0,
                pause: 100,
                sound: None,
            },
            AnimData {
                image: Some(Image::from_path(ctx, "/image/diver/diver6_hand3.png")?),
                x: 943.0,
                y: 781.0,
                pause: 100,
                sound: None,
            },
            AnimData {
                image: None,
                x: 0.0,
                y: 0.0,
                pause: 0,
                sound: None,
            },
        ];
        let filling = Anim::new(filling_data, 1)?;
        let hand_data = [
            AnimData {
                image: Some(Image::from_path(ctx, "/image/diver/diver7_hand1.png")?),
                x: 669.0,
                y: 522.0,
                pause: 200,
                sound: Some(Source::new(ctx, "/sound/hand.wav")?),
            },
            AnimData {
                image: Some(Image::from_path(ctx, "/image/diver/diver7_hand2.png")?),
                x: 709.0,
                y: 477.0,
                pause: 200,
                sound: Some(Source::new(ctx, "/sound/hand.wav")?),
            },
        ];
        let hand = Anim::new(hand_data, 1)?;
        let move_sound = Source::new(ctx, "/sound/move.wav")?;
        let playing_sound = false;
        Ok(Self {
            pos,
            bag_value,
            score,
            diver,
            bag,
            lifting,
            filling,
            hand,
            move_sound,
            playing_sound,
        })
    }

    /// Возвращает значение мешка водолаза
    pub fn bag_value(&self) -> u16 {
        self.bag_value
    }

    /// Устанавливает значение мешка водолаза
    pub fn set_bag_value(&mut self, value: u16) {
        self.bag_value = value;
    }

    /// Устанавливает следующую позицию
    pub fn next(&mut self) {
        if self.hand.playing() || self.filling.playing() || self.lifting.playing() {
            return;
        }

        if self.pos < 5 {
            self.pos += 1;
            self.playing_sound = true;
        }

        if self.pos == 5 {
            self.bag_value += 1;
            self.filling.play();
        }
    }

    /// Устанавливает предыдущую позицию
    pub fn prev(&mut self) {
        if self.hand.playing() || self.filling.playing() || self.lifting.playing() {
            return;
        }

        if self.pos > 0 {
            self.pos -= 1;
            self.playing_sound = true;
        }

        if self.pos == 0 && self.bag_value > 0 {
            self.score += self.bag_value;
            if self.score > 1000 {
                self.score %= 1000;
            }
            self.bag_value = 0;
            self.lifting.play();
        }
    }

    /// Возвращает количество набранных очков
    pub fn score(&self) -> u16 {
        self.score
    }

    /// Устанавливает количество набранных очков
    pub fn set_score(&mut self, value: u16) {
        self.score = value;
    }

    /// Устанавливает позицию водолаза
    pub fn set_pos(&mut self, pos: u8) {
        if pos != 6 && self.pos == 6 {
            self.hand.stop();
            self.filling.stop();
            self.lifting.stop();
        }

        if pos == 6 && self.pos != 6 {
            self.pos = pos;
            self.hand.play();
        } else {
            self.pos = pos;
        }
    }

    /// Возвращает позицию водолаза
    pub fn pos(&self) -> u8 {
        self.pos
    }
}

/// Отрисовывает водолаза
impl Draw for Diver {
    /// Проверяет необходимость отрисовки
    fn is_draw(&mut self) -> bool {
        // Отображение доставки клада на лодку
        if self.pos == 0 && self.lifting.is_draw() {
            return true;
        }

        // Отображение заполнения мешка рукой
        if self.pos == 5 && self.filling.is_draw() {
            return true;
        }

        // Отображение пойманного водолаза
        if self.pos == 6 && self.hand.is_draw() {
            return true;
        }

        true
    }

    /// Рисует
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        if self.playing_sound && self.move_sound.stopped() {
            self.move_sound.play(ctx)?;
            self.playing_sound = false;
        }

        // Отображение водолаза
        let x = self.diver[self.pos as usize].x;
        let y = self.diver[self.pos as usize].y;
        let image = &self.diver[self.pos as usize].image;
        let draw_param = DrawParam::default().dest(Point2 { x, y });
        canvas.draw(image, draw_param);

        // Отображение доставки клада на лодку
        if self.pos == 0 {
            self.lifting.draw(ctx, canvas)?;
        }

        // Отображение мешка
        if self.bag_value > 0 && self.pos > 0 && self.pos < 6 {
            let x = self.bag[self.pos as usize].x;
            let y = self.bag[self.pos as usize].y;
            let image = &self.bag[self.pos as usize].image;
            let draw_param = DrawParam::default().dest(Point2 { x, y });
            canvas.draw(image, draw_param);
        }

        // Отображение заполнения мешка рукой
        if self.pos == 5 {
            self.filling.draw(ctx, canvas)?;
        }

        // Отображение пойманного водолаза
        if self.pos == 6 {
            self.hand.draw(ctx, canvas)?;
        }

        Ok(())
    }
}
