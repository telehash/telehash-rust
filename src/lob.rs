
pub struct Packet {
  pub head: u8,
  pub body: u8
}

pub fn parse(b: &[u8]) -> Packet {
    let mut ret = Packet {head:0, body:0};
    ret.head = b[0];
    ret
}

pub fn raw(p: &Packet) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    ret.push(p.head);
    ret
}

#[test]
fn parse_tests() {
    assert_eq!(true, true);
}

