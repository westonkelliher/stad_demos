use ggez::{Context, ContextBuilder, GameResult, conf};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::{DrawMode, FillOptions};
use ggez::input::mouse::MouseButton;
use ggez::mint::Point2;

mod geometry;
use geometry::*;

/*
    fn get_line(&self) -> Line {
	Line::new_pp(self.a, self.b)
    }
    
    fn get_radial_line_a_towards(&self, ctx: &mut Context, towards: Point<f64>,
			       color: graphics::Color, thickness: f64)
			       -> GameResult<graphics::Mesh> {
	let towards_vec = minus(towards, self.a);
	let frac = self.r / distance(towards, self.a);
	let edge_point = plus(self.a, scaled(towards_vec, frac));
        graphics::Mesh::new_line(ctx, &[self.a, edge_point], thickness,
                                 color)
    }

    fn get_radial_line_b_towards(&self, ctx: &mut Context, towards: Point<f64>,
			       color: graphics::Color, thickness: f64)
			       -> GameResult<graphics::Mesh> {
	let towards_vec = minus(towards, self.b);
	let frac = self.r / distance(towards, self.b);
	let edge_point = plus(self.b, scaled(towards_vec, frac));
        graphics::Mesh::new_line(ctx, &[self.b, edge_point], thickness,
                                 color)
    }

    fn get_extended_parallel(&self, ctx: &mut Context, color: graphics::Color,
                             thickness: f64) -> GameResult<graphics::Mesh> {
        let towards_vec = minus(self.a, self.b);
        let r_ratio = self.r / distance(self.a, self.b);
        let towards_vec_plus = scaled(towards_vec, 1.0 + 2.0*r_ratio);
        let a_extend = plus(self.a, towards_vec_plus);
        let b_extend = minus(self.b, towards_vec_plus);
        graphics::Mesh::new_line(ctx, &[a_extend, b_extend], thickness, color)
    }

    fn get_perpendicular_segment(&self, ctx: &mut Context, p: Point<f64>,
                                 color: graphics::Color, thickness: f64)
                                 -> GameResult<graphics::Mesh> {
	let stad_line = Line::new_pp(self.a, self.b);
	let inv_slope = -1.0/stad_line.slope();
	let perp_line = Line::new_sp(inv_slope, p);
	let intersect = stad_line.intersection(&perp_line);
	graphics::Mesh::new_line(ctx, &[intersect, p], thickness, color)
    }

    fn get_perpendicular_radius(&self, ctx: &mut Context, p: Point<f64>,
                                 color: graphics::Color, thickness: f64)
                                 -> GameResult<graphics::Mesh> {
	let stad_line = Line::new_pp(self.a, self.b);
	let inv_slope = -1.0/stad_line.slope();
	let perp_line = Line::new_sp(inv_slope, p);
	let intersect = stad_line.intersection(&perp_line);
	let towards_vec = normalized(minus(p, intersect));
	let cp = plus(intersect, scaled(towards_vec, self.r));
	graphics::Mesh::new_line(ctx, &[intersect, cp], thickness, color)
    }
*/


struct MyGame {
    stad_a: Stad,
    stad_b: Stad,
    my_color: graphics::Color,
    color1: graphics::Color,
    color2: graphics::Color,
    color3: graphics::Color,
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
    let (mut ctx, event_loop) =
       ContextBuilder::new("game_name", "author_name")
	.window_setup(my_window_settings)
	.window_mode(my_window_mode)
        .build()
        .unwrap();

    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game)
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
	    stad_a: Stad { p1: Point{ x:100.0, y:100.0},
			   p2: Point{ x:200.0, y:100.0}, r: 20.0},
	    stad_b: Stad { p1: Point{ x:100.0, y:300.0},
			   p2: Point{ x:300.0, y:300.0}, r: 120.0},
	    my_color: graphics::Color::new(0.05, 0.7, 0.25, 0.8),
	    color1: graphics::Color::new(0.1, 0.1, 0.1, 0.8),
	    color2: graphics::Color::new(0.1, 0.1, 0.1, 0.8),
	    color3: graphics::Color::new(0.1, 0.1, 0.1, 0.8),
	    dragging: None,
	}
    }
}



impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        macro_rules! ezdraw{
            ($a:expr)=>{
                {
                    graphics::draw(ctx, & $a, (Point2::<f32>{x:0.0, y:0.0},))?
                }
            }
        }

        // Clear
        graphics::clear(ctx, graphics::Color::WHITE);

	// Change color for collision
	//if self.stad_a.collides_stad(self.stad_b, true) {
	if self.stad_b.segment().shadows_point(self.stad_a.p1) {
	    self.my_color = graphics::Color::new(0.7, 0.45, 0.05, 0.8);
	} else {
	    self.my_color = graphics::Color::new(0.05, 0.7, 0.25, 0.8);
	}


	// draw indicators
	let c1 = graphics::Mesh::new_circle(ctx, DrawMode::Fill(FillOptions::even_odd()),
					    Point{x:800.0,y:700.0}.mint(), 20.0, 1.0, self.color1)?;
	ezdraw!(c1);
	
	// Draw the stadiums
	let my_stroke = match DrawMode::stroke((self.stad_a.r*2.0) as f32) {
	    DrawMode::Stroke(so) => {
		so.with_start_cap(graphics::LineCap::Round)
		    .with_end_cap(graphics::LineCap::Round)
	    },
	    DrawMode::Fill(_) => panic!("unreachable"),
	};
	let my_stroke_mode = DrawMode::Stroke(my_stroke);
	let gstad_a = graphics::Mesh::new_polyline(ctx, my_stroke_mode,
                                                 &[self.stad_a.p1.mint(), self.stad_a.p2.mint()],
                                                 self.my_color)?;
	let my_stroke2 = match DrawMode::stroke((self.stad_b.r*2.0) as f32) {
	    DrawMode::Stroke(so) => {
		so.with_start_cap(graphics::LineCap::Round)
		    .with_end_cap(graphics::LineCap::Round)
	    },
	    DrawMode::Fill(_) => panic!("unreachable"),
	};
	let my_stroke_mode2 = DrawMode::Stroke(my_stroke2);
	let gstad_b = graphics::Mesh::new_polyline(ctx, my_stroke_mode2,
                                                 &[self.stad_b.p1.mint(), self.stad_b.p2.mint()],
                                                 self.my_color)?;
	ezdraw!(gstad_a);
	ezdraw!(gstad_b);


	/*
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
	*/
	
	// Draw Radii
        /*
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


	// Draw Perpendicularity
	let orange = graphics::Color::new(0.9, 0.7, 0.15, 0.7);
	let perpen = self.stad.get_perpendicular_segment
	    (ctx, self.circ.a, orange, 6.0)?;
        ezdraw!(perpen);
	 */
	
	// Draw Perpendicular Radii
	/*
        let purple2 = graphics::Color::new(0.7, 0.1, 0.7, 0.9);
	let perp_rad_from_circ = self.circ.get_radial_line_towards_line
	    (ctx, self.stad.get_line(), purple2, 6.0)?;
	let perp_rad_from_stad = self.stad.get_perpendicular_radius
	    (ctx, self.circ.a, purple2, 6.0)?;
	ezdraw!(perp_rad_from_circ);
	ezdraw!(perp_rad_from_stad);
	 */
	
	// Draw selector indicators
        let a1_selec = graphics::Mesh::new_circle
            (ctx, DrawMode::stroke(4.0), self.stad_a.p1.mint(), 15.0, 1.0,
             graphics::Color::BLACK)?;
        let a2_selec = graphics::Mesh::new_circle
            (ctx, DrawMode::stroke(4.0), self.stad_a.p2.mint(), 15.0, 1.0,
             graphics::Color::BLACK)?;
        let b1_selec = graphics::Mesh::new_circle
            (ctx, DrawMode::stroke(4.0), self.stad_b.p1.mint(), 15.0, 1.0,
             graphics::Color::BLACK)?;
        let b2_selec = graphics::Mesh::new_circle
            (ctx, DrawMode::stroke(4.0), self.stad_b.p2.mint(), 15.0, 1.0,
             graphics::Color::BLACK)?;
        ezdraw!(a1_selec);
        ezdraw!(a2_selec);
        ezdraw!(b1_selec);
        ezdraw!(b2_selec);
        
        graphics::present(ctx)
    }


    fn mouse_button_down_event(&mut self, _ctx: &mut Context,
			       _button: MouseButton, x: f32, y: f32) {
	for (n, p) in [self.stad_a.p1, self.stad_a.p2, self.stad_b.p1, self.stad_b.p2].iter().enumerate() {
	    if p.distance(Point{ x:x as f64, y:y as f64}) < 15.0 {
	        self.dragging = Some(n);
	        return;
            }
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context,
			       _button: MouseButton, _x: f32, _y: f32) {
	self.dragging = None;
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32,
			  _xrel: f32, _yrel: f32) {
        match self.dragging {
	    Some(0) => {
	        self.stad_a.p1.x = x as f64;
	        self.stad_a.p1.y = y as f64;		
            },
	    Some(1) => {
	        self.stad_a.p2.x = x as f64;
	        self.stad_a.p2.y = y as f64;		
            },
	    Some(2) => {
	        self.stad_b.p1.x = x as f64;
	        self.stad_b.p1.y = y as f64;		
            },
	    Some(3) => {
	        self.stad_b.p2.x = x as f64;
	        self.stad_b.p2.y = y as f64;		
            },
	    _ => (),
        };
    }
    
}
