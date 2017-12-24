use std::io::stdin;

/// https://math.stackexchange.com/a/163101
/// 
/// function spiral(n)
///         k=ceil((sqrt(n)-1)/2)
///         t=2*k+1
///         m=t^2 
///         t=t-1
///         if n>=m-t then return k-(m-n),-k        else m=m-t end
///         if n>=m-t then return -k,-k+(m-n)       else m=m-t end
///         if n>=m-t then return -k+(m-n),k else return k,k-(m-n-t) end
/// end
fn spiral(n: i32) -> (i32, i32) {
    let k = (((n as f32).sqrt() - 1f32) / 2f32).ceil() as i32;
    let t = 2*k+1;
    let mut m = t*t;
    let t = t-1;
    if n>=(m-t) {
        return (k-(m-n), -k)
    } else {
        m = m-t;
    }
    if n>=(m-t) {
        return (-k, -k+(m-n));
    } else {
        m = m-t;
    }
    return if n>=(m-t) {
        (-k+(m-n), k)
    } else {
        (k, k-(m-n-t))
    }
}

fn main() {
    let mut string = String::new();
    stdin().read_line(&mut string).unwrap();
    let val: i32 = string.trim().parse().unwrap();
    let coords = spiral(val);
    println!("{}", coords.0.abs() + coords.1.abs());
}