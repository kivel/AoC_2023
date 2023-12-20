use std::collections::HashMap;
use std::num::ParseIntError;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
    fn from_str(s: &str) -> Self {
        let pattern: &[_] = &['{', '}'];
        let v = s
            .split(',')
            .map(|split| split.trim_matches(pattern))
            // .inspect(|s| println!("{:?}", s))
            .filter_map(|c| c.split('=').nth(1).unwrap().parse::<u32>().ok())
            .collect::<Vec<u32>>();
        Self {
            x: v[0],
            m: v[1],
            a: v[2],
            s: v[3],
        }
    }

    fn get_val(&self, cat: char) -> Option<u32> {
        match cat {
            'x' => Some(self.x),
            'm' => Some(self.m),
            'a' => Some(self.a),
            's' => Some(self.s),
            _ => None,
        }
    }

    fn apply_rules(&self, rules: &mut HashMap<String, Vec<Rule>>) -> Option<Self> {
        let mut key: String = "in".to_owned(); // always start with "in"

        loop {
            // println!("workflow: {key}");
            let workflow: &mut Vec<Rule> = rules.get_mut(&key).unwrap();
            for rule in workflow {
                match rule.evaluate(self.clone()) {
                    None => {
                        continue;
                    }
                    Some(k) => {
                        key = k;
                        break;
                    }
                }
            }
            // println!("got rule: {:?}", key);
            match key.as_str() {
                "A" => return Some(self.clone()),
                "R" => break,
                _ => {}
            }
        }
        None
    }
}
#[derive(Debug, Clone)]
struct Rule {
    category: Option<char>,
    operator: Option<char>,
    value: Result<u32, ParseIntError>,
    target: Option<String>,
}

impl Rule {
    fn from_str(s: String) -> Self {
        let mut category = None;
        let mut operator = None;
        let mut value = Ok(0);
        let mut target = None;
        // does contain target only
        if s.contains(&[':']) {
            category = s.chars().nth(0);
            operator = s.chars().nth(1);
            target = Some(s.split(':').nth(1).unwrap().to_owned());
            value = s.split(':').nth(0).unwrap()[2..].parse::<u32>();
        } else {
            target = Some(s.to_owned());
        }
        Self {
            category,
            operator,
            value,
            target,
        }
    }

    fn evaluate(&mut self, part: Part) -> Option<String> {
        let check_val = match self.category {
            Some(cat) => part.get_val(cat).unwrap(),
            None => return self.target.clone(),
        };

        let check_result = match self.operator.unwrap() {
            '>' => check_val > self.value.clone().unwrap(),
            '<' => check_val < self.value.clone().unwrap(),
            _ => unreachable!(),
        };

        if check_result {
            return self.target.clone();
        }
        // return None to indicate that the next rule should be used
        None
    }
}

fn day19_1(data: &str) -> u32 {
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut lines = data.lines();

    while let Some(line) = lines.next() {
        // skip empty lines
        if line.len() == 0 {
            continue;
        };
        // parts start with {
        if line.starts_with('{') {
            parts.push(Part::from_str(line));
        } else {
            let mut split = line.split(&['{', '}']);
            let key = split.next().unwrap();
            let payload = &split.next().unwrap();
            let payloads: Vec<&str> = payload.split(',').collect();
            let mut rules: Vec<Rule> = Vec::new();
            for p in payloads {
                let r = Rule::from_str(p.to_owned());
                // println!("{:?} -> {:?}", &p, &r);
                rules.push(r);
            }
            workflows.insert(key.to_owned(), rules);
        }
    }

    let valid_parts = parts
        .iter()
        .filter_map(|p| p.apply_rules(&mut workflows))
        .map(|valid_parts| valid_parts.value())
        .collect::<Vec<u32>>();

    println!("{:?}", valid_parts);

    valid_parts.iter().sum::<u32>()
}

fn main() {
    let d = include_str!("../../input/day19_test.txt");
    let result = day19_1(d);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::day19_1;

    #[test]
    fn day19_1_test() {
        let d = include_str!("../../input/day19_test.txt");
        let result = day19_1(d);
        println!("{result:?}");
        assert_eq!(result, 19114)
    }

    #[test]
    fn day19_1_final() {
        let d = include_str!("../../input/day19.txt");
        let result = day19_1(d);
        println!("{}", result);
        assert_eq!(result, 348378);
    }
}
