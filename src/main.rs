#[derive(Debug, Clone)]
struct PrimeField {
    char: u16,
    num: i64,
}

impl PrimeField {
    fn new(char: u16, num: i64) -> PrimeField {
        let new_num = num % char as i64;
        PrimeField { char, num: new_num }
    }
    fn add(&self, other: &PrimeField) -> PrimeField {
        PrimeField {
            char: self.char,
            num: (self.num + other.num) % self.char as i64,
        }
    }
    fn sub(&self, other: &PrimeField) -> PrimeField {
        let mut minus_num = (self.num - other.num) % self.char as i64;
        if minus_num < 0 {
            minus_num += self.char as i64;
        }
        PrimeField {
            char: self.char,
            num: minus_num,
        }
    }
    fn mul(&self, other: &PrimeField) -> PrimeField {
        let mut mul_num = (self.num * other.num) % self.char as i64;
        if mul_num < 0 {
            mul_num += self.char as i64;
        }

        PrimeField {
            char: self.char,
            num: mul_num,
        }
    }
    fn pow(&self, other: i64) -> PrimeField {
        PrimeField {
            char: self.char,
            num: (self.num.pow(other as u32)) % self.char as i64,
        }
    }
    fn div(&self, other: &PrimeField) -> PrimeField {
        let mut t = self.extended_euclidean(self.char as i64, other.num);
        if t < 0 {
            let mut i = 1;

            while (t + i * self.char as i64) < 0 {
                i += 1;
            }
            t = (t + i * self.char as i64) % self.char as i64;
        }
        PrimeField {
            char: self.char,
            num: (self.num * t) % self.char as i64,
        }
    }
    // ユークリッドの互除法
    fn extended_euclidean(&self, u: i64, v: i64) -> i64 {
        let mut r0 = u;
        let mut r1 = v;
        let mut s0 = 1;
        let mut s1 = 0;
        let mut t0 = 0;
        let mut t1 = 1;
        while r1 != 0 {
            let q = r0 / r1;
            let r = r0 - q * r1;
            let s = s0 - q * s1;
            let t = t0 - q * t1;
            r0 = r1;
            s0 = s1;
            t0 = t1;
            r1 = r;
            s1 = s;
            t1 = t;
        }
        if t0 < 0 {
            t0 + u
        } else {
            t0
        }
    }
}

// fn pow_e_mod_f(g:Vec<PrimeField>, f:Vec<PrimeField>, e:u32, char:u32) -> Vec<PrimeField> {
// 	let mut result = vec![PrimeField::new(char, 0); g.len()*e];
// 	for i in 0..g.len() {
// 		result[i] = g[i].pow(e);
// 	}
// 	result
// }

// calc f(x) * g(x)
fn function_multiple(f: &Vec<PrimeField>, g: &Vec<PrimeField>) -> Vec<PrimeField> {
    let mut result = vec![PrimeField::new(f[0].char, 0); f.len() + g.len() - 1];
    for i in 0..f.len() {
        for j in 0..g.len() {
            result[i + j] = result[i + j].add(&f[i].mul(&g[j]));
        }
    }
    result
}

// calc f(x) div g(x)
fn function_div(f: Vec<PrimeField>, g: Vec<PrimeField>) -> (Vec<PrimeField>, Vec<PrimeField>) {
    let mut quotient: Vec<PrimeField> = Vec::new();
    let mut f_inv: Vec<PrimeField> = f.into_iter().rev().collect();
    let mut g_inv: Vec<PrimeField> = g.into_iter().rev().collect();

    // drop zero
    for i in 0..f_inv.len() {
        if f_inv[0].num != 0 {
            break;
        } else {
            f_inv.remove(0);
        }
    }

    for i in 0..g_inv.len() {
        if g_inv[0].num != 0 {
            break;
        } else {
            g_inv.remove(0);
        }
    }

    if f_inv.len() >= g_inv.len() {
        for i in 0..f_inv.len() - g_inv.len() + 1 {
            let mut temp = f_inv[i].div(&g_inv[0]);
            for j in 0..g_inv.len() {
                f_inv[i + j] = f_inv[i + j].sub(&g_inv[j].mul(&temp));
            }
            quotient.push(temp);
        }
    } else {
        quotient.push(PrimeField::new(g_inv[0].char, 0));
    }

    // drop zero
    for i in 0..f_inv.len() {
        if f_inv[0].num != 0 {
            break;
        } else {
            f_inv.remove(0);
        }
    }

    quotient = quotient.into_iter().rev().collect::<Vec<PrimeField>>();
    let mut remainder: Vec<PrimeField>;
    if f_inv.len() == 0 {
        remainder = vec![PrimeField::new(g_inv[0].char, 0)];
    } else {
        remainder = f_inv.into_iter().rev().collect::<Vec<PrimeField>>();
    }
    (quotient, remainder)
}
fn change_base_from10_to_n(x: u32, n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut x = x;
    while x > 0 {
        result.push(x % n);
        x = x / n;
    }
    result
}

fn gcd(
    devided_polynomial: Vec<PrimeField>,
    deviding_polynomial: Vec<PrimeField>,
) -> Vec<PrimeField> {
    // def(devided_polynomial ) > def(deviding_polynomial)
    let mut f: Vec<PrimeField> = devided_polynomial;
    let mut g: Vec<PrimeField> = deviding_polynomial;
	let mut answer : Vec<PrimeField> = vec![];

	if f.len() < g.len() {
		(f, g) = (g, f);
	}

    loop {
        g = adjust_function(g);
        // If the number to be divided is 1, end
        if g.len() == 1 && g[0].num == 1 {
			answer = g;
			break;
        } else if g.len() == 1 && g[0].num == 0 {
            answer =  vec![PrimeField::new(f[0].char, 0)];
			break;
        }
        // if f.len() == 1 && f[0].num == 0 {
        // 	return vec![PrimeField::new(f[0].char, 1)];
        // }
        let remainder = function_div(f, g.clone()).1;
        (f, g) = (g, remainder);


        // モニック多項式にする
        for i in 0..f.len() {
            f[i] = f[i].div(&f[f.len() - 1]);
        }
        for i in 0..g.len() {
            g[i] = g[i].div(&g[g.len() - 1]);
        }

    }

    answer
}

fn adjust_function(func: Vec<PrimeField>) -> Vec<PrimeField> {
    let mut func_inv = func.clone().into_iter().rev().collect::<Vec<PrimeField>>();
    // drop zero
    for i in 0..func_inv.len() {
        if func_inv[0].num != 0 {
            break;
        } else {
            func_inv.remove(0);
        }
    }
    let mut f = func_inv.into_iter().rev().collect::<Vec<PrimeField>>();
    if f.len() == 0 {
        f.push(PrimeField::new(func[0].char, 0));
    }
    f
}
fn main() {
    let char = 5;
    let n = 5;
    // let f = vec![PrimeField::new(char as u16, 1), PrimeField::new(char as u16, 1), PrimeField::new(char as u16, 0), PrimeField::new(char as u16, 1)];
    // let g = vec![PrimeField::new(char as u16, 0), PrimeField::new(char as u16, 1), PrimeField::new(char as u16, 1)];
    // let f = vec![PrimeField::new(char as u16, 1), PrimeField::new(char as u16, 1)];
    // let g = vec![PrimeField::new(char as u16, 1)];

    // let (quotient, remainder) = function_div(f.clone(), g.clone());
    // println!("f= {:?}",f);
    // println!("g= {:?}",g);
    // println!("quotient = {:?}", quotient);
    // println!("remainder = {:?}", remainder);

    for i in 2..((char as u32).pow(n as u32) ) {
        println!("i = {}", i);
        // for i in 1..((char as u32).pow(n as u32)) {
        // f :nth order monic polynomial on F_p
        let mut f = change_base_from10_to_n(i.into(), char.into());
        for j in 0..n - f.len() + 1 {
            f.push(0);
        }
        f.pop();
        f.push(1);
        let f: Vec<PrimeField> = f
            .into_iter()
            .map(|x| PrimeField::new(char, x as i64))
            .collect();

        let f_num: Vec<i64> = f.clone().into_iter().map(|x| x.num).collect();
        println!("f = {:?}", f_num);

        let mut end_flag: bool = true;

        // g = x
        let mut g: Vec<PrimeField> = vec![
            PrimeField::new(char as u16, 0),
            PrimeField::new(char as u16, 1),
        ];

        for m in 0..(n / 2) {
            let g_temp: Vec<PrimeField> = g.clone();

            // g^p
            for j in 0..char - 1 {
                g = function_multiple(&g, &g_temp);
            }
            // println!("g^p = {:?}", g);

            // g = g^p mod f
            g = function_div(g, f.clone()).1;

            // println!("g^p mod f = {:?}", g);
            //let mut g = pow_e_mod_f(g.clone(), f.clone(), char as u32, char);

            let mut g_2 = g.clone();
            // g^p-1 mod f
            if g_2.len() == 0 {
                g_2.push(PrimeField::new(char as u16, 0));
                g_2.push(PrimeField::new(char as u16, (char - 1).into()));
            } else if g_2.len() == 1 {
                g_2.push(PrimeField::new(char as u16, (char - 1).into()));
            } else {
                g_2[1] = g_2[1].sub(&PrimeField::new(char, 1));
            }
            g_2 = adjust_function(g_2);
            // println!("g^p-1 mod f = {:?}", g_2);
            let mut h = gcd(f.clone(), g_2);

            h = adjust_function(h);
            println!("h = {:?}", h);
            if !(h.len() <= 1 && h[0].num == 1) {
                end_flag = false;
                break;
            }
        }

        if end_flag == true {
            println!("answer = {:?}", f);
            break;
        }
    }
}
