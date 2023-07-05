use std::{env::args, path::Path, fs, str::FromStr};

use chrono::Utc;
use fake::{faker::{internet::en::FreeEmail, company::en::CompanyName, address::en::{ZipCode, StreetName, CityName, StateAbbr}, phone_number::en::PhoneNumber, lorem::en::Words}, Fake};
use rand::{distributions::{Alphanumeric, DistString}, random, Rng};
use regex::Regex;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
enum DataFormatType {
    String,
    Name,
    Company,
    Address,
    Email,
    Price,
    Quantity,
    Date,
    City,
    State,
    Zip,
    Phone,
}

impl FromStr for DataFormatType {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        Ok(match s.as_str() {
            "string" => Self::String,
            "name" => Self::Name,
            "company" => Self::Company,
            "address" => Self::Address,
            "email" => Self::Email,
            "price" => Self::Price,
            "quantity" => Self::Quantity,
            "date" => Self::Date,
            "city" => Self::City,
            "state" => Self::State,
            "zip" => Self::Zip,
            "phone" => Self::Phone,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Parse error: Unknown format type #{s}#")))
        })
    }
}

fn main() {
    let args = args().collect::<Vec<_>>();

    if args.len() != 4 {
        println!("Usage: {} <template.tex> <num to generate> <output path>", args[0]);
        return;
    }

    let template = fs::read_to_string(&args[1]).unwrap();
    let format_regex = Regex::new(r"#(?P<type>\w+)#").unwrap();

    // Find all format regexes and replace one-by-one based on the type
    // The first capture is the type to fill in with
    let mut types = Vec::new();
    for cap in format_regex.captures_iter(&template) {
        types.push(DataFormatType::from_str(&cap["type"]).unwrap());
    }

    let num_to_generate = args[2].parse::<usize>().unwrap();

    // Create directory
    let output_dir = Path::new(&args[3]);
    fs::create_dir_all(output_dir).unwrap();

    for i in 1..=num_to_generate {
        println!("Generating filled template {i}...");
        let mut template = template.to_string();
        for typ in &types {
            template = format_regex.replace(&template, match typ {
                DataFormatType::String => Words(1..4).fake::<Vec<_>>().join(" "), // Alphanumeric.sample_string(&mut rand::thread_rng(), 10)
                DataFormatType::Name => fake::faker::name::en::Name().fake(),
                DataFormatType::Company => CompanyName().fake(),
                DataFormatType::Address => format!("{} {}", random::<u8>(), StreetName().fake::<String>()),
                DataFormatType::Email => FreeEmail().fake(),
                DataFormatType::Price => Decimal::from_f64_retain(rand::thread_rng().gen_range(10.00..=1_000_000.00)).unwrap().round_dp(2).to_string(),
                DataFormatType::Quantity => random::<u16>().to_string(),
                DataFormatType::Date => Utc::now().to_rfc2822(),
                DataFormatType::City => CityName().fake(),
                DataFormatType::State => StateAbbr().fake(),
                DataFormatType::Zip => ZipCode().fake(),
                DataFormatType::Phone => PhoneNumber().fake(),
            }).to_string()
        }

        fs::write(output_dir.join(format!("{}.pdf", Alphanumeric.sample_string(&mut rand::thread_rng(), 10))), tectonic::latex_to_pdf(template).unwrap()).unwrap();
    }

    println!("Done!");
}
