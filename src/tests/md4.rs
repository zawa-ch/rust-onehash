///	MD4 test pattern (expects 31d6cfe0d16ae931b73c59d7e0c089c0)
const TEST_PATTERN_1: &'static [u8] = b"";
///	MD4 test pattern (expects bde52cb31de33e46245e05fbdbd6fb24)
const TEST_PATTERN_2: &'static [u8] = b"a";
///	MD4 test pattern (expects a448017aaf21d8525fc10ae87aa6729d)
const TEST_PATTERN_3: &'static [u8] = b"abc";
///	MD4 test pattern (expects d9130a8164549fe818874806e1c7014b)
const TEST_PATTERN_4: &'static [u8] = b"message digest";
///	MD4 test pattern (expects d79e1c308aa5bbcdeea8ed63df412da9)
const TEST_PATTERN_5: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
///	MD4 test pattern (expects 043f8582f241db351ce627e153e7f0e4)
const TEST_PATTERN_6: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
///	MD4 test pattern (expects e33b4ddc9c38f2199c3e7b164fcc0536)
const TEST_PATTERN_7: &'static [u8] = b"12345678901234567890123456789012345678901234567890123456789012345678901234567890";
///	MD4 test pattern (expects 2812c6c7136898c51f6f6739ad08750e)
const TEST_PATTERN_8: &'static [u8] = b"The quick brown fox jumps over the lazy dog.";
///	MD4 test pattern (expects 2926f4d148a71c0fd9674b92b046c99e)
///	Quoted from `fortune` Napoleon I
const TEST_PATTERN_9: &'static [u8] = b"Ten persons who speak make more noise than ten thousand who are silent.";
///	MD4 test pattern (expects 3103934386b4bb925a1dc7829754ca43)
///	Quoted from `fortune` T. Cheatham
const TEST_PATTERN_10: &'static [u8] = b"If a group of N persons implements a COBOL compiler, there will be N-1 passes.  Someone in the group has to be the manager.";
///	MD4 test pattern (expects ea36aebd349f858760e258cc7eea8692)
///	Quoted from `fortune`
const TEST_PATTERN_11: &'static [u8] = b"The time was the 19th of May, 1780.  The place was Hartford, Connecticut. The day has gone down in New England history as a terrible foretaste of Judgement Day.  For at noon the skies turned from blue to grey and by mid-afternoon had blackened over so densely that, in that religious age, men fell on their knees and begged a final blessing before the end came. The Connecticut House of Representatives was in session.  And, as some of the men fell down and others clamored for an immediate adjournment, the Speaker of the House, one Col. Davenport, came to his feet.  He silenced them and said these words: \"The day of judgment is either approaching or it is not.  If it is not, there is no cause for adjournment.  If it is, I choose to be found doing my duty.  I wish therefore that candles may be brought.\"  -- Alistair Cooke";

#[test]
fn pattern1() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_1);
    assert_eq!(f.digest(), [0x31, 0xd6, 0xcf, 0xe0, 0xd1, 0x6a, 0xe9, 0x31, 0xb7, 0x3c, 0x59, 0xd7, 0xe0, 0xc0, 0x89, 0xc0]);
}
#[test]
fn pattern2() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_2);
    assert_eq!(f.digest(), [0xbd, 0xe5, 0x2c, 0xb3, 0x1d, 0xe3, 0x3e, 0x46, 0x24, 0x5e, 0x05, 0xfb, 0xdb, 0xd6, 0xfb, 0x24]);
}
#[test]
fn pattern3() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_3);
    assert_eq!(f.digest(), [0xa4, 0x48, 0x01, 0x7a, 0xaf, 0x21, 0xd8, 0x52, 0x5f, 0xc1, 0x0a, 0xe8, 0x7a, 0xa6, 0x72, 0x9d]);
}
#[test]
fn pattern4() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_4);
    assert_eq!(f.digest(), [0xd9, 0x13, 0x0a, 0x81, 0x64, 0x54, 0x9f, 0xe8, 0x18, 0x87, 0x48, 0x06, 0xe1, 0xc7, 0x01, 0x4b]);
}
#[test]
fn pattern5() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_5);
    assert_eq!(f.digest(), [0xd7, 0x9e, 0x1c, 0x30, 0x8a, 0xa5, 0xbb, 0xcd, 0xee, 0xa8, 0xed, 0x63, 0xdf, 0x41, 0x2d, 0xa9]);
}
#[test]
fn pattern6() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_6);
    assert_eq!(f.digest(), [0x04, 0x3f, 0x85, 0x82, 0xf2, 0x41, 0xdb, 0x35, 0x1c, 0xe6, 0x27, 0xe1, 0x53, 0xe7, 0xf0, 0xe4]);
}
#[test]
fn pattern7() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_7);
    assert_eq!(f.digest(), [0xe3, 0x3b, 0x4d, 0xdc, 0x9c, 0x38, 0xf2, 0x19, 0x9c, 0x3e, 0x7b, 0x16, 0x4f, 0xcc, 0x05, 0x36]);
}
#[test]
fn pattern8() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_8);
    assert_eq!(f.digest(), [0x28, 0x12, 0xc6, 0xc7, 0x13, 0x68, 0x98, 0xc5, 0x1f, 0x6f, 0x67, 0x39, 0xad, 0x08, 0x75, 0x0e]);
}
#[test]
fn pattern9() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_9);
    assert_eq!(f.digest(), [0x29, 0x26, 0xf4, 0xd1, 0x48, 0xa7, 0x1c, 0x0f, 0xd9, 0x67, 0x4b, 0x92, 0xb0, 0x46, 0xc9, 0x9e]);
}
#[test]
fn pattern10() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_10);
    assert_eq!(f.digest(), [0x31, 0x03, 0x93, 0x43, 0x86, 0xb4, 0xbb, 0x92, 0x5a, 0x1d, 0xc7, 0x82, 0x97, 0x54, 0xca, 0x43]);
}
#[test]
fn pattern11() {
    use crate::HashFunction;
    use crate::md4::MD4HushFunction;
    let mut f = MD4HushFunction::new();
    f.put_message(TEST_PATTERN_11);
    assert_eq!(f.digest(), [0xea, 0x36, 0xae, 0xbd, 0x34, 0x9f, 0x85, 0x87, 0x60, 0xe2, 0x58, 0xcc, 0x7e, 0xea, 0x86, 0x92]);
}
