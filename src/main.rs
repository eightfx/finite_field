use std::ops;
// type of number , ex: i32
type NumType = i32;


#[derive(Debug, Clone)]
enum Element {
    PrimeField{element:NumType},
	GaloisField {
        element: Vec<NumType>,
        primitive_polynomial: Vec<FiniteField>,
    }
}
#[derive(Debug, Clone)]
struct Polynomial{
	char: u32,
	coef: Vec<FiniteField>,
	prmitive_polynomial: Vec<FiniteField>
}

#[derive(Debug, Clone)]
struct FiniteField {
    char: u32,
    element: Element,
}
// impl ops::Add for Polynomial{
//     type Output = Polynomial;
//     fn add(self, other: Polynomial) -> Polynomial{
// 		match self.coef{

// 		}

// 	}
// }

// prime fields
impl ops::Add for FiniteField {
	type Output = FiniteField;
	fn add(self, other: FiniteField) -> FiniteField {
		match self.element {
			Element::PrimeField{element:_} => {
				let mut x: NumType = 0;
				let mut y: NumType = 0;

				// get element from enum
				if let Element::PrimeField{element:a} = self.element {
					x = a;
				}
				if let Element::PrimeField{element:a} = other.element {
					y = a;
				}
				let mut tmp = (x + y) % self.char as NumType;

				if tmp < 0 {
					tmp += self.char as NumType;
				} else if tmp == self.char as NumType {
					tmp = 0;
				}

				FiniteField {
					char: self.char,
					element: Element::PrimeField{element:tmp},
				}
			}
            Element::GaloisField {
                element: _,
                primitive_polynomial: _,
            } => {
                let mut result: Vec<NumType> = Vec::new();
                let mut f: Vec<NumType> = Vec::new();
                let mut g: Vec<NumType> = Vec::new();
                let mut h: Vec<FiniteField> = Vec::new();

                // get element from enum
                if let Element::GaloisField {
                    element: func_vec,
                    primitive_polynomial: primitive_func,
                } = &self.element
                {
                    f = func_vec.clone();
                    h = primitive_func.clone();
                }
                if let Element::GaloisField {
                    element: func_vec,
                    primitive_polynomial: primitive_func,
                } = &other.element
                {
                    g = func_vec.clone();
                    h = primitive_func.clone();
                }

                // deg f <= deg g
                if f.len() > g.len() {
                    (f, g) = (g, f);
                }

                // calculate
                for i in 0..f.len() {
                    let finite_f = FiniteField {
                        char: self.char,
                        element: Element::PrimeField{element:f[i]},
                    };
                    let finite_g = FiniteField {
                        char: self.char,
                        element: Element::PrimeField{element:g[i]},
                    };
                    let temp = finite_f + finite_g;
                    let mut answer = 0;
                    if let Element::PrimeField{element:a} = temp.element {
                        answer = a;
                    }
                    result.push(answer);
                }

                for i in f.len()..g.len() {
                    let finite_f = FiniteField {
                        char: self.char,
                        element: Element::PrimeField{element:0},
                    };
                    let finite_g = FiniteField {
                        char: self.char,
                        element: Element::PrimeField{element:g[i]},
                    };
                    let temp = finite_f + finite_g;
                    let mut answer = 0;
                    if let Element::PrimeField{element:a} = temp.element {
                        answer = a;
                    }
                    result.push(answer);
                }

                // drop0
                let result = drop0(result);
                FiniteField {
                    char: self.char,
                    element: Element::GaloisField {
                        element: result,
                        primitive_polynomial: h,
                    },
                }
            }
		}
	}
}

impl ops::Sub for FiniteField {
	type Output = FiniteField;
	fn sub(self, other: FiniteField) -> FiniteField {
		match self.element {
			Element::PrimeField{element:_} => {
				let mut x: NumType = 0;
				let mut y: NumType = 0;

				// get element from enum
				if let Element::PrimeField{element:a} = self.element {
					x = a;
				}
				if let Element::PrimeField{element:a} = other.element {
					y = a;
				}
				let mut tmp = (x - y) % self.char as NumType;
				if tmp < 0 {
					tmp += self.char as NumType;
				} else if tmp == self.char as NumType {
					tmp = 0;
				}

				FiniteField {
					char: self.char,
					element: Element::PrimeField{element:tmp},
				}
			}
			Element::GaloisField {
				element: _,
				primitive_polynomial: _,
			} => {
				let mut result: Vec<NumType> = Vec::new();
				let mut f: Vec<NumType> = Vec::new();
				let mut g: Vec<NumType> = Vec::new();
				let mut h: Vec<FiniteField> = Vec::new();

				// get element from enum
				if let Element::GaloisField {
					element: func_vec,
					primitive_polynomial: primitive_func,
				} = &self.element
				{
					f = func_vec.clone();
					h = primitive_func.clone();
				}
				if let Element::GaloisField {
					element: func_vec,
					primitive_polynomial: primitive_func,
				} = &other.element
				{
					g = func_vec.clone();
					h = primitive_func.clone();
				}

				// get each degree
				let (mut max_degree, mut min_degree) = get_maxmin_degree(&f, &g);

				// calculate
				for i in 0..min_degree {
					let finite_f = FiniteField {
						char: self.char,
						element: Element::PrimeField{element:f[i]},
					};
					let finite_g = FiniteField {
						char: self.char,
						element: Element::PrimeField{element:g[i]},
					};
					let temp = finite_f - finite_g;
					let mut answer = 0;
					if let Element::PrimeField{element:a} = temp.element {
						answer = a;
					}
					result.push(answer);
				}

				for i in min_degree..max_degree {
					let mut answer = 0;
					if f.len() > g.len() {
						let finite_f = FiniteField {
							char: self.char,
							element: Element::PrimeField{element:f[i]},
						};
						let finite_g = FiniteField {
							char: self.char,
							element: Element::PrimeField{element:0},
						};
						let temp = finite_f - finite_g;
						if let Element::PrimeField{element:a} = temp.element {
							answer = a;
						}
					} else {
						let finite_f = FiniteField {
							char: self.char,
							element: Element::PrimeField{element:0},
						};
						let finite_g = FiniteField {
							char: self.char,
							element: Element::PrimeField{element:g[i]},
						};
						let temp = finite_f - finite_g;
						if let Element::PrimeField{element:a} = temp.element {
							answer = a;
						}
					}

					result.push(answer);
				}

				// drop0
				let result = drop0(result);
				FiniteField {
					char: self.char,
					element: Element::GaloisField {
						element: result,
						primitive_polynomial: h,
					},
				}
			}
			_ => {
				panic!("not implemented");
			}

		}
	}
}

impl ops::Mul for FiniteField {
	type Output = FiniteField;
	fn mul(self, other: FiniteField) -> FiniteField {
		match self.element {
			Element::PrimeField{element:_} => {
				let mut x = 0;
				let mut y = 0;
				if let Element::PrimeField{element:a} = self.element {
					x = a;
				}
				if let Element::PrimeField{element:a} = other.element {
					y = a;
				}
				let tmp = (x * y) % self.char as NumType;
				FiniteField {
					char: self.char,
					element: Element::PrimeField{element:tmp},
				}
			}
			Element::GaloisField { element:_, primitive_polynomial:_}=>{
				let mut f: Vec<NumType> = Vec::new();
				let mut g: Vec<NumType> = Vec::new();
				let mut primitive_polynomial: Vec<FiniteField> = Vec::new();

				let element0:Element = Element::PrimeField{element:0};
				let prime0: FiniteField = FiniteField {
					char: self.char,
					element: element0,
				};
				// get element from enum
				if let Element::GaloisField{element:func_vec, primitive_polynomial:pp} = &self.element {
					f = func_vec.clone();
					primitive_polynomial = pp.clone();
				}
				if let Element::GaloisField{element:func_vec, primitive_polynomial:pp} = &other.element {
					g = func_vec.clone();
					primitive_polynomial = pp.clone();
				}
				let mut result = vec![prime0; f.len() + g.len() - 1];
				for i in 0..f.len(){
					for j in 0..g.len(){
						let r_tmp = FiniteField {
							char: self.char,
							element: result[i+j].element.clone(),
						};
						let f_tmp = FiniteField {
							char: self.char,
							element: Element::PrimeField{element:f[i]},
						};
						let g_tmp = FiniteField {
							char: self.char,
							element: Element::PrimeField{element:g[j]},
						};

						result[i + j] = r_tmp + f_tmp * g_tmp;
					}
				}
				
				let mut result_inv = result.clone();
				result_inv.reverse();
				let mut primitive_polynomial_inv = primitive_polynomial.clone();
				primitive_polynomial_inv.reverse();

				if result_inv.len() >= primitive_polynomial_inv.len(){
					for i in 0..result_inv.len()-primitive_polynomial_inv.len() + 1{
						let mut temp = result_inv[i].clone() / primitive_polynomial_inv[0].clone();
						for j in 0..primitive_polynomial_inv.len(){
							result_inv[i+j] = result_inv[i+j].clone() - (temp.clone() * primitive_polynomial_inv[j].clone());
						}
					}
				}
				// drop0
				for i in 0..result_inv.len(){
					if let Element::PrimeField{element:a} = result_inv[0].element {
						if a != 0 {
							break;
						}else{
							result_inv.remove(0);
						}
						}
				}
				let mut result = result_inv.clone();
				result.reverse();

				// PrimeField -> NumType
				let mut result_num: Vec<NumType> = Vec::new();
				for i in 0..result.len(){
					if let Element::PrimeField{element:a} = result[i].element {
						result_num.push(a);
					}
				}

				
				FiniteField {
					char: self.char,
					element: Element::GaloisField{element:result_num, primitive_polynomial:primitive_polynomial},
				}
				
			}
		}
	}
}

impl ops::Div for FiniteField {
	type Output = FiniteField;
	// ユークリッドの互除法
	fn div(self, other: FiniteField) -> FiniteField {
		match self.element{
			Element::PrimeField { element:_}=>{
				
				let mut x: NumType = 0;
				let mut y: NumType = 0;
				if let Element::PrimeField{element:a} = self.element {
					x = a;
				}
				if let Element::PrimeField{element:a} = other.element {
					y = a;
				}
				let mut t = extended_euclidean(self.char as NumType, y);

				if t < 0 {
					let mut i = 1;

					while (t + i * self.char as NumType) < 0 {
						i += 1;
					}
					t = (t + i * self.char as NumType) % self.char as NumType;
				}
				FiniteField{
					char: self.char,
					element: Element::PrimeField{element:(x* t) % self.char as NumType}
				}
			}
			_=>{
				panic!("not implemented");
			}


		}
	}
}

fn drop0(vec: Vec<NumType>) -> Vec<NumType> {
	let mut vec_inverse = vec.into_iter().rev().collect::<Vec<NumType>>();
	for i in 0..vec_inverse.len() - 1 {
		if vec_inverse[0] != 0 {
			break;
		} else {
			vec_inverse.remove(0);
		}
	}
	let vec = vec_inverse.into_iter().rev().collect::<Vec<NumType>>();
	vec
}

impl ops::Rem for FiniteField {
	type Output = FiniteField;
	// ユークリッドの互除法
	fn rem(self, other: FiniteField) -> FiniteField {
		match self.element{
			Element::PrimeField{ element :_} =>{
				panic!("PrimeField do not correspond to remainders");
			}
			Element::GaloisField{ element :_, primitive_polynomial:_} =>{
				panic!("GaloisField do not correspond to remainders");
			}

		}
	}
}


fn get_maxmin_degree(f: &Vec<NumType>, g: &Vec<NumType>) -> (usize, usize) {
	let mut max_degree = 0;
	let mut min_degree = 0;
	if f.len() < g.len() {
		max_degree = g.len();
		min_degree = f.len();
	} else {
		max_degree = f.len();
		min_degree = g.len();
	}
	(max_degree, min_degree)
}
fn extended_euclidean( u: NumType, v: NumType) -> NumType{
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


fn main() {
	let char: u32 = 2;
	// let element1:Element = Element::PrimeField{element:3};
	// let element2:Element = Element::PrimeField{element:2};

	// let element1: Element = Element::PrimePolynomial{element:vec![1, 2,2,4]};
	// let element2: Element = Element::PrimePolynomial{element:vec![1,2,3]};


	let primitive_polynomial = vec![FiniteField{char:char,element:Element::PrimeField{element:1}},FiniteField{char:char,element:Element::PrimeField{element:1}},FiniteField{char:char,element:Element::PrimeField{element:0}},FiniteField{char:char,element:Element::PrimeField{element:0}},FiniteField{char:char,element:Element::PrimeField{element:1}}];
	let element1:Element = Element::GaloisField{element:vec![1,1,0,0],primitive_polynomial:primitive_polynomial.clone()};
	let element2:Element = Element::GaloisField{element:vec![0,1,0,1],primitive_polynomial:primitive_polynomial.clone()};

	let x: FiniteField = FiniteField {
		char: char,
		element: element1,
	};
	let y: FiniteField = FiniteField {
		char: char,
		element: element2,
	};
	println!("{:?}", (x * y).element);
}
