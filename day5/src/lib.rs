use std::io::BufRead;

pub struct MapRule {
    pub src_start: u32,
    pub dst_start: u32,
    pub range: u32,
}

pub struct Map {
    maps: Vec<MapRule>,
}

impl Map {
    pub fn from_unsorted_rules(mut rules: Vec<MapRule>) -> Self {
        rules.sort_by_key(|r| r.src_start);
        Self { maps: rules }
    }

    pub fn map(&self, src: u32) -> u32 {
        match self.maps.binary_search_by_key(&src, |r| r.src_start) {
            Ok(i) => self.maps[i].dst_start, // found direct mapping
            Err(i) if i == 0 || (self.maps[i - 1].src_start + self.maps[i - 1].range - 1) < src => {
                src
            } // no mapping found
            Err(i) => src + self.maps[i - 1].dst_start - self.maps[i - 1].src_start, // do the mapping
        }
    }
}

pub fn compute_loc(maps: &[Map], seed: u32) -> u32 {
    maps.iter().fold(seed, |src, map| map.map(src))
}

pub fn compute_lowest_location(reader: impl BufRead) -> u32 {
    let mut lines = reader.lines();
    let seeds_str = lines
        .next()
        .expect("missing input")
        .expect("failed to read line");
    let seeds = seeds_str
        .split_whitespace()
        .flat_map(|s| s.parse::<u32>())
        .collect::<Vec<u32>>();

    lines.next();
    let mut maps = Vec::new();
    while let Some(_) = lines.next() {
        let mut rules = Vec::new();
        while let Some(line) = lines.next() {
            let line = line.expect("failed to read line");
            if line == "" {
                break;
            }
            let rule_vals = line
                .split_whitespace()
                .flat_map(|s| s.parse::<u32>())
                .collect::<Vec<u32>>();
            rules.push(MapRule {
                src_start: rule_vals[1],
                dst_start: rule_vals[0],
                range: rule_vals[2],
            });
        }
        maps.push(Map::from_unsorted_rules(rules));
    }

    seeds
        .into_iter()
        .map(|seed| compute_loc(&maps, seed))
        .min()
        .unwrap()
}
