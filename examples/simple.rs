use tinybit::events::{events, Event, KeyCode, KeyEvent, EventModel};
use tinybit::{
    term_size, Camera, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport,
    WorldPos, WorldSize, Pixel
};

fn main() {
    let (width, height) = term_size().expect("Can't get the term size? Can't play the game!");

    // Viewport
    let viewport_size = ScreenSize::new(width / 2, height / 2);
    let mut viewport = Viewport::new(ScreenPos::new(0, 4), viewport_size);

    // Camera
    let (width, height) = (width as f32, height as f32);
    let camera_size = WorldSize::new(width / 2.0, height / 2.0); let camera_pos =
    WorldPos::new(width, height);
    let mut camera = Camera::new(camera_pos, camera_size);

    // Renderer
    let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
    let mut renderer = Renderer::new(stdout_renderer);

    // Player
    let mut player = ('@', camera_pos);

    for event in events(EventModel::Fps(20)) {
        match event {
            Event::Tick => {
                let pixel = Pixel::new(player.0, camera.to_screen(player.1), None, None);
                viewport.draw_pixel(pixel);
                let _ = renderer.render(&mut viewport);
            }
            Event::Key(KeyEvent { code: KeyCode::Esc, ..  }) => break,
            Event::Key(KeyEvent { code: kc, .. }) => {
                match kc {
                    KeyCode::Left => { player.1.x -= 1.0; }
                    KeyCode::Right => { player.1.x += 1.0; }
                    KeyCode::Up => { player.1.y -= 1.0; }
                    KeyCode::Down => { player.1.y += 1.0; }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {}
        }
    }
}
