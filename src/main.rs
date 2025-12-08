use ggez::*;
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;

// ========================================
// CONSTANTS - Configuration values
// ========================================

// Physics constants
const GRAVITY: f32 = 0.5;
const JUMP_FORCE: f32 = -10.0;
const PIPE_SPEED: f32 = 5.0;
const PIPE_GAP: f32 = 150.0;

// Screen dimensions
const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 400.0;

// Bird constants
const BIRD_START_X: f32 = 100.0;
const BIRD_START_Y: f32 = SCREEN_HEIGHT / 2.0;

// Pipe constants
const INITIAL_PIPE_COUNT: usize = 3;
const PIPE_SPACING: f32 = 300.0;
const PIPE_SPAWN_DISTANCE: f32 = 300.0;
const PIPE_OFF_SCREEN_X: f32 = -100.0;
const PIPE_MIN_GAP_Y: f32 = 100.0;
const PIPE_MAX_GAP_Y: f32 = 300.0;
const PIPE_TOP_OFFSET: f32 = 200.0;

// Asset paths
const ASSET_BIRD: &str = "/sprites/bird.png";
const ASSET_PIPE: &str = "/sprites/pipe.png";
const ASSET_BACKGROUND: &str = "/sprites/background.png";

// UI constants
const SCORE_TEXT_X: f32 = 10.0;
const SCORE_TEXT_Y: f32 = 10.0;
const GAME_OVER_TEXT_X_OFFSET: f32 = 100.0;
const GAME_OVER_TEXT_Y: f32 = SCREEN_HEIGHT / 2.0;

// Colors
const SKY_BLUE_R: u8 = 135;
const SKY_BLUE_G: u8 = 206;
const SKY_BLUE_B: u8 = 235;

// ========================================
// ENUMS - Game state representation
// ========================================

// Game state enum - represents current game status
#[derive(Debug, Clone, Copy, PartialEq)]
enum GameStatus {
    Playing,
    GameOver,
}

// ========================================
// STRUCTS - Data structures
// ========================================

// Bird struct - OWNERSHIP: owns all its fields
struct Bird {
    x: f32,
    y: f32,
    velocity: f32,
    image: Image,
}

impl Bird {
    // BORROWING: &mut Context borrowed mutably
    // OWNERSHIP: Returns Bird, transferring ownership to caller
    fn new(ctx: &mut Context) -> GameResult<Bird> {
        Ok(Bird {
            x: BIRD_START_X,
            y: BIRD_START_Y,
            velocity: 0.0,
            image: Image::from_path(ctx, ASSET_BIRD)?,
        })
    }

    // BORROWING: &mut self = borrow Bird mutably
    fn update(&mut self) {
        self.velocity += GRAVITY;
        self.y += self.velocity;
    }

    // BORROWING: &mut self = borrow Bird mutably
    fn jump(&mut self) {
        self.velocity = JUMP_FORCE;
    }

    // BORROWING: &self = borrow Bird immutably
    fn draw(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(
            &self.image,
            DrawParam::default().dest([self.x, self.y]),
        );
    }

    // Check if bird is out of bounds
    fn is_out_of_bounds(&self) -> bool {
        self.y < 0.0 || self.y > SCREEN_HEIGHT
    }
}

// Pipe struct - OWNERSHIP: owns position and image data
struct Pipe {
    x: f32,
    gap_y: f32,
    image: Image,
}

impl Pipe {
    // BORROWING: &mut Context borrowed mutably
    fn new(ctx: &mut Context, x: f32) -> GameResult<Pipe> {
        let mut rng = rand::thread_rng();
        Ok(Pipe {
            x,
            gap_y: rng.gen_range(PIPE_MIN_GAP_Y..PIPE_MAX_GAP_Y),
            image: Image::from_path(ctx, ASSET_PIPE)?,
        })
    }

    // BORROWING: &mut self = borrow Pipe mutably
    fn update(&mut self) {
        self.x -= PIPE_SPEED;
    }

    // BORROWING: &self = borrow Pipe immutably
    fn draw(&self, canvas: &mut graphics::Canvas) {
        // Top pipe (flipped 180 degrees)
        canvas.draw(
            &self.image,
            DrawParam::default()
                .dest([self.x, self.gap_y - PIPE_GAP / 2.0 - PIPE_TOP_OFFSET])
                .rotation(std::f32::consts::PI),
        );

        // Bottom pipe
        canvas.draw(
            &self.image,
            DrawParam::default()
                .dest([self.x, self.gap_y + PIPE_GAP / 2.0]),
        );
    }

    fn is_off_screen(&self) -> bool {
        self.x < PIPE_OFF_SCREEN_X
    }
}

// GameState struct - OWNERSHIP: owns all game objects
struct GameState {
    bird: Bird,
    pipes: Vec<Pipe>,
    background: Image,
    score: i32,
    status: GameStatus,
}

impl GameState {
    // BORROWING: &mut Context borrowed mutably
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let background = Image::from_path(ctx, ASSET_BACKGROUND)?;
        let bird = Bird::new(ctx)?;

        let mut pipes = Vec::new();
        for i in 0..INITIAL_PIPE_COUNT {
            pipes.push(Pipe::new(ctx, SCREEN_WIDTH + i as f32 * PIPE_SPACING)?);
        }

        Ok(GameState {
            bird,
            pipes,
            background,
            score: 0,
            status: GameStatus::Playing,
        })
    }
}

// ========================================
// EVENT HANDLER - Game loop implementation
// ========================================

impl event::EventHandler<GameError> for GameState {
    // BORROWING: &mut self and &mut Context
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Match on enum to check game status
        match self.status {
            GameStatus::Playing => {
                // Update bird
                self.bird.update();

                // Update pipes
                for pipe in &mut self.pipes {
                    pipe.update();
                }

                // Remove off-screen pipes and add new ones
                self.pipes.retain(|pipe| !pipe.is_off_screen());
                
                if let Some(last_pipe) = self.pipes.last() {
                    if last_pipe.x < SCREEN_WIDTH - PIPE_SPAWN_DISTANCE {
                        self.pipes.push(Pipe::new(ctx, SCREEN_WIDTH + PIPE_TOP_OFFSET)?);
                    }
                }

                // Check collisions
                if self.bird.is_out_of_bounds() {
                    self.status = GameStatus::GameOver;
                    println!("Game Over! Score: {}", self.score);
                }
            }
            GameStatus::GameOver => {
                // Do nothing when game is over
            }
        }

        Ok(())
    }

    // BORROWING: &mut self and &mut Context
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, 
            Color::from_rgb(SKY_BLUE_R, SKY_BLUE_G, SKY_BLUE_B)
        );

        // Draw background
        canvas.draw(&self.background, DrawParam::default());

        // Draw pipes
        for pipe in &self.pipes {
            pipe.draw(&mut canvas);
        }

        // Draw bird
        self.bird.draw(&mut canvas);

        // Draw score
        let score_text = graphics::Text::new(format!("Score: {}", self.score));
        canvas.draw(
            &score_text,
            DrawParam::default()
                .dest([SCORE_TEXT_X, SCORE_TEXT_Y])
                .color(Color::WHITE),
        );

        // Match on enum to display game over message
        if self.status == GameStatus::GameOver {
            let game_over_text = graphics::Text::new("Game Over! Press R to restart");
            canvas.draw(
                &game_over_text,
                DrawParam::default()
                    .dest([SCREEN_WIDTH / 2.0 - GAME_OVER_TEXT_X_OFFSET, GAME_OVER_TEXT_Y])
                    .color(Color::RED),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    // BORROWING: &mut self and &mut Context
    fn key_down_event(
        &mut self, 
        _ctx: &mut Context, 
        input: KeyInput, 
        _repeat: bool
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Space => {
                    if self.status == GameStatus::Playing {
                        self.bird.jump();
                    }
                }
                KeyCode::R => {
                    if self.status == GameStatus::GameOver {
                        *self = GameState::new(_ctx)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// ========================================
// MAIN - Entry point
// ========================================

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("flappy_bird", "YourName")
        .window_setup(conf::WindowSetup::default().title("Flappy Bird"))
        .window_mode(conf::WindowMode::default()
            .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("./resources")
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}