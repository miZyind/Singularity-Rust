pub enum Function {
    Linear(f32),
    QuadraticIn(f32),
    QuadraticOut(f32),
    QuadraticInOut(f32),
    CubicIn(f32),
    CubicOut(f32),
    CubicInOut(f32),
}
impl Function {
    pub fn apply(function: Function) -> f32 {
        match function {
            Function::Linear(t) => t,
            Function::QuadraticIn(t) => t * t,
            Function::QuadraticOut(t) => -t * (t - 2.0),
            Function::QuadraticInOut(t) => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    (-2.0 * t * t) + (4.0 * t) - 1.0
                }
            }
            Function::CubicIn(t) => t * t * t,
            Function::CubicOut(t) => {
                let f = t - 1.0;
                f * f * f + 1.0
            }
            Function::CubicInOut(t) => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let f = (2.0 * t) - 2.0;
                    0.5 * f * f * f + 1.0
                }
            }
        }
    }
}
