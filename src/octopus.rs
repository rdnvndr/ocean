use ggez::audio::{SoundSource, Source};
use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rand;

use std::time::{Instant, Duration};
use crate::object::Draw;

/// Данные объекта
struct Data {
    image: Image,
    x: f32,
    y: f32,
}

/// Параметры осьминога
pub struct Octopus {
    is_diver: bool,
    main: Data,
    playing: bool,
    sound: Source,
    speed: u64,
    tentacles: [Vec<Data>; 6],
    tentacles_len: [u8; 5],
    tentacles_sign: [i8; 5],
    ticks: Instant,
}

/// Управляет осьминогом
impl Octopus {
    /// Конструктор
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let is_diver = false;
        let main = Data { image: Image::from_path(ctx, "/image/octopus/main.png")?, x: 545.0, y: 209.0 };
        let playing = false;
        let sound = Source::new(ctx, "/sound/tentacles.wav")?;
        let speed = 500;
        let tentacles = [
            vec!(
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles11.png")?, x: 436.0, y: 319.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles12.png")?, x: 332.0, y: 333.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles13.png")?, x: 248.0, y: 290.0 },
            ),
            vec!(
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles11.png")?, x: 436.0, y: 319.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles22.png")?, x: 383.0, y: 379.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles23.png")?, x: 349.0, y: 428.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles24.png")?, x: 296.0, y: 491.0 },
            ),
            vec!(
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles31.png")?, x: 601.0, y: 396.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles32.png")?, x: 590.0, y: 451.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles33.png")?, x: 573.0, y: 497.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles34.png")?, x: 562.0, y: 570.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles35.png")?, x: 534.0, y: 637.0 },
            ),
            vec!(
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles41.png")?, x: 750.0, y: 484.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles42.png")?, x: 751.0, y: 533.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles43.png")?, x: 769.0, y: 589.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles44.png")?, x: 761.0, y: 662.0 },
            ),
            vec!(
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles51.png")?, x: 1028.0, y: 557.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles52.png")?, x: 1057.0, y: 618.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles53.png")?, x: 1093.0, y: 680.0 },
            ),
            vec!(
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles31.png")?,  x: 601.0, y: 396.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles32.png")?,  x: 590.0, y: 451.0 },
                Data{ image: Image::from_path(ctx, "/image/octopus/tentacles33a.png")?, x: 628.0, y: 500.0 },
            ),
        ];
        let tentacles_len = [0, 0, 0, 0, 0];
        let tentacles_sign = [1, 1, 1, 1, 1];
        let ticks = Instant::now();
        Ok(Self{is_diver, main, playing, sound, speed, tentacles, tentacles_len, tentacles_sign, ticks })
    }

    /// Возвращает скорость осьминога в мс
    pub fn speed(&self) -> u64 {
        self.speed
    }

    /// Устанавливает скорость осьминога в мс
    pub fn set_speed(&mut self, value: u64) {
        self.speed = value;
    }

    /// Возвращает флаг отображения щупальцы пойманого дайвера
    pub fn is_diver(&self) -> bool {
        self.is_diver
    }

    /// Устанавливает флаг отображения щупальцы пойманого дайвера
    pub fn set_diver(&mut self, value: bool) {
        self.is_diver = value;
    }

    /// Возвращает является ли максимальной длина щупальцы
    pub fn is_tentacles_max(&self, num: u8) -> bool {
        let n = num as usize;
        n > 5 || self.tentacles[n].len() == self.tentacles_len[n] as usize
    }

    /// Генерирует новое состояние осьминога
    pub fn generate(&mut self) {
        self.playing = true;
        let mut num = rand::random_range(0..=4);

        // Определение активной щупальцы если выбрали 0 или 1
        if num == 0 && self.tentacles_len[1] > 1 {
            num = 1;
        }
        if num == 1 && self.tentacles_len[0] > 1 {
            num = 0;
        }

        let length = self.tentacles_len[num];
        let max = self.tentacles[num].len() as u8;
        let sign = self.tentacles_sign[num];

        // Увеличиваем или уменьшаем активную щупальцу
        if (sign > 0 && length != max) || (sign < 0 && length == 0) {
            self.tentacles_len[num] += 1;
            self.tentacles_sign[num] = 1;
        } else {
            self.tentacles_len[num] -= 1;
            self.tentacles_sign[num] = -1;
        }

        // Одновременное изменение 0 и 1 щупальцы для секций меньше 2
        if num == 0 && self.tentacles_len[num] < 2 {
            self.tentacles_len[1] = self.tentacles_len[num];
            self.tentacles_sign[1] = 1;
        }
        if num == 1 && self.tentacles_len[num] < 2 {
            self.tentacles_len[0] = self.tentacles_len[num];
            self.tentacles_sign[0] = 1;
        }
    }
}

/// Отрисовка осьминога
impl Draw for Octopus {
    /// Проверяет необходимость отрисовки
    fn is_draw(&mut self) -> bool {
        // Генерирует новое состояние осьминога с указанной скоростью
        if Instant::now() > self.ticks + Duration::from_millis(self.speed)  {
            self.generate();
            self.ticks = Instant::now();
        }
        true
    }

    // Рисует
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Рисует тело осьминога
        let draw_param = DrawParam::default().dest(Point2{ x: self.main.x, y: self.main.y });
        canvas.draw(&self.main.image, draw_param);

         // Рисует щупальцы
        for tentacles_num in 0..5 {
            let tentacles_len = self.tentacles_len[tentacles_num];
            if tentacles_len == 0 {
                continue;
            }
            for section_num in 0..tentacles_len as usize {
                let tentacles_section = &self.tentacles[tentacles_num][section_num];
                let x = tentacles_section.x;
                let y = tentacles_section.y;
                let draw_param = DrawParam::default().dest(Point2{ x, y });
                canvas.draw(&tentacles_section.image, draw_param);
            }
        }

        // Отображает щупальцу пойманого водолаза
        if self.is_diver {
            let tentacles_section = &self.tentacles[3][0];
            let x = tentacles_section.x;
            let y = tentacles_section.y;
            let draw_param = DrawParam::default().dest(Point2{ x, y });
            canvas.draw(&tentacles_section.image, draw_param);
        }

        if self.playing && self.sound.stopped() {
            self.sound.play(ctx)?;
            self.playing = false;
        }

        Ok(())
    }
}
