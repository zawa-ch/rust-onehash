///	MD2 test pattern (expects 8350e5a3e24c153df2275c9f80692773)
const TEST_PATTERN_1: &'static [u8] = b"";
///	MD2 test pattern (expects 32ec01ec4a6dac72c0ab96fb34c0b5d1)
const TEST_PATTERN_2: &'static [u8] = b"a";
///	MD2 test pattern (expects da853b0d3f88d99b30283a69e6ded6bb)
const TEST_PATTERN_3: &'static [u8] = b"abc";
///	MD2 test pattern (expects ab4f496bfb2a530b219ff33031fe06b0)
const TEST_PATTERN_4: &'static [u8] = b"message digest";
///	MD2 test pattern (expects 4e8ddff3650292ab5a4108c3aa47940b)
const TEST_PATTERN_5: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
///	MD2 test pattern (expects da33def2a42df13975352846c30338cd)
const TEST_PATTERN_6: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
///	MD2 test pattern (expects d5976f79d83d3a0dc9806c3c66f3efd8)
const TEST_PATTERN_7: &'static [u8] = b"12345678901234567890123456789012345678901234567890123456789012345678901234567890";
///	MD2 test pattern (expects 71eaa7e440b611e41a6f0d97384b342a)
const TEST_PATTERN_8: &'static [u8] = b"The quick brown fox jumps over the lazy dog.";
///	MD2 test pattern (expects 0da567bd1760493813144ac1cf46a6b7)
///	Quoted from `fortune` Napoleon I
const TEST_PATTERN_9: &'static [u8] = b"Ten persons who speak make more noise than ten thousand who are silent.";
///	MD2 test pattern (expects b6604572515a26cbe519bfa8f6aa2cf4)
///	Quoted from `fortune` T. Cheatham
const TEST_PATTERN_10: &'static [u8] = b"If a group of N persons implements a COBOL compiler, there will be N-1 passes.  Someone in the group has to be the manager.";
///	MD2 test pattern (expects b0bffa3d59160be11bcf21eb188d0954)
///	Quoted from `fortune`
const TEST_PATTERN_11: &'static [u8] = b"The time was the 19th of May, 1780.  The place was Hartford, Connecticut. The day has gone down in New England history as a terrible foretaste of Judgement Day.  For at noon the skies turned from blue to grey and by mid-afternoon had blackened over so densely that, in that religious age, men fell on their knees and begged a final blessing before the end came. The Connecticut House of Representatives was in session.  And, as some of the men fell down and others clamored for an immediate adjournment, the Speaker of the House, one Col. Davenport, came to his feet.  He silenced them and said these words: \"The day of judgment is either approaching or it is not.  If it is not, there is no cause for adjournment.  If it is, I choose to be found doing my duty.  I wish therefore that candles may be brought.\"  -- Alistair Cooke";

#[test]
fn pattern1() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_1);
    assert_eq!(f.digest(), [0x83, 0x50, 0xe5, 0xa3, 0xe2, 0x4c, 0x15, 0x3d, 0xf2, 0x27, 0x5c, 0x9f, 0x80, 0x69, 0x27, 0x73]);
}
#[test]
fn pattern2() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_2);
    assert_eq!(f.digest(), [0x32, 0xec, 0x01, 0xec, 0x4a, 0x6d, 0xac, 0x72, 0xc0, 0xab, 0x96, 0xfb, 0x34, 0xc0, 0xb5, 0xd1]);
}
#[test]
fn pattern3() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_3);
    assert_eq!(f.digest(), [0xda, 0x85, 0x3b, 0x0d, 0x3f, 0x88, 0xd9, 0x9b, 0x30, 0x28, 0x3a, 0x69, 0xe6, 0xde, 0xd6, 0xbb]);
}
#[test]
fn pattern4() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_4);
    assert_eq!(f.digest(), [0xab, 0x4f, 0x49, 0x6b, 0xfb, 0x2a, 0x53, 0x0b, 0x21, 0x9f, 0xf3, 0x30, 0x31, 0xfe, 0x06, 0xb0]);
}
#[test]
fn pattern5() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_5);
    assert_eq!(f.digest(), [0x4e, 0x8d, 0xdf, 0xf3, 0x65, 0x02, 0x92, 0xab, 0x5a, 0x41, 0x08, 0xc3, 0xaa, 0x47, 0x94, 0x0b]);
}
#[test]
fn pattern6() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_6);
    assert_eq!(f.digest(), [0xda, 0x33, 0xde, 0xf2, 0xa4, 0x2d, 0xf1, 0x39, 0x75, 0x35, 0x28, 0x46, 0xc3, 0x03, 0x38, 0xcd]);
}
#[test]
fn pattern7() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_7);
    assert_eq!(f.digest(), [0xd5, 0x97, 0x6f, 0x79, 0xd8, 0x3d, 0x3a, 0x0d, 0xc9, 0x80, 0x6c, 0x3c, 0x66, 0xf3, 0xef, 0xd8]);
}
#[test]
fn pattern8() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_8);
    assert_eq!(f.digest(), [0x71, 0xea, 0xa7, 0xe4, 0x40, 0xb6, 0x11, 0xe4, 0x1a, 0x6f, 0x0d, 0x97, 0x38, 0x4b, 0x34, 0x2a]);
}
#[test]
fn pattern9() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_9);
    assert_eq!(f.digest(), [0x0d, 0xa5, 0x67, 0xbd, 0x17, 0x60, 0x49, 0x38, 0x13, 0x14, 0x4a, 0xc1, 0xcf, 0x46, 0xa6, 0xb7]);
}
#[test]
fn pattern10() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_10);
    assert_eq!(f.digest(), [0xb6, 0x60, 0x45, 0x72, 0x51, 0x5a, 0x26, 0xcb, 0xe5, 0x19, 0xbf, 0xa8, 0xf6, 0xaa, 0x2c, 0xf4]);
}
#[test]
fn pattern11() {
    use crate::HashFunction;
    use crate::md2::MD2HushFunction;
    let mut f = MD2HushFunction::new();
    f.put_message(TEST_PATTERN_11);
    assert_eq!(f.digest(), [0xb0, 0xbf, 0xfa, 0x3d, 0x59, 0x16, 0x0b, 0xe1, 0x1b, 0xcf, 0x21, 0xeb, 0x18, 0x8d, 0x09, 0x54]);
}
