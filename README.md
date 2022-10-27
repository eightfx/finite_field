
# Table of Contents

1.  [finite fields](#org068f34c)
2.  [What makes it different from other libraries?](#orgf6d5f0f)
    1.  [Pros:](#org47b98b4)
    2.  [Cons:](#orga9135ee)
3.  [Usage](#org029522d)
4.  [Examples](#org8c95d1c)
    1.  [Prime Field](#orgccf1c10)
    2.  [Galois Field](#orgf02c6c0)
    3.  [Polynomial over F<sub>p</sub>](#org5b8e57a)
    4.  [Polynomial over GF(p<sup>n</sup>)](#orge7bbdc3)
    5.  [Matrix over FiniteField](#orgb027911)
5.  [Link](#org1821e10)


<a id="org068f34c"></a>

# finite fields

A Rust library for operations on finite field, featuring:

-   Sum, difference, product, and quotient of elements F<sub>p</sub>
-   Sum, difference, product, and quotient of elements GF(p<sup>n</sup>)
-   Obtaining the primitive polynomial
-   Sum, difference, product, quotient, and remainder of polynomial over F<sub>p</sub>
-   Sum, difference, product, quotient, and remainder of polynomial over GF(p<sup>n</sup>)
-   Sum, product of Matrix F<sub>p</sub>
-   Sum, product of Matrix GF(p<sup>n</sup>)
-   The sweep method (or Gaussian elimination) of matrices on finite bodies (F<sub>p</sub>, GF(p<sup>n</sup>)) is also available.


<a id="orgf6d5f0f"></a>

# What makes it different from other libraries?


<a id="org47b98b4"></a>

## Pros:

-   Can be calculated with F<sub>p</sub>, GF(p<sup>n</sup>) for any prime and any multiplier, not limited to a char 2.
-   Can freely compute four types of element: prime, Galois, polynomial of prime, and polynomial of Galois.
-   Each can be calculated with +-\*/, so you can write natural code.
-   Matrix operations on finite bodies (F<sub>p</sub>, GF(p<sup>n</sup>)) can also be performed.
-   The sweep method can be available.


<a id="orga9135ee"></a>

## Cons:

-   It takes longer than other libraries because it is not optimized for each character.


<a id="org029522d"></a>

# Usage

Add this to your Cargo.toml:

    [dependencies]
    galois_field = "0.1.5"


<a id="org8c95d1c"></a>

# Examples


<a id="orgccf1c10"></a>

## Prime Field

    use galois_field::*;
    
    fn main() {
    	let char: u32 = 2;
    	let n = 4;
    	
    	let primitive_polynomial = Polynomial::get_primitive_polynomial(char, n);
    	let x:FiniteField = FiniteField{
    		char: char,
    		element:Element::GaloisField{element:vec![0,1],primitive_polynomial:primitive_polynomial.clone()} // i.e. [0,1] = x -> 2 over GF(2^4)
    	};
    	let y:FiniteField = FiniteField{
    		char: char,
    		element:Element::GaloisField{element:vec![0,0,1,1],primitive_polynomial:primitive_polynomial.clone()} // i.e. [0,0,1,1] = x^3 + x^2 -> 12 over GF(2^4)
    	};
    	
    	println!("x + y = {:?}", (x.clone() + y.clone()).element);
    	println!("x - y = {:?}", (x.clone() - y.clone()).element);
    	println!("x * y = {:?}", (x.clone() * y.clone()).element);
    	println!("x / y = {:?}", (x.clone() / y.clone()).element);
    }


<a id="orgf02c6c0"></a>

## Galois Field

    use galois_field::*;
    
    fn main(){
    	// consider GF(2^4)
    	let char: u32 = 2;
    	let n = 4;
    	let primitive_polynomial = Polynomial::get_primitive_polynomial(char, n);
    	let x:FiniteField = FiniteField{
     		char: char,
     		element:Element::GaloisField{element:vec![0,1],primitive_polynomial:primitive_polynomial.clone()} // i.e. [0,1] = x -> 2 over GF(2^4)
    	};
    	let y:FiniteField = FiniteField{
     		char: char,
     		element:Element::GaloisField{element:vec![0,0,1,1],primitive_polynomial:primitive_polynomial.clone()} // i.e. [0,0,1,1] = x^3 + x^2 -> 12 over GF(2^4)
    	};
    	println!("x + y = {:?}", (x.clone() + y.clone()).element);
    	println!("x - y = {:?}", (x.clone() - y.clone()).element);
    	println!("x * y = {:?}", (x.clone() * y.clone()).element);
    	println!("x / y = {:?}", (x.clone() / y.clone()).element);
    
    }


<a id="org5b8e57a"></a>

## Polynomial over F<sub>p</sub>

    use galois_field::*;
    
    fn main() {
    	// character
        let char: u32 = 2;
    
    	let element0:FiniteField = FiniteField{
    		char: char,
    		element:Element::PrimeField{element:0} // 0 in F_5
    	};
    	let element1:FiniteField = FiniteField{
    		char: char,
    		element:Element::PrimeField{element:1} // 1 in F_5
    	};
    
    
    	let f: Polynomial = Polynomial {
            coef: vec![element1.clone(),element0.clone(),element0.clone(),element0.clone(),element1.clone()]
    	};
        let g: Polynomial = Polynomial {
    		coef: vec![element1.clone(),element0.clone(),element0.clone(),element1.clone(),element1.clone()]
        };
        println!("f + g = {:?}", (f.clone()+g.clone()).coef);
    	println!("f - g = {:?}", (f.clone()-g.clone()).coef);
    	println!("f * g = {:?}", (f.clone()*g.clone()).coef);
    	println!("f / g = {:?}", (f.clone()/g.clone()).coef);
    	println!("f % g = {:?}", (f.clone()%g.clone()).coef);
    	
    }


<a id="orge7bbdc3"></a>

## Polynomial over GF(p<sup>n</sup>)

Same as above


<a id="orgb027911"></a>

## Matrix over FiniteField

    use galois_field::*;
    
    let char = 3;
    let element0: FiniteField = FiniteField {
        char: char,
        element: Element::PrimeField { element: 0 },
    };
    let element1: FiniteField = FiniteField {
        char: char,
        element: Element::PrimeField { element: 1 },
    };
    let element2: FiniteField = FiniteField {
        char: char,
        element: Element::PrimeField { element: 2 },
    };
    
    
    let mut matrix_element:Vec<Vec<FiniteField>> = vec![
        vec![element0.clone(),element1.clone(), element0.clone()],
        vec![element2.clone(),element2.clone(), element1.clone()],
        vec![element1.clone(),element0.clone(), element1.clone()]
    ];
    let mut matrix = Matrix{
        element: matrix_element,
    };
    
    println!("m+m = {:?}", m.clone()+m.clone());
    println!("m*m = {:?}", m.clone()*m.clone());
    
    let mut sweep_matrix = m.sweep_method();
    println!("{:?}", sweep_matrix);


<a id="org1821e10"></a>

# Link

<https://crates.io/crates/galois_field>

