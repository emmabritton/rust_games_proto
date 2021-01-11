use crate::menu::menu_items::ITEMS;
use crate::system::math::WrappedUsize;

pub mod controller;
mod menu_items;
mod renderer;

pub fn print_rules(name: &str) {
    let game = ITEMS
        .iter()
        .map(|(item, list)| {
            let mut full_list = vec![];
            if let Some(list) = list {
                list.iter().for_each(|item| full_list.push(item.clone()));
            }
            full_list.push(item.clone());
            full_list
        })
        .flatten()
        .find(|item| item.code == name)
        .expect("Invalid game");
    println!("{}\n{}", game.name, game.desc);
}

struct State {
    cursor: WrappedUsize,
    subcursor: Option<WrappedUsize>,
}
