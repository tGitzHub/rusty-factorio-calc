enum ModuleKind<>{
    Efficiency,
    Productivity,
    Speed,
}
enum ModulePower{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

struct Module{
    module_kind: ModuleKind,
    module_power: ModulePower,
}

trait AcceptsModule{
    fn apply_module(&self);
}

/* 
impl Module {
    fn get_modifier(&self) -> u8 {
        let mut modifier: u8 = 0;
        if let ModuleKind::Efficiency = self.module_kind {
            match self.module_power {
                1 => modifier= 3,
                2 => modifier= 3,
                3 => modifier= 3,
                4 => modifier= 3,
                5 => modifier= 3,
                6 => modifier= 3,
                7 => modifier= 3,
                8 => modifier= 3,
                9 => modifier= 3,
                _ => modifier= 3
            };
        }
        else if let ModuleKind::Productivity = self.module_kind {
            match self.module_power {
                1 => modifier= 3,
                2 => modifier= 3,
                3 => modifier= 3,
                4 => modifier= 3,
                5 => modifier= 3,
                6 => modifier= 3,
                7 => modifier= 3,
                8 => modifier= 3,
                9 => modifier= 3,
                _ => modifier= 0
            };
        }
        else if let ModuleKind::Speed = self.module_kind{
            match self.module_power {
                1 => modifier= 3,
                2 => modifier= 3,
                3 => modifier= 3,
                4 => modifier= 3,
                5 => modifier= 3,
                6 => modifier= 3,
                7 => modifier= 3,
                8 => modifier= 3,
                9 => modifier= 3,
                _ =>  modifier= 0,
            };
        }
        else {
            modifier=0
        }
        modifier
    }
    
}
*/

pub struct Assembler{
    pub craft_speed: u8,
}

impl AcceptsModule for Assembler{
    fn apply_module(&self) {
        
    }
}

pub struct Production<'a>{
    assembler: &'a Assembler,
    recipe: &'a Recipe,
}
impl<'a> Production<'a>{
    pub fn builder() -> ProductionBuilder<'a>{
        ProductionBuilder::new()
    }
}
pub struct ProductionBuilder<'a>{
    assembler: Option<&'a Assembler>,
    recipe: Option<&'a Recipe>,
}

impl<'a> ProductionBuilder<'a>{
    fn new()-> Self{
        ProductionBuilder{
            assembler: None,
            recipe: None,
        }
    }
    pub fn set_assembler(mut self, assembler: &'a Assembler)-> Self{
        self.assembler = Some(assembler);
        self
    }
    pub fn set_recipe(mut self, recipe: &'a Recipe)-> Self{
        self.recipe = Some(recipe);
        self
    }
    pub fn build(self) -> Production<'a>{
            Production {assembler: self.assembler.unwrap(),recipe: self.recipe.unwrap()}
    }
}

pub struct Recipe{
    pub production_time: u32,
    //total_raw: HashMap<&'a str, u32>
}