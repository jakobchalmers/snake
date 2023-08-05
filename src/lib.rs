use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Mesh, Rect};
// use ggez::graphics::{Text, TextFragment, TextLayout};
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::{self, GameResult};
use image::imageops;
use rand::Rng;

pub mod constants {
    use super::*;

    pub const RECT_SIZE: f32 = 40.0;
    // pub const RECT_SIZE: f32 = 60.0;
    pub const SCREEN_SIZE: f32 = 800.0;
    pub const SLEEP_TIME: u32 = 100;
    pub const MY_GREEN: Color = Color::new(5.0 / 255.0, 155.0 / 255.0, 4.0 / 255.0, 1.0);
    pub const RESOURCES_PATH: &'static str = "C:/Users/jakob/OneDrive/Rust/snake/resources";
}

pub mod utils {

    use super::constants::*;
    use super::*;

    fn resize_img(img_name: &str) -> GameResult {
        let img_path = format!("{}/{}", RESOURCES_PATH, img_name);
        let raw_img = image::open(&img_path)?;
        let resized_img = imageops::resize(
            &raw_img,
            RECT_SIZE as u32,
            RECT_SIZE as u32,
            imageops::FilterType::Lanczos3,
        );
        resized_img.save(&img_path)?;
        Ok(())
    }

    #[derive(Debug, Clone, Copy)]
    pub enum ConversionError {
        InvalidKey,
    }

    #[derive(Clone, Copy)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl TryFrom<KeyCode> for Direction {
        type Error = ConversionError;

        fn try_from(keycode: KeyCode) -> Result<Self, Self::Error> {
            match keycode {
                KeyCode::Up => Ok(Direction::Up),
                KeyCode::Down => Ok(Direction::Down),
                KeyCode::Left => Ok(Direction::Left),
                KeyCode::Right => Ok(Direction::Right),
                _ => Err(ConversionError::InvalidKey),
            }
        }
    }

    // -------------------------------------------------------

    #[derive(Debug, Clone, Copy)]
    pub struct Apple {
        pub x: f32,
        pub y: f32,
        pub color: Color,
        pub img_name: &'static str,
    }

    impl Apple {
        pub fn new() -> Self {
            let mut rng = rand::thread_rng();
            Apple {
                x: rng.gen::<f32>() * (SCREEN_SIZE - RECT_SIZE),
                y: rng.gen::<f32>() * (SCREEN_SIZE - RECT_SIZE),
                color: Color::RED,
                img_name: "apple.png",
            }
        }

        pub fn resize(self) -> GameResult {
            resize_img(self.img_name)?;
            Ok(())
        }

        pub fn draw(&mut self, ctx: &Context, canvas: &mut graphics::Canvas) -> GameResult {
            let rect = Rect::new(self.x, self.y, RECT_SIZE, RECT_SIZE);
            let rect_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, self.color)?;
            canvas.draw(&rect_mesh, Vec2::new(0.0, 0.0));

            let img = graphics::Image::from_path(ctx, format!("/{}", self.img_name))?;
            canvas.draw(&img, Vec2::new(self.x, self.y));

            Ok(())
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    impl Point {
        pub fn new() -> Self {
            let center = (SCREEN_SIZE - RECT_SIZE) / 2.0;
            Point {
                x: center,
                y: center,
            }
        }

        pub fn update(&mut self, direction: Direction) {
            match direction {
                Direction::Up => self.y -= RECT_SIZE,
                Direction::Down => self.y += RECT_SIZE,
                Direction::Left => self.x -= RECT_SIZE,
                Direction::Right => self.x += RECT_SIZE,
            }

            self.x %= SCREEN_SIZE;
            self.y %= SCREEN_SIZE;

            if self.x <= 0.0 {
                self.x += SCREEN_SIZE;
            }
            if self.y <= 0.0 {
                self.y += SCREEN_SIZE;
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Img {
        // img_dir: &'static str,
        pub img: &'static str,
        pub img_90: &'static str,
        pub img_180: &'static str,
        pub img_270: &'static str,
    }

    impl Img {
        pub fn new() -> Self {
            Img {
                // img_dir: "C:/Users/jakob/OneDrive/Rust/my_snake/resources/",
                img: "snake.png",
                img_90: "snake_90.png",
                img_180: "snake_180.png",
                img_270: "snake_270.png",
            }
        }

        pub fn resize(self) -> GameResult {
            resize_img(self.img)?;

            Ok(())
        }

        pub fn rotate(self) -> GameResult {
            let img = image::open(format!("{}/{}", RESOURCES_PATH, self.img))?;

            let img = imageops::rotate90(&img);
            img.save(format!("{}/{}", RESOURCES_PATH, self.img_90))?;

            let img = imageops::rotate90(&img);
            img.save(format!("{}/{}", RESOURCES_PATH, self.img_180))?;

            let img = imageops::rotate90(&img);
            img.save(format!("{}/{}", RESOURCES_PATH, self.img_270))?;

            Ok(())
        }
    }
    pub struct Snake {
        pub body: Vec<Point>,
        pub body_color: Color,
        pub head_color: Color,
        pub direction: Option<Direction>,
        pub img: Img,
    }

    impl Snake {
        pub fn new() -> Self {
            Snake {
                body: vec![Point::new()],
                head_color: Color::MAGENTA,
                body_color: MY_GREEN,
                direction: None,
                img: Img::new(),
            }
        }

        pub fn draw(&mut self, ctx: &Context, canvas: &mut graphics::Canvas) -> GameResult {
            let rect = Rect::new(0.0, 0.0, RECT_SIZE, RECT_SIZE);

            let head = &self.body[0];

            let img = match self.direction {
                Some(direction) => match direction {
                    Direction::Up => graphics::Image::from_path(ctx, format!("/{}", self.img.img))?,
                    Direction::Down => {
                        graphics::Image::from_path(ctx, format!("/{}", self.img.img_180))?
                    }
                    Direction::Left => {
                        graphics::Image::from_path(ctx, format!("/{}", self.img.img_270))?
                    }
                    Direction::Right => {
                        graphics::Image::from_path(ctx, format!("/{}", self.img.img_90))?
                    }
                },
                None => graphics::Image::from_path(ctx, format!("/{}", self.img.img))?,
            };
            canvas.draw(&img, Vec2::new(head.x, head.y));

            let segment = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, self.body_color)?;
            for point in self.body[1..].iter() {
                canvas.draw(&segment, Vec2::new(point.x, point.y));
            }

            Ok(())
        }
    }
}
