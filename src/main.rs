use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::{self, GameResult};
use snake::constants::{RECT_SIZE, SCREEN_SIZE, SLEEP_TIME};
use snake::utils::{Apple, Direction, Snake, ScoreBoard};
use std::thread;
use std::time::Duration;

struct GameState {
    gameover: bool,
    snake: Snake,
    apple: Apple,
    score: ScoreBoard,
}

impl GameState {
    fn new() -> Self {
        GameState {
            gameover: false,
            snake: Snake::new(),
            apple: Apple::new(),
            score: ScoreBoard::new(),
        }
    }
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
                    // Todo: Make sure this throws you out immidiately
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
                self.score.update();
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
        self.score.draw(ctx, &mut canvas);

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
