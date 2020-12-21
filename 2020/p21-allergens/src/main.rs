use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let filename = std::env::args().nth(1).expect("No filename given");
    let mut all_allergens = HashSet::new();
    let mut all_ingredients = HashSet::new();
    let food: Vec<_> = std::fs::read_to_string(filename)
        .expect("Bad input file")
        .replace(")", "")
        .lines()
        .map(|l| {
            let mut parts = l.split(" (contains ");
            let ingredients: HashSet<String> =
                parts.next().unwrap().split(' ').map(String::from).collect();
            let allergens: HashSet<String> = parts
                .next()
                .unwrap()
                .split(", ")
                .map(String::from)
                .collect();
            all_allergens.extend(allergens.clone());
            all_ingredients.extend(ingredients.clone());
            (ingredients, allergens)
        })
        .collect();
    let mut maybe_with_allergen = HashMap::with_capacity(all_allergens.len());
    for (ingredients, allergens) in &food {
        for allergen in allergens {
            let known_allergens = maybe_with_allergen
                .entry(allergen)
                .or_insert_with(|| ingredients.to_owned());
            *known_allergens = known_allergens
                .intersection(&ingredients)
                .cloned()
                .collect();
        }
    }

    let ingredients_that_appear = maybe_with_allergen
        .values()
        .fold(HashSet::new(), |ingredients, acc| {
            acc.union(&ingredients).cloned().collect()
        });
    dbg!(&ingredients_that_appear.len());
    dbg!(&all_allergens.len());
    let those_that_not: HashSet<_> = all_ingredients
        .difference(&ingredients_that_appear)
        .collect();

    let mut times = 0;
    for (ingredients, _allergens) in &food {
        for ingredient in ingredients {
            if those_that_not.contains(&ingredient) {
                times += 1;
            }
        }
    }
    dbg!(times);

    // now to solve them (Part 2)
    let mut allergen_mapping: HashMap<String, String> = HashMap::new(); // from allergen to ingredient
    while let Some(allergen) =
        maybe_with_allergen
            .iter()
            .find_map(|(allergen, maybe_ingredients)| {
                if maybe_ingredients.len() == 1 {
                    Some(allergen.to_string())
                } else {
                    None
                }
            })
    {
        // remove the allergen and get the dangerous ingredient
        let dangerous_ingredient = maybe_with_allergen
            .remove(&allergen)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        // remove the ingredient from all the remaining maybe ingredients
        for set in maybe_with_allergen.values_mut() {
            set.remove(&dangerous_ingredient);
        }
        allergen_mapping.insert(allergen, dangerous_ingredient);
    }

    let mut allergen_mapping: Vec<_> = allergen_mapping.iter().collect();
    allergen_mapping.sort_unstable();
    let ans = allergen_mapping
        .iter()
        .map(|(_allergen, ingredient)| ingredient.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", ans);
}
