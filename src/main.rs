// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc, sync::{Arc, RwLock}};

use items::Items;
use materials::Materials;
use orders::Orders;
use project::Projects;
use slint::{ModelRc, StandardListViewItem, VecModel};

mod items;
mod materials;
mod orders;
mod project;

slint::include_modules!();

#[derive(Debug, Clone, Copy)]
enum Actions {
    Items,
    Orders,
    Materials,
    Projects,
    New,
    Search,
    Save,
    Back,
}

#[derive(Debug, Clone, Copy)]
enum AppState {
    Items(items::States),
    Orders(orders::States),
    Materials(materials::States),
    Projects(project::States),
}

#[derive(Debug, Clone)]
struct App {
    state: AppState,
    back_stack: Vec<AppState>,
    search: String,
    data: Vec<Vec<String>>,
    clipboard_id: Option<i64>,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::Projects(project::States::Default),
            search: String::new(),
            data: Vec::<Vec<String>>::new(),
            back_stack: Vec::new(),
            clipboard_id: None,
        }
    }

    fn action(&mut self, action: Actions) {
        match action {
            Actions::Items => {
                self.back_stack.push(self.state);
                self.data = Vec::<Vec<String>>::new();
                self.set_state(AppState::Items(items::States::Default));
            }
            Actions::Orders => {
                self.back_stack.push(self.state);
                self.data = Vec::<Vec<String>>::new();
                self.set_state(AppState::Orders(orders::States::Default));
            }
            Actions::Materials => {
                self.back_stack.push(self.state);
                self.data = Vec::<Vec<String>>::new();
                self.set_state(AppState::Materials(materials::States::Default));
            }
            Actions::Projects => {
                self.back_stack.push(self.state);
                self.data = Vec::<Vec<String>>::new();
                self.set_state(AppState::Projects(project::States::Default));
            }
            Actions::New => {
                self.back_stack.push(self.state);
                self.data = Vec::<Vec<String>>::new();
                match self.state {
                    AppState::Items(_) => {
                        self.set_state(AppState::Items(items::States::New));
                    },
                    AppState::Orders(_) => {
                        self.set_state(AppState::Orders(orders::States::New));
                    },
                    AppState::Materials(_) => {
                        self.set_state(AppState::Materials(materials::States::New));
                    },
                    AppState::Projects(_) => {
                        self.set_state(AppState::Projects(project::States::New));
                    },
                };
            },
            Actions::Search => {
                match self.state {
                    AppState::Items(_) => Items::search(self),
                    AppState::Orders(_) => Orders::search(self),
                    AppState::Materials(_) => Materials::search(self),
                    AppState::Projects(_) => Projects::search(self),
                };
            }
            Actions::Save => todo!(),
            Actions::Back => {
                let state = match self.back_stack.pop() {
                    Some(new_state) => {
                        self.data = Vec::<Vec<String>>::new();
                        println!("{:?}", new_state);
                        new_state
                    }
                    None => return,
                };
                self.set_state(state);
            }
        }
    }

    fn set_state(&mut self, state: AppState) {
        self.state = state;
    }
}

pub(crate) trait Entity<const N: usize> {
    fn headers() -> [&'static str; N];
    fn search(state: &mut App);
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Arc::new(RwLock::new(App::new()));
    let ui = Rc::new(RwLock::new(AppWindow::new()?));

    let app_clone = Arc::clone(&app);
    let ui_clone = Rc::clone(&ui);
    ui.write().unwrap().on_items(move || {
        let mut app = app_clone.write().unwrap();
        let ui = ui_clone.read().unwrap();
        print!("from {:?} ", app.state);
        app.action(Actions::Items);
        ui.set_state(format!("{:?}", app.state).into());
        println!("to {:?}", app.state);
    });

    let app_clone = Arc::clone(&app);
    let ui_clone = Rc::clone(&ui);
    ui.write().unwrap().on_orders(move || {
        let mut app = app_clone.write().unwrap();
        let ui = ui_clone.read().unwrap();
        print!("from {:?} ", app.state);
        app.action(Actions::Orders);
        ui.set_state(format!("{:?}", app.state).into());
        println!("to {:?}", app.state);
    });

    let app_clone = Arc::clone(&app);
    let ui_clone = Rc::clone(&ui);
    ui.write().unwrap().on_materials(move || {
        let mut app = app_clone.write().unwrap();
        let ui = ui_clone.read().unwrap();
        print!("from {:?} ", app.state);
        app.action(Actions::Materials);
        ui.set_state(format!("{:?}", app.state).into());
        println!("to {:?}", app.state);
    });

    let app_clone = Arc::clone(&app);
    let ui_clone = Rc::clone(&ui);
    ui.write().unwrap().on_projects(move || {
        let mut app = app_clone.write().unwrap();
        let ui = ui_clone.read().unwrap();
        print!("from {:?} ", app.state);
        app.action(Actions::Projects);
        ui.set_state(format!("{:?}", app.state).into());
        println!("to {:?}", app.state);
    });

    let app_clone = Arc::clone(&app);
    let ui_clone = Rc::clone(&ui);
    ui.write().unwrap().on_new(move || {
        let mut app = app_clone.write().unwrap();
        let ui = ui_clone.read().unwrap();
        print!("from {:?} ", app.state);
        app.action(Actions::New);
        ui.set_state(format!("{:?}", app.state).into());
        println!("to {:?}", app.state);
    });

    let app_clone = Arc::clone(&app);
    let ui_clone = Rc::clone(&ui);
    ui.write().unwrap().on_search(move || {
        let mut app = app_clone.write().unwrap();
        let ui = ui_clone.read().unwrap();
        print!("from {:?} ", app.state);
        app.action(Actions::Search);
        ui.set_state(format!("{:?}", app.state).into());
        println!("to {:?}", app.state);
    });

    let app_clone = Arc::clone(&app);
    let ui_clone = Rc::clone(&ui);
    ui.write().unwrap().on_save(move || {
        let mut app = app_clone.write().unwrap();
        let ui = ui_clone.read().unwrap();
        print!("from {:?} ", app.state);
        app.action(Actions::Save);
        ui.set_state(format!("{:?}", app.state).into());
        println!("to {:?}", app.state);
    });

    let app_clone = Arc::clone(&app);
    let ui_clone = Rc::clone(&ui);
    ui.write().unwrap().on_back(move || {
        let mut app = app_clone.write().unwrap();
        let ui = ui_clone.read().unwrap();
        print!("from {:?} ", app.state);
        app.action(Actions::Back);
        ui.set_state(format!("{:?}", app.state).into());
        println!("to {:?}", app.state);
    });


    // experimental from here
    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    for r in 1..101 {
        let items = Rc::new(VecModel::default());

        for c in 1..5 {
            items.push(slint::format!("Item {r}.{c}").into());
        }

        row_data.push(items.into());
    }

    ui.write().unwrap().set_row_data(row_data.clone().into());
    // to here


    let _ = ui.read().unwrap().run();

    Ok(())
}
