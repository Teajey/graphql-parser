use std::fs::File;
use std::io::Write;

const KITCHEN_SINK_DOCUMENT: &str = include_str!("queries/kitchen-sink.graphql");

#[test]
fn kitchen_sink() {
    let document = graphql_parser::parse_query::<String>(KITCHEN_SINK_DOCUMENT).unwrap();
    let json = serde_json::to_value(document).unwrap();
    let mut file = File::create("./tests/queries/kitchen-sink_non-canonical.json").unwrap();
    let string = serde_json::to_string_pretty(&json).unwrap();
    write!(file, "{}", string).unwrap();
}
