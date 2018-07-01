use crypto::random;

const TOK_SIZE: usize = 64;

/// An authentication token that can be compared in constant time
/// 
/// ```
/// 
/// use lockchain_core::users::auth::Token;
/// let t1 = Token::new();
/// let t2 = Token::new();
/// 
/// // Will fail, but no expose failure length
/// assert_eq!(t1, t2);
/// ```
pub struct Token {
    tok: [u8; TOK_SIZE],
}

impl Token {
    pub fn new() -> Self {
        let v = random::bytes(TOK_SIZE);
        let mut tok = [0; TOK_SIZE];
        tok.copy_from_slice(v.as_slice());

        Self { tok }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        let mut ret = true;
        for i in 0..TOK_SIZE {
            if self.tok[i] != other.tok[i] {
                ret = false;
            }
        }
        ret
    }
}

impl Eq for Token {}
