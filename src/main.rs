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
use std::{thread, time};

const LIST_SIZE: usize = 64;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    list: [u32;LIST_SIZE],
}

impl App {
    fn insertion_sort(&mut self, args: &RenderArgs) {
        for i in 1..self.list.len() {
            let x = self.list[i]; // Save current element
            
            // Loop through all previous elements until the beginning
            // or until an element greater than x appears
            let mut j = i;
            while j >= 1 && self.list[j - 1] > x {
                self.list[j] = self.list[j - 1]; // Move every element forward one step
                self.render(&args); // Update rendered list

                // Delay to slow down the algorithm
                // let duration = time::Duration::from_millis(100);
                // thread::sleep(duration);
                j -= 1;
            }
            self.list[j] = x;
        }
    }
    
    fn selection_sort(&mut self, args: &RenderArgs) {
        
        
        self.render(&args); // Update rendered list
    }
    
    fn merge_sort(&mut self, args: &RenderArgs) {
        
        
        self.render(&args); // Update rendered list
    }
    
    fn gnome_sort(&mut self, args: &RenderArgs) {
        
        
        self.render(&args); // Update rendered list
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

    // fn update(&mut self, args: &UpdateArgs) {
    //     // Run sorting algorithm
    // }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Sorting algorithms", [800, 600])
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
    
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        list,
    };
    
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.insertion_sort(&args);
            // app.selection_sort(&args);
            // app.merge_sort(&args);
            // app.gnome_sort(&args);
        }

        // if let Some(args) = e.update_args() {
        //     app.update(&args);
        // }
    }
    println!("Sorted: {:?}", app.list);
}
