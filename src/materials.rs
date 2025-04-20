use std::error::Error;

use serde::Deserialize;

use crate::{AppState, Entity};

#[derive(Deserialize)]
pub struct MaterialsORM {
    id: i64,
    project: i64,
    item: i64,
    quantity: i64,
    description: String,
    is_fulfilled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum States {
    Default,
    New,
    Searched,
    Error,
}

pub struct Materials;

impl Entity<5> for Materials {
    fn headers() -> [&'static str; 5] {
        ["Project", "Item", "Quantity", "Description", "Fulfilled"]
    }

    fn search(state: &mut crate::App) {
        if !matches!(
            state.state,
            AppState::Materials(States::Default) | AppState::Materials(States::Searched)
        ) {
            return;
        }

        let search_result = search_by_string(state.search.clone());
        match search_result {
            Ok(search_result) => {
                state.data = search_result;

                state.back_stack.push(state.state);
                state.state = AppState::Materials(States::Searched);
            }
            Err(err) => {
                state.back_stack.push(state.state);
                state.state = AppState::Materials(States::Error);

                println!("{}", err);
            }
        }
    }
}

fn search_by_string(sequence: String) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    Ok(vec![
        vec![
            sequence,
            "data12".to_string(),
            "data13".to_string(),
            "data14".to_string(),
            "data15".to_string(),
        ],
        vec![
            "data21".to_string(),
            "data22".to_string(),
            "data23".to_string(),
            "data24".to_string(),
            "data25".to_string(),
        ],
    ])
}
