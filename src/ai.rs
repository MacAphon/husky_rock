use std::f64::consts::{PI, TAU};
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use specs::prelude::*;

use crate::components::*;
use crate::rays::cast_ray;

pub struct AI;

impl<'a> System<'a> for AI {
    type SystemData = (
        Read<'a, LevelMap>,
        Read<'a, PlayerPosition>,
        WriteStorage<'a, HasAI>,
        ReadStorage<'a, IsEntity>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
        ReadStorage<'a, VelocityMultiplier>,
        WriteStorage<'a, VelocityRelative>,
    );

    fn run(&mut self, (lmap, ppos, mut hai, ien, pos, rot, vmul, mut vrel): Self::SystemData) {
        let level = &lmap.0;
        let ppos = &ppos.0;
        'e: for(
            hai, _, pos, rot, vmul, vrel
        ) in (
            &mut hai, &ien, &pos, &rot, &vmul, &mut vrel
        ).join() {

            let pdx = pos.x - ppos.0;
            let pdy = -(pos.y - ppos.1);
            let anglep = pdy.atan2(pdx);
            let dist_to_player = (pdx).hypot(pdy);
            let dist_to_wall = cast_ray((pos.x, pos.y), anglep, &level);

            // calculate waypoints once a second to improve performance
            if hai.time_to_next_update > 0 {
                hai.time_to_next_update -= 1;
            }
            else {
                hai.path = calc_path(&level, &ppos, &pos);
                hai.path.reverse();
                hai.time_to_next_update = 60; // 1 second
            }

            // if the entity is very close to the current waypoint, pop it
            if (pos.x - ((hai.path[0].0 << 6) + 32) as f64).hypot(pos.y - ((hai.path[0].1 << 6) + 32) as f64) <= 20. {
                hai.path.pop();
                if hai.path.len() == 0 {
                    hai.path.push((1, 1));
                }
            }


            // decide weather to go to the player or follow the path
            let dest = if dist_to_player < dist_to_wall.0 || hai.path.len() == 1 {
                    (ppos.0, ppos.1)
                }
                else {
                    (((hai.path[0].0 << 6) + 32) as f64, ((hai.path[0].1 << 6) + 32) as f64)
                };

            //let dest = (((hai.path[0].0 << 6) + 32) as f64, ((hai.path[0].1 << 6) + 32) as f64);

            // go to the destination
            let dx = -(pos.x - dest.0);
            let dy = pos.y - dest.1;
            let angle = dy.atan2(dx);

            let mut angle_rel = angle - rot.r;
            while angle_rel < 0. {
                angle_rel += TAU;
            } while angle_rel > TAU {
                angle_rel -= TAU;
            }

            if angle_rel <= PI/6. || angle_rel >= TAU-PI/6. {
                // go straight at full speed
                vrel.movement_rel.0 = vmul.speed;
                vrel.movement_rot = 0.;
                continue 'e;
            }
            else if angle_rel <= PI/3. || angle_rel >= TAU-PI/3. {
                // turn slowly at medium speed
                vrel.movement_rel.0 = vmul.speed*0.75;
                if angle_rel < PI {
                    vrel.movement_rot = -vmul.speed_rot/2.;
                }
                else {
                    vrel.movement_rot = vmul.speed_rot/2.;
                }
                continue 'e;
            }
            else {
                // turn fast while going slow
                vrel.movement_rel.0 = vmul.speed*0.5;
                if angle_rel < PI {
                    vrel.movement_rot = -vmul.speed_rot;
                }
                else {
                    vrel.movement_rot = vmul.speed_rot;
                }
                continue 'e;
            }
        }
    }
}

/**************************************************************************************************/
// A* algorithm implementation

// to be stored in al 2d vector
//
#[derive(Clone, Debug)]
struct Cell {
    parent_x: isize,
    parent_y: isize,
    // f = g + h
    f: usize,
    // g = distance so far
    g: usize,
    // h = distance to target
    h: usize,
}

// check if a cell exists
fn is_valid(lvl_size: usize, x: isize, y: isize) -> bool {
    (x >= 0) && (x < lvl_size as isize) && (y >= 0) && (y < lvl_size as isize)
}

// check if a cell is not a wall
fn is_free(lvl: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    lvl[y][x] == 0
}

// check if the current location is the destination
fn is_dest(xy: (usize, usize), dest: (usize, usize)) -> bool {
    (xy.0 == dest.0) && (xy.1 == dest.1)
}

// trace the path from the start to the destination
fn trace_path(cell_details: Vec<Vec<Cell>>, dest: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path: Vec<(usize, usize)> = Vec::new();

    let mut x = dest.0;
    let mut y = dest.1;

    while (cell_details[y][x].parent_x != x as isize) || (cell_details[y][x].parent_y != y as isize) {
        path.push((x, y));

        let tx = cell_details[y][x].parent_x;
        let ty = cell_details[y][x].parent_y;
        x = tx as usize;
        y = ty as usize;
    }
    path
}

// distance to the destination
fn calc_h(xy: (usize, usize), dest: (usize, usize)) -> usize {
    ((xy.0 as isize - dest.0 as isize).abs() + (xy.1 as isize - dest.1 as isize).abs()) as usize
}

// actual A* algorithm
fn calc_path(lvl: &Vec<Vec<u32>>, ppos: &(f64, f64), pos: &Position) -> Vec<(usize, usize)>{

    let dest = (ppos.0 as usize >> 6, ppos.1 as usize >> 6);
    let start = (pos.x as usize >> 6, pos.y as usize >> 6);
    let lvl_size = lvl.len();


    if is_dest(start, dest) {
        return vec!(dest)
    }

    let mut closed_list: Vec<Vec<bool>> = vec![vec![false; lvl_size]; lvl_size];

    // initialize a grid of cells with implausibly huge values such that they are not considered valid
    let mut cell_details: Vec<Vec<Cell>> = vec![vec![Cell{
        parent_x: -1,
        parent_y: -1,
        f: 100000,
        g: 100000,
        h: 100000,
    }; lvl_size]; lvl_size];

    // initialize starting position with itself as its parent
    cell_details[start.1][start.0] = Cell{
        parent_x: start.0 as isize,
        parent_y: start.1 as isize,
        f: 0,
        g: 0,
        h: 0,
    };

    // cue for cells to look at next
    // f, (x, y)
    let mut open_list: BinaryHeap<Reverse<(usize, (usize, usize))>> = BinaryHeap::new();
    open_list.push(Reverse((0, (start.0, start.1)))); // push the start position with an f of 0

    while open_list.len() != 0 {
        let p = open_list.pop().unwrap().0;

        let x = p.1.0 as usize;
        let y = p.1.1 as usize;

        closed_list[y][x] = true;

        // generate successors of the current cell

        let mut f_new: usize;
        let mut g_new: usize;
        let mut h_new: usize;

        for val in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            // left
            // if the cell is in the map
            if is_valid(lvl_size, x as isize + val.0, y as isize + val.1) {
                // if the cell is the destination
                if is_dest(((x as isize + val.0) as usize, (y as isize + val.1) as usize), dest) {

                    cell_details[(y as isize + val.1) as usize][(x as isize + val.0) as usize].parent_x = x as isize;
                    cell_details[(y as isize + val.1) as usize][(x as isize + val.0) as usize].parent_y = y as isize;

                    return trace_path(cell_details, dest)
                }
                // if the cell is already on the closed list or blocked ignore it,
                // else:
                else if
                    !closed_list[(y as isize + val.1) as usize][(x as isize + val.0) as usize]
                    && is_free(&lvl, (x as isize + val.0) as usize, (y as isize + val.1) as usize)
                {
                    g_new = cell_details[y][x].g + 1;
                    h_new = calc_h(((x as isize + val.0) as usize, (y as isize + val.1) as usize), dest);
                    f_new = g_new + h_new;

                    // if it isn't on the open list, add it to it, make the current square parent of this one
                    // if it is on the open list, but this path is better, do the same
                    if
                        cell_details[(y as isize + val.1) as usize][(x as isize + val.0) as usize].f == 100000
                        || cell_details[(y as isize + val.1) as usize][(x as isize + val.0) as usize].f > f_new
                    {
                        open_list.push(Reverse((f_new, ((x as isize + val.0) as usize, (y as isize + val.1) as usize))));
                        cell_details[(y as isize + val.1) as usize][(x as isize + val.0) as usize] = Cell {
                            f: f_new,
                            g: g_new,
                            h: h_new,
                            parent_y: y as isize,
                            parent_x: x as isize,
                        };
                    }
                }
            }
        }
    }
    println!("couldn't reach destination");
    vec![(9, 9)]
}