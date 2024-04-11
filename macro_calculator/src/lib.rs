pub use json::JsonValue;
use json::object;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;
    for food in foods {
        let c: String = food.calories.get(1).unwrap().to_string();
        cals += food.nbr_of_portions * c[..c.len()-4].parse::<f64>().unwrap();
        carbs += food.nbr_of_portions * food.carbs;
        proteins += food.nbr_of_portions * food.proteins;
        fats += food.nbr_of_portions * food.fats;

    }
    object!{
        "cals": (cals * 100.0).round() / 100.0,
        "carbs": (carbs * 100.0).round() / 100.0,
        "proteins": (proteins * 100.0).round() / 100.0,
        "fats": (fats * 100.0).round() / 100.0
    }
}