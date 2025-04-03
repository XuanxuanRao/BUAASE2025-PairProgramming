use astar::astar::Node;
use map::map::{Direction, Map, Position, Snake};
use std::collections::{BinaryHeap, HashSet};
use wasm_bindgen::prelude::*;

pub mod map;
pub mod astar;
pub mod random;

#[wasm_bindgen]
pub fn greedy_snake_step(board_size: i32, input_my_snake: Vec<i32>, snake_num: i32, input_other_snakes: Vec<i32>, food_num: i32, input_foods: Vec<i32>, _round: i32) -> i32 {
    let (my_snake, other_snakes, foods) = process_input(&input_my_snake, snake_num, &input_other_snakes, food_num, &input_foods);
    let map = Map::new(my_snake, other_snakes, foods, board_size);
    let path = find_astar_path(&map);
    if path.len() > 0 {
        println!("find astar path :{:#?}", path);
        match path[0] {
            Direction::Up => return 0,
            Direction::Left => return 1,
            Direction::Down => return 2,
            Direction::Right => return 3,
        }
    } else {
        // 这里是在严格检查的情况下没有astar路径
        // 这时的策略是严格检查 但只检查下一步
        let head = map.get_my_snake().get_head();
        let mut rec = Direction::Up;
        let mut min_distance = 114514;
        for direction in Direction::iter() {
            let mut pos = *head;
            match direction {
                Direction::Up => pos = Position::up(&pos),
                Direction::Down => pos = Position::down(&pos),
                Direction::Left => pos = Position::left(&pos),
                Direction::Right => pos = Position::right(&pos),
            }
            if !map.colision_check(&pos) {
                let distance = calculate_distance(&pos, &map.get_foods());
                if distance < min_distance {
                    min_distance = distance;
                    rec = direction;
                }
            }
        }

        if min_distance != 114514 {
            println!("find eular path");
            match rec {
                Direction::Up => return 0,
                Direction::Left => return 1,
                Direction::Down => return 2,
                Direction::Right => return 3,
            }
        }

        println!("this is a dellema");

        // 完全没有吃果实路径 这时宽松检查
        let safe_dirs = Direction::iter()
            .filter(|dir| {
                let next_pos = match dir {
                    Direction::Up => Position::up(head),
                    Direction::Down => Position::down(head),
                    Direction::Left => Position::left(head),
                    Direction::Right => Position::right(head),
                };
                !map.strict_obstacles_check(&next_pos)
            })
        .collect::<Vec<_>>();
        if !safe_dirs.is_empty() {
            let mut min_distance = 32;
            let mut best_dir = Direction::Up;
            for dir in safe_dirs {
                let next_pos = match dir {
                    Direction::Up => Position::up(head),
                    Direction::Down => Position::down(head),
                    Direction::Left => Position::left(head),
                    Direction::Right => Position::right(head),
                };
                // TODO: 这里的距离需要调整
                // 距离棋盘中心最近
                let distance = ((board_size / 2) - next_pos.get_x()).abs() + ((board_size / 2) - next_pos.get_y()).abs();
                if distance < min_distance {
                    min_distance = distance;
                    best_dir = dir;
                }
            }
            match best_dir {
                Direction::Up => return 0,
                Direction::Left => return 1,
                Direction::Down => return 2,
                Direction::Right => return 3,
            }
        } else {
            return 0;
        }
    }
}

fn calculate_distance(pos1: &Position, foods: &Vec<Position>) -> i32 {
    let mut min_distance = i32::MAX;
    for food in foods {
        let distance = (pos1.get_x() - food.get_x()).abs() + (pos1.get_y() - food.get_y()).abs();
        if distance < min_distance {
            min_distance = distance;
        }
    }
    min_distance
}

fn process_input(input_my_snake: &Vec<i32>, snake_num: i32, input_other_snakes: &Vec<i32>, food_num: i32, input_foods: &Vec<i32>) -> (Snake, Vec<Snake>, Vec<Position>){
    let mut my_snake = Snake::new();
    let mut i:usize = 0;
    while i < 8 {
        my_snake.append_body(Position::new(input_my_snake[i], input_my_snake[i + 1]));
        i += 2;
    }

    let mut _j:usize = 0;
    let mut other_snakes:Vec<Snake> = Vec::new();
    for i in 0..snake_num {
        let mut snake = Snake::new();
        _j = 0;
        while _j < 8 {
            let pos = Position::new(input_other_snakes[8 * i as usize + _j], input_other_snakes[8 * i as usize + _j + 1]);
            snake.append_body(pos);
            _j += 2;
        }
        other_snakes.push(snake);
    }

    i = 0;
    let mut foods:Vec<Position> = Vec::new();
    while i < input_foods.len() as usize {
        let food = Position::new(input_foods[i], input_foods[i + 1]);
        foods.push(food);
        i += 2;
    }

    if other_snakes.len() != snake_num as usize {
        panic!("other_snakes len is not equal to snake_num, what the hell!");
    }
    if foods.len() != food_num as usize {
        panic!("foods len is not equal to food_num, what the hell!");
    }
    (my_snake, other_snakes, foods)
}

fn find_astar_path(map: &Map) -> Vec<Direction> {
    let foods = map.get_foods();

    let head = *map.get_my_snake().get_head();
    // 当前是选择最近的果子作为目标
    let mut shortest_path:Vec<Direction> = Vec::new();
    for food in foods {
        let mut _tmp_path:Vec<Direction>  = Vec::new();
        let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
        let mut closed_set: HashSet<Position> = HashSet::new();
        let node = Node::new((head).clone(), 0, Node::heuristic(&head, &food), None);
        open_set.push(node);
        while !open_set.is_empty() {
            let cur = match open_set.pop() {
                Some(pos) => pos,
                None => panic!("Got a None Node!"),
            };
            if cur.get_pos() == food {
                _tmp_path = reconstruct_path(&cur);
                println!("find astar path :{:#?}", _tmp_path);
                if shortest_path.is_empty() || shortest_path.len() > _tmp_path.len() {
                    shortest_path = _tmp_path;
                }
                break;
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
                if (!closed_set.contains(&_neighbor)) && (!map.colision_check(&_neighbor)){
                    let g = cur.get_g() + 1;
                    let h = Node::heuristic(&_neighbor, food);
                    open_set.push(Node::new((_neighbor).clone(), g, h, Some((cur).clone())));
                }
            }
        }
    }

    return shortest_path

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
mod test {
    // "1v1": {
    //     boardSize: 5,
    //     snakeCount: 2,
    //     foodCount: 5,
    //     maxRounds: 50,
    //     // Two snakes positioned at diagonal corners
    //     initialSnakePositions: [
    //       [1, 4, 1, 3, 1, 2, 1, 1],
    //       [5, 2, 5, 3, 5, 4, 5, 5]
    //     ]
    //   },

    use crate::greedy_snake_step;

    // board_size: i32
    // input_my_snake: Vec<i32>
    // snake_num: i32
    // input_other_snakes: Vec<i32>
    // food_num: i32
    // input_foods: Vec<i32>
    // _round: i32) 
    #[test]
    fn testcase_pub() {
        let board_size = 5;
        let input_my_snake = vec![3, 5, 2, 5, 1, 5, 1, 4];
        let snake_num = 1;
        let input_other_snakes = vec![4, 4, 4, 3, 4, 2, 5, 2];
        let food_num = 5;
        let input_foods = vec![1, 3, 2, 1, 4, 5, 5, 3, 5, 5];
        let round = 50;  
        match greedy_snake_step(board_size, input_my_snake, snake_num, input_other_snakes, food_num, input_foods, round) {
            -1 => println!("Can't find path"),
            0 => println!("Move UP!"),
            1 => println!("Move Left"),
            2 => println!("Move Down"),
            3 => println!("Move Right"),
            _ => println!("Invalid Return"),
        }
    }
}
