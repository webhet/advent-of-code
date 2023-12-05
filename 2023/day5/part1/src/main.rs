use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening input file: {err}");
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();

    let seeds = parse_seeds(&mut lines_iter);

    let mut category_map = CategoryMap::new();

    for _ in 0..7 {
        let (src, dst, ranges) = parse_category_map(&mut lines_iter);
        category_map.insert((src, dst), MapRanges(ranges));
    }

    let nearest_location = seeds
        .into_iter()
        .map(|seed| map_seed_to_location(seed, &category_map))
        .min()
        .expect("No seeds");

    println!("Nearest location num: {nearest_location}");
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for Category {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err("Unknown category"),
        }
    }
}

// (src, dst) -> ranges
type CategoryMap = HashMap<(Category, Category), MapRanges>;

#[derive(Debug)]
struct MapRanges(Vec<MapRange>);

impl MapRanges {
    pub fn map_num(&self, num: u64) -> u64 {
        for range in self.0.iter() {
            if range.contains(num) {
                return range.map_num(num);
            }
        }

        num
    }
}

#[derive(Debug)]
struct MapRange {
    src: u64,
    dst: u64,
    len: u64,
}

impl MapRange {
    pub fn contains(&self, num: u64) -> bool {
        num >= self.src && num < (self.src + self.len)
    }

    pub fn map_num(&self, num: u64) -> u64 {
        num - self.src + self.dst
    }
}

fn parse_seeds(lines_iter: &mut Lines<BufReader<File>>) -> Vec<u64> {
    let seeds_str = lines_iter
        .next()
        .expect("Unexpected EOF")
        .expect("Failed reading line");

    let (_, seeds_part) = seeds_str.split_once(": ").expect("Seed part split failed");

    let seed_strs = seeds_part.split_ascii_whitespace();

    let mut seeds = Vec::new();

    for seed_str in seed_strs {
        let seed_num = seed_str.parse().expect("Seed num parse failed");
        seeds.push(seed_num);
    }

    // Consume empty line
    lines_iter.next().expect("Unexpected EOF").ok();

    seeds
}

fn parse_category_map(
    lines_iter: &mut Lines<BufReader<File>>,
) -> (Category, Category, Vec<MapRange>) {
    let header_line = lines_iter
        .next()
        .expect("Unexpected EOF")
        .expect("Failed reading line");
    let mapping = header_line
        .split_once(' ')
        .expect("Map header split failed")
        .0;

    let (src, dst) = mapping.split_once("-to-").expect("Mapping split failed");
    let src_category = Category::try_from(src).expect("Unknown catoegory");
    let dst_category = Category::try_from(dst).expect("Unknown catoegory");

    let mut ranges = Vec::new();

    while let Some(Ok(line)) = lines_iter.next() {
        if line.is_empty() {
            break;
        }

        let mut range_raw = line.splitn(3, ' ');

        let dst = range_raw
            .next()
            .unwrap()
            .parse()
            .expect("Range num parse failed");
        let src = range_raw
            .next()
            .unwrap()
            .parse()
            .expect("Range num parse failed");
        let len = range_raw
            .next()
            .unwrap()
            .parse()
            .expect("Range num parse failed");

        ranges.push(MapRange { src, dst, len })
    }

    (src_category, dst_category, ranges)
}

fn map_seed_to_location(seed: u64, category_map: &CategoryMap) -> u64 {
    const MAPPING: [(Category, Category); 7] = [
        (Category::Seed, Category::Soil),
        (Category::Soil, Category::Fertilizer),
        (Category::Fertilizer, Category::Water),
        (Category::Water, Category::Light),
        (Category::Light, Category::Temperature),
        (Category::Temperature, Category::Humidity),
        (Category::Humidity, Category::Location),
    ];

    let mut num = seed;

    for m in MAPPING {
        num = category_map
            .get(&m)
            .expect("Mapping should exist")
            .map_num(num);
    }

    num
}
