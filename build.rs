// This has been adapted from the chrono-tz build script.
// https://github.com/chronotope/chrono-tz/blob/main/chrono-tz-build/src/lib.rs
// Chrono-TZ is dual-licensed under the MIT License and Apache 2.0 Licence.
// Copyright (c) 2016 Djzin
extern crate parse_zoneinfo;

use std::path::Path;

use parse_zoneinfo::line::Line;

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

fn main() {
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

    let _table = table.build();
}
