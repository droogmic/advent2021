use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

const DIM: usize = 3;

// #[derive(Clone, Debug)]
// pub struct Beacon([isize; DIM]);

type Position = [isize; DIM];
type Scanner = Position;
type Beacon = Position;
type BeaconSet = BTreeSet<Beacon>;

#[derive(Clone, Debug)]
pub struct Scan {
    idx: usize,
    beacons: BeaconSet,
    orientations: Vec<BeaconSet>,
}

impl std::str::FromStr for Scan {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r#"^--- scanner (\d+) ---$"#).unwrap();
        let caps = re.captures(s.lines().next().unwrap()).unwrap();
        let idx = caps
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .map_err(ParseError::Int)?;
        let beacons = s
            .lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split(',');
                let point = [
                    parts
                        .next()
                        .ok_or(ParseError::Empty)?
                        .parse()
                        .map_err(ParseError::Int)?,
                    parts
                        .next()
                        .ok_or(ParseError::Empty)?
                        .parse()
                        .map_err(ParseError::Int)?,
                    parts
                        .next()
                        .ok_or(ParseError::Empty)?
                        .parse()
                        .map_err(ParseError::Int)?,
                ];
                if parts.next().is_some() {
                    return Err(ParseError::Str("too much".to_owned()));
                }
                Ok(point)
            })
            .collect::<ParseResult<_>>()?;
        Ok(Self::new(idx, beacons))
    }
}

impl Scan {
    fn new(idx: usize, beacons: BeaconSet) -> Self {
        let mut orientations = Vec::new();
        // I can throw half of these away, but I don't know which ones...
        for x in 0..DIM {
            for y in 0..DIM {
                for z in 0..DIM {
                    if x == y || y == z || x == z {
                        continue;
                    }
                    for x_sig in [false, true] {
                        for y_sig in [false, true] {
                            for z_sig in [false, true] {
                                orientations.push(
                                    beacons
                                        .iter()
                                        .map(|p| {
                                            let mut x = p[x];
                                            if x_sig {
                                                x = x.checked_neg().unwrap();
                                            }
                                            let mut y = p[y];
                                            if y_sig {
                                                y = y.checked_neg().unwrap();
                                            }
                                            let mut z = p[z];
                                            if z_sig {
                                                z = z.checked_neg().unwrap();
                                            }
                                            [x, y, z]
                                        })
                                        .collect(),
                                );
                            }
                        }
                    }
                }
            }
        }
        Scan {
            idx,
            beacons,
            orientations,
        }
    }

    /// Get beasons relative to some reference
    fn beacons_relative_to(&self, reference: &Beacon) -> BeaconSet {
        beacons_relative_to(&self.beacons, reference)
    }
}

/// Get beasons relative to some reference
fn beacons_relative_to(beacons: &BeaconSet, reference: &[isize; DIM]) -> BeaconSet {
    beacons
        .iter()
        .filter(|b| *b != reference)
        .map(|b| {
            [
                b[0] - reference[0],
                b[1] - reference[1],
                b[2] - reference[2],
            ]
        })
        .collect()
}

/// Some sample of the beacons to check
fn beacons_sample(beacons: &BeaconSet) -> BeaconSet {
    let mut edges = BeaconSet::new();
    for dim in 0..DIM {
        edges.insert(*beacons.iter().max_by_key(|beacon| beacon[dim]).unwrap());
        edges.insert(*beacons.iter().min_by_key(|beacon| beacon[dim]).unwrap());
    }
    edges
}

#[derive(Clone, Debug)]
pub struct Report {
    scans: Vec<Scan>,
}

impl std::str::FromStr for Report {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scans = s
            .split("\n\n")
            .map(|lines| lines.parse())
            .collect::<ParseResult<_>>()?;
        Ok(Self { scans })
    }
}

impl Report {
    pub fn map(&self) -> (Vec<Scanner>, BeaconSet) {
        let first_scan = self.scans.first().unwrap();
        let mut beacons: BeaconSet = first_scan.beacons.clone();
        log::trace!("beacons: {:?}", beacons);
        let mut reference_beacons: BTreeMap<Beacon, BeaconSet> = beacons
            .iter()
            .cloned()
            .map(|beacon| (beacon, first_scan.beacons_relative_to(&beacon)))
            .collect();
        let mut edge_beacons = beacons_sample(&beacons);
        let mut prev_scans_seen = Vec::new();
        let mut scans_seen = vec![first_scan.idx];
        let mut scanners = vec![[0, 0, 0]];
        while scans_seen.len() < self.scans.len() {
            if prev_scans_seen == scans_seen {
                log::warn!("dead end!");
            }
            prev_scans_seen = scans_seen.clone();
            'scan: for scan in self
                .scans
                .iter()
                .filter(|s| !prev_scans_seen.contains(&s.idx))
            {
                let orientations = &scan.orientations;
                log::debug!(
                    "scan: {} - orientations: {} - reference_beacons: {}",
                    scan.idx,
                    orientations.len(),
                    reference_beacons.len()
                );
                log::trace!("scan: {:?}", scan);
                for orientation_beacons in orientations {
                    log::trace!("orientation_beacons: {:?}", orientation_beacons);
                    let heuristic_beacons = if prev_scans_seen == scans_seen {
                        &beacons
                    } else {
                        &edge_beacons
                    };
                    for reference_beacon in heuristic_beacons {
                        let reference_relatives = reference_beacons.get(reference_beacon).unwrap();
                        log::trace!("reference_relatives: {:?}", reference_relatives);
                        for beacon in orientation_beacons {
                            let relatives = beacons_relative_to(orientation_beacons, beacon);
                            log::trace!("relatives: {:?}", relatives);
                            let intersection: BeaconSet = relatives
                                .intersection(reference_relatives)
                                .cloned()
                                .collect();
                            log::trace!("intersection: {:?}", intersection);
                            if intersection.len() >= 11 {
                                // We count the centre beacon
                                log::debug!("match! scan {}", scan.idx);
                                let scanner = [
                                    reference_beacon[0] - beacon[0],
                                    reference_beacon[1] - beacon[1],
                                    reference_beacon[2] - beacon[2],
                                ];
                                log::debug!("scanner: {:?}", scanner);
                                scanners.push(scanner);
                                let remapped_beacons: BeaconSet = relatives
                                    .into_iter()
                                    .map(|b| {
                                        [
                                            reference_beacon[0] + b[0],
                                            reference_beacon[1] + b[1],
                                            reference_beacon[2] + b[2],
                                        ]
                                    })
                                    .collect();
                                beacons.extend(remapped_beacons);
                                log::trace!("beacons: {:?}", beacons);
                                reference_beacons = beacons
                                    .iter()
                                    .cloned()
                                    .map(|beacon| (beacon, beacons_relative_to(&beacons, &beacon)))
                                    .collect();
                                edge_beacons = beacons_sample(&beacons);
                                scans_seen.push(scan.idx);
                                continue 'scan;
                            }
                        }
                    }
                }
                log::info!("no match! scan {}", scan.idx);
            }
        }
        (scanners, beacons)
    }
}

pub fn parse(input: &str) -> ParseResult<Report> {
    input.parse()
}

pub fn part1(report: &Report) -> PartOutput<usize> {
    let (_scanners, beacons) = report.map();
    PartOutput {
        answer: beacons.len(),
    }
}

pub fn part2(report: &Report) -> PartOutput<usize> {
    let (scanners, _beacons) = report.map();
    let mut max = 0;
    for a_scanner in &scanners {
        for b_scanner in &scanners {
            let manhattan: usize = (0..DIM)
                .map(|dim| (b_scanner[dim] - a_scanner[dim]).abs())
                .sum::<isize>()
                .try_into()
                .unwrap();
            if manhattan > max {
                max = manhattan
            }
        }
    }
    PartOutput { answer: max }
}

pub const DAY: Day<Report, usize> = Day {
    title: "Beacon Scanner",
    display: (
        "There are {answer} beacons",
        "The largest Manhattan distance is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day19.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::get_input;
    use test_log::test;

    #[test]
    fn test_scan() {
        let scan: Scan = "--- scanner 0 ---\n-1,-1,1\n8,0,7".parse().unwrap();
        log::debug!("scan: {:?}", scan);
        log::debug!("scan.orientations: {:#?}", scan.orientations)
    }

    #[test]
    fn test_example_report() {
        let report = parse(DAY.example).unwrap();
        log::debug!("report: {:?}", report);
        let (_scanners, beacons) = report.map();
        log::debug!("beacons: {:?}", beacons);
        log::debug!("beacons.len(): {}", beacons.len());
        assert_eq!(beacons.len(), 79)
    }

    // #[test]
    // fn test_example_part1() {
    //     let report = parse(DAY.example).unwrap();
    //     let result = play(&something);
    //     assert_eq!(result, -1);
    // }

    // #[test]
    // fn test_example_part2() {
    //     let something = parse(DAY.example).unwrap();
    //     let result = play(&something);
    //     assert_eq!(result, -1);
    // }

    // #[test]
    // fn test_main() {
    //     let something = parse(&get_input(0)).unwrap();
    //     assert_eq!(part1(&something).answer.to_string(), "-1");
    //     assert_eq!(part2(&something).answer.to_string(), "-1");
    // }
}
