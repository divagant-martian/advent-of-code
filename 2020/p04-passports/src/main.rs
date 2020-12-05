use std::fs::read_to_string;

#[derive(Debug)]
enum EyeColor {
    AMB,
    BLU,
    BRN,
    GRY,
    GRN,
    HZL,
    OTH,
}

#[derive(Debug)]
enum Height {
    In(usize),
    Cm(usize),
}

#[derive(Debug)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Height,
    hcl: String,
    ecl: EyeColor,
    pid: usize,
    cid: Option<String>,
}

#[derive(Default, Debug)]
struct PassportBuilder {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportBuilder {
    fn include_field(self, key: &str, val: &str) -> Self {
        match key {
            "byr" => PassportBuilder {
                byr: Some(val.into()),
                ..self
            },
            "iyr" => PassportBuilder {
                iyr: Some(val.into()),
                ..self
            },
            "eyr" => PassportBuilder {
                eyr: Some(val.into()),
                ..self
            },
            "hgt" => PassportBuilder {
                hgt: Some(val.into()),
                ..self
            },
            "hcl" => PassportBuilder {
                hcl: Some(val.into()),
                ..self
            },
            "ecl" => PassportBuilder {
                ecl: Some(val.into()),
                ..self
            },
            "pid" => PassportBuilder {
                pid: Some(val.into()),
                ..self
            },
            "cid" => PassportBuilder {
                cid: Some(val.into()),
                ..self
            },
            _ => unreachable!("bad input"),
        }
    }

    fn build(self) -> Result<Passport, ()> {
        Ok(Passport {
            byr: self
                .byr
                .and_then(|num| {
                    let parsed = num.parse::<usize>().ok()?;
                    if parsed >= 1920 && parsed <= 2002 {
                        Some(parsed)
                    } else {
                        None
                    }
                })
                .ok_or(())?,
            iyr: self
                .iyr
                .and_then(|num| {
                    let parsed = num.parse::<usize>().ok()?;
                    if parsed >= 2010 && parsed <= 2020 {
                        Some(parsed)
                    } else {
                        None
                    }
                })
                .ok_or(())?,
            eyr: self
                .eyr
                .and_then(|num| {
                    let parsed = num.parse::<usize>().ok()?;
                    if parsed >= 2020 && parsed <= 2030 {
                        Some(parsed)
                    } else {
                        None
                    }
                })
                .ok_or(())?,
            hgt: self
                .hgt
                .and_then(|height| {
                    if let Some(height) = height.strip_suffix("in") {
                        if let Ok(height) = height.parse() {
                            if height >= 59 && height <= 76 {
                                return Some(Height::In(height));
                            }
                        }
                    } else if let Some(height) = height.strip_suffix("cm") {
                        if let Ok(height) = height.parse() {
                            if height >= 150 && height <= 193 {
                                return Some(Height::Cm(height));
                            }
                        }
                    }
                    None
                })
                .ok_or(())?,
            hcl: self
                .hcl
                .and_then(|color| {
                    if let Some(color) = color.strip_prefix('#') {
                        let parsed = color
                            .chars()
                            .filter(|c| {
                                (c.is_ascii_hexdigit() && c.is_lowercase()) || c.is_ascii_digit()
                            })
                            .collect::<String>();
                        if parsed.len() == 6 && color.len() == 6 {
                            return Some(parsed);
                        }
                    }
                    None
                })
                .ok_or(())?,
            ecl: self
                .ecl
                .and_then(|color| match color.as_str() {
                    "amb" => Some(EyeColor::AMB),
                    "blu" => Some(EyeColor::BLU),
                    "brn" => Some(EyeColor::BRN),
                    "gry" => Some(EyeColor::GRY),
                    "grn" => Some(EyeColor::GRN),
                    "hzl" => Some(EyeColor::HZL),
                    "oth" => Some(EyeColor::OTH),
                    _ => None,
                })
                .ok_or(())?,
            pid: self
                .pid
                .and_then(|num| {
                    if num.len() != 9 {
                        return None;
                    }
                    let num = num.parse().ok()?;
                    if num <= 999999999 {
                        Some(num)
                    } else {
                        None
                    }
                })
                .ok_or(())?,
            cid: self.cid,
        })
    }
}

fn main() {
    let mut passports = Vec::new();
    for passport in read_to_string("data/input1.txt").unwrap().split("\n\n") {
        let mut builder = PassportBuilder::default();
        for key_val in passport.split_ascii_whitespace() {
            builder = builder.include_field(&key_val[..3], &key_val[4..]);
        }
        if let Ok(valid_passport) = builder.build() {
            println!("{:?}", valid_passport);
            passports.push(valid_passport);
        }
    }

    println!("Valid passports: {}", passports.len());
}
