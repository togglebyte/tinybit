# Florp engine: a terminal based game engine

## Example:

```rust
use florp_engine::events::{events, Event, KeyCode, KeyEvent};
use florp_engine::{
    term_size, Camera, DebugOutput, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport,
    WorldPos, WorldSize,
};

fn main() {
    let (width, height) = term_size().expect("Can't get the term size? Can't play the game!");

    // Viewport
    let viewport_size = ScreenSize::new(width / 2, height / 2);
    let mut viewport = Viewport::new(ScreenPos::new(0, 4), viewport_size);

    // Camera
    let camera_size = WorldSize::new(width / 2, height / 2);
    let camera_pos = WorldPos::new(width, height);
    let mut camera = Camera::new(camera_pos, camera_size);
    camera.set_limit(5, 5);

    // Renderer
    let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
    let mut renderer = Renderer::new(stdout_renderer);

    // Player
    let mut player = ('@', camera_pos);

    for event in events(20) {
        match event {
            Event::Tick => {
                let mut pixels = vec![player];

                viewport.draw(&camera, pixels);
                let _ = renderer.render(&mut viewport);
            }
            Event::Key(KeyEvent { code: KeyCode::Esc, ..  }) => break,
            Event::Key(KeyEvent { code: kc, .. }) => {
                match kc {
                    KeyCode::Left => { player.1.x -= 1; }
                    KeyCode::Right => { player.1.x += 1; }
                    KeyCode::Up => { player.1.y -= 1; }
                    KeyCode::Down => { player.1.y += 1; }
                    _ => {}
                }

                // camera.track(player.1); // Add this in to track the player
            }
        }
    }
}
```
