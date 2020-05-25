//! # rparif-influxdb
//!
//! Print on standard output (using [influxdb line protocol](https://docs.influxdata.com/influxdb/v1.8/write_protocols/line_protocol_reference/)
//! for use with [telegraf's exec plugin](https://docs.influxdata.com/telegraf/v1.14/plugins/plugin-list/#exec)) metrics
//! from [Airparif](https://www.airparif.asso.fr/) (pollution index for Ile-de-France, France).
//!
//! # Arguments
//!
//! * `-h, --help` : display help
//! * `-a, --api-key` : AirParif [API key](https://www.airparif.asso.fr/rss/api)
//! * `-n, --name` : flag that allow converting [INSEE](https://www.data.gouv.fr/en/datasets/correspondance-entre-les-codes-postaux-et-codes-insee-des-communes-francaises/) code into city name.
//! It has no effect if no INSEE code are given
//! * `-c, --city` : city [INSEE](https://www.data.gouv.fr/en/datasets/correspondance-entre-les-codes-postaux-et-codes-insee-des-communes-francaises/) code
//!
//! To fetch data for multiple cities, use `-c` or `--city` for each cities, eg:
//! ```
//! rparif-influxdb --city 75101 --city 94028
//! ```
//!
//! # Tag day
//!
//! The tag day show is an 'offset' from when the measure was made and the timestamp is set accordingly (adding or removing a day if day=next or day=previous)
//!
//!
//! # Examples
//! * Fetch global and per pollutant indices for yesterday, today and tomorrow :
//! ```
//! rparif-influxdb --api-key my-api-key
//!
//! pollution,insee=0,day="previous",pollutant="global" index=35 1590184800000000000
//! pollution,insee=0,day="previous",pollutant="no2" index=17 1590184800000000000
//! pollution,insee=0,day="previous",pollutant="o3" index=35 1590184800000000000
//! pollution,insee=0,day="previous",pollutant="pm10" index=31 1590184800000000000
//! pollution,insee=0,day="current",pollutant="global" index=34 1590271200000000000
//! pollution,insee=0,day="current",pollutant="no2" index=17 1590271200000000000
//! pollution,insee=0,day="current",pollutant="o3" index=34 1590271200000000000
//! pollution,insee=0,day="current",pollutant="pm10" index=23 1590271200000000000
//! pollution,insee=0,day="next",pollutant="global" index=45 1590357600000000000
//! pollution,insee=0,day="next",pollutant="no2" index=28 1590357600000000000
//! pollution,insee=0,day="next",pollutant="o3" index=45 1590357600000000000
//! pollution,insee=0,day="next",pollutant="pm10" index=25 1590357600000000000
//! ```
//!
//! * Fetch indices for INSEE 75101 (Paris 1er arr.) and 94028 (Créteil) without fetching city name (note that the index is
//! computed from all pollutants listed in `pollutant` tag) :
//! ```
//! rparif-influxdb --api-key my-api-key --city 75101 --city 94028
//!
//! pollution,insee=75101,day="previous",pollutant="o3 pm10" index=32 1590184800000000000
//! pollution,insee=75101,day="current",pollutant="o3" index=35 1590271200000000000
//! pollution,insee=75101,day="next",pollutant="o3" index=40 1590357600000000000
//! pollution,insee=94028,day="previous",pollutant="o3" index=33 1590184800000000000
//! pollution,insee=94028,day="current",pollutant="o3" index=35 1590271200000000000
//! pollution,insee=94028,day="next",pollutant="o3" index=45 1590357600000000000
//! ```
//!
//! * Fetch indices for INSEE 75101 (Paris 1er arr.) and 94028 (Créteil) and fetch city name (note that the index is
//! computed from all pollutants listed in `pollutant` tag) :
//! ```
//! rparif-influxdb --api-key my-api-key --city 75101 --city 94028 --name
//!
//! pollution,insee=75101,city="Paris 1er Arrondissement",day="previous",pollutant="o3 pm10" index=32 1590184800000000000
//! pollution,insee=75101,city="Paris 1er Arrondissement",day="current",pollutant="o3" index=35 1590271200000000000
//! pollution,insee=75101,city="Paris 1er Arrondissement",day="next",pollutant="o3" index=40 1590357600000000000
//! pollution,insee=94028,city="Creteil",day="previous",pollutant="o3" index=33 1590184800000000000
//! pollution,insee=94028,city="Creteil",day="current",pollutant="o3" index=35 1590271200000000000
//! pollution,insee=94028,city="Creteil",day="next",pollutant="o3" index=45 1590357600000000000
//! ```
//!
//! # Cross-compiling for raspberry pi
//! Install locally openssl (see https://stackoverflow.com/a/37378989)  
//! Install and setup toolchain & co : https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b
//!  
//! When build, add environment variable OPENSSL_STATIC=1
//! ```.env
//! OPENSSL_STATIC=1 cargo build --target=armv7-unknown-linux-gnueabihf
//! ```
#![cfg_attr(test, deny(warnings))]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Formatter;
use std::{env, fmt};

use args::Args;
use chrono::{Local, NaiveDateTime, NaiveTime, TimeZone};
use getopts::Occur;
use json::JsonValue;
use rparif::client::RParifClient;
use rparif::objects::{Day, Index};

use titlecase::titlecase;

const PROGRAM_DESC: &'static str =
    "Print on the standard output metric generated from AirParif using influxdb line protocol";

trait InfluxLineProtocol {
    fn line_protocol(&self, names: &BTreeMap<String, String>) -> String;
}

impl InfluxLineProtocol for Index {
    fn line_protocol(&self, names: &BTreeMap<String, String>) -> String {
        let mut result = String::new();
        result.push_str("pollution");

        if self.insee().is_some() {
            result.push_str(format!(",insee={}", self.insee().unwrap()).as_str());
            if !names.is_empty() {
                let name = names.get(self.insee().unwrap().as_str());
                match name {
                    None => result.push_str(r#",city="none""#),
                    Some(v) => result.push_str(format!(r#",city="{}""#, v).as_str()),
                }
            }
        } else {
            result.push_str(r#",insee=0"#);
        }

        let current = Local::today().naive_utc();
        if current > self.date() {
            result.push_str(r#",day="previous""#);
        } else if current == self.date() {
            result.push_str(r#",day="current""#);
        } else {
            result.push_str(r#",day="next""#);
        }

        if !self.pollutants().is_empty() {
            // Sort to have a deterministic order for tag value
            self.pollutants().sort();
            let pol = self.pollutants().join(" ");
            result.push_str(format!(r#",pollutant="{}""#, pol).as_str());
        }

        result.push_str(format!(" index={}", self.index()).as_str());

        let local = Local::now();
        let offset = local.offset();
        let time = NaiveTime::from_hms_micro(0, 0, 0, 0);
        let date_time = NaiveDateTime::new(self.date(), time);

        result.push_str(
            format!(
                " {}",
                offset
                    .from_local_datetime(&date_time)
                    .unwrap()
                    .timestamp_nanos()
            )
            .as_str(),
        );

        result
    }
}

#[derive(Debug)]
struct InseeConvertError {
    msg: String,
}

impl fmt::Display for InseeConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for InseeConvertError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }
}

fn convert_insee(insee: Vec<String>) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let conc = insee.join(" or ");

    let url = format!("https://public.opendatasoft.com/api/records/1.0/search/?rows={}&q={}&start=0&dataset=correspondance-code-insee-code-postal", insee.len(), conc);
    let url = url.as_str();

    let response = reqwest::blocking::get(url)?.text()?;
    let response = response.as_str();
    let response = json::parse(response)?;

    let mut result: BTreeMap<String, String> = BTreeMap::new();

    if response.has_key("error") {
        return Err(Box::new(InseeConvertError {
            msg: response["error"].to_string(),
        }));
    }

    return match &response["records"] {
        JsonValue::Array(v) => {
            for r in v.into_iter() {
                let city = titlecase(
                    r["fields"]["nom_comm"]
                        .as_str()
                        .unwrap()
                        .replace("-", " ")
                        .as_str(),
                );
                let ins = r["fields"]["insee_com"].to_string();
                result.insert(ins, city);
            }
            Ok(result)
        }
        _ => Err(Box::new(InseeConvertError {
            msg: "Wrong json".to_string(),
        })),
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = env::args().collect();
    let program = input[0].clone();
    let mut args = Args::new(program.as_str(), PROGRAM_DESC);

    args.flag("h", "help", "Print the usage menu");
    args.option(
        "k",
        "api-key",
        "AirParif API key",
        "STRING",
        Occur::Req,
        None,
    );
    args.option("c", "city", "INSEE code", "NUMBER", Occur::Multi, None);
    args.flag("n", "name", "Convert INSEE code into city name");

    args.parse(input)?;

    if args.value_of("help")? {
        println!("{}", args.full_usage());
        return Ok(());
    }

    let api_key: String = args.value_of("api-key")?;
    let mut cities: Vec<u32> = vec![];
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    if args.has_value("city") {
        if args.value_of("name")? {
            map = convert_insee(args.values_of("city")?)?;
        }
        cities.append(args.values_of("city")?.as_mut());
    }

    let client = RParifClient::new(api_key.as_str());
    if cities.is_empty() {
        let v = vec![Day::Yesterday, Day::Today, Day::Tomorrow];
        for day in v.into_iter() {
            let result = client.index_day(day)?;
            for index in result.into_iter() {
                println!("{}", index.line_protocol(&map));
            }
        }
    } else {
        let cities: Vec<String> = cities.iter().map(|c| c.to_string()).collect();
        let cities: Vec<&str> = cities.iter().map(AsRef::as_ref).collect();
        let result = client.index_city(cities)?;
        for index in result.into_iter() {
            println!("{}", index.line_protocol(&map));
        }
    }

    Ok(())
}
