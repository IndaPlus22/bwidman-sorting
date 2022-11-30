extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;
use rand::prelude::*;

const LIST_SIZE: usize = 100;
const FPS: u64 = 10;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window,
    events: Events,
    list: [u32;LIST_SIZE],
}

impl App {
    fn insertion_sort(&mut self) {
        for i in 1..self.list.len() {
            let x = self.list[i]; // Save current element
            
            // Loop through all previous elements until the beginning
            // or until an element greater than x appears
            let mut j = i;
            while j >= 1 && self.list[j - 1] > x {
                self.list[j] = self.list[j - 1]; // Move every element forward one step
                self.update_events(); // Update rendered list
                j -= 1;
            }
            self.list[j] = x;
        }
    }
    
    fn selection_sort(&mut self) {
        
        
        self.update_events(); // Update rendered list
    }
    
    fn merge_sort(&mut self) {
        
        
        self.update_events(); // Update rendered list
    }
    
    fn gnome_sort(&mut self) {
        
        
        self.update_events(); // Update rendered list
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let pillar_width: f64 = args.window_size[0] / (self.list.len() as f64);
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            
            for i in 0..self.list.len() {
                let (x, y) = (i as f64 * pillar_width, args.window_size[1]);
                let pillar = rectangle::rectangle_by_corners(
                    x, y,
                    x + pillar_width - 1.0, y - y * self.list[i] as f64 / LIST_SIZE as f64 // Scale pillar height to cover screen
                );
                // Draw pillar
                rectangle(WHITE, pillar, c.transform, gl);
            }
        });
    }
    
    fn update_events(&mut self) {
        if let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let window: Window = WindowSettings::new("Sorting algorithms", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Fill array with numbers
    let mut list = [0u32; LIST_SIZE];
    for i in 0..LIST_SIZE {
        list[i] = i as u32 + 1;
    }
    list.shuffle(&mut thread_rng());
    println!("Unsorted: {:?}", list);
    
    let events = Events::new(EventSettings::new().max_fps(FPS));

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        window,
        events,
        list,
    };
    
    app.insertion_sort();
    // app.selection_sort();
    // app.merge_sort();
    // app.gnome_sort();
    println!("Sorted: {:?}", app.list);

    // Keep window alive
    while let Some(e) = app.events.next(&mut app.window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
