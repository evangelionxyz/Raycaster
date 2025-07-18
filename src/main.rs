mod entity;
use entity::entity::Entity;

use raylib::prelude::*;
use std::vec::Vec;

#[derive(Default)]
struct Line {
    start: Vector2,
    end: Vector2,
}

fn draw_line(draw_handle: &mut RaylibDrawHandle, line: &Line, color: Color) {
    draw_handle.draw_line(line.start.x as i32, line.start.y as i32, line.end.x as i32, line.end.y as i32, color);
}

#[derive(PartialEq, Debug)]
enum State {
    None = 0,
    CreateWall = 1,
}

struct Editor {
    walls: Vec<Line>,
    state: State,
}

fn main() {
    let (mut rl, thread) = init()
        .size(1080, 640)
        .title("Raylib Rust")
        .resizable()
        .build();

    const REFRESH_INTERVAL: f32 = 1.0;
    let circle_radius: f32 = 3.0;
    let line_radius: f32 = 1000.0;
    let line_count = 30;
    let font_size = 20;
    let mut fps_str = String::from("FPS 0");
    let mut fps_timer: f32 = 0.0;

    let entity: Entity = Entity::create("Evan");
    println!("Entity uuid: {}", entity.uuid);

    let mut editor: Editor = Editor{
        walls: Vec::new(), 
        state: State::None,
    };

    let mut wall_creation: Line = Line{..Line::default()};
    let mut selection: Line = Line{..Line::default()};

    while !rl.window_should_close() {
        let mut h_drawing = rl.begin_drawing(&thread);
        let mouse_pos = h_drawing.get_mouse_position();

        if h_drawing.is_key_pressed(KeyboardKey::KEY_TAB) {
            editor.state = if editor.state == State::None { State::CreateWall }
            else { State::None };
        }

        let mut is_drawing = false;

        if editor.state == State::None {
            if h_drawing.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                selection.start = mouse_pos;
                selection.end = mouse_pos;
            }

            if h_drawing.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                is_drawing = true;
                selection.end = mouse_pos - selection.start;
                h_drawing.draw_rectangle_lines(selection.start.x as i32, selection.start.y as i32, selection.end.x as i32, selection.end.y as i32, Color::BROWN);
            }
        }

        // Drawl wall
        for w in editor.walls.as_slice() {
            draw_line(&mut h_drawing, &w, Color::GREEN);
        }

        match editor.state {
            State::CreateWall => {
                if h_drawing.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    wall_creation.start = mouse_pos;
                    wall_creation.end = mouse_pos;
                }
                
                if h_drawing.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    wall_creation.end = mouse_pos;
                    is_drawing = true;
                }
                
                if h_drawing.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                    if wall_creation.start != wall_creation.end {
                        editor.walls.push(Line {
                            start: wall_creation.start,
                            end: wall_creation.end,
                        });
                    }
                }

                draw_line(&mut h_drawing, &wall_creation, Color::YELLOW);
                if is_drawing {
                    h_drawing.draw_circle(wall_creation.start.x as i32, wall_creation.start.y as i32, circle_radius, Color::RED);
                    h_drawing.draw_circle(wall_creation.end.x as i32, wall_creation.end.y as i32, circle_radius, Color::RED);
                }
            }

            State::None => 'DrawingState: {
                // Skip drawing lines
                if is_drawing {
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
                    let mut shortest_distance = std::f32::MAX;
        
                    for w in editor.walls.as_slice() {
                        let intersect_point = check_collision_lines(
                            w.start, w.end,
                            line.start, line.end
                        );
                        if intersect_point.is_some() {
                            let point = intersect_point.unwrap();
        
                            // Calculate actual distance to intersection point
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
        
                    draw_line(&mut h_drawing, &line, Color::BLUE);
                }
        
                h_drawing.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, circle_radius, Color::RED);
            }
        }
        
        fps_timer -= h_drawing.get_frame_time();
        if fps_timer <= 0.0 {
            fps_timer = REFRESH_INTERVAL;
            fps_str = String::from("FPS ");
            fps_str.push_str(&h_drawing.get_fps().to_string());
        }
        
        h_drawing.draw_text(&fps_str, 12, 12, font_size, Color::GRAY);

        h_drawing.draw_text(format!("{:?}", editor.state).as_str(), 12, font_size + 12, font_size, Color::GRAY);

        h_drawing.clear_background(Color::BLACK);
    }
}