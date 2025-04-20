use std::error::Error;

use serde::Deserialize;

use crate::{AppState, Entity};

#[derive(Deserialize)]
struct ProjectsORM {
    id: i64,
    name: String,
    description: String,
    is_complete: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum States {
    Default,
    New,
    Searched,
    Error,
}

pub struct Projects;

impl Entity<3> for Projects {
    fn headers() -> [&'static str; 3] {
        ["Name", "Description", "Completed"]
    }

    fn search(state: &mut crate::App) {
        if !matches!(
            state.state,
            AppState::Projects(States::Default) | AppState::Projects(States::Searched)
        ) {
            return;
        }

        let search_result = search_by_string(state.search.clone());
        match search_result {
            Ok(search_result) => {
                state.data = search_result;

                state.back_stack.push(state.state);
                state.state = AppState::Projects(States::Searched);
            }
            Err(err) => {
                state.back_stack.push(state.state);
                state.state = AppState::Projects(States::Error);

                println!("{}", err);
            }
        }
    }
}

fn search_by_string(sequence: String) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    Ok(vec![
        vec![sequence, "data12".to_string(), "data13".to_string()],
        vec![
            "data21".to_string(),
            "data22".to_string(),
            "data23".to_string(),
        ],
    ])
}
