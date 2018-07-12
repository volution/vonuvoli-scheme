

<a id='definition__r7rs__eq_3f'></a>

# `eq?` -- `r7rs` Definitions


<a id='definition__r7rs__eq_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__eq_3f__procedure-signature'></a>

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


<a id='definition__r7rs__eq_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__eq_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__eq_3f__description'></a>

#### Description

> ````
> (eq? obj_1 obj_2)
> ````
> 
> 
> The `eq?` procedure is similar to `eqv?` except that in some cases it is
> capable of discerning distinctions finer than those detectable by
> `eqv?`.  It must always return `#f` when `eqv?` also
> would, but may return `#f` in some cases where `eqv?` would return `#t`.
> 
> On symbols, booleans, the empty list, pairs, and records,
> and also on non-empty
> strings, vectors, and bytevectors, `eq?` and `eqv?` are guaranteed to have the same
> behavior.  On procedures, `eq?` must return true if the arguments' location
> tags are equal.  On numbers and characters, `eq?`'s behavior is
> implementation-dependent, but it will always return either true or
> false.  On empty strings, empty vectors, and empty bytevectors, `eq?` may also behave
> differently from `eqv?`.
> 
> 
> ````
> (eq? 'a 'a)                     ===>  #t
> (eq? '(a) '(a))                 ===>  #unspecified
> (eq? (list 'a) (list 'a))       ===>  #f
> (eq? "a" "a")                   ===>  #unspecified
> (eq? "" "")                     ===>  #unspecified
> (eq? '() '())                   ===>  #t
> (eq? 2 2)                       ===>  #unspecified
> (eq? #\A #\A)                   ===>  #unspecified
> (eq? car car)                   ===>  #t
> (let ((n (+ 2 3)))
>   (eq? n n))                    ===>  #unspecified
> (let ((x '(a)))
>   (eq? x x))                    ===>  #t
> (let ((x '#()))
>   (eq? x x))                    ===>  #t
> (let ((p (lambda (x) x)))
>   (eq? p p))                    ===>  #t
> ````
> 
> 
> **Rationale**:  It will usually be possible to implement `eq?` much
> more efficiently than `eqv?`, for example, as a simple pointer
> comparison instead of as some more complicated operation.  One reason is
> that it is not always possible to compute `eqv?` of two numbers in
> constant time, whereas `eq?` implemented as pointer comparison will
> always finish in constant time.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__eq_3f__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);


<a id='definition__r7rs__eq_3f__categories'></a>

#### Categories

 * [`vs:equivalence`](../../r7rs/categories/vs_3a_equivalence.md#category__r7rs__vs_3a_equivalence);


<a id='definition__r7rs__eq_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

