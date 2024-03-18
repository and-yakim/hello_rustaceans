#[macro_export]
macro_rules! get_simd {
    ($arr:ident, $i:expr, $j:expr, $i_dec:expr, $i_inc:expr) => {
        get_triple_simd(vec![
            $arr[$i_dec][$j] as u8,
            $arr[$i][$j] as u8,
            $arr[$i + 1][$j] as u8,
            $arr[$i + 2][$j] as u8,
            $arr[$i + 3][$j] as u8,
            $arr[$i + 4][$j] as u8,
            $arr[$i + 5][$j] as u8,
            $arr[$i + 6][$j] as u8,
            $arr[$i + 7][$j] as u8,
            $arr[$i + 8][$j] as u8,
            $arr[$i + 9][$j] as u8,
            $arr[$i + 10][$j] as u8,
            $arr[$i + 11][$j] as u8,
            $arr[$i + 12][$j] as u8,
            $arr[$i + 13][$j] as u8,
            $arr[$i + 14][$j] as u8,
            $arr[$i + 15][$j] as u8,
            $arr[$i + 16][$j] as u8,
            $arr[$i + 17][$j] as u8,
            $arr[$i + 18][$j] as u8,
            $arr[$i + 19][$j] as u8,
            $arr[$i + 20][$j] as u8,
            $arr[$i + 21][$j] as u8,
            $arr[$i + 22][$j] as u8,
            $arr[$i + 23][$j] as u8,
            $arr[$i + 24][$j] as u8,
            $arr[$i + 25][$j] as u8,
            $arr[$i + 26][$j] as u8,
            $arr[$i + 27][$j] as u8,
            $arr[$i + 28][$j] as u8,
            $arr[$i + 29][$j] as u8,
            $arr[$i + 30][$j] as u8,
            $arr[$i + 31][$j] as u8,
            $arr[$i + 32][$j] as u8,
            $arr[$i + 33][$j] as u8,
            $arr[$i + 34][$j] as u8,
            $arr[$i + 35][$j] as u8,
            $arr[$i + 36][$j] as u8,
            $arr[$i + 37][$j] as u8,
            $arr[$i + 38][$j] as u8,
            $arr[$i + 39][$j] as u8,
            $arr[$i + 40][$j] as u8,
            $arr[$i + 41][$j] as u8,
            $arr[$i + 42][$j] as u8,
            $arr[$i + 43][$j] as u8,
            $arr[$i + 44][$j] as u8,
            $arr[$i + 45][$j] as u8,
            $arr[$i + 46][$j] as u8,
            $arr[$i + 47][$j] as u8,
            $arr[$i + 48][$j] as u8,
            $arr[$i + 49][$j] as u8,
            $arr[$i + 50][$j] as u8,
            $arr[$i + 51][$j] as u8,
            $arr[$i + 52][$j] as u8,
            $arr[$i + 53][$j] as u8,
            $arr[$i + 54][$j] as u8,
            $arr[$i + 55][$j] as u8,
            $arr[$i + 56][$j] as u8,
            $arr[$i + 57][$j] as u8,
            $arr[$i + 58][$j] as u8,
            $arr[$i + 59][$j] as u8,
            $arr[$i + 60][$j] as u8,
            $arr[$i + 61][$j] as u8,
            $arr[$i + 62][$j] as u8,
            $arr[$i + 63][$j] as u8,
            $arr[$i_inc][$j] as u8,
        ])
    };
}
