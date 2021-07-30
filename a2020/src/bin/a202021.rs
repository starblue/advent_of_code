use core::slice::Iter;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;

use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::map;
use nom::named;
use nom::recognize;
use nom::separated_list1;
use nom::tag;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Ingredient(String);
impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Allergen(String);
impl fmt::Display for Allergen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct IngredientList(Vec<Ingredient>);
impl IngredientList {
    fn iter(&self) -> Iter<Ingredient> {
        self.0[..].iter()
    }
}
impl fmt::Display for IngredientList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for i in &self.0 {
            write!(f, "{}{}", sep, i)?;
            sep = " ";
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct AllergenList(Vec<Allergen>);
impl AllergenList {
    fn iter(&self) -> Iter<Allergen> {
        self.0[..].iter()
    }
}
impl fmt::Display for AllergenList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for i in &self.0 {
            write!(f, "{}{}", sep, i)?;
            sep = ", ";
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Food {
    ingredients: IngredientList,
    allergens: AllergenList,
}
impl fmt::Display for Food {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (contains {})", self.ingredients, self.allergens)
    }
}

named!(name<&str, String>,
    map!(recognize!(alpha1), String::from)
);

named!(ingredient<&str, Ingredient>,
    do_parse!(
        n: name >>
            (Ingredient(n))
    )
);
named!(ingredients<&str, IngredientList>,
    do_parse!(
        is: separated_list1!(tag!(" "), ingredient) >>
            (IngredientList(is))
    )
);

named!(allergen<&str, Allergen>,
    do_parse!(
        n: name >>
            (Allergen(n))
    )
);
named!(allergens<&str, AllergenList>,
    do_parse!(
        is: separated_list1!(tag!(", "), allergen) >>
            (AllergenList(is))
    )
);

named!(food<&str, Food>,
    do_parse!(
        ingredients: ingredients >>
        tag!(" (contains ") >>
        allergens: allergens >>
        tag!(")") >>
        line_ending >>
            (Food { ingredients, allergens })
    )
);
named!(input<&str, Vec<Food>>,
    many1!(food)
);

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let foods = result.unwrap().1;

    let mut can_occur_in: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    for f in &foods {
        for a in f.allergens.iter() {
            let ingredient_set = f
                .ingredients
                .iter()
                .cloned()
                .collect::<HashSet<Ingredient>>();
            if let Some(ingredients) = can_occur_in.get_mut(a) {
                *ingredients = ingredients
                    .intersection(&ingredient_set)
                    .cloned()
                    .collect::<HashSet<Ingredient>>();
            } else {
                can_occur_in.insert(a.clone(), ingredient_set);
            }
        }
    }

    let mut allergen_ingredients = HashSet::new();
    for is in can_occur_in.values() {
        for i in is.iter() {
            allergen_ingredients.insert(i.clone());
        }
    }
    let mut count = 0;
    for f in &foods {
        for i in f.ingredients.iter() {
            if !allergen_ingredients.contains(i) {
                count += 1;
            }
        }
    }
    let result_a = count;

    let mut allergen_map: BTreeMap<Allergen, Ingredient> = BTreeMap::new();
    loop {
        let mut find = None;
        for a in can_occur_in.keys() {
            if can_occur_in[a].len() == 1 {
                // found unique ingredient for allergen
                let i = can_occur_in[a].iter().cloned().next().unwrap();
                find = Some((a, i));
            }
        }
        if let Some((a, i)) = find {
            allergen_map.insert(a.clone(), i.clone());
            for is in &mut can_occur_in.values_mut() {
                is.remove(&i);
            }
        } else {
            break;
        }
    }
    let mut s = String::new();
    let mut sep = "";
    for i in allergen_map.values() {
        s.push_str(&format!("{}{}", sep, i));
        sep = ",";
    }
    let result_b = s;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
