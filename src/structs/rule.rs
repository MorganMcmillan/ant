use super::*;
use std::str::FromStr;

#[derive(rand_derive2::RandGen)]
pub enum Rule{
    TurnRight,
    TurnLeft,
    TurnBack,
    Forward
}

// TODO: implement state de/serialization

impl Rule{
    fn from_char(c:char) -> Option<Self>{
        use Rule::*;
        match c.to_ascii_lowercase(){
            'f' => Some(Forward),
            'l' => Some(TurnLeft),
            'r' => Some(TurnRight),
            'b' => Some(TurnBack),
            _ => None
        }
    }

    fn to_char(&self) -> char{
        use Rule::*;
        match *self{
            Forward => 'f',
            TurnLeft => 'l',
            TurnRight => 'r',
            TurnBack => 'b'
        }
    }
}

fn parse_color(s:&str) -> Option<Color>{
    todo!()
}

fn color_to_string(color: Color) -> String{
    todo!()
}
