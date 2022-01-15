use crate::keccak::{KeccakStateArray, KeccakWord64};

const P0: KeccakWord64 = KeccakWord64(0);
const P1: KeccakWord64 = KeccakWord64(0xFEDCBA9876543210);
const P2: KeccakWord64 = KeccakWord64(0x0000000000200000);

#[test] fn get_lane0() {
    let a = KeccakStateArray::<KeccakWord64>([P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(0, 0), P1);
}
#[test] fn get_lane1() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(1, 0), P1);
}
#[test] fn get_lane2() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(2, 0), P1);
}
#[test] fn get_lane3() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(3, 0), P1);
}
#[test] fn get_lane4() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(4, 0), P1);
}
#[test] fn get_lane5() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(0, 1), P1);
}
#[test] fn get_lane6() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(1, 1), P1);
}
#[test] fn get_lane7() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(2, 1), P1);
}
#[test] fn get_lane8() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(3, 1), P1);
}
#[test] fn get_lane9() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(4, 1), P1);
}
#[test] fn get_lane10() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(0, 2), P1);
}
#[test] fn get_lane11() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(1, 2), P1);
}
#[test] fn get_lane12() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(2, 2), P1);
}
#[test] fn get_lane13() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(3, 2), P1);
}
#[test] fn get_lane14() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(4, 2), P1);
}
#[test] fn get_lane15() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(0, 3), P1);
}
#[test] fn get_lane16() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(1, 3), P1);
}
#[test] fn get_lane17() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(2, 3), P1);
}
#[test] fn get_lane18() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(3, 3), P1);
}
#[test] fn get_lane19() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0]);
    assert_eq!(*a.lane(4, 3), P1);
}
#[test] fn get_lane20() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0]);
    assert_eq!(*a.lane(0, 4), P1);
}
#[test] fn get_lane21() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0]);
    assert_eq!(*a.lane(1, 4), P1);
}
#[test] fn get_lane22() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0]);
    assert_eq!(*a.lane(2, 4), P1);
}
#[test] fn get_lane23() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0]);
    assert_eq!(*a.lane(3, 4), P1);
}
#[test] fn get_lane24() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1]);
    assert_eq!(*a.lane(4, 4), P1);
}
#[test] fn set_lane0() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(0, 0) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane1() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(1, 0) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane2() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(2, 0) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane3() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(3, 0) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane4() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(4, 0) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane5() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(0, 1) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane6() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(1, 1) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane7() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(2, 1) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane8() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(3, 1) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane9() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(4, 1) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane10() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(0, 2) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane11() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(1, 2) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane12() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(2, 2) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane13() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(3, 2) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane14() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(4, 2) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane15() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(0, 3) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane16() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(1, 3) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane17() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(2, 3) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane18() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(3, 3) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane19() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(4, 3) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0, P0]));
}
#[test] fn set_lane20() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(0, 4) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0, P0]));
}
#[test] fn set_lane21() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(1, 4) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0, P0]));
}
#[test] fn set_lane22() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(2, 4) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0, P0]));
}
#[test] fn set_lane23() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(3, 4) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1, P0]));
}
#[test] fn set_lane24() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    *a.lane_mut(4, 4) = P1;
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P1]));
}
#[test] fn get_array_bit0() {
    let a = KeccakStateArray::<KeccakWord64>([P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(0, 0, 21), true);
}
#[test] fn get_array_bit1() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(1, 0, 21), true);
}
#[test] fn get_array_bit2() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(2, 0, 21), true);
}
#[test] fn get_array_bit3() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(3, 0, 21), true);
}
#[test] fn get_array_bit4() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(4, 0, 21), true);
}
#[test] fn get_array_bit5() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(0, 1, 21), true);
}
#[test] fn get_array_bit6() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(1, 1, 21), true);
}
#[test] fn get_array_bit7() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(2, 1, 21), true);
}
#[test] fn get_array_bit8() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(3, 1, 21), true);
}
#[test] fn get_array_bit9() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(4, 1, 21), true);
}
#[test] fn get_array_bit10() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(0, 2, 21), true);
}
#[test] fn get_array_bit11() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(1, 2, 21), true);
}
#[test] fn get_array_bit12() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(2, 2, 21), true);
}
#[test] fn get_array_bit13() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(3, 2, 21), true);
}
#[test] fn get_array_bit14() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(4, 2, 21), true);
}
#[test] fn get_array_bit15() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(0, 3, 21), true);
}
#[test] fn get_array_bit16() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(1, 3, 21), true);
}
#[test] fn get_array_bit17() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(2, 3, 21), true);
}
#[test] fn get_array_bit18() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(3, 3, 21), true);
}
#[test] fn get_array_bit19() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(4, 3, 21), true);
}
#[test] fn get_array_bit20() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0]);
    assert_eq!(a.get_array_bit(0, 4, 21), true);
}
#[test] fn get_array_bit21() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0]);
    assert_eq!(a.get_array_bit(1, 4, 21), true);
}
#[test] fn get_array_bit22() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0]);
    assert_eq!(a.get_array_bit(2, 4, 21), true);
}
#[test] fn get_array_bit23() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0]);
    assert_eq!(a.get_array_bit(3, 4, 21), true);
}
#[test] fn get_array_bit24() {
    let a = KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2]);
    assert_eq!(a.get_array_bit(4, 4, 21), true);
}
#[test] fn set_array_bit0() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(0, 0, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit1() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(1, 0, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit2() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(2, 0, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit3() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(3, 0, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit4() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(4, 0, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit5() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(0, 1, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit6() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(1, 1, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit7() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(2, 1, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit8() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(3, 1, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit9() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(4, 1, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit10() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(0, 2, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit11() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(1, 2, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit12() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(2, 2, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit13() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(3, 2, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit14() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(4, 2, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit15() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(0, 3, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit16() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(1, 3, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit17() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(2, 3, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit18() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(3, 3, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit19() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(4, 3, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0, P0]));
}
#[test] fn set_array_bit20() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(0, 4, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0, P0]));
}
#[test] fn set_array_bit21() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(1, 4, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0, P0]));
}
#[test] fn set_array_bit22() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(2, 4, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0, P0]));
}
#[test] fn set_array_bit23() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(3, 4, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2, P0]));
}
#[test] fn set_array_bit24() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.set_array_bit(4, 4, 21, true);
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P0, P2]));
}
#[test] fn op_theta() {
    let mut a = KeccakStateArray::<KeccakWord64>([
        KeccakWord64(0x000000), KeccakWord64(0x00000F), KeccakWord64(0x0000F0), KeccakWord64(0x0000FF), KeccakWord64(0x000F00),
        KeccakWord64(0x000F0F), KeccakWord64(0x000FF0), KeccakWord64(0x000FFF), KeccakWord64(0x00F000), KeccakWord64(0x00F00F),
        KeccakWord64(0x00F0F0), KeccakWord64(0x00F0FF), KeccakWord64(0x00FF00), KeccakWord64(0x00FF0F), KeccakWord64(0x00FFF0),
        KeccakWord64(0x00FFFF), KeccakWord64(0x0F0000), KeccakWord64(0x0F000F), KeccakWord64(0x0F00F0), KeccakWord64(0x0F00FF),
        KeccakWord64(0x0F0F00), KeccakWord64(0x0F0F0F), KeccakWord64(0x0F0FF0), KeccakWord64(0x0F0FFF), KeccakWord64(0x0FF000)]);
    a.op_theta();
    //  C:  0F0F00  00F00F  00FFF0  0000FF  00F000
    //  D:  01101E  0EF0E0  00F1F1  011FF0  1E1EFF
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([
        KeccakWord64(0x01101E), KeccakWord64(0x0EF0EF), KeccakWord64(0x00F101), KeccakWord64(0x011F0F), KeccakWord64(0x1E11FF),
        KeccakWord64(0x011F11), KeccakWord64(0x0EFF10), KeccakWord64(0x00FE0E), KeccakWord64(0x01EFF0), KeccakWord64(0x1EEEF0),
        KeccakWord64(0x01E0EE), KeccakWord64(0x0E001F), KeccakWord64(0x000EF1), KeccakWord64(0x01E0FF), KeccakWord64(0x1EE10F),
        KeccakWord64(0x01EFE1), KeccakWord64(0x01F0E0), KeccakWord64(0x0FF1FE), KeccakWord64(0x0E1F00), KeccakWord64(0x111E00),
        KeccakWord64(0x0E1F1E), KeccakWord64(0x01FFEF), KeccakWord64(0x0FFE01), KeccakWord64(0x0E100F), KeccakWord64(0x11EEFF)]));
}
#[test] fn op_rho() {
    let mut a = KeccakStateArray::<KeccakWord64>([
        KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1),
        KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1),
        KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1),
        KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1),
        KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1), KeccakWord64(1)]);
    a.op_rho();
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([
        KeccakWord64(1 << 0), KeccakWord64(1 << 1), KeccakWord64(1 << 62), KeccakWord64(1 << 28), KeccakWord64(1 << 27),
        KeccakWord64(1 << 36), KeccakWord64(1 << 44), KeccakWord64(1 << 6), KeccakWord64(1 << 55), KeccakWord64(1 << 20),
        KeccakWord64(1 << 3), KeccakWord64(1 << 10), KeccakWord64(1 << 43), KeccakWord64(1 << 25), KeccakWord64(1 << 39),
        KeccakWord64(1 << 41), KeccakWord64(1 << 45), KeccakWord64(1 << 15), KeccakWord64(1 << 21), KeccakWord64(1 << 8),
        KeccakWord64(1 << 18), KeccakWord64(1 << 2), KeccakWord64(1 << 61), KeccakWord64(1 << 56), KeccakWord64(1 << 14)]));
}
#[test] fn op_pi() {
    let mut a = KeccakStateArray::<KeccakWord64>([
        KeccakWord64(0), KeccakWord64(1), KeccakWord64(2), KeccakWord64(3), KeccakWord64(4),
        KeccakWord64(5), KeccakWord64(6), KeccakWord64(7), KeccakWord64(8), KeccakWord64(9),
        KeccakWord64(10), KeccakWord64(11), KeccakWord64(12), KeccakWord64(13), KeccakWord64(14),
        KeccakWord64(15), KeccakWord64(16), KeccakWord64(17), KeccakWord64(18), KeccakWord64(19),
        KeccakWord64(20), KeccakWord64(21), KeccakWord64(22), KeccakWord64(23), KeccakWord64(24)]);
    a.op_pi();
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([
        KeccakWord64(0), KeccakWord64(6), KeccakWord64(12), KeccakWord64(18), KeccakWord64(24),
        KeccakWord64(3), KeccakWord64(9), KeccakWord64(10), KeccakWord64(16), KeccakWord64(22),
        KeccakWord64(1), KeccakWord64(7), KeccakWord64(13), KeccakWord64(19), KeccakWord64(20),
        KeccakWord64(4), KeccakWord64(5), KeccakWord64(11), KeccakWord64(17), KeccakWord64(23),
        KeccakWord64(2), KeccakWord64(8), KeccakWord64(14), KeccakWord64(15), KeccakWord64(21)]));
}
#[test] fn op_chi() {
    let mut a = KeccakStateArray::<KeccakWord64>([
        KeccakWord64(0x000000), KeccakWord64(0x00000F), KeccakWord64(0x0000F0), KeccakWord64(0x0000FF), KeccakWord64(0x000F00),
        KeccakWord64(0x000F0F), KeccakWord64(0x000FF0), KeccakWord64(0x000FFF), KeccakWord64(0x00F000), KeccakWord64(0x00F00F),
        KeccakWord64(0x00F0F0), KeccakWord64(0x00F0FF), KeccakWord64(0x00FF00), KeccakWord64(0x00FF0F), KeccakWord64(0x00FFF0),
        KeccakWord64(0x00FFFF), KeccakWord64(0x0F0000), KeccakWord64(0x0F000F), KeccakWord64(0x0F00F0), KeccakWord64(0x0F00FF),
        KeccakWord64(0x0F0F00), KeccakWord64(0x0F0F0F), KeccakWord64(0x0F0FF0), KeccakWord64(0x0F0FFF), KeccakWord64(0x0FF000)]);
    a.op_chi();
    assert_eq!(a, KeccakStateArray::<KeccakWord64>([
        KeccakWord64(0x0000F0), KeccakWord64(0x000000), KeccakWord64(0x000FF0), KeccakWord64(0x0000FF), KeccakWord64(0x000F0F),
        KeccakWord64(0x000F00), KeccakWord64(0x00FFF0), KeccakWord64(0x000FF0), KeccakWord64(0x00FF00), KeccakWord64(0x00F0FF),
        KeccakWord64(0x00FFF0), KeccakWord64(0x00F0F0), KeccakWord64(0x00FFF0), KeccakWord64(0x00FF0F), KeccakWord64(0x00FFFF),
        KeccakWord64(0x00FFF0), KeccakWord64(0x0F00F0), KeccakWord64(0x0F0000), KeccakWord64(0x0FFFF0), KeccakWord64(0x0000FF),
        KeccakWord64(0x0F0FF0), KeccakWord64(0x0F0F00), KeccakWord64(0x0FFFF0), KeccakWord64(0x0F00FF), KeccakWord64(0x0FF00F)]));
}
#[test] fn op_rc_000() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(0), true); } // 10000000
#[test] fn op_rc_001() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(1), false); } // 01000000
#[test] fn op_rc_002() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(2), false); } // 00100000
#[test] fn op_rc_003() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(3), false); } // 00010000
#[test] fn op_rc_004() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(4), false); } // 00001000
#[test] fn op_rc_005() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(5), false); } // 00000100
#[test] fn op_rc_006() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(6), false); } // 00000010
#[test] fn op_rc_007() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(7), false); } // 00000001
#[test] fn op_rc_008() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(8), true); } // 10001110
#[test] fn op_rc_009() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(9), false); } // 01000111
#[test] fn op_rc_010() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(10), true); } // 10101101
#[test] fn op_rc_011() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(11), true); } // 11011000
#[test] fn op_rc_012() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(12), false); } // 01101100
#[test] fn op_rc_013() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(13), false); } // 00110110
#[test] fn op_rc_014() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(14), false); } // 00011011
#[test] fn op_rc_015() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(15), true); } // 10000011
#[test] fn op_rc_016() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(16), true); } // 11001111
#[test] fn op_rc_017() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(17), true); } // 11101001
#[test] fn op_rc_018() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(18), true); } // 11111010
#[test] fn op_rc_019() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(19), false); } // 01111101
#[test] fn op_rc_020() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(20), true); } // 10110000
#[test] fn op_rc_021() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(21), false); } // 01011000
#[test] fn op_rc_022() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(22), false); } // 00101100
#[test] fn op_rc_023() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(23), false); } // 00010110
#[test] fn op_rc_024() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(24), false); } // 00001011
#[test] fn op_rc_025() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(25), true); } // 10001011
#[test] fn op_rc_026() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(26), true); } // 11001011
#[test] fn op_rc_027() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(27), true); } // 11101011
#[test] fn op_rc_028() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(28), true); } // 11111011
#[test] fn op_rc_029() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(29), true); } // 11110011
#[test] fn op_rc_030() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(30), true); } // 11110111
#[test] fn op_rc_031() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(31), true); } // 11110101
#[test] fn op_rc_032() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(32), true); } // 11110100
#[test] fn op_rc_033() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(33), false); } // 01111010
#[test] fn op_rc_034() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(34), false); } // 00111101
#[test] fn op_rc_035() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(35), true); } // 10010000
#[test] fn op_rc_036() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(36), false); } // 01001000
#[test] fn op_rc_037() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(37), false); } // 00100100
#[test] fn op_rc_038() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(38), false); } // 00010010
#[test] fn op_rc_039() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(39), false); } // 00001001
#[test] fn op_rc_040() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(40), true); } // 10001010
#[test] fn op_rc_041() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(41), false); } // 01000101
#[test] fn op_rc_042() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(42), true); } // 10101100
#[test] fn op_rc_043() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(43), false); } // 01010110
#[test] fn op_rc_044() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(44), false); } // 00101011
#[test] fn op_rc_045() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(45), true); } // 10011011
#[test] fn op_rc_046() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(46), true); } // 11000011
#[test] fn op_rc_047() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(47), true); } // 11101111
#[test] fn op_rc_048() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(48), true); } // 11111001
#[test] fn op_rc_049() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(49), true); } // 11110010
#[test] fn op_rc_050() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(50), false); } // 01111001
#[test] fn op_rc_051() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(51), true); } // 10110010
#[test] fn op_rc_052() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(52), false); } // 01011001
#[test] fn op_rc_053() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(53), true); } // 10100010
#[test] fn op_rc_054() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(54), false); } // 01010001
#[test] fn op_rc_055() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(55), true); } // 10100110
#[test] fn op_rc_056() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(56), false); } // 01010011
#[test] fn op_rc_057() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(57), true); } // 10100111
#[test] fn op_rc_058() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(58), true); } // 11011101
#[test] fn op_rc_059() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(59), true); } // 11100000
#[test] fn op_rc_060() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(60), false); } // 01110000
#[test] fn op_rc_061() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(61), false); } // 00111000
#[test] fn op_rc_062() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(62), false); } // 00011100
#[test] fn op_rc_063() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(63), false); } // 00001110
#[test] fn op_rc_064() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(64), false); } // 00000111
#[test] fn op_rc_065() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(65), true); } // 10001101
#[test] fn op_rc_066() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(66), true); } // 11001000
#[test] fn op_rc_067() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(67), false); } // 01100100
#[test] fn op_rc_068() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(68), false); } // 00110010
#[test] fn op_rc_069() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(69), false); } // 00011001
#[test] fn op_rc_070() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(70), true); } // 10000010
#[test] fn op_rc_071() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(71), false); } // 01000001
#[test] fn op_rc_072() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(72), true); } // 10101110
#[test] fn op_rc_073() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(73), false); } // 01010111
#[test] fn op_rc_074() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(74), true); } // 10100101
#[test] fn op_rc_075() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(75), true); } // 11011100
#[test] fn op_rc_076() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(76), false); } // 01101110
#[test] fn op_rc_077() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(77), false); } // 00110111
#[test] fn op_rc_078() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(78), true); } // 10010101
#[test] fn op_rc_079() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(79), true); } // 11000100
#[test] fn op_rc_080() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(80), false); } // 01100010
#[test] fn op_rc_081() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(81), false); } // 00110001
#[test] fn op_rc_082() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(82), true); } // 10010110
#[test] fn op_rc_083() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(83), false); } // 01001011
#[test] fn op_rc_084() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(84), true); } // 10101011
#[test] fn op_rc_085() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(85), true); } // 11011011
#[test] fn op_rc_086() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(86), true); } // 11100011
#[test] fn op_rc_087() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(87), true); } // 11111111
#[test] fn op_rc_088() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(88), true); } // 11110001
#[test] fn op_rc_089() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(89), true); } // 11110110
#[test] fn op_rc_090() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(90), false); } // 01111011
#[test] fn op_rc_091() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(91), true); } // 10110011
#[test] fn op_rc_092() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(92), true); } // 11010111
#[test] fn op_rc_093() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(93), true); } // 11100101
#[test] fn op_rc_094() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(94), true); } // 11111100
#[test] fn op_rc_095() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(95), false); } // 01111110
#[test] fn op_rc_096() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(96), false); } // 00111111
#[test] fn op_rc_097() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(97), true); } // 10010001
#[test] fn op_rc_098() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(98), true); } // 11000110
#[test] fn op_rc_099() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(99), false); } // 01100011
#[test] fn op_rc_100() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(100), true); } // 10111111
#[test] fn op_rc_101() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(101), true); } // 11010001
#[test] fn op_rc_102() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(102), true); } // 11100110
#[test] fn op_rc_103() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(103), false); } // 01110011
#[test] fn op_rc_104() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(104), true); } // 10110111
#[test] fn op_rc_105() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(105), true); } // 11010101
#[test] fn op_rc_106() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(106), true); } // 11100100
#[test] fn op_rc_107() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(107), false); } // 01110010
#[test] fn op_rc_108() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(108), false); } // 00111001
#[test] fn op_rc_109() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(109), true); } // 10010010
#[test] fn op_rc_110() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(110), false); } // 01001001
#[test] fn op_rc_111() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(111), true); } // 10101010
#[test] fn op_rc_112() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(112), false); } // 01010101
#[test] fn op_rc_113() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(113), true); } // 10100100
#[test] fn op_rc_114() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(114), false); } // 01010010
#[test] fn op_rc_115() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(115), false); } // 00101001
#[test] fn op_rc_116() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(116), true); } // 10011010
#[test] fn op_rc_117() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(117), false); } // 01001101
#[test] fn op_rc_118() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(118), true); } // 10101000
#[test] fn op_rc_119() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(119), false); } // 01010100
#[test] fn op_rc_120() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(120), false); } // 00101010
#[test] fn op_rc_121() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(121), false); } // 00010101
#[test] fn op_rc_122() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(122), true); } // 10000100
#[test] fn op_rc_123() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(123), false); } // 01000010
#[test] fn op_rc_124() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(124), false); } // 00100001
#[test] fn op_rc_125() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(125), true); } // 10011110
#[test] fn op_rc_126() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(126), false); } // 01001111
#[test] fn op_rc_127() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(127), true); } // 10101001
#[test] fn op_rc_128() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(128), true); } // 11011010
#[test] fn op_rc_129() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(129), false); } // 01101101
#[test] fn op_rc_130() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(130), true); } // 10111000
#[test] fn op_rc_131() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(131), false); } // 01011100
#[test] fn op_rc_132() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(132), false); } // 00101110
#[test] fn op_rc_133() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(133), false); } // 00010111
#[test] fn op_rc_134() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(134), true); } // 10000101
#[test] fn op_rc_135() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(135), true); } // 11001100
#[test] fn op_rc_136() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(136), false); } // 01100110
#[test] fn op_rc_137() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(137), false); } // 00110011
#[test] fn op_rc_138() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(138), true); } // 10010111
#[test] fn op_rc_139() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(139), true); } // 11000101
#[test] fn op_rc_140() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(140), true); } // 11101100
#[test] fn op_rc_141() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(141), false); } // 01110110
#[test] fn op_rc_142() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(142), false); } // 00111011
#[test] fn op_rc_143() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(143), true); } // 10010011
#[test] fn op_rc_144() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(144), true); } // 11000111
#[test] fn op_rc_145() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(145), true); } // 11101101
#[test] fn op_rc_146() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(146), true); } // 11111000
#[test] fn op_rc_147() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(147), false); } // 01111100
#[test] fn op_rc_148() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(148), false); } // 00111110
#[test] fn op_rc_149() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(149), false); } // 00011111
#[test] fn op_rc_150() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(150), true); } // 10000001
#[test] fn op_rc_151() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(151), true); } // 11001110
#[test] fn op_rc_152() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(152), false); } // 01100111
#[test] fn op_rc_153() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(153), true); } // 10111101
#[test] fn op_rc_154() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(154), true); } // 11010000
#[test] fn op_rc_155() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(155), false); } // 01101000
#[test] fn op_rc_156() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(156), false); } // 00110100
#[test] fn op_rc_157() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(157), false); } // 00011010
#[test] fn op_rc_158() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(158), false); } // 00001101
#[test] fn op_rc_159() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(159), true); } // 10001000
#[test] fn op_rc_160() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(160), false); } // 01000100
#[test] fn op_rc_161() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(161), false); } // 00100010
#[test] fn op_rc_162() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(162), false); } // 00010001
#[test] fn op_rc_163() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(163), true); } // 10000110
#[test] fn op_rc_164() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(164), false); } // 01000011
#[test] fn op_rc_165() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(165), true); } // 10101111
#[test] fn op_rc_166() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(166), true); } // 11011001
#[test] fn op_rc_167() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(167), true); } // 11100010
#[test] fn op_rc_168() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(168), false); } // 01110001
#[test] fn op_rc_169() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(169), true); } // 10110110
#[test] fn op_rc_170() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(170), false); } // 01011011
#[test] fn op_rc_171() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(171), true); } // 10100011
#[test] fn op_rc_172() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(172), true); } // 11011111
#[test] fn op_rc_173() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(173), true); } // 11100001
#[test] fn op_rc_174() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(174), true); } // 11111110
#[test] fn op_rc_175() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(175), false); } // 01111111
#[test] fn op_rc_176() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(176), true); } // 10110001
#[test] fn op_rc_177() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(177), true); } // 11010110
#[test] fn op_rc_178() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(178), false); } // 01101011
#[test] fn op_rc_179() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(179), true); } // 10111011
#[test] fn op_rc_180() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(180), true); } // 11010011
#[test] fn op_rc_181() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(181), true); } // 11100111
#[test] fn op_rc_182() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(182), true); } // 11111101
#[test] fn op_rc_183() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(183), true); } // 11110000
#[test] fn op_rc_184() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(184), false); } // 01111000
#[test] fn op_rc_185() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(185), false); } // 00111100
#[test] fn op_rc_186() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(186), false); } // 00011110
#[test] fn op_rc_187() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(187), false); } // 00001111
#[test] fn op_rc_188() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(188), true); } // 10001001
#[test] fn op_rc_189() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(189), true); } // 11001010
#[test] fn op_rc_190() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(190), false); } // 01100101
#[test] fn op_rc_191() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(191), true); } // 10111100
#[test] fn op_rc_192() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(192), false); } // 01011110
#[test] fn op_rc_193() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(193), false); } // 00101111
#[test] fn op_rc_194() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(194), true); } // 10011001
#[test] fn op_rc_195() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(195), true); } // 11000010
#[test] fn op_rc_196() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(196), false); } // 01100001
#[test] fn op_rc_197() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(197), true); } // 10111110
#[test] fn op_rc_198() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(198), false); } // 01011111
#[test] fn op_rc_199() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(199), true); } // 10100001
#[test] fn op_rc_200() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(200), true); } // 11011110
#[test] fn op_rc_201() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(201), false); } // 01101111
#[test] fn op_rc_202() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(202), true); } // 10111001
#[test] fn op_rc_203() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(203), true); } // 11010010
#[test] fn op_rc_204() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(204), false); } // 01101001
#[test] fn op_rc_205() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(205), true); } // 10111010
#[test] fn op_rc_206() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(206), false); } // 01011101
#[test] fn op_rc_207() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(207), true); } // 10100000
#[test] fn op_rc_208() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(208), false); } // 01010000
#[test] fn op_rc_209() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(209), false); } // 00101000
#[test] fn op_rc_210() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(210), false); } // 00010100
#[test] fn op_rc_211() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(211), false); } // 00001010
#[test] fn op_rc_212() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(212), false); } // 00000101
#[test] fn op_rc_213() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(213), true); } // 10001100
#[test] fn op_rc_214() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(214), false); } // 01000110
#[test] fn op_rc_215() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(215), false); } // 00100011
#[test] fn op_rc_216() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(216), true); } // 10011111
#[test] fn op_rc_217() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(217), true); } // 11000001
#[test] fn op_rc_218() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(218), true); } // 11101110
#[test] fn op_rc_219() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(219), false); } // 01110111
#[test] fn op_rc_220() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(220), true); } // 10110101
#[test] fn op_rc_221() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(221), true); } // 11010100
#[test] fn op_rc_222() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(222), false); } // 01101010
#[test] fn op_rc_223() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(223), false); } // 00110101
#[test] fn op_rc_224() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(224), true); } // 10010100
#[test] fn op_rc_225() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(225), false); } // 01001010
#[test] fn op_rc_226() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(226), false); } // 00100101
#[test] fn op_rc_227() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(227), true); } // 10011100
#[test] fn op_rc_228() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(228), false); } // 01001110
#[test] fn op_rc_229() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(229), false); } // 00100111
#[test] fn op_rc_230() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(230), true); } // 10011101
#[test] fn op_rc_231() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(231), true); } // 11000000
#[test] fn op_rc_232() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(232), false); } // 01100000
#[test] fn op_rc_233() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(233), false); } // 00110000
#[test] fn op_rc_234() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(234), false); } // 00011000
#[test] fn op_rc_235() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(235), false); } // 00001100
#[test] fn op_rc_236() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(236), false); } // 00000110
#[test] fn op_rc_237() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(237), false); } // 00000011
#[test] fn op_rc_238() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(238), true); } // 10001111
#[test] fn op_rc_239() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(239), true); } // 11001001
#[test] fn op_rc_240() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(240), true); } // 11101010
#[test] fn op_rc_241() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(241), false); } // 01110101
#[test] fn op_rc_242() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(242), true); } // 10110100
#[test] fn op_rc_243() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(243), false); } // 01011010
#[test] fn op_rc_244() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(244), false); } // 00101101
#[test] fn op_rc_245() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(245), true); } // 10011000
#[test] fn op_rc_246() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(246), false); } // 01001100
#[test] fn op_rc_247() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(247), false); } // 00100110
#[test] fn op_rc_248() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(248), false); } // 00010011
#[test] fn op_rc_249() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(249), true); } // 10000111
#[test] fn op_rc_250() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(250), true); } // 11001101
#[test] fn op_rc_251() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(251), true); } // 11101000
#[test] fn op_rc_252() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(252), false); } // 01110100
#[test] fn op_rc_253() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(253), false); } // 00111010
#[test] fn op_rc_254() { assert_eq!(KeccakStateArray::<KeccakWord64>::op_rc(254), false); } // 00011101
#[test] fn op_iota_0() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(0);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x0000000000000001));
}
#[test] fn op_iota_1() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(1);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x0000000000008082));
}
#[test] fn op_iota_2() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(2);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x800000000000808A));
}
#[test] fn op_iota_3() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(3);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000080008000));
}
#[test] fn op_iota_4() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(4);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x000000000000808B));
}
#[test] fn op_iota_5() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(5);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x0000000080000001));
}
#[test] fn op_iota_6() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(6);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000080008081));
}
#[test] fn op_iota_7() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(7);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000000008009));
}
#[test] fn op_iota_8() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(8);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x000000000000008A));
}
#[test] fn op_iota_9() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(9);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x0000000000000088));
}
#[test] fn op_iota_10() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(10);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x0000000080008009));
}
#[test] fn op_iota_11() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(11);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x000000008000000A));
}
#[test] fn op_iota_12() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(12);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x000000008000808B));
}
#[test] fn op_iota_13() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(13);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x800000000000008B));
}
#[test] fn op_iota_14() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(14);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000000008089));
}
#[test] fn op_iota_15() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(15);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000000008003));
}
#[test] fn op_iota_16() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(16);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000000008002));
}
#[test] fn op_iota_17() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(17);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000000000080));
}
#[test] fn op_iota_18() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(18);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x000000000000800A));
}
#[test] fn op_iota_19() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(19);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x800000008000000A));
}
#[test] fn op_iota_20() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(20);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000080008081));
}
#[test] fn op_iota_21() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(21);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000000008080));
}
#[test] fn op_iota_22() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(22);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x0000000080000001));
}
#[test] fn op_iota_23() {
    let mut a = KeccakStateArray::<KeccakWord64>::default();
    a.op_iota(23);
    assert_eq!(*a.lane(0, 0), KeccakWord64(0x8000000080008008));
}
