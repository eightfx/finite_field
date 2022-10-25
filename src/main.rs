use std::ops;
// type of number , ex: i32
type NumType = i32;


#[derive(Debug, Clone)]
enum Element {
    PrimeField{element:NumType},
	Polynomial{element:Vec<NumType>},
	GaloisField {
        element: Vec<NumType>,
        primitive_polynomial: Vec<NumType>,
    },
}

#[derive(Debug, Clone)]
struct FiniteField {
    char: u32,
    element: Element,
}

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
            Element::Polynomial{element:_} => {
                let mut result: Vec<NumType> = Vec::new();
                let mut f: Vec<NumType> = Vec::new();
                let mut g: Vec<NumType> = Vec::new();

                // get element from enum
                if let Element::Polynomial{element:func_vec} = &self.element {
                    f = func_vec.clone();
                }
                if let Element::Polynomial{element:func_vec} = &other.element {
                    g = func_vec.clone();
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
                    element: Element::Polynomial{element:result},
                }
            }
            Element::GaloisField {
                element: _,
                primitive_polynomial: _,
            } => {
                let mut result: Vec<NumType> = Vec::new();
                let mut f: Vec<NumType> = Vec::new();
                let mut g: Vec<NumType> = Vec::new();
                let mut h: Vec<NumType> = Vec::new();

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
            Element::Polynomial{element:_} => {
                let mut result: Vec<NumType> = Vec::new();
                let mut f: Vec<NumType> = Vec::new();
                let mut g: Vec<NumType> = Vec::new();

                // get element from enum
                if let Element::Polynomial{element:func_vec} = &self.element {
                    f = func_vec.clone();
                }
                if let Element::Polynomial{element:func_vec} = &other.element {
                    g = func_vec.clone();
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
                    element: Element::Polynomial{element:result},
                }
            }
            Element::GaloisField {
                element: _,
                primitive_polynomial: _,
            } => {
                let mut result: Vec<NumType> = Vec::new();
                let mut f: Vec<NumType> = Vec::new();
                let mut g: Vec<NumType> = Vec::new();
                let mut h: Vec<NumType> = Vec::new();

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

			Element::Polynomial{element:_} => {
				let mut f: Vec<NumType> = Vec::new();
				let mut g: Vec<NumType> = Vec::new();

				let element0:Element = Element::PrimeField{element:0};
				let prime0: FiniteField = FiniteField {
					char: self.char,
					element: element0,
				};
				// get element from enum
				if let Element::Polynomial{element:func_vec} = &self.element {
					f = func_vec.clone();
				}
				if let Element::Polynomial{element:func_vec} = &other.element {
					g = func_vec.clone();
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
				// change to Vec<NumType>
				let mut result_vec: Vec<NumType> = Vec::new();
				for i in 0..result.len() {
					if let Element::PrimeField{element:a} = result[i].element {
						result_vec.push(a);
					}
				}

				// drop0
				let result = drop0(result_vec);

				FiniteField {
					char: self.char,
					element: Element::Polynomial{element:result},
				}

	}
			_ => {
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

fn main() {
	let char: u32 = 5;
	// let element1:Element = Element::PrimeField{element:1};
	// let element2:Element = Element::PrimeField{element:2};

	let element1: Element = Element::Polynomial{element:vec![1, 2, 4]};
	let element2: Element = Element::Polynomial{element:vec![1,2]};

	// let element1:Element = Element::GaloisField{element:vec![4,2,3,4,3],primitive_polynomial:vec![2,2,3]};
	// let element2:Element = Element::GaloisField{element:vec![1,2,3,1],primitive_polynomial:vec![2,2,3]};

	let x: FiniteField = FiniteField {
		char: char,
		element: element1,
	};
	let y: FiniteField = FiniteField {
		char: char,
		element: element2,
	};
	println!("{:?}", (y * x).element);
}
