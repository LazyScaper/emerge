use crate::graph::{Position, Size};
use crate::physics::{edge_by_id, node_positions_by_id};
use hecs::World;
use macroquad::color::{Color, BLACK, DARKGRAY, RED, WHITE};
use macroquad::input::{is_mouse_button_down, mouse_delta_position};
use macroquad::math::Vec2;
use macroquad::prelude::{clear_background, draw_circle, draw_line};
use macroquad::text::{draw_text, get_text_center};

const NODE_SIZE: f32 = 15.0;

pub struct ScrollableView {
    pub offset: Vec2,
}

impl ScrollableView {
    pub fn new() -> Self {
        Self {
            offset: Vec2::new(0.0, 0.0),
        }
    }

    pub fn world_pos_to_screen_pos(&self, world_pos: &Position) -> Position {
        Position {
            x: world_pos.x - self.offset.x,
            y: world_pos.y - self.offset.y,
        }
    }

    pub fn update(&mut self) {
        if is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let mouse_delta = mouse_delta_position();

            self.offset.x += mouse_delta.x * 1000.0;
            self.offset.y += mouse_delta.y * 1000.0;
        }
    }
}

pub(crate) fn render(world: &mut World) {
    clear_background(DARKGRAY);

    render_edges(world);
    render_nodes(&world);
}

fn render_edges(world: &mut World) {
    let node_data = node_positions_by_id(world);
    let edge_data = edge_by_id(world);

    for (_, edge) in edge_data {
        let edge_source_node_id = edge.source_node_id;
        let edge_destination_node_id = edge.destination_node_id;

        if node_data.contains_key(&edge_source_node_id)
            && node_data.contains_key(&edge_destination_node_id)
        {
            match (
                node_data.get(&edge_source_node_id),
                node_data.get(&edge_destination_node_id),
            ) {
                (Some(source_node_position), Some(destination_node_position)) => {
                    let source_node_position =
                        world_to_screen_position(world, source_node_position);
                    let destination_node_position =
                        world_to_screen_position(world, destination_node_position);

                    draw_arrow_line(
                        Vec2 {
                            x: source_node_position.x,
                            y: source_node_position.y,
                        },
                        Vec2 {
                            x: destination_node_position.x,
                            y: destination_node_position.y,
                        },
                        RED,
                        3.0,
                    )
                }
                _ => {}
            }
        };
    }
}

fn render_nodes(world: &&mut World) {
    for (_id, (position, size, label)) in &mut world.query::<(&Position, &Size, &String)>() {
        let label = &label;
        let center_of_text = get_text_center(label, None, 20, 1.0, 0.0);

        let mut scrollable_view_query = world.query::<&ScrollableView>();
        let (_, scrollable_view) = scrollable_view_query
            .iter()
            .next()
            .expect("No scrollable view found");
        let position = scrollable_view.world_pos_to_screen_pos(position);

        draw_circle(position.x, position.y, size.radius, BLACK);
        draw_text(
            label,
            position.x - center_of_text.x,
            position.y - center_of_text.y / 2.0,
            20.0,
            WHITE,
        );
    }
}

pub(crate) fn view_port_update(world: &mut World) {
    let mut scrollable_view_query = world.query::<&mut ScrollableView>();

    match scrollable_view_query.iter().last() {
        None => {}
        Some((_e, scrollable_view)) => {
            scrollable_view.update();
        }
    }
}

fn world_to_screen_position(world: &mut World, source_node_position: &Position) -> Position {
    let mut scrollable_view_query = world.query::<&ScrollableView>();
    let (_, scrollable_view) = scrollable_view_query
        .iter()
        .next()
        .expect("No scrollable view found");
    let source_node_position = scrollable_view.world_pos_to_screen_pos(source_node_position);
    source_node_position
}

fn render_arrow(
    color: Color,
    thickness: f32,
    arrow_start: Vec2,
    arrow_end: Vec2,
    arrowhead_left: Vec2,
    arrowhead_right: Vec2,
) {
    // Draw main line
    draw_line(
        arrow_start.x,
        arrow_start.y,
        arrow_end.x,
        arrow_end.y,
        thickness,
        color,
    );
    // draw left side of arrow head
    draw_line(
        arrow_end.x,
        arrow_end.y,
        arrowhead_left.x,
        arrowhead_left.y,
        thickness,
        color,
    );
    // draw right side of arrow head
    draw_line(
        arrow_end.x,
        arrow_end.y,
        arrowhead_right.x,
        arrowhead_right.y,
        thickness,
        color,
    );
}

fn calculate_arrow_positions(start_pos: Vec2, end_pos: Vec2) -> (Vec2, Vec2, Vec2, Vec2) {
    // Calculate direction vector
    let direction = (end_pos - start_pos).normalize();

    // Calculate arrow start and end points (edge of circles)
    let arrow_start_pos = start_pos + direction * NODE_SIZE;
    let arrow_end_pos = end_pos - direction * NODE_SIZE;

    // Draw arrowhead
    let arrowhead_size = 15.0;
    // To find the perpendicular vector [x,y] to [i,j] we need the dot product to be zero
    // Thus u dot v = 0 => xi + yj = 0. From inspection we can see x = j and y = -i would suffice.
    let perpendicular = Vec2::new(direction.y, -direction.x);

    let arrowhead_left_end_pos =
        arrow_end_pos - direction * arrowhead_size + perpendicular * (arrowhead_size * 0.5);
    let arrowhead_right_end_pos =
        arrow_end_pos - direction * arrowhead_size - perpendicular * (arrowhead_size * 0.5);
    (
        arrow_start_pos,
        arrow_end_pos,
        arrowhead_left_end_pos,
        arrowhead_right_end_pos,
    )
}

fn draw_arrow_line(start_pos: Vec2, end_pos: Vec2, color: Color, thickness: f32) {
    let (arrow_start_pos, arrow_end_pos, arrowhead_left_end_pos, arrowhead_right_end_pos) =
        calculate_arrow_positions(start_pos, end_pos);

    render_arrow(
        color,
        thickness,
        arrow_start_pos,
        arrow_end_pos,
        arrowhead_left_end_pos,
        arrowhead_right_end_pos,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_arrow_positions() {
        let (start_pos, end_pos, start_node_radius, end_node_radius) =
            (Vec2 { x: 0.0, y: 0.0 }, Vec2 { x: 100.0, y: 0.0 }, 2.0, 2.0);

        let (arrow_start_pos, arrow_end_pos, arrowhead_left_end_pos, arrowhead_right_end_pos) =
            calculate_arrow_positions(start_pos, end_pos);

        assert_eq!(arrow_start_pos, Vec2 { x: 2.0, y: 0.0 });
        assert_eq!(arrow_end_pos, Vec2 { x: 98.0, y: 0.0 });
        assert_eq!(arrowhead_left_end_pos, Vec2 { x: 83.0, y: -7.5 });
        assert_eq!(arrowhead_right_end_pos, Vec2 { x: 83.0, y: 7.5 });
    }
}
