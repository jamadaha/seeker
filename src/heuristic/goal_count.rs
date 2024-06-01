use super::Heuristic;
use pddllib::{state::State, task::Task};

#[derive(Default)]
pub struct GoalCount {}

impl Heuristic for GoalCount {
    fn estimate(&self, task: &Task, state: &State) -> usize {
        task.goal
            .iter()
            .filter(|(fact, value)| state.has_fact(task, fact) != *value)
            .count()
    }
}
