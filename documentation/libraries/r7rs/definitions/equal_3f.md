

<a id='definition__r7rs__equal_3f'></a>

# `equal?` -- `r7rs` Definition


<a id='definition__r7rs__equal_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__equal_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any any) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * `((any) -> (true))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
   * requires: `vonuvoli`
 * `((any ...) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


<a id='definition__r7rs__equal_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__equal_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__equal_3f__description'></a>

#### Description

> ````
> (equal? obj_1 obj_2)
> ````
> 
> 
> The `equal?` procedure, when applied to pairs, vectors, strings and
> bytevectors, recursively compares them, returning `#t` when the
> unfoldings of its arguments into (possibly infinite) trees are equal
> (in the sense of `equal?`)
> as ordered trees, and `#f` otherwise.  It returns the same as
> `eqv?` when applied to booleans, symbols, numbers, characters,
> ports, procedures, and the empty list.  If two objects are `eqv?`,
> they must be `equal?` as well.  In all other cases, `equal?`
> may return either `#t` or `#f`.
> 
> Note that records are `equal?` if their record types are the same
> and their correspondingly named fields are `equal?`.
> 
> Even if its arguments are
> circular data structures, `equal?` must always terminate.
> 
> 
> ````
> (equal? 'a 'a)                  ===>  #t
> (equal? '(a) '(a))              ===>  #t
> (equal? '(a (b) c)
>         '(a (b) c))             ===>  #t
> (equal? "abc" "abc")            ===>  #t
> (equal? 2 2)                    ===>  #t
> (equal? (make-vector 5 'a)
>         (make-vector 5 'a))     ===>  #t
> (equal? '#1=(a b . #1#)
>         '#2=(a b a b . #2#))    ===>  #t
> (equal? (lambda (x) x)
>         (lambda (y) y))         ===>  #unspecified
> ````
> 
> 
> **Note**:  A rule of thumb is that objects are generally `equal?` if they print
> the same.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__equal_3f__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);


<a id='definition__r7rs__equal_3f__categories'></a>

#### Categories

 * [`vs:equivalence`](../../vonuvoli/categories/vs_3a_equivalence.md#category__vonuvoli__vs_3a_equivalence);


<a id='definition__r7rs__equal_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

