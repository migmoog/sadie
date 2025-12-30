use euclid::default::Vector2D;
use logos::Logos;

#[derive(Logos, Debug)]
enum Token {
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i32>().ok())]
    Mult(i32),

    #[token("h")]
    Left,

    #[token("l")]
    Right,

    #[token("k")]
    Up,

    #[token("j")]
    Down,
}

impl Token {
    fn is_direction(&self) -> bool {
        matches!(self, Self::Left | Self::Right | Self::Up | Self::Down)
    }
}

pub enum Action {
    MoveCursor(Vector2D<i32>),
}

pub enum ActionError {

}

fn direction_to_vector(tok: Token, count: i32) -> Option<Vector2D<i32>> {
    match tok {
        Token::Up => Vector2D::new(0, -count),
        Token::Left => Vector2D::new(-count, 0),
        Token::Down => Vector2D::new(0, count),
        Token::Right => Vector2D::new(count, 0),
        _ => return None,
    }
    .into()
}

pub fn parse_move_cursor(input: &str) -> Option<Action> {
    let mut lex = Token::lexer(input);

    let count = match lex.next() {
        Some(Ok(Token::Mult(count))) => count,
        Some(Ok(tok)) if tok.is_direction() => {
            return direction_to_vector(tok, 1).map(Action::MoveCursor)
        }
        _ => return None,
    };

    if let Some(Ok(tok)) = lex.next() {
        direction_to_vector(tok, count).map(Action::MoveCursor)
    } else {
        None
    }
}

pub fn parse_action(buffer: &str) -> Option<Result<Action, ActionError>> {
    None // TODO
}
