

<a id='definition__r7rs__number_3f'></a>

# `number?` -- `r7rs` Definition


<a id='definition__r7rs__number_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__number_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number) -> (true))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any ...) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


<a id='definition__r7rs__number_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__number_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__number_3f__description'></a>

#### Description

> ````
> (number? obj)
> (complex? obj)
> (real? obj)
> (rational? obj)
> (integer? obj)
> ````
> 
> 
> These numerical type predicates can be applied to any kind of
> argument, including non-numbers.  They return `#t` if the object is
> of the named type, and otherwise they return `#f`.
> In general, if a type predicate is true of a number then all higher
> type predicates are also true of that number.  Consequently, if a type
> predicate is false of a number, then all lower type predicates are
> also false of that number.
> 
> If `z` is a complex number, then `(real? z)` is true if
> and only if `(zero? (imag-part z))` is true.
> If `x` is an inexact real number, then
> `(integer? x)` is true if and only if `(= x (round x))`.
> 
> The numbers `+inf.0`, `-inf.0`, and `+nan.0` are real but
> not rational.
> 
> 
> ````
> (complex? 3+4i)         ===>  #t
> (complex? 3)            ===>  #t
> (real? 3)               ===>  #t
> (real? -2.5+0i)         ===>  #t
> (real? -2.5+0.0i)       ===>  #f
> (real? #e1e10)          ===>  #t
> (real? +inf.0)          ===>  #t
> (real? +nan.0)          ===>  #t
> (rational? -inf.0)      ===>  #f
> (rational? 3.5)         ===>  #t
> (rational? 6/10)        ===>  #t
> (rational? 6/3)         ===>  #t
> (integer? 3+0i)         ===>  #t
> (integer? 3.0)          ===>  #t
> (integer? 8/4)          ===>  #t
> ````
> 
> 
> **Note**: The behavior of these type predicates on __inexact__ numbers
> is unreliable, since any inaccuracy might affect the result.
> 
> **Note**:  In many implementations the `complex?` procedure will be the same as
> `number?`, but unusual implementations may represent
> some irrational numbers exactly or may extend the number system to
> support some kind of non-complex numbers.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__number_3f__referenced-types'></a>

#### Referenced-types

 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__number_3f__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);
 * [`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);


<a id='definition__r7rs__number_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

