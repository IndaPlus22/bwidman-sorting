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

// 1: Insertion sort
// 2: Selection sort
// 3: Merge sort
// 4: Slow sort
const ALGORITHM: u8 = 4;
const LIST_SIZE: usize = 50;
const FPS: u64 = 10;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window,
    events: Events,
}

impl App {
    fn insertion_sort(&mut self, list: &mut Vec<u32>) {
        for i in 1..list.len() {
            let x = list[i]; // Save current element
            
            // Loop through all previous elements until the beginning
            // or until an element greater than x appears
            let mut j = i;
            while j >= 1 && list[j - 1] > x {
                list[j] = list[j - 1]; // Move every element forward one step

                self.update_window(list, Some((j - 1, i))); // Update rendered list

                j -= 1;
            }
            list[j] = x;
        }
        return;
    }
    
    fn selection_sort(&mut self, list: &mut Vec<u32>) {
        // Loop through list and for every element search forward for a smaller element to switch place with
        for i in 0..list.len() {
            let mut min_index = i;
            for j in i+1..list.len() {
                if list[j] < list[min_index] {
                    min_index = j;
                }
                self.update_window(list, Some((j, min_index))); // Update rendered list
            }
            if min_index != i {
                list.swap(i, min_index);
                self.update_window(list, None); // Update rendered list
            }
        }
        return;
    }
    
    fn merge_sort(&mut self, list: &mut Vec<u32>) {
        if list.len() == 1 {
            return;
        }
        let length = list.len();
        let (left, right) = list.split_at_mut(length / 2);
        let mut left_vec = left.to_vec();
        let mut right_vec = right.to_vec();
        
        self.merge_sort(&mut left_vec);
        self.merge_sort(&mut right_vec);
        
        self.merge(list, &mut left_vec, &mut right_vec);
        return;
    }
    
    fn merge(&mut self, list: &mut Vec<u32>, left: &mut Vec<u32>, right: &mut Vec<u32>) {
        let mut merge = vec![];

        while left.len() > 0 && right.len() > 0 {
            if left[0] > right[0] {
                merge.push(right[0]);
                right.remove(0);
            } else {
                merge.push(left[0]);
                left.remove(0);
            }
        }
        while left.len() > 0 {
            merge.push(left[0]);
            left.remove(0);
        }
        while right.len() > 0 {
            merge.push(right[0]);
            right.remove(0);
        }

        *list = merge;
        self.update_window(list, None); // Update rendered list
        return;
    }

    /*
    * Sorts an array by ignoring it and then printing out a new, 
    * sorted array with its own "Alternative Values."
    *
    * If the new array does not appear sorted,
    * you have been manipulated by LSD
    */
    fn _conway_sort(&self, list: &mut Vec<u32>) {
        *list = vec![ 15, 16, 17, 18, 19, 20 ];
    }
    
    /*
    Complexity: n^(log_2(n)/2)
    */
    fn slow_sort(&mut self, list: &mut Vec<u32>, l: usize, r: usize) {
        if l >= r {
            return;
        }
        let m = (l + r) / 2; // Middle index

        self.slow_sort(list, l, m); // Find maximum of first half
        self.slow_sort(list, m + 1, r); // Find maximum of second half
        
        // Find the largest of the two maximum's found
        if list[m] > list[r] {
            list.swap(m, r);
        }
        self.update_window(list, Some((m, r))); // Update rendered list

        self.slow_sort(list, l, r - 1); // Recurse without maximum that's at the end
        
        return;
    }

    fn render(&mut self, args: &RenderArgs, list: &[u32], comparisons: Option<(usize, usize)>) {
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
                // Color pillar red if it's being compared against another
                if let Some((comparison1, comparison2)) = comparisons {
                    if i == comparison1 || i == comparison2 {
                        color = RED;
                    }
                }

                // Draw pillar
                rectangle(color, pillar, c.transform, gl);

                // Play tone

            }
        });
    }
    
    fn update_window(&mut self, list: &[u32], comparisons: Option<(usize, usize)>) {
        if let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args, list, comparisons);
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
    };

    let mut vec = list.to_vec();
    match ALGORITHM {
        1 => app.insertion_sort(&mut vec),
        2 => app.selection_sort(&mut vec),
        3 => app.merge_sort(&mut vec),
        4 => app.slow_sort(&mut vec, 0, LIST_SIZE - 1),
        _ => todo!(),
    };
    list = vec.try_into().expect("Wrong Vec size");
    println!("Sorted: {:?}", list);

    // Keep window alive
    while let Some(e) = app.events.next(&mut app.window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &list, None);
        }
    }
}
