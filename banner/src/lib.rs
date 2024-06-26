use std::collections::HashMap;

use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {

            short_hand:  format!("-{}", &l_h[..1]),
            long_hand: format!("--{}", &l_h),
            desc: d.to_string()
        }
    }
}
impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        let f = self.flags.get(&flag).unwrap();
        f(argv[0],argv[1]).unwrap_or("invalid float literal".to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x = a.parse::<f32>();
    let y = b.parse::<f32>();
    match (x,y) {
        (Err(e1), Err(_)) => Err(e1),
        (Err(e1), Ok(_)) => Err(e1),
        (Ok(_), Err(e2)) => Err(e2),
        (Ok(n1), Ok(n2)) => Ok((n1/n2).to_string())
    }
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x = a.parse::<f32>();
    let y = b.parse::<f32>();
    match (x,y) {
        (Err(e1), Err(_)) => Err(e1),
        (Err(e1), Ok(_)) => Err(e1),
        (Ok(_), Err(e2)) => Err(e2),
        (Ok(n1), Ok(n2)) => Ok((n1%n2).to_string())
    }
}
