

<a id='definition__r7rs__eqv_3f'></a>

# `eqv?` -- `r7rs` Definition


<a id='definition__r7rs__eqv_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__eqv_3f__extended-by'></a>

#### Extended by

 * [`equivalent-by-value-strict?`](../../vonuvoli/definitions/equivalent-by-value-strict_3f.md#definition__vonuvoli__equivalent-by-value-strict_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__eqv_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any any) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__eqv_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__eqv_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__eqv_3f__description'></a>

#### Description

> ````
> (eqv? obj_1 obj_2)
> ````
> 
> 
> The `eqv?` procedure defines a useful equivalence relation on objects.
> Briefly, it returns `#t` if `obj_1` and `obj_2` are
> normally regarded as the same object.  This relation is left slightly
> open to interpretation, but the following partial specification of
> `eqv?` holds for all implementations of Scheme.
> 
> 
> The `eqv?` procedure returns `#t` if:
> 
>   * `obj_1` and `obj_2` are both `#t` or both `#f`.
> 
>   * `obj_1` and `obj_2` are both symbols and are the same
> symbol according to the `symbol=?` procedure.
> 
>   * `obj_1` and `obj_2` are both exact numbers and
> are numerically equal (in the sense of `=`).
> 
>   * `obj_1` and `obj_2` are both inexact numbers such that
> they are numerically equal (in the sense of `=`)
> and they yield the same results (in the sense of `eqv?`)
> when passed as arguments to any other procedure
> that can be defined as a finite composition of Scheme's standard
> arithmetic procedures, provided it does not result in a `NaN` value.
> 
>   * `obj_1` and `obj_2` are both characters and are the same
> character according to the `char=?` procedure.
> 
>   * `obj_1` and `obj_2` are both the empty list.
> 
>   * `obj_1` and `obj_2` are pairs, vectors, bytevectors, records,
> or strings that denote the same location in the store.
> 
>   * `obj_1` and `obj_2` are procedures whose location tags are
> equal.
> 
> 
> The `eqv?` procedure returns `#f` if:
> 
>   * `obj_1` and `obj_2` are of different types.
> 
>   * one of `obj_1` and `obj_2` is `#t` but the other is
> `#f`.
> 
>   * `obj_1` and `obj_2` are symbols but are not the same
> symbol according to the `symbol=?` procedure.
> 
>   * one of `obj_1` and `obj_2` is an exact number but the other
> is an inexact number.
> 
>   * `obj_1` and `obj_2` are both exact numbers and
> are numerically unequal (in the sense of `=`).
> 
>   * `obj_1` and `obj_2` are both inexact numbers such that either
> they are numerically unequal (in the sense of `=`),
> or they do not yield the same results (in the sense of `eqv?`)
> when passed as arguments to any other procedure
> that can be defined as a finite composition of Scheme's standard
> arithmetic procedures, provided it does not result in a `NaN` value.
> As an exception, the behavior of `eqv?` is unspecified
> when both `obj_1` and `obj_2` are `NaN`.
> 
>   * `obj_1` and `obj_2` are characters for which the `char=?`
> procedure returns `#f`.
> 
>   * one of `obj_1` and `obj_2` is the empty list but the other
> is not.
> 
>   * `obj_1` and `obj_2` are pairs, vectors, bytevectors, records,
> or strings that denote distinct locations.
> 
>   * `obj_1` and `obj_2` are procedures that would behave differently
> (return different values or have different side effects) for some arguments.
> 
> 
> ````
> (eqv? 'a 'a)                     ===>  #t
> (eqv? 'a 'b)                     ===>  #f
> (eqv? 2 2)                       ===>  #t
> (eqv? 2 2.0)                     ===>  #f
> (eqv? '() '())                   ===>  #t
> (eqv? 100000000 100000000)       ===>  #t
> (eqv? 0.0 +nan.0)                ===>  #f
> (eqv? (cons 1 2) (cons 1 2))     ===>  #f
> (eqv? (lambda () 1)
>       (lambda () 2))             ===>  #f
> (let ((p (lambda (x) x)))
>   (eqv? p p))                    ===>  #t
> (eqv? #f 'nil)                   ===>  #f
> ````
> 
> 
> The following examples illustrate cases in which the above rules do
> not fully specify the behavior of `eqv?`.  All that can be said
> about such cases is that the value returned by `eqv?` must be a
> boolean.
> 
> ````
> (eqv? "" "")             ===>  #unspecified
> (eqv? '#() '#())         ===>  #unspecified
> (eqv? (lambda (x) x)
>       (lambda (x) x))    ===>  #unspecified
> (eqv? (lambda (x) x)
>       (lambda (y) y))    ===>  #unspecified
> (eqv? 1.0e0 1.0f0)       ===>  #unspecified
> (eqv? +nan.0 +nan.0)     ===>  #unspecified
> ````
> 
> Note that `(eqv? 0.0 -0.0)` will return `#f` if negative zero
> is distinguished, and `#t` if negative zero is not distinguished.
> 
> 
> Since it is an error to modify constant objects (those returned by
> literal expressions), implementations may
> share structure between constants where appropriate.  Thus
> the value of `eqv?` on constants is sometimes
> implementation-dependent.
> 
> ````
> (eqv? '(a) '(a))                 ===>  #unspecified
> (eqv? "a" "a")                   ===>  #unspecified
> (eqv? '(b) (cdr '(a b)))         ===>  #unspecified
> (let ((x '(a)))
>   (eqv? x x))                    ===>  #t
> ````
> 
> 
> The above definition of `eqv?` allows implementations latitude in
> their treatment of procedures and literals:  implementations may
> either detect or fail to detect that two procedures or two literals
> are equivalent to each other, and can decide whether or not to
> merge representations of equivalent objects by using the same pointer or
> bit pattern to represent both.
> 
> **Note**:  If inexact numbers are represented as IEEE binary floating-point numbers,
> then an implementation of `eqv?` that simply compares equal-sized
> inexact numbers for bitwise equality is correct by the above definition.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__eqv_3f__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

