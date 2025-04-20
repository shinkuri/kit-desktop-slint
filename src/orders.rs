use std::error::Error;

use serde::Deserialize;

use crate::{AppState, Entity};

#[derive(Deserialize)]
struct OrdersORM {
    id: i64,
    material: i64,
    quantity: i64,
    cost: i64,
    note: String,
    is_complete: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum States {
    Default,
    New,
    Searched,
    Error,
}

pub struct Orders;

impl Entity<6> for Orders {
    fn headers() -> [&'static str; 6] {
        ["Project", "Item", "Quantity", "Cost", "Note", "Completed"]
    }

    fn search(state: &mut crate::App) {
        if !matches!(
            state.state,
            AppState::Orders(States::Default) | AppState::Orders(States::Searched)
        ) {
            return;
        }

        let search_result = search_by_string(state.search.clone());
        match search_result {
            Ok(search_result) => {
                state.data = search_result;

                state.back_stack.push(state.state);
                state.state = AppState::Orders(States::Searched);
            }
            Err(err) => {
                state.back_stack.push(state.state);
                state.state = AppState::Orders(States::Error);

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
            "data16".to_string(),
        ],
        vec![
            "data21".to_string(),
            "data22".to_string(),
            "data23".to_string(),
            "data14".to_string(),
            "data15".to_string(),
            "data26".to_string(),
        ],
    ])
}
