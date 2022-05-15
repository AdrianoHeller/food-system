pub mod infrastructure {
    use chrono::{DateTime, Utc,Local};

    #[derive(Debug)]
    pub struct Employee {
        pub name: String,
        pub last_name: String,
        pub email: String,
        created_at: DateTime<Utc>,
        timestamp: DateTime<Local>,
    }

    pub struct Employer {
        pub name: String,
        pub last_name: String,
        pub email: String,
        company_name: &'static str,
        created_at: DateTime<Utc>,
        timestamp: DateTime<Local>,
    }

    impl Employee {
        pub fn new(name: String, last_name: String,email: String) -> Employee {
            Employee {
                name,
                last_name,
                email,
                created_at: Utc::now(),
                timestamp: Local::now(),
            }
        }
    }
}

pub mod scope {
        pub enum Level {
            Amateur,
            Professional,
        }
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
        pub enum EstabType {
            Restaurant,
            Bar,
            Supermarket,
            Bistro,
            Healthy,
            Other(String),
        }
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

}
