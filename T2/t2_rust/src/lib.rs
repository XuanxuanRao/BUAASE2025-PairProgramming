use astar::astar::Node;
use map::map::{Direction, Map, Position};
use std::collections::{BinaryHeap, HashSet};
use wasm_bindgen::prelude::*;

pub mod map;
pub mod astar;

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(input_snake: Vec<i32>, input_apple: Vec<i32>, input_obtacles: Vec<i32>) -> i32 {
    let (snake, apple, obstacles) = process_input(&input_snake, &input_apple, &input_obtacles);
    let path = find_astar_path(&snake, &apple, &obstacles);
    if path.len() > 0 {
        match path[0] {
            Direction::Up => return 0,
            Direction::Left => return 1,
            Direction::Down => return 2,
            Direction::Right => return 3,
        }
    } else {
        return -1;
    }
}

fn process_input(input_snake: &Vec<i32>, input_apple: &Vec<i32>, input_obtacles: &Vec<i32>) -> (Vec<Position>, Position, Vec<Position>){
    let mut snake:Vec<Position> = Vec::new();
    let mut i = 0;
    loop {
        snake.push(Position::new(input_snake[i], input_snake[i + 1]));
        i += 2;
        if i == 8 { break; }
    }
    let apple = Position::new(input_apple[0], input_apple[1]);
    let mut obstacles:Vec<Position> = Vec::new();
    let mut i = 0;
    loop {
        obstacles.push(Position::new(input_obtacles[i], input_obtacles[i + 1]));
        i += 2;
        if i == 24 { break; }
    }

    (snake, apple, obstacles)

}

fn find_astar_path(snake: &Vec<Position>, apple: &Position, obstacles: &Vec<Position>) -> Vec<Direction> {
    let head = snake[0];
    let goal = apple;

    let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed_set: HashSet<Position> = HashSet::new();
    let node = Node::new((head).clone(), 0, Node::heuristic(&head, &apple), None);
    open_set.push(node);

    while !open_set.is_empty() {
        let cur = match open_set.pop() {
            Some(pos) => pos,
            None => panic!("Got a None Node!"),
        };
        if cur.get_pos() == goal {
            return reconstruct_path(&cur);
        }

        closed_set.insert((*cur.get_pos()).clone());

        for direction in Direction::iter() {
            let mut _neighbor = Position::new(-1, -1);
            match direction {
                Direction::Up => _neighbor = Position::up(cur.get_pos()),
                Direction::Down => _neighbor = Position::down(cur.get_pos()),
                Direction::Left => _neighbor = Position::left(cur.get_pos()),
                Direction::Right => _neighbor = Position::right(cur.get_pos()),
            }
            if (!Map::colision_check(&_neighbor, &snake[0..=2], obstacles)) && (!closed_set.contains(&_neighbor)){
                let g = cur.get_g();
                let h = Node::heuristic(&_neighbor, goal);
                open_set.push(Node::new((_neighbor).clone(), g, h, Some((cur).clone())));
            }
        }
    }


    return vec![];

}

fn reconstruct_path(node: &Node) -> Vec<Direction> {
    let mut path: Vec<Direction> = Vec::new();
    let mut current_node = Some(node);

    while let Some(node) = current_node {
        if let Some(parent) = node.get_parent() {
            let from = parent.get_pos(); 
            let to = node.get_pos();
            let mut _neighbor = Position::new(-1, -1);

            for direction in Direction::iter() {
                match direction {
                    Direction::Up => _neighbor = Position::up(from),
                    Direction::Down => _neighbor = Position::down(from),
                    Direction::Left => _neighbor = Position::left(from),
                    Direction::Right => _neighbor = Position::right(from),
                }
                if &_neighbor == to {
                    path.push(direction);
                    break;
                }
            }

            current_node = Some(parent); 
        } else {
            break;
        }
    }

    path.reverse();
    path
}

#[cfg(test)]
mod tests {

    // use super::*;

    
}
