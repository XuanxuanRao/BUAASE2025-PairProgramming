pub mod map {

    #[derive(Debug)]
    pub struct Map {
        my_snake: Snake,
        other_snakes: Vec<Snake>,
        foods: Vec<Position>,
        size: i32,
    }

    impl Map {

        pub fn new(my_snake: Snake, other_snakes: Vec<Snake>, foods: Vec<Position>, size: i32) -> Map {
            Map { my_snake, other_snakes, foods, size}
        }

        pub fn colision_check(&self, pos: &Position, my_snake: &Snake, other_snakes: &Vec<Snake>) -> bool {
            let my_obstacles = my_snake.get_without_tail();
            let other_obstacles = Snake::build_obstacles(other_snakes);
            if (pos.get_x() <= 0 || pos.get_x() > self.size) || (pos.get_y() <= 0 || pos.get_y() > self.size) {
                return true
            } else if my_obstacles.contains(pos) || other_obstacles.contains(pos) {
                return true
            } else {
                return false
            }
        }

        pub fn get_my_snake(&self) -> &Snake {
            &self.my_snake
        }

        pub fn get_other_snakes(&self) -> &Vec<Snake> {
            &self.other_snakes
        }

        pub fn get_foods(&self) -> &Vec<Position> {
            &self.foods
        }

    }

    #[derive(Debug)]
    pub struct Snake {
        body: Vec<Position>,
    }

    impl Snake {
        pub fn new() -> Snake {
            Snake { body: Vec::new() }
        }

        pub fn append_body(&mut self, pos: Position) {
            self.body.push(pos);
            if self.body.len() > 4 {
                panic!("Error! The length of the snake body has Overflowed!");
            }
        }

        pub fn get_head(&self) -> &Position {
            &self.body[0]
        }

        pub fn get_without_tail(&self) -> &[Position] {
            &self.body[0..=2]
        }

        pub fn build_obstacles(snakes: &Vec<Snake>) -> Vec<Position> {
            let mut obstacles:Vec<Position> = vec![];
            for snake in snakes {
                // 其他蛇的蛇头和蛇头的四周, 这里不需要检测棋盘边界
                // let head = snake.get_head().clone();
                // let head_left = Position::left(&head);
                // let head_right = Position::right(&head);
                // let head_up = Position::up(&head);
                // let head_down = Position::down(&head);
                // // body[1]已经包含在蛇头的四周 
                // let body2 = snake.get_without_tail()[2]; // body[2]
                // obstacles.push(head);
                // obstacles.push(head_left);
                // obstacles.push(head_right);
                // obstacles.push(head_up);
                // obstacles.push(head_down);
                // obstacles.push(body2);
                for ob in snake.get_without_tail() {
                    obstacles.push(*ob);
                }
            }

            obstacles
        }

    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Position {
        x: i32,
        y: i32,
    }

    impl Position {
        pub fn new(x: i32, y: i32) ->Position {
            Position { x, y }
        }

        pub fn up(pos: &Position) -> Position {
            Position {
                x: pos.x,
                y: pos.y + 1,
            }
        }

        pub fn down(pos: &Position) -> Position {
            Position {
                x: pos.x,
                y: pos.y - 1,
            }
        }

        pub fn left(pos: &Position) -> Position {
            Position {
                x: pos.x - 1,
                y: pos.y,
            }
        }

        pub fn right(pos: &Position) -> Position {
            Position {
                x: pos.x + 1,
                y: pos.y,
            }
        }

        pub fn get_x(&self) -> i32 {
            self.x
        }

        pub fn get_y(&self) -> i32 {
            self.y
        }
    }


    #[derive(Debug)]
    pub enum Direction {
        Up, Down, Left, Right, 
    }

    impl Direction {
        pub fn iter() -> impl Iterator<Item = Direction> {
            [Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter()
        }
    }
}