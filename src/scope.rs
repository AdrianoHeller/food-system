#[derive(Debug)]
pub enum Level {
    Amateur,
    Professional,
}
#[derive(Debug)]
pub enum FoodType {
    Asian,
    Chinese,
    Italian,
    French,
    German,
    Fusion,
    Fastfood,
    Other,
}
#[derive(Debug)]
pub enum EstabType {
    Restaurant,
    Bar,
    Supermarket,
    Bistro,
    Healthy,
    Other(String),
}
#[derive(Debug)]
pub struct Establishment {
    pub name: &'static str,
    pub cnpj: &'static str,
    pub address: &'static str,
    pub phone: &'static str,
    pub number_of_employees: u16,
    chef: &'static str,
    estab_type: EstabType,
    food_style: FoodType,
    estab_level: Level,
}
impl Establishment {
    pub fn new(name: &'static str, cnpj: &'static str, address: &'static str, phone: &'static str,number_of_employees: u16) -> Establishment {
        Establishment {
            name,
            cnpj,
            address,
            phone,
            number_of_employees,
            chef: "John Connor",
            estab_type: EstabType::Restaurant,
            food_style: FoodType::Asian,
            estab_level: Level::Professional,
        }
    }
}
