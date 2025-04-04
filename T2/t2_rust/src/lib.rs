use astar::astar::Node;
use map::map::{Direction, Map, Position};
use std::collections::{BinaryHeap, HashSet};
use wasm_bindgen::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod map;
pub mod astar;
pub mod random;

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
    use super::*;
    use random::RandomGenerator;

    /// 检查位置是否与已有位置重叠
    fn is_position_overlap(x: i32, y: i32, positions: &[(i32, i32)]) -> bool {
        positions.iter().any(|&(px, py)| px == x && py == y)
    }

    /// 生成不重叠的随机位置
    fn generate_unique_position(existing_positions: &[(i32, i32)]) -> (i32, i32) {
        // 获取当前时间戳作为种子
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut rng = RandomGenerator::new(timestamp);
        
        loop {
            let x = rng.generate_int(0, 10);
            let y = rng.generate_int(0, 10);
            if !is_position_overlap(x, y, existing_positions) {
                return (x, y);
            }
        }
    }


    /// 进行一轮贪吃蛇游戏，直到撞墙、吃到苹果或判断无法吃到
    /// 如果顺利吃到了苹果，返回0；如果蛇死了，返回-1
    fn game(snake: &mut Vec<i32>, apple: &Vec<i32>, obstacles: &Vec<i32>) -> i32 {
        loop {
            let dir = greedy_snake_move_barriers(snake.clone(), apple.clone(), obstacles.clone());
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
    
            // 检查是否与障碍物碰撞
            for i in (0..obstacles.len()).step_by(2) {
                if new_snake[0] == obstacles[i] && new_snake[1] == obstacles[i + 1] {
                    println!("\n撞到了障碍物");
                    return -1;
                }
            }
    
            // 检查是否与蛇身碰撞
            for i in (2..8).step_by(2) {
                if new_snake[0] == new_snake[i] && new_snake[1] == new_snake[i + 1] {
                    println!("\n撞到了自己");
                    return -1;
                }
            }
    
            // 检查是否超出边界
            if new_snake[0] < 0 || new_snake[0] > 10 || new_snake[1] < 0 || new_snake[1] > 10 {
                println!("\n撞墙了.....");
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
    fn test_no_way_to_apple() {
        let mut snake = vec![5, 5, 5, 6, 5, 7, 5, 8];
        let apple = vec![1, 1];
        let obstacles = vec![1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 2, 1, 4, 4, 4, 5, 4, 6, 4, 7];

        let result = game(&mut snake, &apple, &obstacles);
        assert_eq!(result, 1); // 应该返回1，表示无法吃到苹果
    }

    #[test]
    fn random_test() {
        let mut positions = Vec::new();
        
        // 生成蛇的位置（4个段）
        let mut snake = Vec::new();
        for _ in 0..4 {
            let (x, y) = generate_unique_position(&positions);
            positions.push((x, y));
            snake.push(x);
            snake.push(y);
        }
        
        // 生成苹果位置
        let (apple_x, apple_y) = generate_unique_position(&positions);
        positions.push((apple_x, apple_y));
        let apple = vec![apple_x, apple_y];
        
        // 生成障碍物位置（12个）
        let mut obstacles = Vec::new();
        for _ in 0..12 {
            let (x, y) = generate_unique_position(&positions);
            positions.push((x, y));
            obstacles.push(x);
            obstacles.push(y);
        }
        
        // 打印测试场景
        println!("测试场景：");
        println!("蛇的位置: {:?}", snake);
        println!("苹果位置: {:?}", apple);
        println!("障碍物位置: {:?}", obstacles);
        
        // 测试游戏
        let result = game(&mut snake, &apple, &obstacles);
        println!("游戏结果: {}", result);
        assert!(result == 0 || result == 1);
    }
    
}
