use bevy::prelude::*;

// use crate::DEBUG_MODE;
use crate::GameState;
use crate::TIME_STEP;

pub fn score_system(
    mut game_state: ResMut<GameState>,
) {
   game_state.elapsed_time += TIME_STEP as f64;
//    info!("current elapsed time is {}", game_state.elapsed_time)
}

