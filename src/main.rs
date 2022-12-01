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
    comparisons: Option<(usize, usize)>
}

impl App {
    fn insertion_sort(&mut self, list: &mut [u32]) -> Vec<u32> {
        for i in 1..list.len() {
            let x = list[i]; // Save current element
            
            // Loop through all previous elements until the beginning
            // or until an element greater than x appears
            let mut j = i;
            while j >= 1 && list[j - 1] > x {
                list[j] = list[j - 1]; // Move every element forward one step

                self.comparisons = Some((j - 1, i));
                self.update_window(list); // Update rendered list

                j -= 1;
            }
            list[j] = x;
        }
        list.to_vec()
    }
    
    fn selection_sort(&mut self, list: &mut [u32]) -> Vec<u32> {
        // Loop through list and for every element search forward for a smaller element to switch place with
        for i in 0..list.len() {
            let mut minIndex = i;
            for j in i+1..list.len() {
                if list[j] < list[minIndex] {
                    minIndex = j;
                }

                self.comparisons = Some((j, minIndex));
                self.update_window(list); // Update rendered list
            }
            if minIndex != i {
                list.swap(i, minIndex);
                self.update_window(list); // Update rendered list
            }
        }
        list.to_vec()
    }
    
    fn merge_sort(&mut self, list: &mut [u32]) -> Vec<u32> {
        if list.len() == 1 {
            return list.to_vec()
        }
        let newlist = &mut list[0..5];
        self.merge_sort(newlist);
        
        self.update_window(list); // Update rendered list
        list.to_vec()
    }
    
    fn gnome_sort(&mut self, list: &mut [u32]) -> Vec<u32> {
        
        
        self.update_window(list); // Update rendered list
        list.to_vec()
    }

    fn render(&mut self, args: &RenderArgs, list: &[u32]) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        
        let pillar_width: f64 = args.window_size[0] / (list.len() as f64);
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            
            for i in 0..list.len() {
                let (x, y) = (i as f64 * pillar_width, args.window_size[1]);
                let pillar = rectangle::rectangle_by_corners(
                    x, y,
                    x + pillar_width - 1.0, y - y * list[i] as f64 / LIST_SIZE as f64 // Scale pillar height to cover screen
                );
                let mut color = WHITE;
                // Color pillar red if it's beign compared against another
                if let Some((comparison1, comparison2)) = self.comparisons {
                    if i == comparison1 || i == comparison2 {
                        color = RED;
                    }
                }

                // Draw pillar
                rectangle(color, pillar, c.transform, gl);
            }
        });
        self.comparisons = None; // Wipe potential comparison markings
    }
    
    fn update_window(&mut self, list: &[u32]) {
        if let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args, list);
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
        comparisons: None
    };

    list = app.insertion_sort(&mut list).try_into().unwrap();
    // list = app.selection_sort(&mut list).try_into().unwrap();
    // list = app.merge_sort(&mut list).try_into().unwrap();
    // list = app.gnome_sort(&mut list).try_into().unwrap();
    println!("Sorted: {:?}", list);

    // Keep window alive
    while let Some(e) = app.events.next(&mut app.window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &list);
        }
    }
}
