use std::collections::{HashMap, HashSet};
use raylib::{check_collision_lines, init, RaylibHandle, RaylibThread};
use raylib::color::Color;
use raylib::consts::{KeyboardKey, MouseButton};
use raylib::drawing::RaylibDraw;
use raylib::math::Vector2;
use uuid::Uuid;
use crate::entity::components::*;
use crate::entity::world::World;

#[derive(PartialEq, Debug)]
enum State {
    None = 0,
    CreateWall = 1,
}

struct Editor {
    walls: HashMap<Uuid, Line>,
    state: State,
    wall_creation: Line,
    selection: Line,
}

impl Editor {
    fn new() -> Editor {
        Editor {
            walls: HashMap::new(),
            state: State::None,
            wall_creation: Line{..Line::default()},
            selection: Line{..Line::default()},
        }
    }
}

pub struct Game {
    screen_width: i32,
    screen_height: i32,
    editor: Editor,
    world: World,
    rl: RaylibHandle,
    rl_thread: RaylibThread,
}

impl Game {
    pub fn create(screen_width: i32, screen_height: i32) -> Game {
        let (rl, rl_thread): (RaylibHandle, RaylibThread) = init()
            .size(screen_width, screen_height)
            .title("Raylib Rust")
            //.resizable()
            .build();

        let mut editor = Editor::new();

        // Add border walls
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(0.0, 0.0), end: Vector2::new(screen_width as f32, 0.0) });
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(screen_width as f32, 0.0), end: Vector2::new(screen_width as f32, screen_height as f32) });
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(screen_width as f32, screen_height as f32), end: Vector2::new(0.0, screen_height as f32) });
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(0.0, screen_height as f32), end: Vector2::new(0.0, 0.0) });

        // Add some random walls inside
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(100.0, 150.0), end: Vector2::new(300.0, 100.0) });
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(200.0, 500.0), end: Vector2::new(450.0, 550.0) });
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(600.0, 600.0), end: Vector2::new(700.0, 400.0) });
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(900.0, 150.0), end: Vector2::new(1100.0, 250.0) });
        editor.walls.insert(Uuid::new_v4(), Line{ start: Vector2::new(950.0, 500.0), end: Vector2::new(1150.0, 650.0) });

        Game {
            screen_width,
            screen_height,
            editor,
            world: World::new(),
            rl,
            rl_thread
        }
    }

    pub fn run(&mut self) {
        const REFRESH_INTERVAL: f32 = 1.0;
        let circle_radius: f32 = 3.0;
        let line_radius: f32 = 1000.0;
        let line_count = 50;
        let font_size = 20;
        let mut fps_str = String::from("FPS 0");
        let mut fps_timer: f32 = 0.0;
        let mut selection = Line{..Line::default()};
        let mut selected_lines: HashSet<Uuid> = HashSet::new();

        while !self.rl.window_should_close() {
            let mut h_drawing = self.rl.begin_drawing(&self.rl_thread);
            let mouse_pos = h_drawing.get_mouse_position();

            self.screen_width = h_drawing.get_screen_width();
            self.screen_height = h_drawing.get_screen_width();

            if h_drawing.is_key_pressed(KeyboardKey::KEY_TAB)
                || h_drawing.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                self.editor.state = if self.editor.state == State::None { State::CreateWall }
                else { State::None };
                self.editor.wall_creation.start = Vector2::zero();
                self.editor.wall_creation.end = Vector2::zero();
                selected_lines.clear();
            }

            let mut is_drawing = false;
            let mut is_selecting = false;

            if self.editor.state == State::None {
                // Deleting selected walls
                if h_drawing.is_key_pressed(KeyboardKey::KEY_DELETE)
                    || h_drawing.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_MIDDLE){
                    for uuid in selected_lines.clone().into_iter() {
                        HashMap::remove(&mut self.editor.walls, &uuid);
                    }
                    selected_lines.clear();
                }

                // Reset
                if h_drawing.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                    self.editor.selection.start = Vector2::zero();
                    self.editor.selection.end = Vector2::zero();
                }

                if h_drawing.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    selection.start = mouse_pos;
                    selection.end = mouse_pos;
                    selected_lines.clear();
                }

                if h_drawing.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    is_selecting = true;

                    selection.end = mouse_pos - selection.start;

                    // Outline
                    h_drawing.draw_rectangle_lines(selection.start.x as i32, selection.start.y as i32,
                                                   selection.end.x as i32, selection.end.y as i32,
                                                   Color::BROWN);

                    // Transparent
                    if selection.start.x > mouse_pos.x + selection.end.x || selection.start.y > mouse_pos.y + selection.end.y {
                        let start_flip = Vector2{
                            x: if selection.start.x > mouse_pos.x {
                                mouse_pos.x
                            }
                            else {
                                selection.start.x
                            },
                            y: if selection.start.y > mouse_pos.y + selection.end.y {
                                mouse_pos.y
                            }
                            else {
                                selection.start.y
                            },
                        };
                        let end_flip = Vector2 {
                            x: if selection.start.x > mouse_pos.x {
                                selection.start.x - mouse_pos.x
                            }
                            else {
                                mouse_pos.x - selection.start.x
                            },
                            y: if selection.start.y > mouse_pos.y + selection.end.y {
                                selection.start.y - mouse_pos.y
                            }
                            else {
                                mouse_pos.y - selection.start.y
                            },
                        };

                        h_drawing.draw_rectangle(start_flip.x as i32, start_flip.y as i32,
                                                 end_flip.x as i32, end_flip.y as i32,
                                                 Color::new(100, 0, 0, 50));

                        self.editor.selection.start = start_flip;
                        self.editor.selection.end = start_flip + end_flip;
                    }
                    else {
                        h_drawing.draw_rectangle(selection.start.x as i32, selection.start.y as i32,
                                                 selection.end.x as i32, selection.end.y as i32,
                                                 Color::new(100, 0, 0, 50));

                        self.editor.selection.start = selection.start;
                        self.editor.selection.end = selection.start + selection.end;
                    }
                }
            }

            // Drawl wall
            for (uuid, w) in self.editor.walls.iter() {
                if is_selecting {
                    if w.start.x >= self.editor.selection.start.x && w.end.x <= self.editor.selection.end.x
                        && w.start.y >= self.editor.selection.start.y && w.end.y <= self.editor.selection.end.y {
                        if !selected_lines.contains(&uuid) {
                            selected_lines.insert(*uuid);
                        }
                    }
                    else {
                        if selected_lines.contains(&uuid) {
                            selected_lines.remove(&uuid);
                        }
                    }
                }

                draw_line(&mut h_drawing, &w, if selected_lines.contains(&uuid) {Color::YELLOW} else {Color::GREEN});
            }

            match self.editor.state {
                State::CreateWall => {
                    if h_drawing.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        self.editor.wall_creation.start = mouse_pos;
                        self.editor.wall_creation.end = mouse_pos;
                    }

                    if h_drawing.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                        self.editor.wall_creation.end = mouse_pos;
                        is_drawing = true;
                    }

                    if h_drawing.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                        if self.editor.wall_creation.start != self.editor.wall_creation.end {

                            if self.editor.wall_creation.start.x > self.editor.wall_creation.end.x || self.editor.wall_creation.start.y > self.editor.wall_creation.end.y {
                                let start = self.editor.wall_creation.start;
                                self.editor.wall_creation.start = self.editor.wall_creation.end;
                                self.editor.wall_creation.end = start;
                            }

                            self.editor.walls.insert(
                                Uuid::new_v4(),
                                Line {
                                    start: self.editor.wall_creation.start,
                                    end: self.editor.wall_creation.end,
                                }
                            );
                        }
                    }

                    draw_line(&mut h_drawing, &self.editor.wall_creation, Color::YELLOW);

                    if is_drawing {
                        h_drawing.draw_circle(self.editor.wall_creation.start.x as i32, self.editor.wall_creation.start.y as i32, circle_radius, Color::RED);
                        h_drawing.draw_circle(self.editor.wall_creation.end.x as i32, self.editor.wall_creation.end.y as i32, circle_radius, Color::RED);
                    }
                }

                State::None => 'DrawingState: {
                    // Skip drawing lines
                    if is_drawing || is_selecting {
                        break 'DrawingState;
                    }

                    // Intersection Lines
                    for i in 0..line_count {
                        let angle = (i as f32) * ((2.0 * std::f32::consts::PI) /  line_count as f32);
                        let mut line = Line {
                            start: mouse_pos,
                            end: Vector2{
                                x: mouse_pos.x + angle.cos() * line_radius,
                                y: mouse_pos.y + angle.sin() * line_radius},
                        };

                        let mut collided = false;
                        let mut closest_point = None;
                        let mut shortest_distance = f32::MAX;

                        for (_, w) in self.editor.walls.iter_mut() {
                            let intersect_point = check_collision_lines(
                                w.start, w.end,
                                line.start, line.end
                            );
                            if intersect_point.is_some() {
                                let point = intersect_point.unwrap();

                                // Calculate actual distance to the intersection point
                                let dist = mouse_pos.distance_to(point);
                                // let dist = ((mouse_pos.x - point.x).powi(2) + (mouse_pos.y - point.y).powi(2)).sqrt();

                                if dist < shortest_distance {
                                    shortest_distance = dist;
                                    closest_point = Some(point);
                                }

                                collided = true;
                            }
                        }

                        if collided {
                            line.end = closest_point.unwrap();
                            h_drawing.draw_circle(
                                line.end.x as i32,
                                line.end.y as i32,
                                circle_radius,
                                Color::RED
                            );
                        }

                        draw_line(&mut h_drawing, &line, Color::MIDNIGHTBLUE);
                    }

                    h_drawing.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, circle_radius, Color::RED);
                }
            }

            fps_timer -= h_drawing.get_frame_time();
            if fps_timer <= 0.0 {
                fps_timer = REFRESH_INTERVAL;
                fps_str = String::from(format!("FPS: {:?}", h_drawing.get_fps()));
            }

            h_drawing.draw_text(&fps_str, 12, 12, font_size, Color::GRAY);
            h_drawing.draw_text(format!("State: {:?}", self.editor.state).as_str(), 12, font_size + 12, font_size, Color::GRAY);
            h_drawing.draw_text(format!("Selected Walls: {}", selected_lines.len()).as_str(), 12, font_size * 2 + 12, font_size, Color::GRAY);

            h_drawing.clear_background(Color::new(10, 10, 23, 255));
        }
    }
}
