#![allow(dead_code)]

use ggez::mint::Point2;

// TODO: find all places where 'failure if vertical' and do a double invert around those

// Misc ////////
pub fn inverse(a: f64) -> f64 {
    -1.0/a
}
///////////////



/////////////////////////////////////////////////////////////////////////////////////
// Vector
//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}
impl Vector {

    pub fn magnitude(&self) -> f64 {
	(self.x.powf(2.0) + self.y.powf(2.0)).powf(0.5)
    }

    pub fn scaled(&self, s: f64) -> Vector {
	Vector {
	    x: self.x * s,
	    y: self.y * s,
	}
    }

    pub fn normalized(&self) -> Vector {
	self.scaled(1.0/self.magnitude())
    }

    pub fn slope(&self) -> f64 {
	self.y/self.x
    }
}
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^



/////////////////////////////////////////////////////////////////////////////////////
// Point
//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {

    pub fn mint(&self) -> Point2::<f32> {
	Point2::<f32> {
	    x: self.x as f32,
	    y: self.y as f32,
	}
    }
    
    pub fn inverted(&self) -> Point {
	Point {
	    x: self.y,
	    y: self.x,
	}
    }

    pub fn distance(&self, other: Point) -> f64 {
	((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).powf(0.5)
    }

    pub fn plus(&self, other: Vector) -> Point {
	Point {
	    x: self.x + other.x,
	    y: self.y + other.y,	    
	}
    }

    pub fn minus(&self, other: Vector) -> Point {
	Point {
	    x: self.x - other.x,
	    y: self.y - other.y,	    
	}
    }

    pub fn towards(&self, other: Point) -> Vector {
	Vector {
	    x: other.x - self.x,
	    y: other.y - self.y,
	}
    }

    pub fn above(&self, line: Line) -> bool {
	self.y > line.y(self.x)
    }
}
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^



/////////////////////////////////////////////////////////////////////////////////////
// Line
//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
// `ax + by + c = 0`
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl Line {

    pub fn inverted(&self) -> Line {
	Line {
	    a: self.b,
	    b: self.a,
	    c: self.c,
	}
    }
    
    // the line that passes through p1 and p2
    pub fn from_point_point(p1: Point, p2: Point) -> Line {
        let niff = p1.towards(p2).normalized(); // normalized diff
        Line {
            a: niff.y,
            b: -niff.x,
            c: -niff.y*p1.x + niff.x*p1.y,
        }
    }

    // the line that passes through point p with slope s
    pub fn from_point_slope(p: Point, s: f64) -> Self {
        let niff = Vector{x:1.0, y:s}.normalized(); // normalized diff
        Line {
            a: niff.y,
            b: -niff.x,
            c: -niff.y*p.x + niff.x*p.y,
        }
    }

    pub fn slope(&self) -> f64 {
	-self.a/self.b
    }

    pub fn x(&self, y: f64) -> f64 {
	// x = -(by + c)/a
	-(self.b*y + self.c)/self.a
    }

    pub fn y(&self, x: f64) -> f64 {
	// y = -(ax + c)/b
	-(self.a*x + self.c)/self.b
    }

    // The point where self and other intersect (via Cramer's rule)
    pub fn intersection(&self, other: Line) -> Point {
	let x = ( self.b*other.c - other.b*self.c) / (self.a*other.b - other.a*self.b);
	let y = (-self.a*other.c + other.a*self.c) / (self.a*other.b - other.a*self.b);
	Point{x:x, y:y}
    }

    // The line that is perpendicular to self and passes through p
    pub fn perpendicular_through(&self, p: Point) -> Line {
	Line::from_point_slope(p, inverse(self.slope()))
    }

    pub fn distance_to_point(&self, p: Point)  -> f64{
	let projection = self.intersection(self.perpendicular_through(p));
	projection.distance(p)
    }
}
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^



/////////////////////////////////////////////////////////////////////////////////////
// Segment
//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Segment {
    pub p1: Point,
    pub p2: Point,
}
impl Segment {

    fn length(&self) -> f64 {
	self.p1.distance(self.p2)
    }

    fn line(&self) -> Line {
	Line::from_point_point(self.p1, self.p2)
    }
    
    // the line perpendicular to the stad that passes through p
    pub fn perpendicular_through(&self, p: Point) -> Line {
	let slope = self.p1.towards(self.p2).slope();
	Line::from_point_slope(p, inverse(slope))
    }

    // the line perpendicular to the segment that passes through p1
    pub fn perpendicular_1(&self) -> Line {
	self.perpendicular_through(self.p1)
    }
    
    // the line perpendicular to the segment that passes through p2
    pub fn perpendicular_2(&self) -> Line {
	self.perpendicular_through(self.p2)
    }

    // returns true if p falls within the shadow of self where you can imagine
    // the segment casting a shadow on both sides perpendicular to itself such
    // that the width of the shadow is the length of the segment
    pub fn shadows_point(&self, p: Point) -> bool {
	p.above(self.perpendicular_1()) != p.above(self.perpendicular_2())
    }

    pub fn distance_to_point(&self, p: Point) -> f64 {
	if self.shadows_point(p) {
	    self.line().distance_to_point(p)
	} else {
	    self.p1.distance(p).min(self.p2.distance(p))
	}
    }

    // returns whichever end (p1 or p2) is nearest p, with the restriction that
    // p must be on self.line() and p does not fall on the segment
    pub fn near_point_along(&self, p: Point) -> Point {
	if (p.x > self.p1.x) == (self.p1.x > self.p2.x) {
	    self.p1
	} else {
	    self.p2
	}
	// TODO: fails if vertical
    }

    pub fn distance_to_segment(&self, other: Segment) -> f64 {
	let intersec = self.line().intersection(other.line());
	//if print { println!("{}, {}", self.shadows_point(intersec), other.shadows_point(intersec)); }
	match (self.shadows_point(intersec), other.shadows_point(intersec)) {
	    (true, true)  => 0.0,
	    (true, false) => self.line().distance_to_point(
		other.near_point_along(intersec)),
	    (false, true) => other.line().distance_to_point(
		self.near_point_along(intersec)),
	    (false, false) => {
		let self_near_p  = self.near_point_along(intersec);
		let other_near_p = other.near_point_along(intersec);
		if self.shadows_point(other_near_p) {
		    self.line().distance_to_point(other_near_p)
		} else if other.shadows_point(self_near_p) {
		    other.line().distance_to_point(self_near_p)
		} else {
		    self_near_p.distance(other_near_p)
		}
	    }
	}
	// How this function works is not obvious but I'm not keen on explaining it
	// here ;)
    }
}
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^



/////////////////////////////////////////////////////////////////////////////////////
// Circ
//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Circle {
    pub p: Point,
    pub r: f64,
}
impl Circle {
    // TODO: collides_circle(), collides_stad()
}
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^



/////////////////////////////////////////////////////////////////////////////////////
// Stad
//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Stad {
    pub p1: Point,
    pub p2: Point,
    pub r: f64,
}
impl Stad {

    pub fn new(x1: f64, y1: f64, mut x2: f64, mut y2: f64, r: f64) -> Stad {
	// a cheap hack to avoid perfectly vertical slopes
	// TODO: handle vertical slopes well
	if x1 == x2 {
	    x2 += 0.001;
	}
	if y1 == y2 {
	    y2 += 0.001;
	}
	Stad {
	    p1: Point{x:x1, y:y1},
	    p2: Point{x:x2, y:y2},
	    r: r,
	}
    }
    
    pub fn segment(&self) -> Segment {
	Segment {
	    p1: self.p1,
	    p2: self.p2,
	}
    }

    pub fn line(&self) -> Line {
	Line::from_point_point(self.p1, self.p2)
    }

    pub fn collides_stad(&self, other: Stad) -> bool {
	self.segment().distance_to_segment(other.segment()) < self.r + other.r
    }
}
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
