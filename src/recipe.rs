
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>
}

impl Recipe {
    pub fn new() -> Self {
        Recipe {name: "New recipe".to_string(), ingredients: Vec::new()}
    }

    pub fn add_new_ingredient(&mut self) {
        self.ingredients.push(Ingredient::new());
    }

    pub fn remove_last_ingredient(&mut self) {
        self.ingredients.pop();
    }

    pub fn calc_calorie_factor(&self) -> u32 {
        let weight: u32 = self.ingredients.iter().map(|i| i.weight_g).sum();
        let calories: u32 = self.ingredients.iter().map(|i| i.calories_per_100g * i.weight_g).sum();
        if weight != 0  {
            calories / weight
        } else {
            0
        }
    }
}

pub struct Ingredient {
    pub name: String,
    pub weight_g: u32,
    pub calories_per_100g: u32,
}

impl Ingredient {
    fn new() -> Self {
        Ingredient { name: "".to_string(), weight_g: 0, calories_per_100g: 0 }
    }
}

impl Default for Ingredient {
    fn default() -> Self {
        Ingredient::new()
    }
}