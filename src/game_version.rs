struct Factorio1_1{
    game_version_name: &'static str,
    game_version_number: f32,

    building_list: BuildingList,
    beacon_list: BeaconList,
    recipe_list: RecipeList,
    module_list: ModuleList,
}

struct BuildingList{
    assembler: Assembler,
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