
# Table of Contents

1.  [finite<sub>fields</sub>](#orgf45fd8d)
2.  [Usage](#org8057ed2)
3.  [Examples](#orge955a46)
    1.  [Prime Field](#orgb574a95)
    2.  [Galois Field](#org0f6e3c0)
    3.  [Polynomial over F<sub>p</sub>](#orgf80df41)


<a id="orgf45fd8d"></a>

# finite<sub>fields</sub>

A Rust library for operations on finite field, featuring:

-   Sum, difference, product, and quotient of elements F<sub>p</sub>
-   Sum, difference, product, and quotient of elements GF(p<sup>n</sup>)
-   Obtaining the primitive polynomial
-   Sum, difference, product, quotient, and remainder of polynomial over F<sub>p</sub>
-   Sum, difference, product, quotient, and remainder of polynomial over GF(p<sup>n</sup>)


<a id="org8057ed2"></a>

# Usage

Add this to your Cargo.toml:

    [dependencies]
    finite_fields = "0.1.0"


<a id="orge955a46"></a>

# Examples


<a id="orgb574a95"></a>

## Prime Field

    use finite_field::*;
    
    // consider F_5
    let char:u32 = 5
    let x:FiniteField = FiniteField{
        char:char,
        element:Element::PrimeField(2)
        };
    let y:FiniteField = FiniteField{
        char:char,
        element:Element::PrimeField(3)
        };
    println!("x + y = {:?}", x + y);
    println!("x - y = {:?}", x - y);
    println!("x * y = {:?}", x * y);
    println!("x / y = {:?}", x / y);


<a id="org0f6e3c0"></a>

## Galois Field

    use finite_field::*;
    
    // consider GF(5^3)
    let char:u32 = 5
    let n:u32 = 3
    let primitive_polynomial = Polynomial::get_primitive_polynomial(char, n);
    let element1:Element = Element::GaloisField{element:vec![4,2,3,4,3],primitive_polynomial:primitive_polynomial.clone()};
    let element2:Element = Element::GaloisField{element:vec![1,2,3,1],primitive_polynomial:primitive_polynomial.clone()};
    
    let x: FiniteField = FiniteField {
        char: char,
        element: element1,
    };
    let y: FiniteField = FiniteField {
        char: char,
        element: element2,
    };
        println!("x + y = {:?}", (x + y).element);
        println!("x - y = {:?}", (x - y).element);
        println!("x * y = {:?}", (x * y).element);
        println!("x / y = {:?}", (x / y).element);


<a id="orgf80df41"></a>

## Polynomial over F<sub>p</sub>

    use finite_field::*;
    
    let char:u32 = 2;
    
    let element0 = FiniteField {
        char: char,
        element: Element::PrimeField { element: 0 },
    };
    let element1 = FiniteField {
        char: char,
        element: Element::PrimeField { element: 1 },
    };
    
    let f1: Polynomial = Polynomial {
        char: char,
        coef: vec![element0.clone(), element1.clone()], // 0 + 1*x = x
    };
    let f2: Polynomial = Polynomial {
        char: char,
        coef: vec![element0.clone(), element1.clone(), element1.clone()], // 0 + 1*x + 1*x^2 = x + x^2
    };
    println!("f1 + f2 = {:?}", (f1 + f2).coef);
    println!("f1 - f2 = {:?}", (f1 - f2).coef);
    println!("f1 * f2 = {:?}", (f1 * f2).coef);
    println!("f1 / f2 = {:?}", (f1 / f2).coef);
    println!("f1 % f2 = {:?}", (f1 % f2).coef);

