use std::collections::HashMap;
use std::fs;

type QuantityIngredients = (i64, Vec<(String, i64)>);
fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 843220
    println!("{}", part2(contents)); // 2169535
}

fn part1(input: String) -> i64 {
    let formulas = parse_input(input);
    let mut inventory: HashMap<String, i64> = HashMap::new();
    inventory.insert("FUEL".to_string(), 1);
    alchemy(&mut inventory, &formulas)
}

fn part2(input: String) -> i64 {
    let formulas = parse_input(input);
    let limit = 1_000_000_000_000_i64;
    let mut success = 0_i64;
    for x in 2_169_000_i64.. {
        //limit / 843220 = 1,185,930
        let mut inventory: HashMap<String, i64> = HashMap::new();
        inventory.insert("FUEL".to_string(), x);
        if alchemy(&mut inventory, &formulas) < limit {
            success = x;
        } else {
            break;
        }
    }
    success
}

fn alchemy(
    inventory: &mut HashMap<String, i64>,
    formulas: &HashMap<String, QuantityIngredients>,
) -> i64 {
    let mut ore = 0;
    let mut surplus: HashMap<String, i64> = HashMap::new();
    while !inventory.is_empty() {
        let key: String;
        {
            let (k, _) = inventory.iter().next().unwrap();
            key = k.clone();
        }
        let (item, mut needed) = inventory.remove_entry(&key).unwrap();
        //println!("{} {}", item, needed);

        if surplus.contains_key(&item) {
            let x = *surplus.get(&item).unwrap();
            //println!("Some in surplus {}", x);
            if x <= needed {
                needed -= x;
                surplus.remove(&item);
            } else {
                surplus.insert(item.clone(), x - needed);
                needed = 0;
            }
        }
        if needed == 0 {
            //println!("Fulfilled from surplus");
            continue;
        }

        let (amount, ingredients) = formulas.get(&item).unwrap();

        //println!("#:{} {:?}", amount, ingredients);
        let mut num = needed / amount;
        if needed % amount != 0 {
            num += 1;
        }

        //println!("{} conversions", num);
        if num * amount != needed {
            //println!("Putting extra in surplus: {}", num * amount - needed);
            surplus
                .entry(item)
                .and_modify(|a| *a += num * amount - needed)
                .or_insert(num * amount - needed);
        }

        for (ingredient, quantity) in ingredients {
            if *ingredient == "ORE" {
                ore += num * *quantity;
                //println!("Ore updated {}", ore)
            } else {
                inventory
                    .entry(ingredient.clone())
                    .and_modify(|a| *a += num * *quantity)
                    .or_insert(num * *quantity);
            }
        }
        //println!("{:?}\n\n", inventory);
    }
    ore
}

fn parse_input(input: String) -> HashMap<String, QuantityIngredients> {
    let mut formulas: HashMap<String, QuantityIngredients> = HashMap::new();
    for line in input.lines() {
        let mut fields = line.split(" => ");
        let ingredients_field = fields.next().unwrap();
        let product = fields.next().unwrap();

        let mut li: Vec<(String, i64)> = Vec::new();
        let ingredients = ingredients_field.split(", ");
        for ingredient in ingredients {
            li.push(parse_ingredient(ingredient));
        }

        let quantity_product = parse_ingredient(product);
        let key = quantity_product.0.clone();
        formulas.insert(key, (quantity_product.1, li));
    }
    formulas
}

fn parse_ingredient(field: &str) -> (String, i64) {
    let mut quantity_ingredient = field.split(' ');
    let quantity = quantity_ingredient.next().unwrap().parse::<i64>().unwrap();
    let ingredient = quantity_ingredient.next().unwrap().to_string();
    (ingredient, quantity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_part1() {
        let contents = fs::read_to_string("input_simple.txt").expect("File not found");
        assert_eq!(part1(contents), 31);
    }

    #[test]
    fn test_simple2_part1() {
        let contents = fs::read_to_string("input_simple2.txt").expect("File not found");
        assert_eq!(part1(contents), 165);
    }

    #[test]
    fn test_medium_part1() {
        let contents = fs::read_to_string("input_medium.txt").expect("File not found");
        assert_eq!(part1(contents), 13312);
    }

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 843220);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 2169535);
    }
}
