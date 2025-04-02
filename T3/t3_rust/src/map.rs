pub mod map {

    #[derive(Debug)]
    pub struct Map {
        my_snake: Snake,
        other_snakes: Vec<Snake>,
        obstacles: Vec<Position>,
        strict_obstacles: Vec<Position>,
        foods: Vec<Position>,
        size: i32,
    }

    impl Map {

        pub fn new(my_snake: Snake, other_snakes: Vec<Snake>, foods: Vec<Position>, size: i32) -> Map {
            let obstacles = Map::build_obstacles(&other_snakes);
            let strict_obstacles = Map::build_strict_obstacles(&other_snakes);
            Map { my_snake, other_snakes, obstacles, strict_obstacles, foods, size}
        }

        pub fn build_obstacles(snakes: &Vec<Snake>) -> Vec<Position> {
            let mut obstacles:Vec<Position> = vec![];
            for snake in snakes {
                for ob in snake.get_without_tail() {
                    obstacles.push(*ob);
                }
                let head = snake.get_head();
                for direction in Direction::iter() {
                    let mut pos = *head;
                    match direction {
                        Direction::Up => pos = Position::up(&pos),
                        Direction::Down => pos = Position::down(&pos),
                        Direction::Left => pos = Position::left(&pos),
                        Direction::Right => pos = Position::right(&pos),
                    }
                    if !obstacles.contains(&pos) {
                        obstacles.push(pos);
                    }
                }
            }
            obstacles
        }

        pub fn build_strict_obstacles(snakes: &Vec<Snake>) -> Vec<Position> {
            let mut obstacles:Vec<Position> = vec![];
            for snake in snakes {
                for ob in snake.get_without_tail() {
                    obstacles.push(*ob);
                }
            }

            obstacles
        }

        pub fn colision_check(&self, pos: &Position) -> bool {
            if self.foods.contains(pos)  && pos.get_x() > 1 && pos.get_y() > 1  && pos.get_x() < self.size && pos.get_y() < self.size {
                return false
            } else if (pos.get_x() < 1 || pos.get_x() > self.size) || (pos.get_y() < 1 || pos.get_y() > self.size) {
                return true
            } else if self.my_snake.get_without_tail().contains(pos) || self.obstacles.contains(pos) {
                return true
            } else {
                return false
            }
        }

        pub fn strict_obstacles_check(&self, pos: &Position) -> bool {
            if (pos.get_x() < 1 || pos.get_x() > self.size) || (pos.get_y() < 1 || pos.get_y() > self.size) {
                return true
            } else if self.my_snake.get_without_tail().contains(pos) || self.strict_obstacles.contains(pos) {
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