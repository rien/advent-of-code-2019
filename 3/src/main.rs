use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::ops::{Add};
use std::cmp;

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Self { x, y }
    }

    pub fn scale(self: &mut Self, scalar: isize) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    pub fn delay(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Segment {
    start: Point,
    end: Point,
    start_delay: usize,
    delay: usize,
}

impl Segment {

    pub fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn from_start(start: Point, start_delay: usize, directive: &str) -> Segment {
        let mut directive = directive.chars();
        let direction = directive.next().expect("should not be empty");
        let distance = directive.as_str().trim().parse::<usize>().expect("valid distance");
        let mut dir = match direction {
            'U' => Point::new(0, 1),
            'D' => Point::new(0, -1),
            'L' => Point::new(-1, 0),
            'R' => Point::new(1, 0),
            _   => panic!("invalid direction"),
        };
        dir.scale(distance as isize);

        let end = start.clone() + dir;
        Self {
            start,
            end,
            start_delay,
            delay: distance,
        }
    }

    pub fn crosses(&self, other: &Self) -> Option<Point> {
        let (hor, ver) = if self.horizontal() && !other.horizontal() {
            (self, other)
        } else if !self.horizontal() && other.horizontal() {
            (other, self)
        } else {
            return None;
        };

        Some(Point::new(ver.start.x, hor.start.y)).filter(|p| {
            let (min_x, max_x) = minmax(hor.start.x, hor.end.x);
            let (min_y, max_y) = minmax(ver.start.y, ver.end.y);

            min_x <= p.x && p.x <= max_x &&
            min_y <= p.y && p.y <= max_y
        })
    }

    pub fn from_directives(line: &str) -> Vec<Self> {
        let mut start = Point::new(0, 0);
        let mut delay = 0;
        let mut segments = Vec::new();
        for directive in line.split(',') {
            let next = Segment::from_start(start, delay, directive);
            delay += next.delay;
            start = next.end.clone();
            segments.push(next);
        }
        segments
    }
}

fn minmax(a: isize, b: isize) -> (isize, isize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}


fn distance(line1: &str, line2: &str) -> (usize, usize) {
    let line1 = Segment::from_directives(line1);
    let line2 = Segment::from_directives(line2);

    let mut min: Option<(usize, usize)> = None;
    for i in 0..line1.len() {
        for j in 0..line2.len() {
            if i == 0 && j == 0 {
                continue;
            } if let Some(cross) = line1[i].crosses(&line2[j]) {

                let s1 = &line1[i];
                let s2 = &line2[j];

                let cross_dist = cross.manhattan();
                let cross_delay = cross.delay(&s1.start) + s1.start_delay +
                        cross.delay(&s2.start) + s2.start_delay;

                min = Some(match min {
                    Some((dist, delay)) => (
                        cmp::min(dist, cross_dist),
                        cmp::min(delay, cross_delay)
                        ),
                    None => (cross_dist, cross_delay)
                });
            }
        }
    }
    min.expect("should have one intersection")
}

fn main() -> io::Result<()> {
    let mut lines = BufReader::new(File::open("input")?).lines();
    let (dist, delay) = distance(
        &lines.next().expect("first line").expect("line should exist"),
        &lines.next().expect("second line").expect("line should exist")
    );
    println!("Distance: {}", dist);
    println!("Delay: {}", delay);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\n",
                "U62,R66,U55,R34,D71,R55,D58,R83\n"
                ).0, 159);
    }
}
