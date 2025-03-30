pub mod astar {
    use crate::map::map::Position;

    #[derive(Clone, Hash)]
    pub struct Node {
        pos: Position,
        g: i32,
        h: i32,
        f: i32,
        parent: Option<Box<Node>>
    }

    impl Node {
        pub fn new(pos: Position, g: i32, h: i32, parent: Option<Node>) -> Node {
            Node {pos, g, h, f: g + h, parent: parent.map(Box::new)}
        }

        pub fn heuristic(a: &Position, b: &Position) -> i32 {
            (a.get_x() - b.get_x()).abs() + (a.get_y() - b.get_y()).abs()
        }

        pub fn get_pos(&self) -> &Position {
            &self.pos
        }

        pub fn get_g(&self) -> i32 {
            self.g
        }

        pub fn get_h(&self) -> i32 {
            self.h
        }

        pub fn get_f(&self) -> i32 {
            self.f
        }

        pub fn get_parent(&self) -> &Option<Box<Node>> {
            &self.parent
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.f == other.f
        }
    }

    impl Eq for Node {}

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.f.cmp(&self.f)
        }
    }
}