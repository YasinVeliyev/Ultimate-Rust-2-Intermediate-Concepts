use rand::prelude::*;
use rusty_engine::{
    game,
    prelude::{bevy::prelude::default, *},
};
struct GameState {
    high_score: u32,
    score: u32,
    enemy_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            enemy_index: 0,
            spawn_timer: Timer::from_seconds(1.0, true),
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.5);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.collision = true;

    let score = game.add_text("score", "Score:0");
    score.translation = Vec2::new(520., 320.);

    let high_score = game.add_text("high_score", "High Score:0");
    high_score.translation = Vec2::new(-520., 320.);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let score = engine.texts.get_mut("score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 100.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 50.0;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 100.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 50.0;

    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in event.pair {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.score += 1;

            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);

            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High score: {}", game_state.high_score);
            }
        }
    }

    let player = engine.sprites.get_mut("player").unwrap();
    // player.translation.x += 100.0 * engine.delta_f32;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += 100.0 * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= 100.0 * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= 100.0 * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += 100.0 * engine.delta_f32;
    }

    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("Enemy Car {}", game_state.enemy_index);
            game_state.enemy_index += 1;

            let enemy = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
            enemy.translation = mouse_location;
            enemy.collision = true;
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("Enemy Car {}", game_state.enemy_index);
        game_state.enemy_index += 1;

        let enemy = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
        enemy.translation.x = thread_rng().gen_range(-500.0..500.);
        enemy.translation.y = thread_rng().gen_range(-325.0..325.);
        enemy.collision = true;
    }

    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score :0".to_owned();
    }
}
