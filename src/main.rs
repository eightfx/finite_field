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
	}
	else{
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
	let mut remainder:Vec<PrimeField>;
	if f_inv.len()== 0 {
		remainder = vec![PrimeField::new(g_inv[0].char, 0)];
	}else{

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


	loop {
		(f, g) = function_div(f, g);

		if g.len() == 1 && g[0].num == 1 {
			return g;
			
		}
		let mut temp = 0;
		for i in 0..g.len() {
			temp += g[i].num as u32;
		}
		if temp == 0 {
			break;
		}
	}

    f
}
fn pow_e_mod_f(g: Vec<PrimeField>, f: Vec<PrimeField>, e: u32, char: u16) -> Vec<PrimeField> {
	let mut h_:Vec<PrimeField> = vec![PrimeField::new(char as u16, 1)];
	let mut g_ = g;
	let mut e_ = e;
	while e_ > 0{
		
		if e_ % 2 == 1 {
			h_ = function_div(function_multiple(&h_, &g_), f.clone()).1;
		}
		g_ = function_div(function_multiple(&g_, &g_), f.clone()).1;
		e_ = e_ / 2;
		
	}

	h_


}

fn main() {
    let char = 2;
    let n = 8;
	// let f = vec![PrimeField::new(char as u16, 1), PrimeField::new(char as u16, 1), PrimeField::new(char as u16, 0), PrimeField::new(char as u16, 1)];
	// let g = vec![PrimeField::new(char as u16, 0), PrimeField::new(char as u16, 1), PrimeField::new(char as u16, 1)];
	// let f = vec![PrimeField::new(char as u16, 1), PrimeField::new(char as u16, 1)];
	// let g = vec![PrimeField::new(char as u16, 1)];

	// let (quotient, remainder) = function_div(f.clone(), g.clone());
	// println!("f= {:?}",f);
	// println!("g= {:?}",g);
	// println!("quotient = {:?}", quotient);
	// println!("remainder = {:?}", remainder);
	
	

	
	for i in 2..((char as u32).pow(n as u32)) {
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
		
		println!("f = {:?}", f);

		let mut end_flag: bool = true;

		// g = x
		let mut g: Vec<PrimeField> = vec![PrimeField::new(char as u16, 0), PrimeField::new(char as u16, 1)];

		for m in 0..(n/2) {
			let g_temp:Vec<PrimeField> = g.clone();

			// g^p
			for j in 0..char-1{
				g = function_multiple(&g, &g_temp);
			}

			// g = g^p mod f
			g = function_div(g, f.clone()).1;

			//let mut g = pow_e_mod_f(g.clone(), f.clone(), char as u32, char);



			let mut g_2 = g.clone();
			// g^p-1 mod f
			g_2[1] = g_2[1].sub(&PrimeField::new(char, 1));

			let mut h = gcd(f.clone(),g_2);
			println!("h = {:?}", h);
			
			let mut h_inv = h.clone().into_iter().rev().collect::<Vec<PrimeField>>();
			// drop zero
			for i in 0..h_inv.len() {
				if h_inv[0].num != 0 {
					break;
				} else {
					h_inv.remove(0);
				}
			}
			let mut h = h_inv.into_iter().rev().collect::<Vec<PrimeField>>();
			if h.len() == 0{
				h.push(PrimeField::new(char, 0));
			}

			if !(h.len() <= 1 && h[0].num == 1) {
				println!("h = {:?}", h);
				end_flag = false;
				break;
			}


			
		}

		if end_flag == true{
			println!("answer = {:?}", f);
			break;
			
			
		}
		
	}

}
