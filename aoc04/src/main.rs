use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let passports = parse_passports(&input);

    part1(&passports)?;
    part2(&passports)?;
    Ok(())
}

fn parse_passports(input: &str) -> Vec<HashMap<&str, &str>> {
    let mut passports_parsed = Vec::new();
    let passports: Vec<&str> = input.split("\n\n").collect();

    for passport in passports.into_iter() {
        let passport: HashMap<&str, &str> = passport
            .split(|c| c == ' ' || c == '\n')
            .map(|a| {
                let mut b = a.trim().split(':');
                (b.next().unwrap(), b.next().unwrap())
            })
            .collect(); // Potential panic!

        passports_parsed.push(passport)
    }

    passports_parsed
}

fn part1(passports: &Vec<HashMap<&str, &str>>) -> Result<(), Box<dyn Error>> {
    let result = passports
        .into_iter()
        .filter(|passport| is_valid_passport(&passport))
        .count();

    writeln!(io::stdout(), "{:?}", result)?;
    Ok(())
}

fn part2(passports: &Vec<HashMap<&str, &str>>) -> Result<(), Box<dyn Error>> {
    let result = passports
        .into_iter()
        .filter(|passport| is_valid_passport(&passport) && is_valid_passport_pt2(&passport))
        .count();

    writeln!(io::stdout(), "{:?}", result)?;
    Ok(())
}

fn is_valid_passport(passport: &HashMap<&str, &str>) -> bool {
    let passport_fields: HashSet<&str> = passport.keys().cloned().collect();
    let required_fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();
    required_fields.is_subset(&passport_fields)
}


// TODO: The best way is to create a struct to handle all this verification
fn is_valid_passport_pt2(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&k, v)| match k {
        "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
        "hcl" => {
            v.starts_with('#') && v.len() == 7 && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        "hgt" => {
            let height = v[0..(v.len() - 2)].parse().unwrap_or(0);
            match &v[(v.len() - 2)..] {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => false,
            }
        }
        _ => unreachable!(),
    })
}
