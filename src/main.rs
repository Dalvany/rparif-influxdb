//! # rparif-influxdb
//!
//! Print on standard output (using [influxdb line protocol](https://!docs.influxdata.com/influxdb/v1.8/write_protocols/line_protocol_reference/)
//! for use with [telegraf's exec plugin](https://!docs.influxdata.com/telegraf/v1.14/plugins/plugin-list/#exec)) metrics
//! from [Airparif](https://!www.airparif.asso.fr/) (pollution index for Ile-de-France, France).
//
//! # Arguments
//!
//! * `-a, --api-key` : AirParif [API key](https://!www.airparif.asso.fr/rss/api)
//! * `-n, --name` : flag that allow converting [INSEE](https://!www.data.gouv.fr/en/datasets/correspondance-entre-les-codes-postaux-et-codes-insee-des-communes-francaises/) code into city name.
//! It has no effect if no INSEE code are given
//! * `-c, --city` : city [INSEE](https://!www.data.gouv.fr/en/datasets/correspondance-entre-les-codes-postaux-et-codes-insee-des-communes-francaises/) code
//! * `-p, --pollutant` : use pollutant name as field name instead of `index` only works if no INSEE code is given as it is
//! impossible in case of multiple pollutant to get index for each one
//!
//! To fetch data for multiple cities, use `-c` or `--city` for each cities, eg:
//! ```
//! rparif-influxdb --city 75101 --city 94028
//! ```
//!
//! # Tag day
//!
//! The tag day is an 'offset' from when the measure was made and the timestamp is set accordingly (adding or removing a day if day=next or day=previous)
//!
//! # Examples
//! * Fetch global and per pollutant indices for yesterday, today and tomorrow :
//! ```
//! rparif-influxdb --api-key my-api-key
//!
//! pollution,insee=0,day=previous,pollutant=global index=35 1590184800000000000
//! pollution,insee=0,day=previous,pollutant=no2 index=17 1590184800000000000
//! pollution,insee=0,day=previous,pollutant=o3 index=35 1590184800000000000
//! pollution,insee=0,day=previous,pollutant=pm10 index=31 1590184800000000000
//! pollution,insee=0,day=current,pollutant=global index=34 1590271200000000000
//! pollution,insee=0,day=current,pollutant=no2 index=17 1590271200000000000
//! pollution,insee=0,day=current,pollutant=o3 index=34 1590271200000000000
//! pollution,insee=0,day=current,pollutant=pm10 index=23 1590271200000000000
//! pollution,insee=0,day=next,pollutant=global index=45 1590357600000000000
//! pollution,insee=0,day=next,pollutant=no2 index=28 1590357600000000000
//! pollution,insee=0,day=next,pollutant=o3 index=45 1590357600000000000
//! pollution,insee=0,day=next,pollutant=pm10 index=25 1590357600000000000
//! ```
//!
//! * Fetch global and per pollutant indices for yesterday, today and tomorrow with
//! pollutant name as key field instead of 'index' :
//! ```
//! rparif-influxdb --api-key my-api-key --pollutant
//!
//! pollution,insee=0,day=previous global=35,no2=17,o3=35,pm10=31 1590184800000000000
//! pollution,insee=0,day=current global=34,no2=17,o3=34,pm10=23 1590271200000000000
//! pollution,insee=0,day=next global=45,no2=28,o3=45,pm10=25 1590357600000000000
//! ```
//!
//! * Fetch indices for INSEE 75101 (Paris 1er arr.) and 94028 (Créteil) without fetching city name (note that the index is
//! computed from all pollutants listed in `pollutant` tag) :
//! ```
//! rparif-influxdb --api-key my-api-key --city 75101 --city 94028
//!
//! pollution,insee=75101,day=previous,pollutant=o3\ pm10 index=32 1590184800000000000
//! pollution,insee=75101,day=current,pollutant=o3 index=35 1590271200000000000
//! pollution,insee=75101,day=next,pollutant=o3 index=40 1590357600000000000
//! pollution,insee=94028,day=previous,pollutant=o3 index=33 1590184800000000000
//! pollution,insee=94028,day=current,pollutant=o3 index=35 1590271200000000000
//! pollution,insee=94028,day=next,pollutant=o3 index=45 1590357600000000000
//! ```
//!
//! * Fetch indices for INSEE 75101 (Paris 1er arr.) and 94028 (Créteil) and fetch city name (note that the index is
//! computed from all pollutants listed in `pollutant` tag) :
//! ```
//! rparif-influxdb --api-key my-api-key --city 75101 --city 94028 --name
//!
//! pollution,insee=75101,city=Paris\ 1er\ Arrondissement,day=previous,pollutant=o3\ pm10 index=32 1590184800000000000
//! pollution,insee=75101,city=Paris\ 1er\ Arrondissement,day=current,pollutant=o3 index=35 1590271200000000000
//! pollution,insee=75101,city=Paris\ 1er\ Arrondissement,day=next,pollutant=o3 index=40 1590357600000000000
//! pollution,insee=94028,city=Creteil,day=previous,pollutant=o3 index=33 1590184800000000000
//! pollution,insee=94028,city=Creteil,day=current,pollutant=o3 index=35 1590271200000000000
//! pollution,insee=94028,city=Creteil,day=next,pollutant=o3 index=45 1590357600000000000
//! ```
//!
//! # Cross-compiling for raspberry pi
//! Install locally openssl (see https://!stackoverflow.com/a/37378989)
//! Install and setup toolchain & co : https://!medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b
//!
//! When build, add environment variable OPENSSL_STATIC=1
//! ```.env
//! OPENSSL_STATIC=1 cargo build --target=armv7-unknown-linux-gnueabihf
//! ```
//!
//! A build script `cross_build_deb_armhf.sh` is provided. Beware of the `rm -rf`it contains.
//!
//! # TODO
//! * Handle errors : instead of failing, error should be reported as metric
//!
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

use std::{env, fmt};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Formatter;

use args::Args;
use chrono::{Local, NaiveDateTime, NaiveTime, TimeZone};
use getopts::Occur;
use json::JsonValue;
use rparif::client::RParifClient;
use rparif::objects::{Day, Index};

use titlecase::titlecase;

const PROGRAM_DESC: &'static str =
    "Print on the standard output metric generated from AirParif using influxdb line protocol";
const MEASUREMENT: &'static str = "pollution";

trait InfluxLineProtocol {
    fn line_protocol(&self, names: &BTreeMap<String, String>) -> String;

    fn get_day_string(&self) -> &str;

    fn get_date_as_timestamp(&self) -> i64;
}


impl InfluxLineProtocol for Index {
    fn line_protocol(&self, names: &BTreeMap<String, String>) -> String {
        let mut result = String::new();
        result.push_str(MEASUREMENT);

        if self.insee().is_some() {
            result.push_str(format!(",insee={}", self.insee().unwrap()).as_str());
            if !names.is_empty() {
                let name = names.get(self.insee().unwrap().as_str());
                match name {
                    None => result.push_str(",city=none"),
                    Some(v) => result.push_str(format!(",city={}", v.replace(" ", "\\ ")).as_str()),
                }
            }
        } else {
            result.push_str(",insee=0");
        }

        result.push_str(",day=");
        result.push_str(self.get_day_string());

        if !self.pollutants().is_empty() {
            // Sort to have a deterministic order for tag value
            self.pollutants().sort();
            let pol = self.pollutants().join("\\ ");
            result.push_str(format!(",pollutant={}", pol).as_str());
        }

        result.push_str(format!(" index={}", self.index()).as_str());

        result.push_str(format!(" {}", self.get_date_as_timestamp()).as_str());

        result
    }

    fn get_day_string(&self) -> &str {
        let current = Local::today().naive_utc();
        if current > self.date() {
            "previous"
        } else if current == self.date() {
            "current"
        } else {
            "next"
        }
    }

    fn get_date_as_timestamp(&self) -> i64 {
        let local = Local::now();
        let offset = local.offset();
        let time = NaiveTime::from_hms_micro(0, 0, 0, 0);
        let date_time = NaiveDateTime::new(self.date(), time);

        offset
            .from_local_datetime(&date_time)
            .unwrap()
            .timestamp_nanos()
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
    args.flag("p", "pollutant", "Use polluant name for index field. If INSEE code is given, no effect.");

    let r = args.parse(input);
    if r.is_err() {
        println!("{}", args.full_usage());
        return Err(Box::new(r.err().unwrap()));
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
    let pollutant_field: bool = args.value_of("pollutant")?;

    let v = vec![Day::Yesterday, Day::Today, Day::Tomorrow];
    let client = RParifClient::new(api_key.as_str());
    if cities.is_empty() {
        for day in v.into_iter() {
            let result = client.index_day(day)?;
            if pollutant_field {
                let tmp = result.get(0).unwrap();
                let timestamp = tmp.get_date_as_timestamp();
                let first = format!("{},insee=0,day={}", MEASUREMENT, tmp.get_day_string());
                let second = result.into_iter().map(|v| format!("{}={}", v.pollutants().get(0).unwrap(), v.index())).collect::<Vec<String>>().join(",");
                println!("{} {} {}", first, second, timestamp);
            } else {
                for index in result.into_iter() {
                    println!("{}", index.line_protocol(&map));
                }
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
