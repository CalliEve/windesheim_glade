use super::weights::*;
use csv::ReaderBuilder;
use rand::seq::IteratorRandom;
use std::{collections::HashMap, iter::FromIterator};

#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    Obstacle,
    Bomb(i32, i32),
    WhiteSquare,
    GraySquare,
    RedSquare,
    OrangeSquare,
    YellowSquare,
    GreenSquare,
    BlueSquare,
    PurpleSquare,
    BlackSquare,
    Griever(i32),
    Money(i32),
    Turner(i32),
    Target(i32),
}

impl Content {
    pub fn parse(text: &mut str) -> Self {
        text.make_ascii_lowercase();
        let mut chars = text.chars();
        let i = chars.next().unwrap();
        let left = String::from_iter(chars.take_while(|c| c.is_numeric()));
        match i {
            'q' => Self::Obstacle,
            'x' => Self::Bomb(
                i32::from_str_radix(&left, 10).expect("no value associated with the bomb"),
                0,
            ),
            'w' => Self::WhiteSquare,
            'g' => Self::GraySquare,
            'r' => Self::RedSquare,
            'o' => Self::OrangeSquare,
            'y' => Self::YellowSquare,
            'e' => Self::GreenSquare,
            'b' => Self::BlueSquare,
            'p' => Self::PurpleSquare,
            'l' => Self::BlackSquare,
            't' => Self::Target(
                i32::from_str_radix(&left, 10).expect("no value associated with the target") - 1,
            ),
            'm' => Self::Money(
                2 ^ i32::from_str_radix(&left, 10).expect("no value associated with the money"),
            ),
            'd' => Self::Turner(
                i32::from_str_radix(&left, 10).expect("no value associated with the turner"),
            ),
            's' => Self::Griever(
                i32::from_str_radix(&left, 10).expect("no value associated with the griever"),
            ),
            _ => panic!("invalid glade value"),
        }
    }

    pub fn get_color_value(&self) -> i32 {
        match self {
            Self::BlackSquare => BLACK_SQUARE,
            Self::BlueSquare => BLUE_SQUARE,
            Self::GraySquare => GRAY_SQUARE,
            Self::GreenSquare => GREEN_SQUARE,
            Self::OrangeSquare => ORANGE_SQUARE,
            Self::PurpleSquare => PURPLE_SQUARE,
            Self::RedSquare => RED_SQUARE,
            Self::WhiteSquare => WHITE_SQUARE,
            Self::YellowSquare => YELLOW_SQUARE,
            Self::Bomb(_, _) => BLACK_SQUARE,
            Self::Griever(_) => BLACK_SQUARE,
            Self::Money(_) => YELLOW_SQUARE,
            Self::Obstacle => BLACK_SQUARE,
            Self::Turner(_) => BLUE_SQUARE,
            Self::Target(_) => YELLOW_SQUARE,
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    pub fn parse(i: i32) -> Self {
        match i {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("invalid direction value for griever"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Glade {
    pub map: HashMap<usize, HashMap<usize, Content>>,
    pub griever: Griever,
    seconds: i32,
    target_count: i32,
    last_target: i32,
}

impl Glade {
    pub fn parse(path: &str) -> Glade {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_path(path)
            .expect("unable to read csv file");
        let mut glade = Glade {
            map: HashMap::new(),
            griever: Griever {
                x: 1,
                y: 1,
                direction: Direction::North,
            },
            seconds: 0,
            target_count: -1,
            last_target: 0,
        };

        for (i, r_row) in csv_reader.records().enumerate() {
            let row = r_row
                .unwrap_or_else(|e| panic!("invalid row in csv table at line {}, err: {:?}", i, e));
            glade.map.insert(i, HashMap::new());
            for (j, r_column) in row.iter().enumerate() {
                let m = glade.map.get_mut(&i).unwrap();
                let mut column = String::from(r_column);
                let mut c = Content::parse(&mut column);

                if let Content::Griever(s) = c {
                    glade.griever = Griever {
                        x: j,
                        y: i,
                        direction: Direction::parse(s),
                    };
                    c = Content::BlackSquare;
                } else if let Content::Target(t) = c {
                    if t > glade.last_target {
                        glade.last_target = t
                    }
                }

                m.insert(j, c);
            }
        }

        glade
    }

    fn s_inc(&mut self) {
        self.seconds += 1;
    }

    fn target_inc(&mut self, n: i32) {
        println!("passed target {}", n + 1);
        if n - 1 == self.target_count {
            self.target_count += 1
        }
    }

    pub fn success(&self) -> bool {
        self.target_count != -1 && self.target_count == self.last_target
    }

    fn get_forward(&self) -> (usize, usize) {
        match self.griever.direction {
            Direction::North => (self.griever.x, self.griever.y - 1),
            Direction::East => (self.griever.x + 1, self.griever.y),
            Direction::South => (self.griever.x, self.griever.y + 1),
            Direction::West => (self.griever.x - 1, self.griever.y),
        }
    }

    fn get_backward(&self) -> (usize, usize) {
        match self.griever.direction {
            Direction::North => (self.griever.x, self.griever.y + 1),
            Direction::East => (self.griever.x - 1, self.griever.y),
            Direction::South => (self.griever.x, self.griever.y - 1),
            Direction::West => (self.griever.x + 1, self.griever.y),
        }
    }

    fn get_pos(&mut self, x: usize, y: usize) -> Content {
        self.map.get(&y).unwrap().get(&x).unwrap().clone()
    }

    fn set_pos(&mut self, x: usize, y: usize, content: Content) {
        self.map.get_mut(&y).unwrap().insert(x, content);
    }

    fn handle_new_pos(&mut self, x: usize, y: usize, c: Content) -> Result<i32, ()> {
        match c {
            Content::Money(a) => {
                self.set_pos(x, y, Content::Money(0));
                return Ok(a);
            },
            Content::Bomb(seconds, last) => {
                if seconds == 0 || last + seconds == self.seconds {
                    panic!("\n------------\n\nBOOM!\nYou're dead\n\n------------\n")
                } else if last == 0 {
                    self.set_pos(x, y, Content::Bomb(seconds, self.seconds));
                }
            },
            Content::Target(times) => self.target_inc(times),
            Content::Obstacle => return Err(()),
            Content::Turner(mut times) => {
                if times == 0 {
                    let mut rng = rand::thread_rng();
                    times = (0..4).choose(&mut rng).unwrap();
                }
                let mut i = 0;
                while i < times {
                    i += 1;
                    self.turn_right()
                }
            },
            _ => {},
        }
        Ok(0)
    }

    pub fn forward(&mut self) -> Result<i32, ()> {
        self.s_inc();
        let f = self.get_forward();
        self.griever.x = f.0;
        self.griever.y = f.1;
        let p = self.get_pos(f.0, f.1);

        self.handle_new_pos(f.0, f.1, p)
    }

    pub fn backward(&mut self) -> Result<i32, ()> {
        self.s_inc();
        let b = self.get_backward();
        self.griever.x = b.0;
        self.griever.y = b.1;
        let p = self.get_pos(b.0, b.1);

        self.handle_new_pos(b.0, b.1, p)
    }

    pub fn bw_eye(&mut self) -> i32 {
        let f = self.get_forward();
        let p = self.get_pos(f.0, f.1);
        match p.get_color_value() {
            1..9 => 1,
            0 => 0,
            _ => panic!("negative aren't possible"),
        }
    }

    pub fn color_eye(&mut self) -> i32 {
        let f = self.get_forward();
        let p = self.get_pos(f.0, f.1);
        p.get_color_value()
    }

    pub fn turn_left(&mut self) {
        self.griever.direction = match self.griever.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn turn_right(&mut self) {
        self.griever.direction = match self.griever.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Griever {
    pub x: usize,
    pub y: usize,
    direction: Direction,
}

impl Griever {
    pub fn kompas(&self) -> i32 {
        self.direction.clone() as i32
    }
}
