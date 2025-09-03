use dioxus::prelude::*;
use crate::recipe::Recipe;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    RecipeScreen,
    #[route("/about")]
    AboutScreen,
}


#[component]
pub fn RecipeScreen() -> Element {
    let mut recipe = use_context::<Signal<Recipe>>();

    rsx! {
        div {
            h3 { "Rezept berechnen" }
            button {
                style: "position: fixed; top: 0.3rem; right: 0.3rem; z-index: 1000;",
                onclick: move |_| { router().push(Route::AboutScreen); },
                "?"
            }

            button {
                style: "width: 2rem; font-size: 14pt; font-weight:bold;",
                onclick: move |_| { recipe.write().add_new_ingredient(); },
                "+"
            }
            button {
                style: "width: 2rem; font-size: 14pt; font-weight:bold;",
                onclick: move |_| { recipe.write().remove_last_ingredient(); },
                " -  "
            }

            table {
                style: "width: 100%; border-collapse: collapse;",
                
                thead { tr { 
                        th { "Zutat" } th { "Gewicht (g)" } th { "KCal/100g" }
                }}
                tbody { 
                    for (index, ingredient) in recipe.read().ingredients.iter().enumerate() {
                        tr {
                            key: "{index}",
                            td { {input_line(false, ingredient.name.clone(), 
                                move|e| recipe.write().ingredients[index].name = e.value() )} }
                            td { {input_line(true, ingredient.weight_g, 
                                move|e| recipe.write().ingredients[index].weight_g = get_int(e) )} }
                            td { {input_line(true, ingredient.calories_per_100g, 
                                move|e| recipe.write().ingredients[index].calories_per_100g = get_int(e) )} }
                        }
                    }
                }
            }
            div {
                "Kalorien pro 100g im Schnitt: {recipe.read().calc_calorie_factor()}"
            }
        }
    }
}

#[component]
pub fn AboutScreen() -> Element {
    rsx! {
        div {
            h3 { "Über diese Anwendung" }
            div {"Kalorienrechner (c) muejoe"}
            br {}
            a {
                color: "#6e9ad3ff",
                href: "https://github.com/muejoe/recipe-calories",
                "https://github.com/muejoe/recipe-calories"
            }
            br {} br {}
            div {"Nutzbar unter der MIT-Lizenz"}
            br{} br{}
            button {
                onclick: move |_| { router().push(Route::RecipeScreen); },
                "back"
            }
        }
    }
}


pub fn input_line<F, V>(number: bool, value: V, on_input: F) -> Element 
where
    F: FnMut(FormEvent) + 'static, 
    V: std::fmt::Display
{
    let inputtype = if number {"number"} else {"text"};
    rsx!{
        input { 
            r#type: inputtype, 
            value: "{value}",
            oninput: on_input
        }
    }
}

pub fn get_int(event: Event<FormData>) -> u32 {
    event.value().parse().unwrap_or(0)
}