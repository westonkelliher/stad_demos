use ggez::mint::Point2;

fn distance(a: Point2<f32>, b: Point2<f32>) -> f32 {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).powf(0.5)
}
fn distance_ref(a: Point2<f32>, b: &Point2<f32>) -> f32 {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).powf(0.5)
}
fn length(a: Point2<f32>) -> f32 {
    (a.x.powf(2.0) + a.y.powf(2.0)).powf(0.5)
}

fn plus(a: Point2<f32>, b: Point2<f32>) -> Point2<f32> {
    Point2 {
	x: a.x + b.x,
	y: a.y + b.y,
    }
}

fn minus(a: Point2<f32>, b: Point2<f32>) -> Point2<f32> {
    Point2 {
	x: a.x - b.x,
	y: a.y - b.y,
    }
}

fn scaled(a: Point2<f32>, s: f32) -> Point2<f32> {
    Point2 {
	x: a.x * s,
	y: a.y * s,
    }
}

fn normalized(a: Point2<f32>) -> Point2<f32> {
    scaled(a, 1.0/length(a))
}






// `az + by + c = 0`
struct Line {
    a: f32,
    b: f32,
    c: f32,
}

impl Line {
    fn new(p1: Point2<f32>, p2: Point<f32>) -> Self {
	let diff = normalized(minus(p1, p2));
	let mut line: Line;
	line.a = -diff.y;
	line.b = diff.x;
	line.c = -(line.a*p1.x + line.b*p1.y);
	line
    }
    
    fn slope(&self) -> f32 {
	-x/y
    }
}
