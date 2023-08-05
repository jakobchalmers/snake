use ggez::conf::WindowSetup;
use ggez::event;
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::graphics::{Text, TextFragment, TextLayout};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::Context;
use ggez::{self, GameResult};
use image::imageops;
use rand::Rng;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

const RECT_SIZE: f32 = 40.0;
// const RECT_SIZE: f32 = 60.0;
const SCREEN_SIZE: f32 = 800.0;
const SLEEP_TIME: u32 = 100;
const MY_GREEN: Color = Color::new(5.0 / 255.0, 155.0 / 255.0, 4.0 / 255.0, 1.0);
const RESOURCES_PATH: &'static str = "C:/Users/jakob/OneDrive/Rust/my_snake/resources";

#[derive(Debug)]
enum ConversionError {
    InvalidKey,
}

#[derive(Debug, Clone, Copy)]
struct Apple {
    x: f32,
    y: f32,
    color: Color,
    img_name: &'static str,
}

impl Apple {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Apple {
            x: rng.gen::<f32>() * (SCREEN_SIZE - RECT_SIZE),
            y: rng.gen::<f32>() * (SCREEN_SIZE - RECT_SIZE),
            color: Color::RED,
            img_name: "apple.png",
        }
    }

    fn resize(self) -> GameResult {
        resize_img(self.img_name)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &Context, canvas: &mut graphics::Canvas) -> GameResult {
        let rect = Rect::new(self.x, self.y, RECT_SIZE, RECT_SIZE);
        let rect_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, self.color)?;
        canvas.draw(&rect_mesh, Vec2::new(0.0, 0.0));

        let img = graphics::Image::from_path(ctx, format!("/{}", self.img_name))?;
        canvas.draw(&img, Vec2::new(self.x, self.y));

        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Direction {
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

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new() -> Self {
        let center = (SCREEN_SIZE - RECT_SIZE) / 2.0;
        Point {
            x: center,
            y: center,
        }
    }

    fn update(&mut self, direction: Direction) {
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
struct Img {
    // img_dir: &'static str,
    img: &'static str,
    img_90: &'static str,
    img_180: &'static str,
    img_270: &'static str,
}

impl Img {
    fn new() -> Self {
        Img {
            // img_dir: "C:/Users/jakob/OneDrive/Rust/my_snake/resources/",
            img: "snake.png",
            img_90: "snake_90.png",
            img_180: "snake_180.png",
            img_270: "snake_270.png",
        }
    }

    fn resize(self) -> GameResult {
        resize_img(self.img)?;

        Ok(())
    }

    fn rotate(self) -> GameResult {
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
struct Snake {
    body: Vec<Point>,
    body_color: Color,
    head_color: Color,
    direction: Option<Direction>,
    img: Img,
}

impl Snake {
    fn new() -> Self {
        Snake {
            body: vec![Point::new()],
            head_color: Color::MAGENTA,
            body_color: MY_GREEN,
            direction: None,
            img: Img::new(),
        }
    }

    fn draw(&mut self, ctx: &Context, canvas: &mut graphics::Canvas) -> GameResult {
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

struct GameState {
    gameover: bool,
    snake: Snake,
    apple: Apple,
}

impl GameState {
    fn new() -> Self {
        GameState {
            gameover: false,
            snake: Snake::new(),
            apple: Apple::new(),
        }
    }
}

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


impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        thread::sleep(Duration::from_millis(SLEEP_TIME.into()));

        if self.gameover {
            ctx.request_quit();
        }

        if let Some(direction) = self.snake.direction {
            let mut head = self.snake.body[0];

            // Check ate itself
            for point in self.snake.body[1..].iter() {
                if head.x + RECT_SIZE > point.x
                    && head.x < point.x + RECT_SIZE
                    && head.y + RECT_SIZE > point.y
                    && head.y < point.y + RECT_SIZE
                {
                    self.gameover = true;
                    break;
                }
            }

            // Move head
            head.update(direction);
            self.snake.body.insert(0, head);

            // Check ate apple
            if head.x + RECT_SIZE >= self.apple.x
                && head.x <= self.apple.x + RECT_SIZE
                && head.y + RECT_SIZE >= self.apple.y
                && head.y <= self.apple.y + RECT_SIZE
            {
                self.apple = Apple::new();
            } else {
                let _ = self.snake.body.pop();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        self.apple.draw(ctx, &mut canvas)?;
        self.snake.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Escape => ctx.request_quit(),
                _ => match Direction::try_from(keycode) {
                    Ok(direction) => self.snake.direction = Some(direction),
                    Err(e) => eprintln!("Can't convert keycode to direction: {:?}", e),
                },
            }
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Snake", "Jakob")
        // .window_setup(ggez::conf::WindowSetup::default().title("Snake"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE, SCREEN_SIZE))
        .add_resource_path(std::path::PathBuf::from("./resources"));

    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Snake");

    let state = GameState::new();
    state.snake.img.resize()?;
    state.snake.img.rotate()?;
    state.apple.resize()?;

    event::run(ctx, event_loop, state);

    Ok(())
}
