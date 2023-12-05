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

    let seed_ranges = parse_seeds(&mut lines_iter);

    let mut category_map = CategoryMap::new();

    for _ in 0..7 {
        let (src, dst, ranges) = parse_category_map(&mut lines_iter);
        category_map.insert((src, dst), MapRanges::new(ranges));
    }

    let nearest_location = seed_ranges
        .into_iter()
        .map(|seed_range| map_seed_to_location(seed_range, &category_map))
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

#[derive(Debug)]
struct Range(std::ops::Range<u64>);

impl Range {
    pub fn is_valid(&self) -> bool {
        self.0.end > self.0.start
    }

    pub fn intersection(&self, other: &Range) -> Self {
        let start = std::cmp::max(self.0.start, other.0.start);
        let end = std::cmp::min(self.0.end, other.0.end);

        Self(start..end)
    }

    pub fn offset(&self, offset: i64) -> Self {
        let start = (self.0.start as i64 + offset) as u64;
        let end = (self.0.end as i64 + offset) as u64;

        Self(start..end)
    }
}

// (src, dst) -> ranges
type CategoryMap = HashMap<(Category, Category), MapRanges>;

#[derive(Debug)]
struct MapRanges(Vec<MapRange>);

impl MapRanges {
    pub fn new(mut ranges: Vec<MapRange>) -> Self {
        ranges.sort_by(|a, b| a.src_range.0.start.cmp(&b.src_range.0.start));
        Self(ranges)
    }

    fn get_contiguous_src_ranges(&self) -> Vec<(i64, Range)> {
        let mut contiguous_ranges = Vec::new();

        let mut last_end = 0;

        // Relies on the sorting done by the counstructor.
        for mr in self.0.iter() {
            if mr.src_range.0.start > last_end {
                contiguous_ranges.push((0, Range(last_end..mr.src_range.0.start)));
            }

            contiguous_ranges.push((
                mr.dst_offset,
                Range(mr.src_range.0.start..mr.src_range.0.end),
            ));

            last_end = mr.src_range.0.end;
        }

        contiguous_ranges.push((0, Range(last_end..u64::MAX)));

        contiguous_ranges
    }

    pub fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut res = Vec::new();

        for (offset, mr) in self.get_contiguous_src_ranges() {
            for r in ranges.iter() {
                let intersection = mr.intersection(r);

                if intersection.is_valid() {
                    res.push(intersection.offset(offset));
                }
            }
        }

        res
    }
}

#[derive(Debug)]
struct MapRange {
    src_range: Range,
    dst_offset: i64,
}

impl MapRange {
    pub fn new(src: u64, dst: u64, len: u64) -> Self {
        Self {
            src_range: Range(src..(src + len)),
            dst_offset: dst as i64 - src as i64,
        }
    }
}

fn parse_seeds(lines_iter: &mut Lines<BufReader<File>>) -> Vec<Range> {
    let seeds_str = lines_iter
        .next()
        .expect("Unexpected EOF")
        .expect("Failed reading line");

    let (_, seeds_part) = seeds_str.split_once(": ").expect("Seed part split failed");

    let mut seed_strs = seeds_part.split_ascii_whitespace();

    let mut seeds = Vec::new();

    while let Some(seed_str) = seed_strs.next() {
        let seed_num: u64 = seed_str.parse().expect("Seed num parse failed");
        let seed_len: u64 = seed_strs
            .next()
            .expect("Seed len not there")
            .parse()
            .expect("Seed len parse failed");

        seeds.push(Range(seed_num..(seed_num + seed_len)));
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

        ranges.push(MapRange::new(src, dst, len))
    }

    (src_category, dst_category, ranges)
}

fn map_seed_to_location(seed_range: Range, category_map: &CategoryMap) -> u64 {
    const MAPPING: [(Category, Category); 7] = [
        (Category::Seed, Category::Soil),
        (Category::Soil, Category::Fertilizer),
        (Category::Fertilizer, Category::Water),
        (Category::Water, Category::Light),
        (Category::Light, Category::Temperature),
        (Category::Temperature, Category::Humidity),
        (Category::Humidity, Category::Location),
    ];

    let mut ranges = vec![seed_range];

    for m in MAPPING {
        ranges = category_map
            .get(&m)
            .expect("Mapping should exist")
            .map_ranges(ranges);
    }

    ranges
        .into_iter()
        .map(|r| r.0.start)
        .min()
        .expect("Got empty range")
}
