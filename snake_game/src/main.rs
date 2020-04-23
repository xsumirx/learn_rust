extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent, ButtonEvent, ButtonArgs};
use piston::window::WindowSettings;
use piston::{Button, keyboard, ButtonState};

use std::collections::LinkedList;
use rand::{thread_rng, Rng};


trait SnakeMotion {
    fn run(&mut self);
    fn turn(&mut self, d: SnakeDirection);
}

struct SnakeGame {
    gl:GlGraphics,
    snake:Snake,
}

#[derive(Copy, Clone, Debug)]
enum SnakeDirection {
    Left,
    Right,
    Top,
    Bottom
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x:f64,
    y:f64
}

#[derive(Clone, Copy, Debug)]
struct Color {
    color:[f32; 4]
}

struct SnakeNode {
    point:Point,
    color:Color
}

impl SnakeNode {
    fn run(&mut self, d:SnakeDirection) {
        match d {
            SnakeDirection::Top => { 
                self.point.y -= 10.0;
                if self.point.y < 0.0 {
                    self.point.y = 500.0;
                }
            }
            SnakeDirection::Bottom => { 
                self.point.y += 10.0;
                if self.point.y > 500.0 {
                    self.point.y = 0.0;
                }
            }
            SnakeDirection::Right => { 
                self.point.x += 10.0;
                if self.point.x > 500.0 {
                    self.point.x = 0.0;
                }
            }
            SnakeDirection::Left => { 
                self.point.x -= 10.0;
                if self.point.x < 0.0 {
                    self.point.x = 500.0;
                }
            }
        }
    }

    fn walk(&mut self, pos:Point) {
        self.point = pos;
    }
}

struct Snake {
    //Snake Link List
    head:LinkedList<SnakeNode>,

    //Current direction in which snake has to move
    direction:SnakeDirection,

    //A random snake node which snake needs to eat to grow
    food:Option<SnakeNode>,

    //Pause
    is_pause:bool,
}

impl SnakeMotion for Snake {

    // Move all snake node
    fn run(&mut self) {
        let mut snake_head:SnakeNode;
        let mut snake_feed = false;
        if let Some(_snake_head) = self.head.pop_front() {
            // Acquire new position for head snake according to
            // the direction of movement
            
            //TODO: Check why can't we move out food in Snake
            //Check if food equal to head
            let food = self.food.take();
            if let Some(_food) = food {
                if _food == _snake_head {
                    self.feed(_food);
                    self.cook();
                    snake_feed = true;
                }else {
                    //Restore all the peeling we did
                    self.food = Some(_food);
                }
            }

            snake_head = _snake_head;


        }else {return}


        if snake_feed == false {
            //Update all chaining node
            let mut last_point:Option<Point> = Some(snake_head.point);
            for node in self.head.iter_mut() {
                if let Some(_point) = last_point {
                    let swap = node.point;
                    node.walk(_point);
                    last_point = Some(swap);
                }
                
            }
        }

        //New snake position
        snake_head.run(self.direction);

        //Put back the head snake
        self.head.push_front(snake_head);
    }

    fn turn(&mut self, d:SnakeDirection) {
        self.direction = d;
    }
}

impl Snake {  

    fn new() -> Self {

        //Snake Chain
        let mut head = LinkedList::new();

        //Color of the first snake Node
        let color = Color {
            //Red Color
            color:[1.0, 0.0, 0.0, 1.0],
        };

        //First Snake Node
        let node = SnakeNode {
            point:Point::random(),
            color:color,
        };

        //Insert first snake node into snake chain
        head.push_front(node);



        let mut snake = Snake {
            head:head,
            direction:SnakeDirection::Right,
            food:None,
            is_pause:false,
        };

        snake.cook();

        snake

    }

    fn feed(&mut self, node:SnakeNode) {
        // Feeding expands tail
        self.head.push_front(node);
    }

    fn cook(&mut self) {
        loop {
            let point = Point::random();
            let mut picked = true;
            for node in self.head.iter() {
                if node.point == point {
                    picked = false;
                    break;
                }
            }

            if picked {
                self.food = Some(SnakeNode {
                    point:point,
                    color:Color {
                        color:[0.0, 0.0, 1.0, 1.0]
                    }
                });
                break;
            }
            
        }

    }

}

impl Point {
    fn random() -> Self {
        let mut rng = thread_rng();
        let mut x:f64 = rng.gen_range(0.0, 500.0);
        let mut y:f64 = rng.gen_range(0.0, 500.0);
        x = x - (x%10.0);
        y = y - (y%10.0);

        Point {
            x:x,
            y:y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other:&Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for SnakeNode {
    fn eq(&self, other:&Self) -> bool {
        self.point == other.point
    }
}

impl SnakeGame {

    //Render the window
    fn render(&mut self, args:RenderArgs) {
        use graphics::*;

        const GREEN:[f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:[f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:[f32; 4] = [0.0, 0.0, 1.0, 1.0];

        //Clear viewport
        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
        });

        //Draw Food Postion
        if let Some(food) = &self.snake.food {
            self.gl.draw(args.viewport(), |c, gl| {
                let square = rectangle::square(food.point.x, food.point.y, 10.0);
                rectangle(BLUE, square, c.transform, gl);
            });
        }

        //Draw snake Node
        for node in self.snake.head.iter(){
            self.gl.draw(args.viewport(), |c, gl| {
                let square = rectangle::square(node.point.x, node.point.y, 10.0);
                rectangle(RED, square, c.transform, gl);
            });
        }
        
    }


    fn update(&mut self, args:UpdateArgs) {
        if self.snake.is_pause == false {
            self.snake.run();
        }
    }

    //Called when keyboard, mouse, joystick button is pressed
    fn button_press(&mut self, args:ButtonArgs) {

        if args.state == ButtonState::Release {
            return
        }

        match args.button {
            Button::Keyboard(k) => {
                match k {
                    keyboard::Key::Left => { self.snake.turn(SnakeDirection::Left); }

                    keyboard::Key::Right => { self.snake.turn(SnakeDirection::Right); }

                    keyboard::Key::Up => { self.snake.turn(SnakeDirection::Top); }

                    keyboard::Key::Down => { self.snake.turn(SnakeDirection::Bottom); }

                    keyboard::Key::Space => { self.snake.is_pause = !self.snake.is_pause; }
                    
                    _ => {}
                }
            }

            _ => {
                println!("Event not handled");
            }
        }
    }
}

fn main() {
    //Define the OpenGL Version
    let opengl = OpenGL::V4_3;

    //Create a Window
    let mut window:Window = WindowSettings::new("Snake Game", (500,500))
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    
    //Create a game context
    let mut game:SnakeGame = SnakeGame {
        gl:GlGraphics::new(opengl),
        snake:Snake::new(),
    };

    let mut event_settings = EventSettings::new();
    event_settings.ups = 10;

    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {

        //Mouse
        if let Some(cursor) = e.mouse_cursor_args() {
            println!("Cursor : {},{} ", cursor[0], cursor[1]);
        }

        if let Some(key) = e.button_args() {
            game.button_press(key);
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
