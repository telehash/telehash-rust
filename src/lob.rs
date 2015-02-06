
pub mod lob {

    struct Packet {
      pub head: u8,
      pub body: u8
    }
    
    pub fn parse(b: &[u8]) {
    }

    pub fn raw(p: &Packet) {
    }

}

#[test]
fn parse_tests() {
    assert_eq!(true, true);
}

