use rusty_engine::{prelude::*, game};

/// Game and player state data
struct GameState {
    current_score: u32,
    ferris_index: i32,
    // high_score: u32,
    // enemy_labels: Vec<String>,
    // spawn_timer: Timer,
}

/// Default initialization for game state
impl Default for GameState {
    fn default() -> Self {
        Self {
            // high_score: 0,
            current_score: 0,
            ferris_index: 0,
            // enemy_labels: Vec::new(),
            // spawn_timer: Timer::from_seconds(1.0, false)
        }
    }
}

fn main() {
    let mut game = Game::new();

    // Register the player sprite
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.collision = true;

    // Adds a new logic/behavior into our game
    game.add_logic(game_logic);

    // Starts the game
    game.run(GameState::default());
}


/// Returns a game logic to be used in our game
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Handling collision events
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for sprite in [event.pair.0, event.pair.1] {
                if sprite != "player" {
                    engine.sprites.remove(&sprite);
                }
            }
            game_state.current_score +=1;
            println!("Current score: {}", game_state.current_score);
        }
    }

    // Handling keyboard events
    let player = engine.sprites.get_mut("player").unwrap();
    const MOV_SPEED: f32 = 10.0;
    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        player.translation.y += MOV_SPEED + engine.delta_f32;
        player.rotation = UP;
    }

    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        player.translation.y -= MOV_SPEED + engine.delta_f32;
        player.rotation = DOWN;
    }

    if engine.keyboard_state.pressed_any(&[KeyCode::Right, KeyCode::D]) {
        player.translation.x += MOV_SPEED + engine.delta_f32;
        player.rotation = RIGHT;
    }

    if engine.keyboard_state.pressed_any(&[KeyCode::Left, KeyCode::A]) {
        player.translation.x -= MOV_SPEED + engine.delta_f32;
        player.rotation = LEFT;
    }

    // Handling mouse events
    if engine.mouse_state.pressed(MouseButton::Left) {
        // If we could get the mouse location
        if let Some(mouse_location) = engine.mouse_state.location() {
            // Create the new car name using our index counting
            let label = format!("Ferris{} ", game_state.ferris_index);
            game_state.ferris_index += 1;
            
            // Create a new car with the new index
            let car = engine.add_sprite(label, SpritePreset::RacingCarRed);
            car.translation = mouse_location;
            car.collision = true;
        }
    }
}