// This has been adapted from the chrono-tz build script.
// https://github.com/chronotope/chrono-tz/blob/main/chrono-tz-build/src/lib.rs
// Chrono-TZ is dual-licensed under the MIT License and Apache 2.0 Licence.
// Copyright (c) 2016 Djzin
extern crate parse_zoneinfo;

use std::io::Write;
use std::path::Path;

use parse_zoneinfo::line::Line;
use parse_zoneinfo::table::Table;
use parse_zoneinfo::transitions::{FixedTimespan, TableTransitions};

// This function is needed until zoneinfo_parse handles comments correctly.
// Technically a '#' symbol could occur between double quotes and should be
// ignored in this case, however this never happens in the tz database as it
// stands.
fn strip_comments(mut line: String) -> String {
    if let Some(pos) = line.find('#') {
        line.truncate(pos);
    };
    line
}

// Convert all '/' to '__', all '+' to 'Plus' and '-' to 'Minus', unless
// it's a hyphen, in which case remove it. This is so the names can be used
// as rust identifiers.
fn convert_bad_chars(name: &str) -> String {
    let name = name.replace("/", "__").replace("+", "Plus");
    if let Some(pos) = name.find('-') {
        if name[pos + 1..].chars().next().map(char::is_numeric).unwrap_or(false) {
            name.replace("-", "Minus")
        } else {
            name.replace("-", "")
        }
    } else {
        name
    }
}

fn write_timezone_file(f: &mut std::fs::File, table: &Table) -> std::io::Result<()> {
    let zones =
        table.zonesets.keys().chain(table.links.keys()).collect::<std::collections::BTreeSet<_>>();
    writeln!(f, "use crate::timezone::{{TzInfo, TzOffset}};\n\n")?;
    writeln!(f, "#[derive(Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(f, "pub enum Tz {{")?;
    for zone in &zones {
        writeln!(f, "    /// {}", zone)?;
        writeln!(f, "   {},", convert_bad_chars(zone))?;
    }
    writeln!(f, "}}\n")?;

    writeln!(f, "impl Tz {{")?;
    writeln!(f, "    pub fn name(self) -> &'static str {{")?;
    writeln!(f, "        match self {{")?;
    for zone in &zones {
        let zone_name = convert_bad_chars(zone);
        writeln!(f, "            Tz::{} => \"{}\",", zone_name, zone)?;
    }
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    writeln!(f, "impl std::fmt::Debug for Tz {{")?;
    writeln!(f, "    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{")?;
    writeln!(f, "        f.write_str(self.name().as_ref())")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    writeln!(f, "impl std::fmt::Display for Tz {{")?;
    writeln!(f, "    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{")?;
    writeln!(f, "        f.write_str(self.name().as_ref())")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    writeln!(f, "impl Tz {{")?;
    writeln!(f, "    pub fn tz_info(&self) -> TzInfo {{")?;
    writeln!(f, "        match self {{")?;
    for zone in &zones {
        let zone_name = convert_bad_chars(zone);
        let timespans = table.timespans(zone).unwrap();
        writeln!(f, "            Self::{} => {{", zone_name)?;
        writeln!(f, "                const TZ: TzInfo = TzInfo {{")?;
        writeln!(
            f,
            "                    first: TzOffset {{ utc_offset: {}, dst_offset: {} }},",
            timespans.first.utc_offset, timespans.first.dst_offset
        )?;
        writeln!(f, "                    rest: &[")?;
        for (start, FixedTimespan { utc_offset, dst_offset, .. }) in timespans.rest {
            writeln!(
                f,
                "                    ({}, TzOffset {{ utc_offset: {}, dst_offset: {} }}),",
                start, utc_offset, dst_offset
            )?;
        }
        writeln!(f, "                    ],")?;
        writeln!(f, "                }};")?;
        writeln!(f, "                TZ")?;
        writeln!(f, "        }},")?;
    }
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    Ok(())
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let parser = parse_zoneinfo::line::LineParser::new();
    let mut table = parse_zoneinfo::table::TableBuilder::new();

    let tzfiles = [
        "tz/africa",
        "tz/antarctica",
        "tz/asia",
        "tz/australasia",
        "tz/backward",
        "tz/etcetera",
        "tz/europe",
        "tz/northamerica",
        "tz/southamerica",
    ];

    let lines = tzfiles
        .iter()
        .map(Path::new)
        .map(|p| {
            Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::new()))
                .join(p)
        })
        .map(|path| {
            std::fs::File::open(&path)
                .unwrap_or_else(|e| panic!("cannot open {}: {}", path.display(), e))
        })
        .map(std::io::BufReader::new)
        .flat_map(std::io::BufRead::lines)
        .map(Result::unwrap)
        .map(strip_comments);

    for line in lines {
        match parser.parse_str(&line).unwrap() {
            Line::Zone(zone) => table.add_zone_line(zone).unwrap(),
            Line::Continuation(cont) => table.add_continuation_line(cont).unwrap(),
            Line::Rule(rule) => table.add_rule_line(rule).unwrap(),
            Line::Link(link) => table.add_link_line(link).unwrap(),
            Line::Space => {}
        }
    }

    let table = table.build();
    let timezone_path = Path::new(&out_dir).join("timezone_data.rs");
    let mut timezone_file = std::fs::File::create(&timezone_path).unwrap();
    write_timezone_file(&mut timezone_file, &table).unwrap();
}
