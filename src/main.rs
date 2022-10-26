use std::ops;
// type of number , ex: i32
type NumType = i32;

#[derive(Debug, Clone)]
enum Element {
    PrimeField {
        element: NumType,
    },
    GaloisField {
        element: Vec<NumType>,
        primitive_polynomial: Vec<FiniteField>,
    },
}
#[derive(Debug, Clone)]
struct Polynomial {
    char: u32,
    coef: Vec<FiniteField>,
}

#[derive(Debug, Clone)]
struct FiniteField {
    char: u32,
    element: Element,
}

impl FiniteField{
	fn get_0(&self) -> FiniteField{
		match &self.element{
			Element::PrimeField{element} => FiniteField{char: self.char, element: Element::PrimeField{element: 0}},
			Element::GaloisField{element, primitive_polynomial} => FiniteField{char: self.char, element: Element::GaloisField{element: vec![0], primitive_polynomial: primitive_polynomial.clone()}},
		}
	}
	fn get_1(&self) -> FiniteField{
		match &self.element{
			Element::PrimeField{element} => FiniteField{char: self.char, element: Element::PrimeField{element: 1}},
			Element::GaloisField{element, primitive_polynomial} => FiniteField{char: self.char, element: Element::GaloisField{element: vec![1], primitive_polynomial: primitive_polynomial.clone()}},
		}
	}

	fn is_0(&self) -> bool{
		match &self.element{
			Element::PrimeField{element} => *element == 0,
			Element::GaloisField{element, primitive_polynomial} => (element[0] == 0)&&(element.len() == 1),
		}
	}

}

// Polynomial
impl ops::Add for Polynomial {
    type Output = Polynomial;
    fn add(self, other: Polynomial) -> Polynomial {
        let mut result = Polynomial {
            char: self.char,
            coef: Vec::new(),
        };
        let max_degree = if self.coef.len() > other.coef.len() {
            self.coef.len()
        } else {
            other.coef.len()
        };
        let min_degree = if self.coef.len() < other.coef.len() {
            self.coef.len()
        } else {
            other.coef.len()
        };

        for i in 0..min_degree {
            result
                .coef
                .push(self.coef[i].clone() + other.coef[i].clone());
        }
        for i in min_degree..max_degree {
            if self.coef.len() > other.coef.len() {
                result.coef.push(self.coef[i].clone());
            } else {
                result.coef.push(other.coef[i].clone());
            }
        }
        result
    }
}

impl ops::Sub for Polynomial {
    type Output = Polynomial;
    fn sub(self, other: Polynomial) -> Polynomial {
        let mut result = Polynomial {
            char: self.char,
            coef: Vec::new(),
        };
        let max_degree = if self.coef.len() > other.coef.len() {
            self.coef.len()
        } else {
            other.coef.len()
        };
        let min_degree = if self.coef.len() < other.coef.len() {
            self.coef.len()
        } else {
            other.coef.len()
        };
        for i in 0..min_degree {
            result
                .coef
                .push(self.coef[i].clone() - other.coef[i].clone());
        }
        for i in min_degree..max_degree {
            if self.coef.len() > other.coef.len() {
                result.coef.push(self.coef[i].clone());
            } else {
                result.coef.push(-other.coef[i].clone());
            }
        }
        result
    }
}
impl ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, other: Polynomial) -> Polynomial {
		let element0 = self.coef[0].clone().get_0();
		let mut result = Polynomial {
			char: self.char,
			coef: vec![element0; self.coef.len() + other.coef.len() - 1],
		};
		for i in 0..self.coef.len() {
			for j in 0..other.coef.len() {
				let tmp = self.coef[i].clone() * other.coef[j].clone();
				result.coef[i + j] = result.coef[i + j].clone() + tmp;
			}
		}
		result
	}
}
impl ops::Div for Polynomial {
    type Output = Polynomial;
    fn div(self, other: Polynomial) -> Polynomial {
		let mut quotient = Polynomial {
			char: self.char,
			coef: Vec::new(),
		};
		let mut f_inv = self.coef.clone();
		f_inv.reverse();
		let mut g_inv = other.coef.clone();
		g_inv.reverse();

		// drop0
		for i in 0..f_inv.len(){
			if f_inv[i].is_0(){
				f_inv.remove(0);
			}else{
				break;
			}
		}
		for i in 0..g_inv.len(){
			if g_inv[i].is_0(){
				g_inv.remove(0);
			}else{
				break;
			}
		}

		if f_inv.len() < g_inv.len(){
			quotient = Polynomial{
				char: self.char,
				coef: vec![self.coef[0].clone().get_0()],
			};
		}
	else{
		for i in 0..f_inv.len()-g_inv.len()+1{
			let mut temp = f_inv[i].clone() / g_inv[0].clone();
			for j in 0..g_inv.len(){
				f_inv[i+j] = f_inv[i+j].clone() - (temp.clone() * g_inv[j].clone());
			}
			quotient.coef.push(temp);
		}
		
	}

		// reverse
		quotient = Polynomial{
			char: self.char,
			coef: quotient.coef.clone().into_iter().rev().collect(),
		};
		quotient

	}
}


// prime fields
impl ops::Add for FiniteField {
	type Output = FiniteField;
	fn add(self, other: FiniteField) -> FiniteField {
		match self.element {
			Element::PrimeField { element: _ } => {
				let mut x: NumType = 0;
				let mut y: NumType = 0;

				// get element from enum
				if let Element::PrimeField { element: a } = self.element {
					x = a;
				}
				if let Element::PrimeField { element: a } = other.element {
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
					element: Element::PrimeField { element: tmp },
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
						element: Element::PrimeField { element: f[i] },
					};
					let finite_g = FiniteField {
						char: self.char,
						element: Element::PrimeField { element: g[i] },
					};
					let temp = finite_f + finite_g;
					let mut answer = 0;
					if let Element::PrimeField { element: a } = temp.element {
						answer = a;
					}
					result.push(answer);
				}

				for i in f.len()..g.len() {
					let finite_f = FiniteField {
						char: self.char,
						element: Element::PrimeField { element: 0 },
					};
					let finite_g = FiniteField {
						char: self.char,
						element: Element::PrimeField { element: g[i] },
					};
					let temp = finite_f + finite_g;
					let mut answer = 0;
					if let Element::PrimeField { element: a } = temp.element {
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
			Element::PrimeField { element: _ } => {
				let mut x: NumType = 0;
				let mut y: NumType = 0;

				// get element from enum
				if let Element::PrimeField { element: a } = self.element {
					x = a;
				}
				if let Element::PrimeField { element: a } = other.element {
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
					element: Element::PrimeField { element: tmp },
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
				let max_degree = if f.len() > g.len() {
					f.len()
				} else {
					g.len()
				};
				let min_degree = if f.len() < g.len() {
					f.len()
				} else {
					g.len()
				};

				// calculate
				for i in 0..min_degree {
					let finite_f = FiniteField {
						char: self.char,
						element: Element::PrimeField { element: f[i] },
					};
					let finite_g = FiniteField {
						char: self.char,
						element: Element::PrimeField { element: g[i] },
					};
					let temp = finite_f - finite_g;
					let mut answer = 0;
					if let Element::PrimeField { element: a } = temp.element {
						answer = a;
					}
					result.push(answer);
				}

				for i in min_degree..max_degree {
					let mut answer = 0;
					if f.len() > g.len() {
						let finite_f = FiniteField {
							char: self.char,
							element: Element::PrimeField { element: f[i] },
						};
						let finite_g = FiniteField {
							char: self.char,
							element: Element::PrimeField { element: 0 },
						};
						let temp = finite_f - finite_g;
						if let Element::PrimeField { element: a } = temp.element {
							answer = a;
						}
					} else {
						let finite_f = FiniteField {
							char: self.char,
							element: Element::PrimeField { element: 0 },
						};
						let finite_g = FiniteField {
							char: self.char,
							element: Element::PrimeField { element: g[i] },
						};
						let temp = finite_f - finite_g;
						
						if let Element::PrimeField { element: a } = temp.element {
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
		}
	}
}

impl ops::Mul for FiniteField {
	type Output = FiniteField;
	fn mul(self, other: FiniteField) -> FiniteField {
		match self.element {
			Element::PrimeField { element: _ } => {
				let mut x = 0;
				let mut y = 0;
				if let Element::PrimeField { element: a } = self.element {
					x = a;
				}
				if let Element::PrimeField { element: a } = other.element {
					y = a;
				}
				let tmp = (x * y) % self.char as NumType;
				FiniteField {
					char: self.char,
					element: Element::PrimeField { element: tmp },
				}
			}
			Element::GaloisField {
				element: _,
				primitive_polynomial: _,
			} => {
				let mut f: Vec<NumType> = Vec::new();
				let mut g: Vec<NumType> = Vec::new();
				let mut primitive_polynomial: Vec<FiniteField> = Vec::new();

				let element0: Element = Element::PrimeField { element: 0 };
				let prime0: FiniteField = FiniteField {
					char: self.char,
					element: element0,
				};
				// get element from enum
				if let Element::GaloisField {
					element: func_vec,
					primitive_polynomial: pp,
				} = &self.element
				{
					f = func_vec.clone();
					primitive_polynomial = pp.clone();
				}
				if let Element::GaloisField {
					element: func_vec,
					primitive_polynomial: pp,
				} = &other.element
				{
					g = func_vec.clone();
					primitive_polynomial = pp.clone();
				}
				let mut result = vec![prime0; f.len() + g.len() - 1];
				for i in 0..f.len() {
					for j in 0..g.len() {
						let r_tmp = FiniteField {
							char: self.char,
							element: result[i + j].element.clone(),
						};
						let f_tmp = FiniteField {
							char: self.char,
							element: Element::PrimeField { element: f[i] },
						};
						let g_tmp = FiniteField {
							char: self.char,
							element: Element::PrimeField { element: g[j] },
						};

						result[i + j] = r_tmp + f_tmp * g_tmp;
					}
				}

				let mut result_inv = result.clone();
				result_inv.reverse();
				let mut primitive_polynomial_inv = primitive_polynomial.clone();
				primitive_polynomial_inv.reverse();

				if result_inv.len() >= primitive_polynomial_inv.len() {
					for i in 0..result_inv.len() - primitive_polynomial_inv.len() + 1 {
						let temp = result_inv[i].clone() / primitive_polynomial_inv[0].clone();
						for j in 0..primitive_polynomial_inv.len() {
							result_inv[i + j] = result_inv[i + j].clone()
								- (temp.clone() * primitive_polynomial_inv[j].clone());
						}
					}
				}
				// drop0
				for _ in 0..result_inv.len() {
					if let Element::PrimeField { element: a } = result_inv[0].element {
						if a != 0 {
							break;
						} else {
							result_inv.remove(0);
						}
					}
				}
				let mut result = result_inv.clone();
				result.reverse();

				// PrimeField -> NumType
				let mut result_num: Vec<NumType> = Vec::new();
				for i in 0..result.len() {
					if let Element::PrimeField { element: a } = result[i].element {
						result_num.push(a);
					}
				}

				FiniteField {
					char: self.char,
					element: Element::GaloisField {
						element: result_num,
						primitive_polynomial: primitive_polynomial,
					},
				}
			}
		}
	}
}

impl ops::Div for FiniteField {
	type Output = FiniteField;
	// ユークリッドの互除法
	fn div(self, other: FiniteField) -> FiniteField {
		match self.element {
			Element::PrimeField { element: _ } => {
				let mut x: NumType = 0;
				let mut y: NumType = 0;
				if let Element::PrimeField { element: a } = self.element {
					x = a;
				}
				if let Element::PrimeField { element: a } = other.element {
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
				FiniteField {
					char: self.char,
					element: Element::PrimeField {
						element: (x * t) % self.char as NumType,
					},
				}
			}
			Element::GaloisField {
				element: _,
				primitive_polynomial: _,
			} => {
				let mut f: FiniteField = FiniteField {
					char: self.char,
					element: Element::PrimeField { element: 0 },
				};

				let mut g: FiniteField = FiniteField {
					char: self.char,
					element: Element::PrimeField { element: 0 },
				};

				let element0: Element = Element::PrimeField { element: 0 };
				let mut primitive_polynomial: Vec<FiniteField> = vec![FiniteField {
					char: self.char,
					element: element0.clone(),
				}];

				// get element from enum
				if let Element::GaloisField {
					element: func_vec,
					primitive_polynomial: pp,
				} = &self.element
				{
					f = FiniteField {
						char: self.char,
						element: Element::GaloisField {
							element: func_vec.clone(),
							primitive_polynomial: pp.clone(),
						},
					};
					primitive_polynomial = pp.clone();
				}
				if let Element::GaloisField {
					element: func_vec,
					primitive_polynomial: pp,
				} = &other.element
				{
					g = FiniteField {
						char: self.char,
						element: Element::GaloisField {
							element: func_vec.clone(),
							primitive_polynomial: pp.clone(),
						},
					};
				}

				// g^(-1) = g^(q-2) mod primitive_polynomial
				let pow_count = self
					.char
					.pow((primitive_polynomial.len() - 1).try_into().unwrap())
					- 2;
				let g_origin = g.clone();
				for _ in 0..pow_count - 1 {
					g = g * g_origin.clone();
				}

				// f * g^(-1) mod primitive_polynomial
				let result: FiniteField = f * g;

				result
			}
		}
	}
}

impl ops::Neg for FiniteField {
	type Output = FiniteField;
	// ユークリッドの互除法
	fn neg(self) -> FiniteField {
		match self.element {
			Element::PrimeField { element: _ } => {
				let prime0 = FiniteField {
					char: self.char,
					element: Element::PrimeField { element: 0 },
				};
				let result = prime0 - self;
				result
			}
			Element::GaloisField {
				element: _,
				primitive_polynomial: _,
			} => {
				let mut primitive_polynomial: Vec<FiniteField> = vec![];
				if let Element::GaloisField {
					element: func_vec,
					primitive_polynomial: pp,
				} = &self.element
				{
					primitive_polynomial = pp.clone();
				}
				let galois0 = FiniteField {
					char: self.char,
					element: Element::GaloisField {
						element: vec![0],
						primitive_polynomial: primitive_polynomial.clone(),
					},
				};
				let result = galois0 - self;
				println!("neg result: {:?}", result);
				result
			}
		}
	}
}

fn drop0(vec: Vec<NumType>) -> Vec<NumType> {
	let mut vec_inverse = vec.into_iter().rev().collect::<Vec<NumType>>();
	for _ in 0..vec_inverse.len() - 1 {
		if vec_inverse[0] != 0 {
			break;
		} else {
			vec_inverse.remove(0);
		}
	}
	let vec = vec_inverse.into_iter().rev().collect::<Vec<NumType>>();
	vec
}

fn extended_euclidean(u: NumType, v: NumType) -> NumType {
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
	let element1: Element = Element::PrimeField { element: 0 };
	let element2: Element = Element::PrimeField { element: 1 };
	let element3: Element = Element::PrimeField { element: 1 };

	// let primitive_polynomial = vec![
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 1 },
	// 	},
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 1 },
	// 	},
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 0 },
	// 	},
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 0 },
	// 	},
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 1 },
	// 	},
	// ];
	// let element1: Element = Element::GaloisField {
	// 	element: vec![1, 0, 0, 1],
	// 	primitive_polynomial: primitive_polynomial.clone(),
	// };
	// let element2: Element = Element::GaloisField {
	// 	element: vec![0, 1, 0, 1],
	// 	primitive_polynomial: primitive_polynomial.clone(),
	// };
	// let element3: Element = Element::GaloisField {
	// 	element: vec![1, 1, 0, 1],
	// 	primitive_polynomial: primitive_polynomial.clone(),
	// };

	let element1: FiniteField = FiniteField {
		char: char,
		element: element1,
	};
	let element2: FiniteField = FiniteField {
		char: char,
		element: element2,
	};
	let element3: FiniteField = FiniteField {
		char: char,
		element: element3,
	};
	let poly1 = Polynomial {
    	char: char,
    	coef: vec![element2.clone(), element3.clone()],
	};
	let poly2 = Polynomial {
    	char: char,
    	coef: vec![element1.clone(),element2.clone()],
	};

		// println!("{:?}", (element2*element1).element);
	println!("{:?}", (poly1/poly2).coef);
}
