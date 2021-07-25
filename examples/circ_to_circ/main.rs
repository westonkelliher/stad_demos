use ggez::{Context, ContextBuilder, GameResult, conf};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::mint::Point2;
use ggez::input::mouse::MouseButton;


fn distance(a: Point2<f32>, b: Point2<f32>) -> f32{
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).powf(0.5)
}

struct Circ {
    a: Point2<f32>,
    r: f32,
}

fn main() -> Result<(), ggez::GameError> {

    let my_window_settings = conf::WindowSetup {
	title: "Circle Collision".to_owned(),
	samples: conf::NumSamples::One,
	vsync: true,
	icon: "".to_owned(),
	srgb: true,
    };

    let my_window_mode = conf::WindowMode {
	width: 1200.0,
	height: 1200.0,
	maximized: false,
	fullscreen_type: conf::FullscreenType::Windowed,
	borderless: false,
	min_width: 0.0,
	max_width: 0.0,
	min_height: 0.0,
	max_height: 0.0,
	resizable: false,
	visible: true,
	resize_on_scale_factor_change: false,
    };
    
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("game_name", "author_name")
	.window_setup(my_window_settings)
	.window_mode(my_window_mode)
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game)
}

struct MyGame {
    circs: [Circ;2],
    my_color: graphics::Color,
    dragging: Option<usize>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame {
	    circs: [
		Circ { a: Point2{ x:200.0, y:500.0}, r: 95.0},
		Circ { a: Point2{ x:800.0, y:300.0}, r: 185.0},		
	    ],
	    my_color: graphics::Color::new(0.05, 0.7, 0.25, 0.8),
	    dragging: None,
	}
    }
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
	if (distance(self.circs[0].a, self.circs[1].a) <
	    self.circs[0].r + self.circs[1].r) {
	    self.my_color =  graphics::Color::new(0.7, 0.45, 0.05, 0.8); 
	} else {
	    self.my_color = graphics::Color::new(0.05, 0.7, 0.25, 0.8);
	}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::WHITE);

	// Draw our two circles
	for c in self.circs.iter() {
	    let gcirc = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(),
						   c.a, c.r, 1.0, self.my_color)?;
	    graphics::draw(ctx, &gcirc, (Point2{x:0.0, y:0.0},))?;
	}

	// Draw distance line
	let yellow = graphics::Color::new(0.9, 0.7, 0.15, 0.7);
	let dist_line = graphics::Mesh::new_line(ctx, &[self.circs[0].a,
							self.circs[1].a],
						 8.0, yellow)?;
	graphics::draw(ctx, &dist_line, (Point2{x:0.0, y:0.0},))?;	

	// Draw Radii
	let purple = graphics::Color::new(0.5, 0.0, 0.8, 0.9);
	let total_dist = distance(self.circs[0].a, self.circs[1].a);
	let frac0 = self.circs[0].r / total_dist;
	let frac1 = self.circs[1].r / total_dist;
	let rad_x0 = (self.circs[0].a.x * (1.0 - frac0)) + (self.circs[1].a.x * frac0);
	let rad_y0 = (self.circs[0].a.y * (1.0 - frac0)) + (self.circs[1].a.y * frac0);
	let rad_x1 = (self.circs[1].a.x * (1.0 - frac1)) + (self.circs[0].a.x * frac1);
	let rad_y1 = (self.circs[1].a.y * (1.0 - frac1)) + (self.circs[0].a.y * frac1);
	let rad_p0 = Point2 { x: rad_x0, y: rad_y0 };
	let rad_p1 = Point2 { x: rad_x1, y: rad_y1 };
	let rad_line0 = graphics::Mesh::new_line(ctx, &[self.circs[0].a, rad_p0],
						 8.5, purple)?;
	let rad_line1 = graphics::Mesh::new_line(ctx, &[self.circs[1].a, rad_p1],
						 8.5, purple)?;
	graphics::draw(ctx, &rad_line0, (Point2{x:0.0, y:0.0},))?;
	graphics::draw(ctx, &rad_line1, (Point2{x:0.0, y:0.0},))?;
	
	// Draw selector indicators
	for c in self.circs.iter() {
	    let selector_circ = graphics
		::Mesh::new_circle(ctx, graphics::DrawMode::stroke(4.0), c.a, 15.0, 1.0,
				   graphics::Color::BLACK)?;
	    graphics::draw(ctx, &selector_circ, (Point2{x:0.0, y:0.0},))?;
	}
	
        graphics::present(ctx)
    }


    fn mouse_button_down_event(&mut self, _ctx: &mut Context,
			       button: MouseButton, x: f32, y: f32) {
	let mut n = 0;
	for c in self.circs.iter() {
	    if (distance(Point2{ x:x, y:y}, c.a ) < 15.0) {
		self.dragging = Some(n);
		return;
	    }
	    n += 1;
	}
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context,
			       button: MouseButton, x: f32, y: f32) {
	self.dragging = None;
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32,
			  xrel: f32, yrel: f32) {
	match self.dragging {
	    Some(i) => {
		self.circs[i].a.x = x;
		self.circs[i].a.y = y;		
	    },
	    None => (),
	};
    }
    
}
