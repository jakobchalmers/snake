use ggez::event;
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::{PxScale, Text, TextFragment, TextLayout};
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::{self, GameResult};
use snake::constants::{RECT_SIZE, SCREEN_SIZE, SLEEP_TIME};
use snake::utils::{Apple, Direction, ScoreBoard, Snake};
use std::thread;
use std::time::Duration;

enum GamePage {
    Menu,
    GameOn,
    LeaderBoard,
}

struct GameState {
    game_page: GamePage,
    snake: Snake,
    apple: Apple,
    score: ScoreBoard,
}

impl GameState {
    fn new() -> Self {
        GameState {
            game_page: GamePage::GameOn,
            snake: Snake::new(),
            apple: Apple::new(),
            score: ScoreBoard::new(),
        }
    }

    fn reset(&mut self) {
        self.game_page = GamePage::GameOn;
        self.snake = Snake::new();
        self.apple = Apple::new();
        self.score.reset_score();
    }
}

impl GameState {
    fn draw_home(&self, canvas: &mut Canvas) {
        let text_string = "Press R to restart, S to see score board or Q to quit.".to_string();
        let mut text = Text::new(TextFragment {
            text: text_string,
            color: Some(Color::BLACK),
            scale: Some(PxScale::from(20.0)),
            ..Default::default()
        });
        text.set_layout(TextLayout::center());

        let text_x = SCREEN_SIZE / 2.0;
        let text_y = SCREEN_SIZE / 2.0;
        canvas.draw(&text, Vec2::new(text_x, text_y));
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        thread::sleep(Duration::from_millis(SLEEP_TIME.into()));

        if let Some(direction) = self.snake.direction {
            let mut head = self.snake.body[0];

            // Check ate itself
            for point in self.snake.body[1..].iter() {
                if head.x + RECT_SIZE > point.x
                    && head.x < point.x + RECT_SIZE
                    && head.y + RECT_SIZE > point.y
                    && head.y < point.y + RECT_SIZE
                {
                    // Todo: Make sure this throws you out immediately
                    if self.score.is_highscore() {
                        self.score.insert_highscore();
                        self.game_page = GamePage::LeaderBoard;
                    } else {
                        self.game_page = GamePage::Menu;
                    }
                    break
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
                self.score.increase();
            } else {
                let _ = self.snake.body.pop();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        match self.game_page {
            GamePage::Menu => self.draw_home(&mut canvas),
            GamePage::LeaderBoard => self.score.draw_scoreboard(&mut canvas),
            GamePage::GameOn => {
                self.apple.draw(ctx, &mut canvas)?;
                self.snake.draw(ctx, &mut canvas)?;
                self.score.draw_score(ctx, &mut canvas);
            }
        }

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
                KeyCode::Escape | KeyCode::Q => ctx.request_quit(),
                _ => {},
            }

            match self.game_page {
                GamePage::Menu => match keycode {
                    KeyCode::R => self.reset(),
                    KeyCode::S => self.game_page = GamePage::LeaderBoard,
                    _ => println!("Press another key."),
                },
                GamePage::GameOn => match keycode {
                    KeyCode::M => self.game_page = GamePage::Menu,
                    KeyCode::S => self.game_page = GamePage::LeaderBoard,
                    _ => match Direction::try_from(keycode) {
                        Ok(direction) => self.snake.direction = Some(direction),
                        Err(e) => eprintln!("Can't convert keycode to direction: {:?}", e),
                    },
                },
                GamePage::LeaderBoard => match keycode {
                    KeyCode::M => self.game_page = GamePage::Menu,
                    KeyCode::R => self.reset(),
                    _ => println!("Press another key."),
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
}
