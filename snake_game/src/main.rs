extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent};
use piston::window::WindowSettings;

use std::collections::LinkedList;


trait SnakeMotion {
    fn run(&mut self);
    fn turn(&mut self, d: SnakeDirection);
}

struct SnakeGame {
    gl:GlGraphics,
    snake:Snake,
}

enum SnakeDirection {
    Left,
    Right,
    Top,
    Bottom
}

struct SnakeNode {
    point:(f64,f64),
    direction:SnakeDirection,
}

impl SnakeMotion for SnakeNode {
    fn run(&mut self) {
        match self.direction {
            SnakeDirection::Top => { self.point.1 += 10.0; }
            SnakeDirection::Bottom => { self.point.1 -= 10.0}
            SnakeDirection::Right => { self.point.0 += 10.0}
            SnakeDirection::Left => { self.point.0 -= 10.0}
            _ => {}
        }
    }

    fn turn(&mut self, d:SnakeDirection) {
        self.direction = d;
    }
}

struct Snake {
    head:LinkedList<SnakeNode>,
}

impl SnakeMotion for Snake {
    fn run(&mut self) {
        for node in self.head.iter_mut() {
            node.run();
        }
    }

    fn turn(&mut self, d:SnakeDirection) {
        if let Some(node) = self.head.iter_mut().take(1).next() {
            node.turn(d);
        }
    }
}

impl SnakeGame {

    //Render the window
    fn render(&mut self, args:RenderArgs) {
        use graphics::*;

        const GREEN:[f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:[f32; 4] = [1.0, 0.0, 0.0, 1.0];
        

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

        });
    }


    fn update(&mut self, args:UpdateArgs) {
        self.snake.run();
    }
}

fn main() {
    //Define the OpenGL Version
    let opengl = OpenGL::V4_3;

    //Create a Window
    let mut window:Window = WindowSettings::new("Snake Game", (512,512))
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    
    //Create a game context
    let mut game:SnakeGame = SnakeGame {
        gl:GlGraphics::new(opengl),
        snake:Snake {
            head:LinkedList::new()
        }
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        //Mouse
        if let Some(cursor) = e.mouse_cursor_args() {
            println!("Cursor : {},{} ", cursor[0], cursor[1]);
        }

        //Render
        if let Some(args) = e.render_args() {
            game.render(args);
        } 

        //Update 
        if let Some(args) = e.update_args() {
            game.update(args);
        } 
    }

    
}
