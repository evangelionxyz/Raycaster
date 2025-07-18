mod entity;

use std::collections::HashSet;
use entity::entity::Entity;

use raylib::prelude::*;
use std::vec::Vec;

#[derive(Default, Copy, Clone)]
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
    wall_creation: Line,
    selection: Line,
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
    let line_count = 100;
    let font_size = 20;
    let mut fps_str = String::from("FPS 0");
    let mut fps_timer: f32 = 0.0;
    let mut selection = Line{..Line::default()};
    let mut selected_lines: HashSet<usize> = HashSet::new();

    let entity: Entity = Entity::create("Evan");
    println!("Entity uuid: {}", entity.uuid);

    let mut editor: Editor = Editor{
        walls: Vec::new(), 
        state: State::None,
        wall_creation: Line{ ..Line::default() },
        selection: Line{..Line::default()},
    };

    while !rl.window_should_close() {
        let mut h_drawing = rl.begin_drawing(&thread);
        let mouse_pos = h_drawing.get_mouse_position();

        if h_drawing.is_key_pressed(KeyboardKey::KEY_TAB) {
            editor.state = if editor.state == State::None { State::CreateWall }
            else { State::None };
            editor.wall_creation.start = Vector2::zero();
            editor.wall_creation.end = Vector2::zero();
            selected_lines.clear();
        }

        let mut is_drawing = false;
        let mut is_selecting = false;

        if editor.state == State::None {

            // Deleting selected walls
            if h_drawing.is_key_pressed(KeyboardKey::KEY_DELETE) {
                for index in selected_lines.clone().into_iter() {
                    editor.walls.remove(index);
                }
                selected_lines.clear();
            }

            // Reset
            if h_drawing.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                editor.selection.start = Vector2::zero();
                editor.selection.end = Vector2::zero();
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

                    editor.selection.start = start_flip;
                    editor.selection.end = start_flip + end_flip;
                }
                else {
                    h_drawing.draw_rectangle(selection.start.x as i32, selection.start.y as i32,
                                                   selection.end.x as i32, selection.end.y as i32,
                                                   Color::new(100, 0, 0, 50));

                    editor.selection.start = selection.start;
                    editor.selection.end = selection.start + selection.end;
                }
            }
        }

        // Drawl wall
        for (index, w) in editor.walls.iter().enumerate() {
            if is_selecting {
                if w.start.x >= editor.selection.start.x && w.end.x <= editor.selection.end.x
                    && w.start.y >= editor.selection.start.y && w.end.y <= editor.selection.end.y {
                    if !selected_lines.contains(&index) {
                        selected_lines.insert(index);
                    }
                }
                else {
                    if selected_lines.contains(&index) {
                        selected_lines.remove(&index);
                    }
                }
            }

            draw_line(&mut h_drawing, &w, if selected_lines.contains(&index) {Color::YELLOW} else {Color::GREEN});
        }

        match editor.state {
            State::CreateWall => {
                if h_drawing.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    editor.wall_creation.start = mouse_pos;
                    editor.wall_creation.end = mouse_pos;
                }
                
                if h_drawing.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    editor.wall_creation.end = mouse_pos;
                    is_drawing = true;
                }
                
                if h_drawing.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                    if editor.wall_creation.start != editor.wall_creation.end {

                        if editor.wall_creation.start.x > editor.wall_creation.end.x || editor.wall_creation.start.y > editor.wall_creation.end.y {
                           let start = editor.wall_creation.start;
                            editor.wall_creation.start = editor.wall_creation.end;
                            editor.wall_creation.end = start;
                        }

                        editor.walls.push(Line {
                            start: editor.wall_creation.start,
                            end: editor.wall_creation.end,
                        });
                    }
                }

                draw_line(&mut h_drawing, &editor.wall_creation, Color::YELLOW);

                if is_drawing {
                    h_drawing.draw_circle(editor.wall_creation.start.x as i32, editor.wall_creation.start.y as i32, circle_radius, Color::RED);
                    h_drawing.draw_circle(editor.wall_creation.end.x as i32, editor.wall_creation.end.y as i32, circle_radius, Color::RED);
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
        
                    for w in editor.walls.as_slice() {
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
        
                    draw_line(&mut h_drawing, &line, Color::BLUE);
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
        h_drawing.draw_text(format!("State: {:?}", editor.state).as_str(), 12, font_size + 12, font_size, Color::GRAY);
        h_drawing.draw_text(format!("Selected Walls: {}", selected_lines.len()).as_str(), 12, font_size * 2 + 12, font_size, Color::GRAY);

        h_drawing.clear_background(Color::BLACK);
    }
}