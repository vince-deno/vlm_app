/// Core operations for VLM spans
pub trait VLMSpanCore<T> {
    fn new(lo: T, hi: T) -> Self;
    fn range(&self) -> std::ops::Range<T>;
    fn len(&self) -> T;
    fn is_empty(&self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
    fn intersection(&self, other: &Self) -> Option<Self> where Self: Sized;
    fn union(&self, other: &Self) -> Self;
    fn split_at(&self, position: T) -> (Self, Self) where Self: Sized;
}

pub trait VLMSpanUtils<T> {
    fn to_string(&self) -> String;
    fn contains_p(&self, position: T) -> bool;
    fn update(&mut self, vlmspan: std::ops::Range<T>) -> bool;
}

// Define the VLmTextCon trait
pub trait VLMSpans{
    const V: i32 = Self::V;
    const L: i32 = Self::L;
    const M: i32 = Self::M;
    const VLM: i64 = Self::VLM;

    fn vlm_ascii() -> String {
        vec![
            char::from_u32(Self::V as u32).unwrap_or('?'),
            char::from_u32(Self::L as u32).unwrap_or('?'),
            char::from_u32(Self::M as u32).unwrap_or('?'),
        ]
        .into_iter()
        .collect()
    }

    // Define the mathematical function f(x) = x^2 + sin(x) + ln(x) + 97
    fn f(x: f64) -> f64 {
        x.powi(2) + x.sin() + x.ln() + 97.0
    }

    // Calculate the approximate integral using the trapezoidal rule
    fn integral_approximation(lower: f64, upper: f64, steps: u64) -> f64 {
        let step_size = (upper - lower) / steps as f64;
        let mut integral = 0.0;
        for i in 0..steps {
            let x1 = lower + i as f64 * step_size;
            let x2 = lower + (i + 1) as f64 * step_size;
            integral += (Self::f(x1) + Self::f(x2)) / 2.0 * step_size;
        }
        integral
    }

    fn to_hex(&self) -> String;
    fn to_binary(&self) -> String;
    fn to_ascii(&self) -> String;
    fn print(&self);
}
