fn main() {
    let args: Vec<String> = std::env::args().collect();
    if valid_str(&args[1]) == None {
        println!("Error1");
        return;
    }
    rpn(&args[1]);
}

fn valid_str(v: &String) -> Option<&String> {
    if v.chars().filter(|x| !(x.is_numeric() ||
        *x == '+' ||
        *x == '-' ||
        *x == '*' ||
        *x == '/' ||
        *x == '%' ||
        x.is_ascii_whitespace())).count() != 0 {
        return None
    }
    Some(v)
}

fn operate(stack: &mut Vec<i64>, op: &str) -> Result<i64, String> {
    let mut y = stack.pop().unwrap();
    let x = stack.pop().unwrap();
    let mut res: i64 = 0;
    let mut err = false;
    if op == "+" {
        (res, err) = x.overflowing_add(y);
    } else if op == "-" {
        (y, err) = y.overflowing_neg();
        if err {
            return Err("Error".to_string());
        }
        (res, err) = x.overflowing_add(y);
    } else if op == "*" {
        (res, err) = x.overflowing_mul(y);
    } else if op == "/" {
        if y == 0 {
            return Err("Error".to_string());
        }
        (res, err) = x.overflowing_div(y);
    } else if op == "%" {
        (res, err) = x.overflowing_rem(y);
    }
    if err {
        return Err("Error".to_string());
    }
    return Ok(res)
}

fn rpn(v: &String) {
    let mut stack: Vec<i64> =vec![];
    for elem in v.split_whitespace() {
        if !elem.parse::<i64>().is_ok() {
            if stack.len() < 2 {
                println!("Error");
                return;
            }
            let res = operate(&mut stack, elem);
            if !res.is_ok() {
                println!("Error");
                return;
            } else {
                stack.push(res.unwrap());
            }
        } else {
            stack.push(elem.parse::<i64>().unwrap())
        }
    }
    if stack.len() == 1 {
        println!("{}", stack.pop().unwrap());
        return;
    }
    println!("Error");
}