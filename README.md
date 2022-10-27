
# Table of Contents

1.  [finite fields](#org5fe75b0)
2.  [What makes it different from other libraries?](#org1ec413c)
    1.  [Pros:](#org3b73da4)
    2.  [Cons:](#orgf417e11)
3.  [Usage](#org2aba058)
4.  [Examples](#org9688336)
    1.  [Prime Field](#orgb937e46)
    2.  [Galois Field](#orgcf35253)
    3.  [Polynomial over F<sub>p</sub>](#org8f20a17)
    4.  [Polynomial over GF(p<sup>n</sup>)](#orgbb3b7c4)


<a id="org5fe75b0"></a>

# finite fields

A Rust library for operations on finite field, featuring:

-   Sum, difference, product, and quotient of elements F<sub>p</sub>
-   Sum, difference, product, and quotient of elements GF(p<sup>n</sup>)
-   Obtaining the primitive polynomial
-   Sum, difference, product, quotient, and remainder of polynomial over F<sub>p</sub>
-   Sum, difference, product, quotient, and remainder of polynomial over GF(p<sup>n</sup>)


<a id="org1ec413c"></a>

# What makes it different from other libraries?


<a id="org3b73da4"></a>

## Pros:

-   Can be calculated with F<sub>p</sub>, GF(p<sup>n</sup>) for any prime and any multiplier, not limited to a char 2.
-   Can freely compute four types of element: prime, Galois, polynomial of prime, and polynomial of Galois.
-   Each can be calculated with +-\*/, so you can write natural code.


<a id="orgf417e11"></a>

## Cons:

-   It takes longer than other libraries because it is not optimized for each character.


<a id="org2aba058"></a>

# Usage

Add this to your Cargo.toml:

    [dependencies]
    finite_fields = "0.1.0"


<a id="org9688336"></a>

# Examples


<a id="orgb937e46"></a>

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


<a id="orgcf35253"></a>

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


<a id="org8f20a17"></a>

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


<a id="orgbb3b7c4"></a>

## Polynomial over GF(p<sup>n</sup>)

Same as above

