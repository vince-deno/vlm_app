pub trait VLMDefined {
    const V: i32;
    const L: i32;
    const M: i32;
    const VLM: i64;

    fn vlm_ascii() -> String {
        vec![
            char::from_u32(Self::V as u32).unwrap_or('?'),
            char::from_u32(Self::L as u32).unwrap_or('?'),
            char::from_u32(Self::M as u32).unwrap_or('?'),
        ]
        .into_iter()
        .collect()
    }

    fn return_ascii(v: i32, l: i32, m: i32) -> [u32; 3];
    fn ascii_to_string(v: i32, l: i32, m: i32) -> Vec<String>;
    fn ascii_to_hex(vlm: i64) -> isize;
    fn hex_to_ascii(vlm: i64) -> isize;
    fn hybrid_ascii_hex(m: i64);
    fn get_alias() -> String;

        // … other associated constants and methods …
        fn vlm_h_hex_asccii_simd<X>(vlm: X) -> Option<X>
        where
            X: 'static + std::any::Any + Clone + From<[i32; 4]>;
}