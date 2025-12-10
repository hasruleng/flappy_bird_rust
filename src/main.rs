// Cargo.toml dependencies needed:
// [dependencies]
// macroquad = "0.4"

use macroquad::prelude::*;

const WIDTH: f32 = 267.0;
const HEIGHT: f32 = 400.0;
const NUMBER_OF_PAIR_OF_PIPES: usize = 77777;

// Bird struct
struct Bird {
    x: f32,
    y: f32,
    velocity: f32,
}

impl Bird {
    fn new() -> Self {
        Bird {
            x: 100.0,
            y: HEIGHT / 2.0,
            velocity: 0.0,
        }
    }

    fn update(&mut self) {
        self.velocity += 0.5; // Gravity
        self.y += self.velocity;
    }

    fn jump(&mut self) {
        self.velocity = -10.0;
    }

    fn draw(&self, texture: &Texture2D) {
        draw_texture(texture, self.x, self.y, WHITE);
    }
}

// Pipe pair struct
struct PairOfPipes {
    x: f32,
    top_y: f32,
    bottom_y: f32,
}

impl PairOfPipes {
    fn new(x: f32) -> Self {
        let top_bottom_y = rand::gen_range(150.0, 300.0);
        
        PairOfPipes {
            x,
            top_y: top_bottom_y - 320.0,
            bottom_y: top_bottom_y + 100.0,
        }
    }

    fn update(&mut self) {
        self.x -= 5.0;
    }

    fn draw(&self, texture: &Texture2D) {
        // Draw top pipe (flipped)
        draw_texture_ex(
            texture,
            self.x,
            self.top_y,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
        // Draw bottom pipe
        draw_texture(texture, self.x, self.bottom_y, WHITE);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load textures (you'll need these assets)
    let bird_texture: Texture2D = load_texture("./resources/sprites/bird.png")
        .await
        .unwrap();
    let pipe_texture: Texture2D = load_texture("./resources/sprites/pipe.png")
        .await
        .unwrap();
    let bg_texture: Texture2D = load_texture("./resources/sprites/background.png")
        .await
        .unwrap();

    // Create bird (ownership)
    let mut bird = Bird::new();
    
    // Create pipes (ownership of vector)
    let mut pipes: Vec<PairOfPipes> = (0..NUMBER_OF_PAIR_OF_PIPES)
        .map(|i| PairOfPipes::new(WIDTH + i as f32 * 300.0))
        .collect();

    loop {
        // Event handling - check for space key
        if is_key_pressed(KeyCode::Space) {
            bird.jump(); // Mutable borrow of bird
        }

        // Check for quit (Escape or close button)
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Update bird (mutable borrow)
        bird.update();

        // Update pipes (mutable borrow of each pipe)
        for pipe in &mut pipes {
            pipe.update();
        }

        // Render
        draw_texture(&bg_texture, 0.0, 0.0, WHITE);
        bird.draw(&bird_texture); // Immutable borrow
        
        // Immutable borrow of pipes for rendering
        for pipe in &pipes {
            pipe.draw(&pipe_texture);
        }

        next_frame().await;
    }
}