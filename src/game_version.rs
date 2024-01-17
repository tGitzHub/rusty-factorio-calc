use serde_derive::{Serialize, Deserialize};
use serde_json::{json, to_string_pretty, Result};

#[derive(Serialize, Deserialize)]
pub struct FactorioVersion{
    
    game_version_name: &'static str,
    game_version_number: &'static str,
    modded_factorio: bool,

    /* 
    building_list: BuildingList,
    beacon_list: BeaconList,
    recipe_list: RecipeList,
    module_list: ModuleList,
    */
}

impl FactorioVersion{
    pub fn factorio_1_1() -> Self {
        FactorioVersion{
            game_version_name: "Factorio",
            game_version_number: "1.1",
            modded_factorio: false,
            /* 
            building_list: BuildingList::new(),
            beacon_list: BeaconList::new(),
            recipe_list: RecipeList::new(),
            module_list: ModuleList::new(),
            */
        }
    }
}


enum ModuleKind{
    Efficiency,
    Productivity,
    Speed,
}
enum ModulePower{
    One,
    Two,
    Three,
}

struct Module{
    module_kind: ModuleKind,
    module_power: ModulePower,
}
/* 
impl Module{
    fn new(module_kind: ModuleKind, module_power: ModulePower) -> Module{
        Module{
            module_kind,
            module_power,
        }
    }
}

struct BuildingList{
    assembler: [Assembler; N],
    furnace: Furnace,
    drill: Drill,
    pumpjack: Pumpjack,
    lab: Lab,
    beacon: Beacon,
    centrifuge: Centrifuge,
    refinery: Refinery,
    chemical_plant: ChemicalPlant,
    oil_refinery: OilRefinery,
    pump: Pump,
    offshore_pump: OffshorePump,
    cargo_wagon: CargoWagon,
    fluid_wagon: FluidWagon,
}

struct Assembler{
    name: &'static str,
    crafting_speed: f32,
    crafting_categories: Vec<&'static str>,
    allowed_effects: Vec<&'static str>,
    module_inventory_size: u8,
    ingredient_count: u8,
    result_count: u8,
    energy_usage: f32,
    energy_source: EnergySource,
    module_specification: ModuleSpecification,
    allowed_effects: Vec<&'static str>,
    crafting_categories: Vec<&'static str>,
    ingredient_count: u8,
    result_count: u8,
    module_inventory_size: u8,
    energy_usage: f32,
    energy_source: EnergySource,
    module_specification: ModuleSpecification,
}

struct Furnace{
    name: &'static str,
    crafting_speed: f32,
    crafting_categories: Vec<&'static str>,
    allowed_effects: Vec<&'static str>,
    module_inventory_size: u8,
    ingredient_count: u8,
    result_count: u8,
    energy_usage: f32,
    energy_source: EnergySource,
    module_specification: ModuleSpecification,
}

struct Drill{
    name: &'static str,
    mining_speed: f32,
    mining_power: f32,
    resource_categories: Vec<&'static str>,
    allowed_effects: Vec<&'static str>,
    module_inventory_size: u8,
    energy_usage: f32,
    energy_source: EnergySource,
    module_specification: ModuleSpecification,
}

struct Pumpjack{
    name: &'static str,
    mining_speed: f32,
    mining_power: f32,
    resource_categories: Vec<&'static str>,
    allowed_effects: Vec<&'static str>,
    module_inventory_size: u8,
    energy_usage: f32,
    energy_source: EnergySource,
    module_specification: ModuleSpecification,
}

struct Lab{
    name: &'static str,
    researching_speed: f32,
    energy_usage: f32,
    energy_source: EnergySource,
    module_specification: ModuleSpecification,
}

struct RocketSilo{
    name: &'static str,
    crafting_speed: f32,
    crafting_categories: Vec<&'static str>,
    allowed_effects: Vec<&'static str>,
    module_inventory_size: u8,
    ingredient_count: u8,
    result_count: u8,
    energy_usage: f32,
    energy_source: EnergySource,
    module_specification: ModuleSpecification,
}

struct Beacon{
    name: &'static str,
    distribution_effectivity: f32,
    module_slots: u8,
    allowed_effects: Vec<&'static str>,
    energy_usage: f32,
    energy_source: EnergySource,
    module_specification: ModuleSpecification,
}

struct ModuleSpecification{
    module_slots: u8,
    module_kind_allowed: Vec<ModuleKind>,
}
/* 
let assembler_1 = Assembler{
    name: "assembling-machine-1",
    crafting_speed: 0.5,
    crafting_categories: vec!["crafting"],
    allowed_effects: vec!["consumption", "speed", "productivity", "pollution"],
    module_inventory_size: 2,
    ingredient_count: 3,
    result_count: 1,
    energy_usage: 90.0,
    energy_source: EnergySource::Burner{
        effectivity: 1.0,
        fuel_inventory_size: 1,
        burnt_result_inventory_size: 1,
        smoke: Smoke{
            name: "smoke",
            frequency: 10,
            position: vec![0.0, -1.0],
            starting_vertical_speed: 0.08,
            starting_frame_speed: 0.2,
            starting_frame_speed_deviation: 0.1,
            starting_distance: 1.0,
            starting_distance_deviation: 0.3,
            starting_height: 0.0,
            starting_height_deviation: 0.5,
            speed_from_center: 0.01,
            speed_from_center_deviation: 0.05,
        },
    },
    module_specification: ModuleSpecification{
        module_slots: 2,
        module_info_max_icons_per_row: 3,
        module_info_max_icon_rows: 1,
        module_info_icon_shift: vec![0.0, 0.0],
        module_info_multi_row_initial_height_modifier: -0.3
    }
*/
*/