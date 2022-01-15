use crate::HashFunction;
use crate::sha3::SHA3Hash224;

///	test pattern
/// (SHA3-224 expects 6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7)
const TEST_PATTERN_1: &'static [u8] = b"";
///	test pattern
/// (SHA3-224 expects 9e86ff69557ca95f405f081269685b38e3a819b309ee942f482b6a8b)
const TEST_PATTERN_2: &'static [u8] = b"a";
///	test pattern
/// (SHA3-224 expects e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf)
const TEST_PATTERN_3: &'static [u8] = b"abc";
///	test pattern
/// (SHA3-224 expects 18768bb4c48eb7fc88e5ddb17efcf2964abd7798a39d86a4b4a1e4c8)
const TEST_PATTERN_4: &'static [u8] = b"message digest";
///	test pattern
/// (SHA3-224 expects 5cdeca81e123f87cad96b9cba999f16f6d41549608d4e0f4681b8239)
const TEST_PATTERN_5: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
///	test pattern
/// (SHA3-224 expects a67c289b8250a6f437a20137985d605589a8c163d45261b15419556e)
const TEST_PATTERN_6: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
///	test pattern
/// (SHA3-224 expects 0526898e185869f91b3e2a76dd72a15dc6940a67c8164a044cd25cc8)
const TEST_PATTERN_7: &'static [u8] = b"12345678901234567890123456789012345678901234567890123456789012345678901234567890";
///	test pattern
/// (SHA3-224 expects 2d0708903833afabdd232a20201176e8b58c5be8a6fe74265ac54db0)
const TEST_PATTERN_8: &'static [u8] = b"The quick brown fox jumps over the lazy dog.";
///	test pattern
/// (SHA3-224 expects 93a395953a6ba50d6d4d683c7ba92f60e82f37e941dbb6ae3aff6400)
///	Quoted from `fortune` Napoleon I
const TEST_PATTERN_9: &'static [u8] = b"Ten persons who speak make more noise than ten thousand who are silent.";
///	test pattern
/// (SHA3-224 expects a5c916c94f1713d012cca96edac3cf79c473c95b9ca6fff2d3e38470)
///	Quoted from `fortune` T. Cheatham
const TEST_PATTERN_10: &'static [u8] = b"If a group of N persons implements a COBOL compiler, there will be N-1 passes.  Someone in the group has to be the manager.";
///	test pattern
/// (SHA3-224 expects 503d335381336c842dcceab3d66a95122346538139af20c60f322509)
///	Quoted from `fortune`
const TEST_PATTERN_11: &'static [u8] = b"The time was the 19th of May, 1780.  The place was Hartford, Connecticut. The day has gone down in New England history as a terrible foretaste of Judgement Day.  For at noon the skies turned from blue to grey and by mid-afternoon had blackened over so densely that, in that religious age, men fell on their knees and begged a final blessing before the end came. The Connecticut House of Representatives was in session.  And, as some of the men fell down and others clamored for an immediate adjournment, the Speaker of the House, one Col. Davenport, came to his feet.  He silenced them and said these words: \"The day of judgment is either approaching or it is not.  If it is not, there is no cause for adjournment.  If it is, I choose to be found doing my duty.  I wish therefore that candles may be brought.\"  -- Alistair Cooke";

#[test]
fn pattern01() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_1);
    assert_eq!(f.digest(), [0x6b, 0x4e, 0x03, 0x42, 0x36, 0x67, 0xdb, 0xb7, 0x3b, 0x6e, 0x15, 0x45, 0x4f, 0x0e, 0xb1, 0xab, 0xd4, 0x59, 0x7f, 0x9a, 0x1b, 0x07, 0x8e, 0x3f, 0x5b, 0x5a, 0x6b, 0xc7]);
}
#[test]
fn pattern02() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_2);
    assert_eq!(f.digest(), [0x9e, 0x86, 0xff, 0x69, 0x55, 0x7c, 0xa9, 0x5f, 0x40, 0x5f, 0x08, 0x12, 0x69, 0x68, 0x5b, 0x38, 0xe3, 0xa8, 0x19, 0xb3, 0x09, 0xee, 0x94, 0x2f, 0x48, 0x2b, 0x6a, 0x8b]);
}
#[test]
fn pattern03() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_3);
    assert_eq!(f.digest(), [0xe6, 0x42, 0x82, 0x4c, 0x3f, 0x8c, 0xf2, 0x4a, 0xd0, 0x92, 0x34, 0xee, 0x7d, 0x3c, 0x76, 0x6f, 0xc9, 0xa3, 0xa5, 0x16, 0x8d, 0x0c, 0x94, 0xad, 0x73, 0xb4, 0x6f, 0xdf]);
}
#[test]
fn pattern04() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_4);
    assert_eq!(f.digest(), [0x18, 0x76, 0x8b, 0xb4, 0xc4, 0x8e, 0xb7, 0xfc, 0x88, 0xe5, 0xdd, 0xb1, 0x7e, 0xfc, 0xf2, 0x96, 0x4a, 0xbd, 0x77, 0x98, 0xa3, 0x9d, 0x86, 0xa4, 0xb4, 0xa1, 0xe4, 0xc8]);
}
#[test]
fn pattern05() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_5);
    assert_eq!(f.digest(), [0x5c, 0xde, 0xca, 0x81, 0xe1, 0x23, 0xf8, 0x7c, 0xad, 0x96, 0xb9, 0xcb, 0xa9, 0x99, 0xf1, 0x6f, 0x6d, 0x41, 0x54, 0x96, 0x08, 0xd4, 0xe0, 0xf4, 0x68, 0x1b, 0x82, 0x39]);
}
#[test]
fn pattern06() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_6);
    assert_eq!(f.digest(), [0xa6, 0x7c, 0x28, 0x9b, 0x82, 0x50, 0xa6, 0xf4, 0x37, 0xa2, 0x01, 0x37, 0x98, 0x5d, 0x60, 0x55, 0x89, 0xa8, 0xc1, 0x63, 0xd4, 0x52, 0x61, 0xb1, 0x54, 0x19, 0x55, 0x6e]);
}
#[test]
fn pattern07() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_7);
    assert_eq!(f.digest(), [0x05, 0x26, 0x89, 0x8e, 0x18, 0x58, 0x69, 0xf9, 0x1b, 0x3e, 0x2a, 0x76, 0xdd, 0x72, 0xa1, 0x5d, 0xc6, 0x94, 0x0a, 0x67, 0xc8, 0x16, 0x4a, 0x04, 0x4c, 0xd2, 0x5c, 0xc8]);
}
#[test]
fn pattern08() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_8);
    assert_eq!(f.digest(), [0x2d, 0x07, 0x08, 0x90, 0x38, 0x33, 0xaf, 0xab, 0xdd, 0x23, 0x2a, 0x20, 0x20, 0x11, 0x76, 0xe8, 0xb5, 0x8c, 0x5b, 0xe8, 0xa6, 0xfe, 0x74, 0x26, 0x5a, 0xc5, 0x4d, 0xb0]);
}
#[test]
fn pattern09() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_9);
    assert_eq!(f.digest(), [0x93, 0xa3, 0x95, 0x95, 0x3a, 0x6b, 0xa5, 0x0d, 0x6d, 0x4d, 0x68, 0x3c, 0x7b, 0xa9, 0x2f, 0x60, 0xe8, 0x2f, 0x37, 0xe9, 0x41, 0xdb, 0xb6, 0xae, 0x3a, 0xff, 0x64, 0x00]);
}
#[test]
fn pattern10() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_10);
    assert_eq!(f.digest(), [0xa5, 0xc9, 0x16, 0xc9, 0x4f, 0x17, 0x13, 0xd0, 0x12, 0xcc, 0xa9, 0x6e, 0xda, 0xc3, 0xcf, 0x79, 0xc4, 0x73, 0xc9, 0x5b, 0x9c, 0xa6, 0xff, 0xf2, 0xd3, 0xe3, 0x84, 0x70]);
}
#[test]
fn pattern11() {
    let mut f = SHA3Hash224::new();
    f.put_message(TEST_PATTERN_11);
    assert_eq!(f.digest(), [0x50, 0x3d, 0x33, 0x53, 0x81, 0x33, 0x6c, 0x84, 0x2d, 0xcc, 0xea, 0xb3, 0xd6, 0x6a, 0x95, 0x12, 0x23, 0x46, 0x53, 0x81, 0x39, 0xaf, 0x20, 0xc6, 0x0f, 0x32, 0x25, 0x09]);
}
