use std::collections::HashMap;
use std::collections::HashSet;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

const DIM: usize = 3;

#[derive(Clone, Debug)]
pub struct Scan {
    idx: usize,
    beacons: HashSet<[isize; DIM]>,
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
        Ok(Self { idx, beacons })
    }
}

impl Scan {
    fn orientations(&self) -> Vec<Scan> {
        let mut orientations = Vec::new();
        for x in 0..DIM {
            for y in 0..DIM {
                for z in 0..DIM {
                    if x == y || y == z || x == z {
                        continue;
                    }
                    for x_sig in [false, true] {
                        for y_sig in [false, true] {
                            for z_sig in [false, true] {
                                orientations.push(Scan {
                                    idx: self.idx,
                                    beacons: self
                                        .beacons
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
                                });
                            }
                        }
                    }
                }
            }
        }
        orientations
    }

    /// Some sample of the beacons to check
    fn sample(&self) -> Vec<[isize; DIM]> {
        let mut corners = Vec::new();
        for dim in 0..DIM {
            corners.push(
                *self
                    .beacons
                    .iter()
                    .max_by_key(|beacon| beacon[dim])
                    .unwrap(),
            );
            corners.push(
                *self
                    .beacons
                    .iter()
                    .min_by_key(|beacon| beacon[dim])
                    .unwrap(),
            );
        }
        corners
    }

    /// Some sample of the beacons to check
    fn relative_to(&self, reference: &[isize; DIM]) -> HashSet<[isize; DIM]> {
        self
            .beacons
            .iter()
            .filter(|b| *b != reference)
            .map(|b| [b[0] - reference[0], b[1] - reference[1], b[2] - reference[2]])
            .collect()
    }

    /// Some sample of the beacons to check
    fn nearest(&self, reference: &[isize; DIM]) -> Vec<[isize; DIM]> {
        let mut nearest: Vec<[isize; DIM]> = self
            .relative_to(reference)
            .into_iter()
            .collect();
        nearest
            .sort_unstable_by_key(|k| k[0] + k[1] + k[2]);
        nearest
    }
}

#[derive(Debug)]
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
    fn map(&self) -> HashSet<[isize; DIM]> {
        let mut beacons: HashSet<[isize; DIM]> = HashSet::new();
        let first_scan = self.scans.first().unwrap();
        let fingerprint_beacons = first_scan.sample();
        log::trace!("fingerprint_beacons: {:?}", fingerprint_beacons);
        let fingerprint_beacons: HashMap<[isize; DIM], HashSet<[isize; DIM]>> = fingerprint_beacons
            .iter()
            .cloned()
            .map(|fingerprint_beacon| (fingerprint_beacon, first_scan.relative_to(&fingerprint_beacon)))
            .collect();
        'scan: for scan in self.scans.iter().skip(1) {
            let orientations = scan.orientations();
            log::debug!("scan: {} - orientations: {}", scan.idx, orientations.len());
            log::trace!("scan: {:?}", scan);
            for orientation in orientations {
                log::trace!("orientation: {:?}", orientation);
                for (_fingerprint_beacon, fingerprint_relatives) in &fingerprint_beacons {
                    log::trace!("fingerprint_relatives: {:?}", fingerprint_relatives);
                    for beacon in &orientation.beacons {
                        let relatives = orientation.relative_to(beacon);
                        log::trace!("relatives: {:?}", relatives);
                        let intersection: HashSet<[isize; DIM]> = relatives.intersection(&fingerprint_relatives).cloned().collect();
                        log::trace!("intersection: {:?}", intersection);
                        if intersection.len() >= 1{
                            log::debug!("intersection: {}", intersection.len());
                        }
                        if intersection.len() > 12 {
                            log::debug!("match! scan {}", scan.idx);
                            beacons.extend(relatives);
                            continue 'scan;
                        }
                    }
                }
            }
            log::error!("no match! scan {}", scan.idx);
        }
        beacons
    }
}

pub fn parse(input: &str) -> ParseResult<Report> {
    input.parse()
}

pub fn part1(something: &Report) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub fn part2(something: &Report) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<Report, usize> = Day {
    title: "Beacon Scanner",
    display: (
        "Foobar foobar foobar {answer}",
        "Foobar foobar foobar {answer}",
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
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_scan() {
        let scan: Scan = "--- scanner 0 ---\n-1,-1,1\n8,0,7".parse().unwrap();
        log::debug!("scan: {:?}", scan);
        log::debug!("orientations: {:#?}", scan.orientations(),)
    }

    #[test]
    fn test_example_report() {
        let report = parse(DAY.example).unwrap();
        log::debug!("report: {:?}", report);
        let beacons = report.map();
        log::debug!("beacons: {:?}", beacons);
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
