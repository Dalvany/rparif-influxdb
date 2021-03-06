# rparif-influxdb

Print on standard output (using [influxdb line protocol](https://docs.influxdata.com/influxdb/v1.8/write_protocols/line_protocol_reference/)
for use with [telegraf's exec plugin](https://docs.influxdata.com/telegraf/v1.14/plugins/plugin-list/#exec)) metrics
from [Airparif](https://www.airparif.asso.fr/) (pollution index for Ile-de-France, France).

# Arguments

* `-a, --api-key` : AirParif [API key](https://www.airparif.asso.fr/rss/api)
* `-n, --name` : flag that allow converting [INSEE](https://www.data.gouv.fr/en/datasets/correspondance-entre-les-codes-postaux-et-codes-insee-des-communes-francaises/) code into city name.
It has no effect if no INSEE code are given
* `-c, --city` : city [INSEE](https://www.data.gouv.fr/en/datasets/correspondance-entre-les-codes-postaux-et-codes-insee-des-communes-francaises/) code
* `-p, --pollutant` : use pollutant name as field name instead of `index` only works if no INSEE code is given as it is
impossible in case of multiple pollutant to get index for each one

To fetch data for multiple cities, use `-c` or `--city` for each cities, eg:
```
rparif-influxdb --city 75101 --city 94028
```

# Tag day

The tag day is an 'offset' from when the measure was made and the timestamp is set accordingly (adding or removing a day if day=next or day=previous)

# Examples
* Fetch global and per pollutant indices for yesterday, today and tomorrow :
```
rparif-influxdb --api-key my-api-key

pollution,insee=0,day=previous,pollutant=global index=35 1590184800000000000
pollution,insee=0,day=previous,pollutant=no2 index=17 1590184800000000000
pollution,insee=0,day=previous,pollutant=o3 index=35 1590184800000000000
pollution,insee=0,day=previous,pollutant=pm10 index=31 1590184800000000000
pollution,insee=0,day=current,pollutant=global index=34 1590271200000000000
pollution,insee=0,day=current,pollutant=no2 index=17 1590271200000000000
pollution,insee=0,day=current,pollutant=o3 index=34 1590271200000000000
pollution,insee=0,day=current,pollutant=pm10 index=23 1590271200000000000
pollution,insee=0,day=next,pollutant=global index=45 1590357600000000000
pollution,insee=0,day=next,pollutant=no2 index=28 1590357600000000000
pollution,insee=0,day=next,pollutant=o3 index=45 1590357600000000000
pollution,insee=0,day=next,pollutant=pm10 index=25 1590357600000000000
```

* Fetch global and per pollutant indices for yesterday, today and tomorrow with
pollutant name as key field instead of 'index' :
```
rparif-influxdb --api-key my-api-key --pollutant

pollution,insee=0,day=previous global=35,no2=17,o3=35,pm10=31 1590184800000000000
pollution,insee=0,day=current global=34,no2=17,o3=34,pm10=23 1590271200000000000
pollution,insee=0,day=next global=45,no2=28,o3=45,pm10=25 1590357600000000000
```

* Fetch indices for INSEE 75101 (Paris 1er arr.) and 94028 (Créteil) without fetching city name (note that the index is
computed from all pollutants listed in `pollutant` tag) :
```
rparif-influxdb --api-key my-api-key --city 75101 --city 94028

pollution,insee=75101,day=previous,pollutant=o3\ pm10 index=32 1590184800000000000
pollution,insee=75101,day=current,pollutant=o3 index=35 1590271200000000000
pollution,insee=75101,day=next,pollutant=o3 index=40 1590357600000000000
pollution,insee=94028,day=previous,pollutant=o3 index=33 1590184800000000000
pollution,insee=94028,day=current,pollutant=o3 index=35 1590271200000000000
pollution,insee=94028,day=next,pollutant=o3 index=45 1590357600000000000
```

* Fetch indices for INSEE 75101 (Paris 1er arr.) and 94028 (Créteil) and fetch city name (note that the index is
computed from all pollutants listed in `pollutant` tag) :
```
rparif-influxdb --api-key my-api-key --city 75101 --city 94028 --name

pollution,insee=75101,city=Paris\ 1er\ Arrondissement,day=previous,pollutant=o3\ pm10 index=32 1590184800000000000
pollution,insee=75101,city=Paris\ 1er\ Arrondissement,day=current,pollutant=o3 index=35 1590271200000000000
pollution,insee=75101,city=Paris\ 1er\ Arrondissement,day=next,pollutant=o3 index=40 1590357600000000000
pollution,insee=94028,city=Creteil,day=previous,pollutant=o3 index=33 1590184800000000000
pollution,insee=94028,city=Creteil,day=current,pollutant=o3 index=35 1590271200000000000
pollution,insee=94028,city=Creteil,day=next,pollutant=o3 index=45 1590357600000000000
```

# Cross-compiling for raspberry pi
Install locally openssl (see https://stackoverflow.com/a/37378989)  
Install and setup toolchain & co : https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b
 
When build, add environment variable OPENSSL_STATIC=1
```.env
OPENSSL_STATIC=1 cargo build --target=armv7-unknown-linux-gnueabihf
```

A build script `cross_build_deb_armhf.sh` is provided. Beware of the `rm -rf`it contains.

# TODO
* Handle errors : instead of failing, error should be reported as metric

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.