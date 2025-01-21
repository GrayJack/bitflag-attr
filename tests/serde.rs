use bitflag_attr::bitflag;

use serde_test::{assert_tokens, Configure, Token::*};

#[bitflag(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SerdeFlags {
    A = 1,
    B = 2,
    C = 4,
    D = 8,
}

#[test]
fn bytemuck_works() {
    assert_tokens(&SerdeFlags::empty().readable(), &[Str("")]);

    assert_tokens(&SerdeFlags::empty().compact(), &[U32(0)]);

    assert_tokens(&(SerdeFlags::A | SerdeFlags::B).readable(), &[Str("A | B")]);

    assert_tokens(&(SerdeFlags::A | SerdeFlags::B).compact(), &[U32(1 | 2)]);
}
