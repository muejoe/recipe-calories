#![windows_subsystem = "windows"]

mod recipe;
mod gui;

use dioxus::prelude::*;
use gui::RecipeScreen;
use recipe::Recipe;


#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    RecipeScreen {},
}

//const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/main.css");


fn main() {
    dioxus::launch(main_element);
}

fn main_element() -> Element {
    use_context_provider(|| Signal::new(init_recipe()));

    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

fn init_recipe() -> Recipe {
    let mut recipe = Recipe::new();
    recipe.add_new_ingredient();
    recipe.add_new_ingredient();
    recipe.add_new_ingredient();
    recipe
}



