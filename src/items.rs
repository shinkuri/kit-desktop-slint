use std::error::Error;

use reqwest::Client;
use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::{AppState, Entity};

#[derive(Deserialize)]
struct ItemsORM {
    id: i64,
    name: String,
    quantity: i64,
    description: String,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum States {
    Default,
    New,
    Searched,
    Error,
}

pub struct Items;

impl Entity<3> for Items {
    fn headers() -> [&'static str; 3] {
        ["Name", "Quantity", "Note"]
    }

    fn search(state: &mut crate::App) {
        if !matches!(
            state.state,
            AppState::Items(States::Default) | AppState::Items(States::Searched)
        ) {
            return;
        }

        let search_result = search_by_string(state.search.clone());
        match search_result {
            Ok(search_result) => {
                state.data = search_result;

                state.back_stack.push(state.state);
                state.state = AppState::Items(States::Searched);
            }
            Err(err) => {
                state.back_stack.push(state.state);
                state.state = AppState::Items(States::Error);

                println!("{}", err);
            }
        }
    }
}

fn search_by_string(sequence: String) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let rt = Runtime::new().unwrap();

    let result = rt.block_on(api_items_search(sequence));

    let items = result?;

    Ok(items
        .iter()
        .map(|item| {
            vec![
                item.name.clone(),
                item.quantity.to_string(),
                item.description.clone(),
            ]
        })
        .collect())
}

async fn api_items_search(sequence: String) -> Result<Vec<ItemsORM>, Box<dyn Error>> {
    let client = Client::new();

    let response = client
        .get("http://localhost:8080/items/search")
        .query(&[("sequence", sequence)])
        .send()
        .await?
        .json::<Vec<ItemsORM>>()
        .await?;

    Ok(response)
}
