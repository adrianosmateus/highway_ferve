use rusty_engine::{prelude::*, game};

/// Game and player state data
struct GameState {
    score: u32,
    ferris_index: i32,
    high_score: u32,
    // enemy_labels: Vec<String>,
    // spawn_timer: Timer,
}

/// Default initialization for game state
impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
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

    // Register a score text
    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    // Register a highscore text
    let highscore = game.add_text("highscore", "High-Score: 0");
    highscore.translation = Vec2::new(-520.0, 320.0);

    // Adds a new logic/behavior into our game
    game.add_logic(game_logic);

    // Starts the game
    game.run(GameState::default());
}


/// Returns a game logic to be used in our game
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Handling collision events
    for event in engine.collision_events.drain(..) {
        // Check if one of the collided sprites is the player
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // Remove the sprite what is not the player
            for sprite in [event.pair.0, event.pair.1] {
                if sprite != "player" {
                    engine.sprites.remove(&sprite);
                }
            }

            // Increase game score and update screen labels
            game_state.score +=1;
            
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);
            
            if (game_state.score > game_state.high_score) {
                game_state.high_score = game_state.score;
                let highscore = engine.texts.get_mut("highscore").unwrap();
                highscore.value = format!("High-Score: {}", game_state.score);
            }
            
            println!("Current score: {}", game_state.score);
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

    if engine.keyboard_state.pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = format!("Score: 0");
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

// fn update_score(engine: Engine, game_state: GameState) {
//     game_state.score +=1;
            
//     let score = engine.texts.get_mut("score").unwrap();
//     score.value = format!("Score: {}", game_state.score);
    
//     if (game_state.score > game_state.high_score) {
//         game_state.score = game_state.score;
//         let highscore = engine.texts.get_mut("highscore").unwrap();
//         highscore.value = format!("High-Score: {}", game_state.score);
//     }
    
//     println!("Current score: {}", game_state.score);
// }