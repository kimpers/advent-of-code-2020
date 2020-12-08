use std::collections::HashMap;

use shared;

#[derive(Debug)]
struct Bag {
    pub color: String,
    pub contents: Vec<BagContent>,
}
#[derive(Debug)]
struct BagContent {
    pub color: String,
    pub amount: usize,
}

impl Bag {
    pub fn parse_luggage_rule(rule: &str) -> Bag {
        let splt = rule.split(" contain ").collect::<Vec<&str>>();
        let bag_color = splt[0].replace(" bags", "");

        let contains_strs = splt[1]
            .split(",")
            .map(|text| {
                text.replace("bags", "")
                    .replace("bag", "")
                    .replace(".", "")
                    .trim()
                    .to_string()
            })
            .collect::<Vec<String>>();

        if contains_strs.len() == 1 && contains_strs[0] == "no other" {
            return Bag {
                color: bag_color.to_string(),
                contents: vec![],
            };
        }

        let bag_contents = contains_strs.iter().fold(vec![], |mut vec, content| {
            let tokens = content.split(" ").collect::<Vec<&str>>();

            let amount = tokens[0].parse::<usize>().unwrap();
            let color = tokens[1..tokens.len()].join(" ");

            vec.push(BagContent { color, amount });

            vec
        });

        Bag {
            color: bag_color.to_string(),
            contents: bag_contents,
        }
    }

    pub fn can_fit_color(&self, bag_rule_map: &HashMap<String, Bag>, color: &str) -> bool {
        let mut can_fit = false;
        for bag_content in &self.contents {
            let current_bag = bag_rule_map.get(&bag_content.color).unwrap();
            if current_bag.color == color {
                return true;
            }

            let child_can_fit = current_bag.can_fit_color(bag_rule_map, &color);

            if child_can_fit == true {
                can_fit = true;
            }
        }

        can_fit
    }

    fn count_num_bags_inside(&self, bag_rule_map: &HashMap<String, Bag>) -> usize {
        let mut count = 1;
        for bag_content in &self.contents {
            let current_bag = bag_rule_map.get(&bag_content.color).unwrap();
            count += bag_content.amount * current_bag.count_num_bags_inside(bag_rule_map);
        }

        count
    }

    pub fn count_total_num_bags(&self, bag_rule_map: &HashMap<String, Bag>) -> usize {
        // subtract 1 for the initial bag which does not count
        &self.count_num_bags_inside(bag_rule_map) - 1
    }
}

fn parse_rules_to_map(rules: &Vec<&str>) -> HashMap<String, Bag> {
    let mut bag_rule_map: HashMap<String, Bag> = HashMap::new();

    for rule in rules {
        let bag = Bag::parse_luggage_rule(rule);

        let color = bag.color.clone();
        bag_rule_map.insert(color, bag);
    }

    bag_rule_map
}

fn count_bags_that_can_fit_color(bag_rule_map: &HashMap<String, Bag>, bag_color: &str) -> usize {
    let mut count = 0;
    for (_key, bag) in bag_rule_map.iter() {
        if bag.can_fit_color(&bag_rule_map, bag_color) == true {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let lines = shared::read_file("input.txt");
    let rules = lines.iter().map(|s| &**s).collect::<Vec<&str>>();
    let rules_map = parse_rules_to_map(&rules);

    let bags_that_can_fit_count = count_bags_that_can_fit_color(&rules_map, "shiny gold");
    println!(
        "Num bags that can fit a shiny gold bag {}",
        bags_that_can_fit_count
    );

    let bag = rules_map.get("shiny gold").unwrap();
    let num_bags_inside = bag.count_total_num_bags(&rules_map);
    println!("Num bags inside a shiny gold bag {}", num_bags_inside);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_luggage_rules() {
        let bag = Bag::parse_luggage_rule(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        );
        assert_eq!(bag.color, "light red");
        assert_eq!(bag.contents.len(), 2);
        assert_eq!(bag.contents[0].color, "bright white");
        assert_eq!(bag.contents[0].amount, 1);
        assert_eq!(bag.contents[1].color, "muted yellow");
        assert_eq!(bag.contents[1].amount, 2);

        let bag = Bag::parse_luggage_rule("bright white bags contain 1 shiny gold bag.");
        assert_eq!(bag.color, "bright white");
        assert_eq!(bag.contents.len(), 1);
        assert_eq!(bag.contents[0].color, "shiny gold");
        assert_eq!(bag.contents[0].amount, 1);

        let bag = Bag::parse_luggage_rule("faded blue bags contain no other bags.");
        assert_eq!(bag.color, "faded blue");
        assert_eq!(bag.contents.len(), 0);
    }

    #[test]
    fn it_counts_bags_that_can_fit_a_color() {
        let lines = shared::read_file("test_input.txt");

        let rules = lines.iter().map(|s| &**s).collect::<Vec<&str>>();
        let rules_map = parse_rules_to_map(&rules);
        assert_eq!(
            count_bags_that_can_fit_color(&rules_map, "shiny gold"),
            4,
            "counts all bags that can contain a shiny gold bag"
        );
    }

    #[test]
    fn it_counts_bags_inside_a_bag() {
        let lines = shared::read_file("test_2_input.txt");

        let rules = lines.iter().map(|s| &**s).collect::<Vec<&str>>();
        let rules_map = parse_rules_to_map(&rules);

        let bag = rules_map.get("shiny gold").unwrap();
        assert_eq!(bag.count_total_num_bags(&rules_map), 126);
    }
}
