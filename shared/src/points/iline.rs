use std::fmt;

use super::ipoint::IPoint;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ILine {
    pub min: IPoint,
    pub max: IPoint,
    pub from: IPoint,
    pub to: IPoint,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Intersection {
    Point(IPoint),
    Segment(ILine),
    None,
}

impl std::fmt::Display for ILine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}->{}]", self.from, self.to)
    }
}

impl ILine {
    pub fn new(a: IPoint, b: IPoint) -> Self {
        ILine {
            min: a.min(b),
            max: a.max(b),
            from: a,
            to: b,
        }
    }

    pub fn len(&self) -> usize {
        self.min.distance_to(self.max)
    }

    pub fn extended_line_contains(&self, point: IPoint) -> bool {
        (self.max.x - self.min.x) * (point.y - self.min.y)
            - (self.max.y - self.min.y) * (point.x - self.min.x)
            == 0
    }

    pub fn disc_contains(&self, point: IPoint) -> bool {
        (point - self.min).dot(point - self.max) <= 0
    }

    pub fn contains(&self, point: IPoint) -> bool {
        self.extended_line_contains(point) && self.disc_contains(point)
    }

    pub fn intersects(&self, other: &ILine) -> bool {
        Self::cross_points(&self.min, &self.max, &other.min)
            * Self::cross_points(&self.min, &self.max, &other.max)
            < 0
            && Self::cross_points(&other.min, &other.max, &self.min)
                * Self::cross_points(&other.min, &other.max, &self.max)
                < 0
    }

    fn cross_points(a: &IPoint, b: &IPoint, c: &IPoint) -> isize {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }

    pub fn orient(&self, other: &IPoint) -> isize {
        (self.to - self.from).cross(other - self.from)
    }

    pub fn points(&self) -> Vec<IPoint> {
        let dx = self.max.x.abs_diff(self.min.x) as isize;
        let sx = if self.min.x < self.max.x { 1 } else { -1 };
        let dy = -(self.max.y.abs_diff(self.min.y) as isize);
        let sy = if self.min.y < self.max.y { 1 } else { -1 };
        let mut error = dx + dy;

        let mut points = Vec::new();

        let mut x = self.min.x;
        let mut y = self.min.y;

        loop {
            points.push(IPoint::new(x, y));
            if x == self.max.x && y == self.max.y {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                error = error + dy;
                x += sx;
            }

            if e2 <= dx {
                error = error + dx;
                y += sy;
            }
        }

        points
    }

    pub fn crosses_straight(&self, other: &ILine) -> bool {
        let self_horizontal = self.min.x == self.max.x;
        let other_horizontal = other.min.x == other.max.x;

        match (self_horizontal, other_horizontal) {
            (true, true) => false,
            (true, false) => {
                other.min.x <= self.min.x
                    && other.max.x >= self.min.x
                    && self.min.y <= other.min.y
                    && self.max.y >= other.min.y
            }
            (false, true) => {
                other.min.y <= self.min.y
                    && other.max.y >= self.min.y
                    && self.min.x <= other.min.x
                    && self.max.x >= other.min.x
            }
            (false, false) => false,
        }
    }

    pub fn find_intersect(&self, other: &ILine) -> Intersection {
        // Compute determinants to check relative orientation
        let d1 = ((self.max.x - self.min.x).strict_mul(other.min.y - self.min.y)
            - (self.max.y - self.min.y).strict_mul(other.min.x - self.min.x))
            as i128;
        let d2 = ((self.max.x - self.min.x).strict_mul(other.max.y - self.min.y)
            - (self.max.y - self.min.y).strict_mul(other.max.x - self.min.x))
            as i128;

        let d3 = ((other.max.x - other.min.x).strict_mul(self.min.y - other.min.y)
            - (other.max.y - other.min.y).strict_mul(self.min.x - other.min.x))
            as i128;
        let d4 = ((other.max.x - other.min.x).strict_mul(self.max.y - other.min.y)
            - (other.max.y - other.min.y).strict_mul(self.max.x - other.min.x))
            as i128;

        // Case 1: Proper intersection
        if d1.strict_mul(d2) < 0 && d3.strict_mul(d4) < 0 {
            // Compute the intersection point
            let a1 = self.max.y - self.min.y;
            let b1 = self.min.x - self.max.x;
            let c1 = a1 * self.min.x + b1 * self.min.y;

            let a2 = other.max.y - other.min.y;
            let b2 = other.min.x - other.max.x;
            let c2 = a2 * other.min.x + b2 * other.min.y;

            let determinant = a1 * b2 - a2 * b1;

            if determinant == 0 {
                return Intersection::None; // Parallel lines
            }

            let x = (b2.strict_mul(c1) - b1.strict_mul(c2)) / determinant;
            let y = (a1.strict_mul(c2) - a2.strict_mul(c1)) / determinant;

            return Intersection::Point(IPoint::new(x, y));
        }

        // Case 2: Collinear lines
        if d1 == 0 && d2 == 0 && d3 == 0 && d4 == 0 {
            // Compute the overlapping range using bounding boxes
            let overlap_start_x = self.min.x.max(other.min.x);
            let overlap_end_x = self.max.x.min(other.max.x);
            let overlap_start_y = self.min.y.max(other.min.y);
            let overlap_end_y = self.max.y.min(other.max.y);

            if overlap_start_x > overlap_end_x || overlap_start_y > overlap_end_y {
                return Intersection::None; // No overlap
            }

            if overlap_start_x == overlap_end_x && overlap_start_y == overlap_end_y {
                return Intersection::Point(IPoint::new(overlap_start_x, overlap_start_y));
            }

            return Intersection::Segment(ILine::new(
                IPoint::new(overlap_start_x, overlap_start_y),
                IPoint::new(overlap_end_x, overlap_end_y),
            ));
        }

        // Case 3: Endpoint touching or one endpoint lies on the other segment
        if self.contains(other.min) {
            return Intersection::Point(other.min);
        }
        if self.contains(other.max) {
            return Intersection::Point(other.max);
        }
        if other.contains(self.min) {
            return Intersection::Point(self.min);
        }
        if other.contains(self.max) {
            return Intersection::Point(self.max);
        }

        Intersection::None
    }
}

#[cfg(test)]
mod iline_tests {
    use super::ILine;
    use crate::*;
    use points::{iline::Intersection, ipoint::IPoint};
    use test_case::test_case;

    #[test_case(
        ILine::new(IPoint::new(1, 0), IPoint::new(1, 2)),
        ILine::new(IPoint::new(0, 1), IPoint::new(2, 1)),
        IPoint::new(1, 1)
    )]
    #[test_case(
        ILine::new(IPoint::new(0, 0), IPoint::new(0, 1)),
        ILine::new(IPoint::new(0, 0), IPoint::new(1, 0)),
        IPoint::new(0, 0)
    )]
    fn intersection_point_test(a: ILine, b: ILine, expected: IPoint) {
        assert_eq!(a.find_intersect(&b), Intersection::Point(expected));
    }

    #[test_case(
        ILine::new(IPoint::new(1, 0), IPoint::new(3, 0)),
        ILine::new(IPoint::new(0, 0), IPoint::new(2, 0)),
        ILine::new(IPoint::new(1, 0), IPoint::new(2, 0))
    )]
    fn intersection_line_test(a: ILine, b: ILine, expected: ILine) {
        assert_eq!(a.find_intersect(&b), Intersection::Segment(expected));
    }

    #[test_case(
        ILine::new(IPoint::new(1, 0), IPoint::new(1, 2)),
        ILine::new(IPoint::new(0, 0), IPoint::new(0, 2))
    )]
    fn intersection_none_test(a: ILine, b: ILine) {
        assert_eq!(a.find_intersect(&b), Intersection::None);
    }

    #[test_case(
        ILine::new(IPoint::new(2, 6), IPoint::new(8, 6)),
        ILine::new(IPoint::new(4, 1), IPoint::new(4, 6)),
        IPoint::new(4, 6)
    )]
    fn intersection_endpoint_test(a: ILine, b: ILine, point: IPoint) {
        assert_eq!(a.find_intersect(&b), Intersection::Point(point));
    }
}
