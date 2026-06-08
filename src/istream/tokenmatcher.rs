const ADD: u8       = b'+';
const SUB: u8       = b'-';
const RIGHT: u8     = b'>';
const LEFT: u8      = b'<';
const LOOP: u8      = b'[';
const ENDLOOP: u8   = b']';
const OUT: u8       = b'.';
const IN: u8        = b',';

#[derive(Debug, PartialEq, Clone)]
pub enum BNToken {
    Add,
    Sub,
    Right,
    Left,
    Loop,
    EndLoop,
    In,
    Out,
    None
}

pub trait TokenMatcherTrait {
    fn match_token(byte: u8) -> BNToken;
}

pub struct BNTokenMatcher;
impl TokenMatcherTrait for BNTokenMatcher
{
    #[inline]
    fn match_token(byte: u8) -> BNToken
    {
        match byte {
            ADD     => BNToken::Add,
            SUB     => BNToken::Sub,
            RIGHT   => BNToken::Right,
            LEFT    => BNToken::Left,
            LOOP    => BNToken::Loop,
            ENDLOOP => BNToken::EndLoop,
            IN      => BNToken::In,
            OUT     => BNToken::Out,
            _       => BNToken::None
        }
    }
}