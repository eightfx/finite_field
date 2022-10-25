// type of number , ex: i32
type NumType = i32;

#[derive(Debug, Clone)]
struct FiniteField<T> {
    char: u32,
    element: T,
}

enum Element{
	PrimeField(NumType),
	PolynomialField(Vec<NumType>),
	GaloisField(Vec<NumType>, NumType),
}
// polynomial
impl FiniteField<Vec<NumType>>{
    fn add(&self, other: &FiniteField<Vec<NumType>>) ->FiniteField<Vec<NumType>>{
		let mut result_elements = Vec::new();
		let mut f:Vec<NumType> = Vec::new();
		let mut g:Vec<NumType> = Vec::new();

		if self.element.len() > other.element.len() {
			f =  self.element.clone();
			g =  other.element.clone();
		}
		else{
			f =  other.element.clone();
			g =  self.element.clone();
			
		}
		for i in 0..g.len(){
			result_elements.push(f[i] + g[i]);
		}
		for i in g.len()..f.len(){
			result_elements.push(f[i]);
		}
		FiniteField{char: self.char, element: result_elements}
	}
}

// prime fields
impl FiniteField<NumType> {
	fn add(&self, other: &FiniteField<NumType>) -> FiniteField<NumType>{
		FiniteField {
			char: self.char,
			element: (self.element+ other.element) % self.char as NumType,
		}
	}
	fn sub(&self, other: &FiniteField<NumType>) -> FiniteField<NumType> {
		let mut minus_num = (self.element - other.element) % (self.char as NumType);
		if minus_num < 0 {
			minus_num += self.char as NumType;
		}
		FiniteField {
			char: self.char,
			element: minus_num as NumType,
		}
	}
	fn mul(&self, other: &FiniteField<NumType>) -> FiniteField<NumType>{
		let mut mul_num = (self.element* other.element) % self.char as NumType;
		if mul_num < 0 {
			mul_num += self.char as NumType;
		}

		FiniteField{
			char: self.char,
			element : mul_num,
		}
	}
	fn pow(&self, other: NumType) -> FiniteField<NumType>{
		FiniteField{
			char: self.char,
			element : (self.element.pow(other as u32)) % self.char as NumType,
		}
	}
	fn div(&self, other: &FiniteField<NumType>) -> FiniteField<NumType>{
		let mut t = self._extended_euclidean(self.char as NumType, other.element);
		if t < 0 {
			let mut i = 1;

			while (t + i * self.char as NumType) < 0 {
				i += 1;
			}
			t = (t + i * self.char as NumType) % self.char as NumType;
		}
		FiniteField{
			char: self.char,
			element : (self.element * t) % self.char as NumType,
		}
	}
	// ユークリッドの互除法
	fn _extended_euclidean(&self, u: NumType, v: NumType) ->  NumType{
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


fn main() {
	let char:u32 = 5;
	let x:FiniteField<Vec<NumType>> = FiniteField{char:char,element:vec![1,2,3]};
	let y:FiniteField<Vec<NumType>> = FiniteField{char:char,element:vec![1,2]};
	println!("{:?}",x.add(&y));

}
