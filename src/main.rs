use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, to_string_pretty, Result};
use rust_decimal;
use rust_decimal_macros::dec;

mod game_version;
use game_version::FactorioVersion;
use game_version::Assembler;

/* 
// Struct to represent a recipe
struct Recipe<'a> {
    ingredients: HashMap<&'a str, u32>,
    production_time: u32,
    //total_raw: HashMap<&'a str, u32>
}

// Function to calculate the needed resources for a given recipe
fn calculate_resources(recipes: &HashMap<&str, Recipe>, recipe_name: &str, quantity: u32, time_unit: &str) {
    // Check if the recipe exists
    if let Some(recipe) = recipes.get(recipe_name) {
        println!("Calculating resources for recipe: {}", recipe_name);

        // Get the recipe's information
        let ingredients = &recipe.ingredients;
        let production_time = recipe.production_time as f32;

        // Adjust production time based on the time unit
        let adjusted_production_time = if time_unit == "minute" {
            production_time / 60.0
        } else {
            production_time
        };

        // Calculate the needed resources for each ingredient per unit of time
        for (ingredient, ingredient_quantity) in ingredients.iter() {
            let needed_quantity = (ingredient_quantity * quantity) as f32 / adjusted_production_time;
            println!("{}: {} per {}", ingredient, needed_quantity, time_unit);

            // Check if the ingredient is a recipe itself
            if recipes.contains_key(ingredient) {
                calculate_resources(recipes, ingredient, needed_quantity as u32, time_unit);
            }
        }
    } else {
        println!("Recipe not found.");
    }
}
*/
fn main() -> Result<()>{
    let factorio_1_1 = FactorioVersion::factorio_1_1();
    let json = to_string_pretty(&factorio_1_1)?;
    
    let a = dec!(0.1);
    let b = dec!(0.2);
    let c = a + b;
    println!("{}", c);


    let a1 = 0.1;
    let b1 = 0.2;
    let c1 = a1 + b1;
    println!("{}", c1);

    
    println!("{}", json);
    Ok(())
    /*let assembler = Assembler{
        craft_speed: 32,
    };
    let recipe= modules::Recipe{
        production_time: 10,
    };
    let test = Production::builder()
        .set_assembler(&assembler)
        .set_recipe(&recipe)
        .build();

    // HashMap to store the recipe and its ingredients
    let mut recipes: HashMap<&str, Recipe> = HashMap::new();

    // Adding recipes and their ingredients with production time
    let iron_plate = Recipe {
        ingredients: [("iron_ore", 1)].iter().cloned().collect(),
        production_time: 3,
    };
    recipes.insert("iron_plate", iron_plate);

    let copper_plate = Recipe {
        ingredients: [("copper_ore", 1)].iter().cloned().collect(),
        production_time: 2,
    };
    recipes.insert("copper_plate", copper_plate);

    let steel_plate = Recipe {
        ingredients: [("iron_plate", 5)].iter().cloned().collect(),
        production_time: 5,
    };
    recipes.insert("steel_plate", steel_plate);

    let electric_engine_unit = Recipe {
        ingredients: [
            ("iron_plate", 1),
            ("copper_plate", 1),
            ("electronic_circuit", 2),
        ]
        .iter()
        .cloned()
        .collect(),
        production_time: 10,
    };
    recipes.insert("electric_engine_unit", electric_engine_unit);

    let car = Recipe {
        ingredients: [("engine_unit", 1), ("steel_plate", 5)].iter().cloned().collect(),
        production_time: 20,
    };
    recipes.insert("car", car);

    // Calculate the needed resources for a specific recipe and quantity per minute
    calculate_resources(&recipes, "car", 1, "minute");
    */
}
