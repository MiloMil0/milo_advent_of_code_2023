pub fn get_farming_methods() -> Vec<String> {
    vec![
        "seed-to-soil map:".to_string(),
        "soil-to-fertilizer map:".to_string(),
        "fertilizer-to-water map:".to_string(),
        "water-to-light map:".to_string(),
        "light-to-temperature map:".to_string(),
        "temperature-to-humidity map:".to_string(),
        "humidity-to-location map:".to_string(),
    ]
}
