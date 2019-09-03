use super::weights::*;
use csv::ReaderBuilder;
use std::{collections::HashMap, iter::FromIterator};

#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    Obstacle,
    Bomb,
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
    Money(i32, bool),
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
            'x' => Self::Bomb,
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
                i32::from_str_radix(&left, 10).expect("no value associated with the target"),
            ),
            'm' => Self::Money(
                2 ^ i32::from_str_radix(&left, 10).expect("no value associated with the money"),
                false,
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
            Self::Bomb => BLACK_SQUARE,
            Self::Griever(_) => BLACK_SQUARE,
            Self::Money(_, _) => YELLOW_SQUARE,
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
                }

                m.insert(j, c);
            }
        }

        glade
    }

    fn get_forward(&self) -> (usize, usize) {
        match self.griever.direction {
            Direction::North => (self.griever.x, self.griever.y - 1),
            Direction::East => (self.griever.x, self.griever.y - 1),
            Direction::South => (self.griever.x, self.griever.y - 1),
            Direction::West => (self.griever.x, self.griever.y - 1),
        }
    }

    fn get_pos(&self, x: usize, y: usize) -> &Content {
        self.map.get(&y).unwrap().get(&x).unwrap()
    }

    pub fn forward(&mut self) {}

    pub fn bw_eye(&self) -> i32 {
        let f = self.get_forward();
        let p = self.get_pos(f.0, f.1);
        match p.get_color_value() {
            1..9 => 1,
            0 => 0,
            _ => panic!("negative aren't possible"),
        }
    }

    pub fn color_eye(&self) -> i32 {
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
