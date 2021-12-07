use ggez::{Context, ContextBuilder, GameResult, conf};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::{DrawMode};
use ggez::mint::Point2;
use ggez::input::mouse::MouseButton;


fn distance(a: Point2<f32>, b: Point2<f32>) -> f32 {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).powf(0.5)
}
fn distance_ref(a: Point2<f32>, b: &Point2<f32>) -> f32 {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).powf(0.5)
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

struct Circ {
    a: Point2<f32>,
    r: f32,
}
impl Circ {
    
    fn get_mesh(&self, ctx: &mut Context, color: graphics::Color)
		-> GameResult<graphics::Mesh> {
	graphics::Mesh::new_circle(ctx, DrawMode::fill(),
				   self.a, self.r, 1.0, color)
    }

    fn get_radial_line_towards(&self, ctx: &mut Context, towards: Point2<f32>,
			       color: graphics::Color, thickness: f32)
			       -> GameResult<graphics::Mesh> {
	let towards_vec = minus(towards, self.a);
	let frac = self.r / distance(towards, self.a);
	let edge_point = plus(self.a, scaled(towards_vec, frac));
        graphics::Mesh::new_line(ctx, &[self.a, edge_point], thickness,
                                 color)
    }
}

struct Stad {
    a: Point2<f32>,
    b: Point2<f32>,
    r: f32,
}
impl Stad {

    fn get_radial_line_a_towards(&self, ctx: &mut Context, towards: Point2<f32>,
			       color: graphics::Color, thickness: f32)
			       -> GameResult<graphics::Mesh> {
	let towards_vec = minus(towards, self.a);
	let frac = self.r / distance(towards, self.a);
	let edge_point = plus(self.a, scaled(towards_vec, frac));
        graphics::Mesh::new_line(ctx, &[self.a, edge_point], thickness,
                                 color)
    }

    fn get_radial_line_b_towards(&self, ctx: &mut Context, towards: Point2<f32>,
			       color: graphics::Color, thickness: f32)
			       -> GameResult<graphics::Mesh> {
	let towards_vec = minus(towards, self.b);
	let frac = self.r / distance(towards, self.b);
	let edge_point = plus(self.b, scaled(towards_vec, frac));
        graphics::Mesh::new_line(ctx, &[self.b, edge_point], thickness,
                                 color)
    }

    fn get_extended_parallel(&self, ctx: &mut Context, color: graphics::Color,
                             thickness: f32) -> GameResult<graphics::Mesh> {
        let towards_vec = minus(self.a, self.b);
        let r_ratio = self.r / distance(self.a, self.b);
        let towards_vec_plus = scaled(towards_vec, 1.0 + 2.0*r_ratio);
        let a_extend = plus(self.a, towards_vec_plus);
        let b_extend = minus(self.b, towards_vec_plus);
        graphics::Mesh::new_line(ctx, &[a_extend, b_extend], thickness, color)
    }

    fn get_perpendicular_segment(&self, ctx: &mut Context, p: Point2<f32>,
                                 color: graphics::Color, thickness: f32)
                                 -> GameResult<graphics::Mesh> {
        
    }
}


struct MyGame {
    circ: Circ,
    stad: Stad,
    my_color: graphics::Color,
    dragging: Option<usize>,
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

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame {
	    circ: Circ { a: Point2{ x:600.0, y:700.0}, r: 70.0},
	    stad: Stad { a: Point2{ x:500.0, y:300.0},
			 b: Point2{ x:200.0, y:400.0}, r: 120.0},
	    my_color: graphics::Color::new(0.05, 0.7, 0.25, 0.8),
	    dragging: None,
	}
    }
}



impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
	//if (distance(self.circs[0].a, self.circs[1].a) <
	//self.circs[0].r + self.circs[1].r) {
	//self.my_color =  graphics::Color::new(0.7, 0.45, 0.05, 0.8); 
    //} else {
	//self.my_color = graphics::Color::new(0.05, 0.7, 0.25, 0.8);
    //}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        macro_rules! ezdraw{
            ($a:expr)=>{
                {
                    graphics::draw(ctx, & $a, (Point2{x:0.0, y:0.0},))?
                }
            }
        }

        // Clear
        graphics::clear(ctx, graphics::Color::WHITE);

	// Draw the circle
	let cmesh = self.circ.get_mesh(ctx, self.my_color)?;
	ezdraw!(cmesh);

	
	// Draw the stadium
	let my_stroke = match DrawMode::stroke(self.stad.r*2.0) {
	    DrawMode::Stroke(so) => {
		so.with_start_cap(graphics::LineCap::Round)
		    .with_end_cap(graphics::LineCap::Round)
	    },
	    DrawMode::Fill(_) => panic!("unreachable"),
	};
	let my_stroke_mode = DrawMode::Stroke(my_stroke);
	let gstad = graphics::Mesh::new_polyline(ctx, my_stroke_mode,
                                                 &[self.stad.a, self.stad.b],
                                                 self.my_color)?;
	ezdraw!(gstad);


	// Draw parallel extension
        let gray = graphics::Color::new(0.5, 0.5, 0.5, 0.5);
        let extline = self.stad.get_extended_parallel(ctx, gray, 6.0)?;
        ezdraw!(extline);

	// Draw distance lines
	let yellow = graphics::Color::new(0.9, 0.7, 0.15, 0.7);
	let dist_line1 = graphics::Mesh::new_line(ctx, &[self.circ.a, self.stad.a],
                                                  8.0, yellow)?;
	let dist_line2 = graphics::Mesh::new_line(ctx, &[self.circ.a, self.stad.b],
						  8.0, yellow)?;
	ezdraw!(dist_line1);
	ezdraw!(dist_line2);	

	
	// Draw Radii
        let purple = graphics::Color::new(0.5, 0.0, 0.8, 0.9);
        let radline_to_a = self.circ.get_radial_line_towards
            (ctx, self.stad.a, purple, 8.5)?;
        let radline_to_b = self.circ.get_radial_line_towards
            (ctx, self.stad.b, purple, 8.5)?;
	ezdraw!(radline_to_a);
	ezdraw!(radline_to_b);
        let radline_from_a = self.stad.get_radial_line_a_towards
            (ctx, self.circ.a, purple, 8.5)?;
        let radline_from_b = self.stad.get_radial_line_b_towards
            (ctx, self.circ.a, purple, 8.5)?;
	ezdraw!(radline_from_a);        
	ezdraw!(radline_from_b);


	// Draw perpendicular
	let orange = graphics::Color::new(0.9, 0.7, 0.15, 0.7);
	let perpen = self.stad.get_perpendicular_segment(ctx, self.circ.a, orange,
                                                         6.0)?;
        ezdraw!(perpen);

	// Draw selector indicators
        let circ_selec = graphics::Mesh::new_circle
            (ctx, DrawMode::stroke(4.0), self.circ.a, 15.0, 1.0,
             graphics::Color::BLACK)?;
        let a_selec = graphics::Mesh::new_circle
            (ctx, DrawMode::stroke(4.0), self.stad.a, 15.0, 1.0,
             graphics::Color::BLACK)?;
        let b_selec = graphics::Mesh::new_circle
            (ctx, DrawMode::stroke(4.0), self.stad.b, 15.0, 1.0,
             graphics::Color::BLACK)?;
        ezdraw!(circ_selec);
        ezdraw!(a_selec);
        ezdraw!(b_selec);
        
        graphics::present(ctx)
    }


    fn mouse_button_down_event(&mut self, _ctx: &mut Context,
			       button: MouseButton, x: f32, y: f32) {
	let mut n = 0;
	for p in [self.circ.a, self.stad.a, self.stad.b].iter() {
	    if (distance_ref(Point2{ x:x, y:y}, p) < 15.0) {
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
	    Some(0) => {
	        self.circ.a.x = x;
	        self.circ.a.y = y;		
            },
	    Some(1) => {
	        self.stad.a.x = x;
	        self.stad.a.y = y;		
            },
	    Some(2) => {
	        self.stad.b.x = x;
	        self.stad.b.y = y;		
            },
	    _ => (),
        };
    }
    
}
