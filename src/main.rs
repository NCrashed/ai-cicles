use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct Player {
    x: f32,
    y: f32,
    radius: f32,
    speed: f32,
}

impl Player {
    fn new(x: f32, y: f32) -> Player {
        Player {
            x: x,
            y: y,
            radius: 10.0,
            speed: 5.0,
        }
    }
    fn update(&mut self, dx: f32, dy: f32) {
        self.x += dx * self.speed;
        self.y += dy * self.speed;

        // Keep player inside screen bounds
        if self.x - self.radius < 0.0 {
            self.x = self.radius;
        } else if self.x + self.radius > SCREEN_WIDTH as f32 {
            self.x = SCREEN_WIDTH as f32 - self.radius;
        }

        if self.y - self.radius < 0.0 {
            self.y = self.radius;
        } else if self.y + self.radius > SCREEN_HEIGHT as f32 {
            self.y = SCREEN_HEIGHT as f32 - self.radius;
        }
    }
}

struct BlueCircle {
    x: f32,
    y: f32,
    radius: f32,
    speed_x: f32,
    speed_y: f32,
}

impl BlueCircle {
    fn new(x: f32, y: f32) -> BlueCircle {
        let radius = 15.0;
        let speed = 3.0;
        BlueCircle {
            x: x,
            y: y,
            radius: radius,
            speed_x: speed,
            speed_y: speed,
        }
    }

    fn update(&mut self) {
        // Update position
        self.x += self.speed_x;
        self.y += self.speed_y;

        // Check for collision with screen bounds and reverse speed if necessary
        if self.x - self.radius < 0.0 || self.x + self.radius > SCREEN_WIDTH as f32 {
            self.speed_x = -self.speed_x;
        }
        if self.y - self.radius < 0.0 || self.y + self.radius > SCREEN_HEIGHT as f32 {
            self.speed_y = -self.speed_y;
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player = Player::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0);
    let mut blue_circles = Vec::new();
    for _ in 0..10 {
        blue_circles.push(BlueCircle::new(
            rand::random::<f32>() * SCREEN_WIDTH as f32,
            rand::random::<f32>() * SCREEN_HEIGHT as f32,
        ));
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Update player position based on WASD input
        let mut dx = 0.0;
        let mut dy = 0.0;
        let keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        if keys.contains(&Keycode::W) {
            dy -= 1.0;
        }
        if keys.contains(&Keycode::A) {
            dx -= 1.0;
        }
        if keys.contains(&Keycode::S) {
            dy += 1.0;
        }
        if keys.contains(&Keycode::D) {
            dx += 1.0;
        }
        player.update(dx, dy);

        // Update blue circles
        for blue_circle in blue_circles.iter_mut() {
            blue_circle.update();

            // Check for collision with player
            let x_diff = blue_circle.x - player.x;
            let y_diff = blue_circle.y - player.y;
            let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
            if distance < blue_circle.radius + player.radius {
                player.x = SCREEN_WIDTH as f32 / 2.0;
                player.y = SCREEN_HEIGHT as f32 / 2.0;
            }
        }

        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw player
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas
            .filled_circle(
                player.x as i16,
                player.y as i16,
                player.radius as i16,
                Color::RGB(255, 0, 0),
            )
            .unwrap();

        // Draw blue circles
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        for blue_circle in blue_circles.iter() {
            canvas
                .filled_circle(
                    blue_circle.x as i16,
                    blue_circle.y as i16,
                    blue_circle.radius as i16,
                    Color::RGB(0, 0, 255),
                )
                .unwrap();
        }

        // Render screen
        canvas.present();
    }
}
