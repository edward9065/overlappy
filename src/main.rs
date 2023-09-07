struct Region {
    chr: u8,
    start: u64,
    end: u64,
}

struct RegionSet {
    regions: Vec<Region>,
}


fn tokenize_region_set(universe: Vec<Region>, region_set: Vec<Region>) -> RegionSet {
    let universe_iter = universe.iter();
    let region_set_iter = region_set.iter();
    
    let comparator = |a, b| {
        if a.chr != b.chr {
            a.chr.cmp(&b.chr)
        } else if a.start != b.start {
            a.start.cmp(&b.start)
        } else {
            a.end.cmp(&b.end)
        }
    };

    universe.sort_by(comparator);
    region_set.sort_by(comparator);

    let mut new_region_set: Vec<Region> = Vec::new();

    for interval in universe_iter {
        let next_region = region_set_iter.next();
        match next_region {
            None => break,
            Some(region) => {
                if interval.end < region.start || region.end < interval.start {
                    continue;
                } else {
                    new_region_set.push(region);
                }
            }
        }
    }

    return RegionSet {
        regions: new_region_set
    }
}


fn main() {

}
