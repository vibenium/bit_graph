mod itb;
// mod bg8;
mod bg;
use crate::bg::bit_graph::*;
// use crate::bg8::bit_graph8::*;
// use rand::Rng;
use crate::BitGraph;

#[cfg(test)]
mod tests {
    use super::*;
    struct NoData;
    const BITS: usize = std::mem::size_of::<usize>() * 8;

    #[test]
    fn simple_add_verts2() {
        let mut my_bg1: BitGraph<NoData> = BitGraph::new(EdgeScale::BINARY);
        let mut my_bg2: BitGraph<NoData> = BitGraph::new(EdgeScale::U4);
        let mut my_bg3: BitGraph<NoData> = BitGraph::new(EdgeScale::U8);
        let mut my_bg4: BitGraph<NoData> = BitGraph::new(EdgeScale::U16);
        let mut my_bg5: BitGraph<NoData> = BitGraph::new(EdgeScale::U32);
        
        assert_eq!(0, my_bg1.size());
        assert_eq!(0, my_bg2.size());
        assert_eq!(0, my_bg3.size());
        assert_eq!(0, my_bg4.size());
        assert_eq!(0, my_bg5.size());

        for _ in 0..32 {
            my_bg1.add(NoData);
            my_bg2.add(NoData);
            my_bg3.add(NoData);
            my_bg4.add(NoData);
            my_bg5.add(NoData);
        }

        assert_eq!(32, my_bg1.size());
        assert_eq!(32, my_bg2.size());
        assert_eq!(32, my_bg3.size());
        assert_eq!(32, my_bg4.size());
        assert_eq!(32, my_bg5.size());

        for x in 0..32 {
            assert_eq!(32 / (BITS / 2), my_bg1.ev_len_at(x)); 
            assert_eq!(32 / (BITS / 4), my_bg2.ev_len_at(x)); 
            // assert_eq!(???, my_bg3.ev_len_at(x));
            // assert_eq!(???, my_bg4.ev_len_at(x));
            // assert_eq!(???, my_bg5.ev_len_at(x));
        }
    }

    #[test]
    fn simple_add_verts1() {
        let mut my_bg1: BitGraph<NoData> = BitGraph::new(EdgeScale::SAME);
        assert_eq!(0, my_bg1.size());
        for _ in 0..200 { my_bg1.add(NoData); }
        assert_eq!(200, my_bg1.size());

        // verifying the amount of edgeverts within each vertex
        // '+ 1' since there is always at least 1 edgevert per vertex
        for x in 0..200 { assert_eq!(200 / BITS + 1, my_bg1.ev_len_at(x)); }
    }

    #[test]
    fn new_bitgraphs() {
        let my_bg1: BitGraph<NoData> = BitGraph::new(EdgeScale::SAME);
        let my_bg2: BitGraph<String> = BitGraph::new(EdgeScale::U8);
        let my_bg3: BitGraph<i8> = BitGraph::new(EdgeScale::U8);
        let my_bg4: BitGraph<Option<i32>> = BitGraph::new_with_capacity(EdgeScale::SAME, 20);
        let my_bg5: BitGraph<Vec::<i32>> = BitGraph::new_with_capacity(EdgeScale::U32, 100);
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
