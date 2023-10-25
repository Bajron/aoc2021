use aoc2021::input::*;
#[derive(Debug, Clone, Copy)]
struct Cuboid {
    state: bool,
    x: Range,
    y: Range,
    z: Range,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    low: i32,
    high: i32,
}

impl Range {
    fn size(&self) -> usize {
        (self.high.abs_diff(self.low) + 1) as usize
    }
    fn intersects(&self, other: &Range) -> bool {
        self.is_in(other.low)
            || self.is_in(other.high)
            || other.is_in(self.low)
            || other.is_in(self.high)
    }

    fn contains(&self, other: &Range) -> bool {
        self.low <= other.low && other.high <= self.high
    }

    fn is_in(&self, p: i32) -> bool {
        return self.low <= p && p <= self.high;
    }

    fn common(&self, other: &Range) -> Vec<i32> {
        let mut ret = vec![];
        // FIXME: weirdly patched
        if self.is_in(other.low) && other.low != self.low {
            ret.push(other.low);
        }
        if self.is_in(other.high) && other.high != self.high {
            ret.push(other.high);
        }
        ret
    }

    fn split_ranges(&self, other: &Range) -> Vec<Range> {
        if other.contains(self) {
            return vec![self.clone()];
        }
        // FIXME: This logic is wonky
        let c = self.common(other);
        match c[..] {
            [] => {
                vec![]
            }
            [mid] => {
                let m = if mid == other.high { mid } else { mid - 1 };
                [
                    Range {
                        low: self.low,
                        high: m,
                    },
                    Range {
                        low: m + 1,
                        high: self.high,
                    },
                ]
                .into()
            }
            [m1, m2] => [
                Range {
                    low: self.low,
                    high: m1 - 1,
                },
                Range { low: m1, high: m2 },
                Range {
                    low: m2 + 1,
                    high: self.high,
                },
            ]
            .into(),
            _ => panic!("impossible!"),
        }
    }
}

impl Cuboid {
    fn size(&self) -> usize {
        self.x.size() * self.y.size() * self.z.size()
    }
    fn intersects(&self, other: &Cuboid) -> bool {
        self.x.intersects(&other.x) && self.y.intersects(&other.y) && self.z.intersects(&other.z)
    }

    fn contains(&self, other: &Cuboid) -> bool {
        self.x.contains(&other.x) && self.y.contains(&other.y) && self.z.contains(&other.z)
    }

    // fn same_location(&self, other: &Cuboid) -> bool {
    //     return self.x == other.x && self.y == other.y && self.z == other.z;
    // }

    fn split(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.intersects(other) {
            return vec![];
        }
        let x_ranges = self.x.split_ranges(&other.x);
        let y_ranges = self.y.split_ranges(&other.y);
        let z_ranges = self.z.split_ranges(&other.z);

        // println!("x ranges {x_ranges:?}");
        // println!("y ranges {y_ranges:?}");
        // println!("z ranges {z_ranges:?}");

        let mut cuboids = vec![];
        for xr in &x_ranges {
            for yr in &y_ranges {
                for zr in &z_ranges {
                    cuboids.push(Cuboid {
                        state: self.state,
                        x: xr.clone(),
                        y: yr.clone(),
                        z: zr.clone(),
                    })
                }
            }
        }
        cuboids
    }
}

fn to_range(s: &str) -> Option<Range> {
    if let Some(s) = s.split("=").skip(1).next() {
        match &s.split("..").collect::<Vec<&str>>()[..] {
            [l, h] => l.parse::<i32>().ok().and_then(|l| {
                h.parse::<i32>()
                    .ok()
                    .and_then(|h| Some(Range { low: l, high: h }))
            }),
            _ => None,
        }
    } else {
        None
    }
}

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let players = lines
            .filter_map(|l| l.ok())
            .filter_map(|x| match &x.split_whitespace().collect::<Vec<&str>>()[..] {
                [state, coordinates] => match coordinates.split(",").collect::<Vec<&str>>()[..] {
                    [x, y, z] => to_range(x).and_then(|x| {
                        to_range(y).and_then(|y| {
                            to_range(z).and_then(|z| {
                                Some(Cuboid {
                                    state: *state == "on",
                                    x: x,
                                    y: y,
                                    z: z,
                                })
                            })
                        })
                    }),
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<Cuboid>>();
        Some(players)
    });

    if let Some(cuboids) = input {
        let low: i32 = -50;
        let high: i32 = 50;
        let need = (high - low + 1) as usize;

        let mut space: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; need]; need]; need];

        for c in &cuboids {
            println!("{c:?}");
            for x in c.x.low.max(low)..=c.x.high.min(high) {
                for y in c.y.low.max(low)..=c.y.high.min(high) {
                    for z in c.z.low.max(low)..=c.z.high.min(high) {
                        let xx = (x - low) as usize;
                        let yy = (y - low) as usize;
                        let zz = (z - low) as usize;

                        space[xx][yy][zz] = c.state;
                    }
                }
            }
        }

        let lit = space.iter().flatten().flatten().filter(|&&x| x).count();
        println!("Lit: {lit}");

        let mut on: Vec<Cuboid> = vec![];

        for cuboid in cuboids.iter() {
            let mut next = vec![];

            for o in on {
                if o.intersects(cuboid) {
                    // throw away common part, keep what is left
                    let parts = o.split(cuboid);
                    // println!("Split into {}", parts.len());
                    // println!("o: {o:?}");
                    // println!("c: {cuboid:?}");

                    assert!(o.size() == parts.iter().map(|c| c.size()).sum::<usize>());
                    assert!(parts.iter().filter(|p| cuboid.contains(p)).count() == 1);
                    for p in parts {
                        if !cuboid.contains(&p) {
                            next.push(p)
                        }
                    }
                } else {
                    next.push(o)
                }
            }

            assert!(next.iter().filter(|c| cuboid.intersects(c)).count() == 0);

            if cuboid.state {
                next.push(cuboid.clone())
            }
            on = next;

            println!(" ~lit~ {}", on.iter().map(|c| c.size()).sum::<usize>());
        }
        let on = on;
        let total_lit: usize = on.iter().map(|c| c.size()).sum();
        println!("Total lit: {total_lit}")
    }
}
