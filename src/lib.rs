mod itb;
mod bg8;
use crate::bg8::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bg8() {
        let my_bg8: BitGraph8 = BitGraph8::new();
        assert_eq!(0, my_bg8.size());
    }

    #[test] // gaurentees the correct size of each 
    fn edgevert_test() {
        let mut my_bg8: BitGraph8 = BitGraph8::new();
        for i in 0..256 {
            assert_eq!(i, my_bg8.size());
            my_bg8.addv();
            let _v: Vertex8 = my_bg8.getv8(i);
            assert_eq!
            (
                i as usize / 8 + 1,                
                _v.get_ev_size()
            );
        }
        // Post addition of max elements
        // all should have the same amount
        for i in 0..256 {
            let _v: Vertex8 = my_bg8.getv8(i);
            assert_eq!(32, _v.get_ev_size());
        }
    }

    #[test] // Tests for adding/getting up to 255 elements
    fn large_addv_getv_bg8() {
        let mut my_bg8: BitGraph8 = BitGraph8::new();
        
        // Testing addv()
        for i in 0..256 {
            assert_eq!(i, my_bg8.size());
            my_bg8.addv();
        }
        assert_eq!(256, my_bg8.size()); // last of [0..256]

        // Testing getv()
        for i in 0..256 {
            assert_eq!(i as u8, my_bg8.getv(i));
        }
    }

    #[test] // Only tests for adding/getting up to 8 elements
    fn small_addv_getv_bg8() {
        let mut my_bg8: BitGraph8 = BitGraph8::new();

        // Testing addv()
        for i in 0..8 {
            assert_eq!(i, my_bg8.size());
            my_bg8.addv();
        }
        assert_eq!(8, my_bg8.size()); // last of [0..8]

        // Testing getv()
        for i in 0..8 {
            assert_eq!(i as u8, my_bg8.getv(i));
        }
    }

    #[test]
    fn test_itb8() {
        let str1 = itb::int_to_bit8(&0);
        assert_eq!("00000000", str1);
        let str2 = itb::int_to_bit8(&0xff); // 255
        assert_eq!("11111111", str2);
        let str3 = itb::int_to_bit8(&127);
        assert_eq!("01111111", str3);
        let str4 = itb::int_to_bit8(&170);
        assert_eq!("10101010", str4);
    }
    #[test]
    fn test_itb16() {
        let str1 = itb::int_to_bit16(&0);
        assert_eq!("0000000000000000", str1);
        let str2 = itb::int_to_bit16(&0xffff);
        assert_eq!("1111111111111111", str2);
    }
    #[test]
    fn test_itb32() {
        let str1 = itb::int_to_bit32(&0);
        assert_eq!("00000000000000000000000000000000", str1);
        let str2 = itb::int_to_bit32(&0xffffffff);
        assert_eq!("11111111111111111111111111111111", str2);
    }
    #[test]
    fn test_itb64() {
        let str1 = itb::int_to_bit64(&0);
        assert_eq!
        (
            "0000000000000000000000000000000000000000000000000000000000000000",
            str1
        );
        let str2 = itb::int_to_bit64(&0xffffffffffffffff);
        assert_eq!
        (
            "1111111111111111111111111111111111111111111111111111111111111111",
            str2
        );
    }
}
