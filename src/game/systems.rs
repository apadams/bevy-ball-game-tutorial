use bevy::prelude::*;
use crate::game::SimulationState;

pub fn pause_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            next_sim_state.set(SimulationState::Paused);
            println!("Simulation Paused!!")
        }
        if simulation_state.0 == SimulationState::Paused {
            next_sim_state.set(SimulationState::Running);
            println!("Simulation Running!");
        }
    }
}