use astar::astar::Node;
use map::map::{Direction, Map, Position};
use std::collections::{BinaryHeap, HashSet};
use wasm_bindgen::prelude::*;

pub mod map;
pub mod astar;

#[wasm_bindgen]
pub fn greedy_snake_move(input_snake: Vec<i32>, input_apple: Vec<i32>) -> i32 {
    let mut snake:Vec<Position> = Vec::new();
    let mut i = 0;
    loop {
        snake.push(Position::new(input_snake[i], input_snake[i + 1]));
        i += 2;
        if i == 8 {
            break;
        }
    }

    let apple = Position::new(input_apple[0], input_apple[1]);
    let path = find_astar_path(&snake, &apple);
    match path[0] {
        Direction::Up => return 0,
        Direction::Left => return 1,
        Direction::Down => return 2,
        Direction::Right => return 3,
    }


}

fn find_astar_path(snake: &Vec<Position>, apple: &Position) -> Vec<Direction> {
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
            if (!Map::edge_check(&_neighbor, &snake)) && (!closed_set.contains(&_neighbor)){
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

    use super::*;

    /// 进行一轮贪吃蛇游戏，直到撞墙、吃到苹果或判断无法吃到
    /// 如果顺利吃到了苹果，返回0；如果蛇死了，返回-1
    fn game(snake: &mut Vec<i32>, apple: & Vec<i32>) -> i32 {
        loop {
            let dir = greedy_snake_move(snake.clone(), apple.clone());
            if dir == -1 {
                return 1;
            }
    
            let mut new_snake = [0; 8];
    
            new_snake[0] = snake[0] + if dir == 3 { 1 } else { 0 } - if dir == 1 { 1 } else { 0 };
            new_snake[1] = snake[1] + if dir == 0 { 1 } else { 0 } - if dir == 2 { 1 } else { 0 };
    
            for i in (2..8).step_by(2) {
                new_snake[i] = snake[i - 2];
                new_snake[i + 1] = snake[i - 1];
            }
    
            if new_snake[0] < 1 || new_snake[0] > 8 || new_snake[1] < 1 || new_snake[1] > 8 {
                println!("\n撞墙了.....");
                return -1;
            }
    
            if new_snake[0] == new_snake[4] && new_snake[1] == new_snake[5] {
                println!("\n撞到了自己");
                return -1;
            }
    
            snake.clone_from_slice(&new_snake);
    
            if snake[0] == apple[0] && snake[1] == apple[1] {
                println!("\n成功吃到了苹果\n");
                return 0;
            }
        }
    }
    

    #[test]
    #[ignore = "ignore"]
    fn test_no_way_to_eat_apple() {
        let mut snake = vec![1, 1, 1, 2, 2, 2,1];
        let apple = vec![6, 6];
        let res = game(&mut snake, &apple);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_common1() {
        let mut snake = vec![1, 1, 1, 2, 1, 3, 1, 4]; // 初始蛇
        let apple = vec![2, 1]; // 初始苹果
        let res = game(&mut snake, &apple);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_common2() {
        let mut snake = vec![1, 1, 1, 2, 1, 3, 2, 3]; // 初始蛇
        let apple = vec![6, 5]; // 初始苹果
        let res = game(&mut snake, &apple);
        assert_eq!(res, 0);
    }

    // fn generate_apple(snake: &[i32; 8]) -> [i32; 2] {
    //     let mut rng = thread_rng();
    //     let mut snake_set = HashSet::new();
    
    //     for i in (0..8).step_by(2) {
    //         snake_set.insert((snake[i], snake[i + 1]));
    //     }
    
    //     loop {
    //         let x = rng.gen_range(1..=8);
    //         let y = rng.gen_range(1..=8);
    //         if !snake_set.contains(&(x, y)) {
    //             return [x, y];
    //         }
    //     }
    // }
}
