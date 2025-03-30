pub mod map {

    pub struct Map {
        // snake: Vec<Position>,
        // obstacles: Vec<Position>,
        // apple: Position,
    }

    impl Map {
        // pub fn new(snake: Vec<Position>, obstacles: Vec<Position>, apple: Position) -> Map {
        //     Map { snake, obstacles, apple }
        // }

        pub fn colision_check(pos: &Position, snake: &[Position], obstacles: &Vec<Position>) -> bool {
            if pos.x <= 0 || pos.x > 8 || pos.y <= 0 || pos.y > 8 {
                return true;
            } else if snake.contains(pos) || obstacles.contains(pos) {
                return true;
            }
            return false;
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
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