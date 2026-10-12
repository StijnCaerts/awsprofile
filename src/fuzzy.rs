use skim::prelude::*;

pub fn fuzzy_select(items: Vec<String>) -> Option<String> {
    let options = SkimOptionsBuilder::default()
        .height("50%")
        .multi(false)
        .prompt("Select AWS Profile> ")
        .build()
        .unwrap();

    let output = Skim::run_items(options, items).unwrap();

    if let Some(selected) = output.selected_items.first() {
        Some(selected.output().to_string())
    } else {
        None
    }
}
