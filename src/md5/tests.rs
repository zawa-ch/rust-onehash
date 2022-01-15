///	MD5 test pattern (expects d41d8cd98f00b204e9800998ecf8427e)
const TEST_PATTERN_1: &'static [u8] = b"";
///	MD5 test pattern (expects 0cc175b9c0f1b6a831c399e269772661)
const TEST_PATTERN_2: &'static [u8] = b"a";
///	MD5 test pattern (expects 900150983cd24fb0d6963f7d28e17f72)
const TEST_PATTERN_3: &'static [u8] = b"abc";
///	MD5 test pattern (expects f96b697d7cb7938d525a2f31aaf161d0)
const TEST_PATTERN_4: &'static [u8] = b"message digest";
///	MD5 test pattern (expects c3fcd3d76192e4007dfb496cca67e13b)
const TEST_PATTERN_5: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
///	MD5 test pattern (expects d174ab98d277d9f5a5611c2c9f419d9f)
const TEST_PATTERN_6: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
///	MD5 test pattern (expects 57edf4a22be3c955ac49da2e2107b67a)
const TEST_PATTERN_7: &'static [u8] = b"12345678901234567890123456789012345678901234567890123456789012345678901234567890";
///	MD5 test pattern (expects e4d909c290d0fb1ca068ffaddf22cbd0)
const TEST_PATTERN_8: &'static [u8] = b"The quick brown fox jumps over the lazy dog.";
///	MD5 test pattern (expects 119b13e82eaa45291fe89b97fda96ba7)
///	Quoted from `fortune` Napoleon I
const TEST_PATTERN_9: &'static [u8] = b"Ten persons who speak make more noise than ten thousand who are silent.";
///	MD5 test pattern (expects d89f0c2e36ca332482c9c599d417f8dc)
///	Quoted from `fortune` T. Cheatham
const TEST_PATTERN_10: &'static [u8] = b"If a group of N persons implements a COBOL compiler, there will be N-1 passes.  Someone in the group has to be the manager.";
///	MD5 test pattern (expects c8c4cd8d771d0a6f7dc7d08de43d0e18)
///	Quoted from `fortune`
const TEST_PATTERN_11: &'static [u8] = b"The time was the 19th of May, 1780.  The place was Hartford, Connecticut. The day has gone down in New England history as a terrible foretaste of Judgement Day.  For at noon the skies turned from blue to grey and by mid-afternoon had blackened over so densely that, in that religious age, men fell on their knees and begged a final blessing before the end came. The Connecticut House of Representatives was in session.  And, as some of the men fell down and others clamored for an immediate adjournment, the Speaker of the House, one Col. Davenport, came to his feet.  He silenced them and said these words: \"The day of judgment is either approaching or it is not.  If it is not, there is no cause for adjournment.  If it is, I choose to be found doing my duty.  I wish therefore that candles may be brought.\"  -- Alistair Cooke";

#[test]
fn pattern1() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_1);
    assert_eq!(f.digest(), [0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e]);
}
#[test]
fn pattern2() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_2);
    assert_eq!(f.digest(), [0x0c, 0xc1, 0x75, 0xb9, 0xc0, 0xf1, 0xb6, 0xa8, 0x31, 0xc3, 0x99, 0xe2, 0x69, 0x77, 0x26, 0x61]);
}
#[test]
fn pattern3() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_3);
    assert_eq!(f.digest(), [0x90, 0x01, 0x50, 0x98, 0x3c, 0xd2, 0x4f, 0xb0, 0xd6, 0x96, 0x3f, 0x7d, 0x28, 0xe1, 0x7f, 0x72]);
}
#[test]
fn pattern4() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_4);
    assert_eq!(f.digest(), [0xf9, 0x6b, 0x69, 0x7d, 0x7c, 0xb7, 0x93, 0x8d, 0x52, 0x5a, 0x2f, 0x31, 0xaa, 0xf1, 0x61, 0xd0]);
}
#[test]
fn pattern5() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_5);
    assert_eq!(f.digest(), [0xc3, 0xfc, 0xd3, 0xd7, 0x61, 0x92, 0xe4, 0x00, 0x7d, 0xfb, 0x49, 0x6c, 0xca, 0x67, 0xe1, 0x3b]);
}
#[test]
fn pattern6() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_6);
    assert_eq!(f.digest(), [0xd1, 0x74, 0xab, 0x98, 0xd2, 0x77, 0xd9, 0xf5, 0xa5, 0x61, 0x1c, 0x2c, 0x9f, 0x41, 0x9d, 0x9f]);
}
#[test]
fn pattern7() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_7);
    assert_eq!(f.digest(), [0x57, 0xed, 0xf4, 0xa2, 0x2b, 0xe3, 0xc9, 0x55, 0xac, 0x49, 0xda, 0x2e, 0x21, 0x07, 0xb6, 0x7a]);
}
#[test]
fn pattern8() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_8);
    assert_eq!(f.digest(), [0xe4, 0xd9, 0x09, 0xc2, 0x90, 0xd0, 0xfb, 0x1c, 0xa0, 0x68, 0xff, 0xad, 0xdf, 0x22, 0xcb, 0xd0]);
}
#[test]
fn pattern9() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_9);
    assert_eq!(f.digest(), [0x11, 0x9b, 0x13, 0xe8, 0x2e, 0xaa, 0x45, 0x29, 0x1f, 0xe8, 0x9b, 0x97, 0xfd, 0xa9, 0x6b, 0xa7]);
}
#[test]
fn pattern10() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_10);
    assert_eq!(f.digest(), [0xd8, 0x9f, 0x0c, 0x2e, 0x36, 0xca, 0x33, 0x24, 0x82, 0xc9, 0xc5, 0x99, 0xd4, 0x17, 0xf8, 0xdc]);
}
#[test]
fn pattern11() {
    use crate::HashFunction;
    use crate::md5::MD5HushFunction;
    let mut f = MD5HushFunction::new();
    f.put_message(TEST_PATTERN_11);
    assert_eq!(f.digest(), [0xc8, 0xc4, 0xcd, 0x8d, 0x77, 0x1d, 0x0a, 0x6f, 0x7d, 0xc7, 0xd0, 0x8d, 0xe4, 0x3d, 0x0e, 0x18]);
}
