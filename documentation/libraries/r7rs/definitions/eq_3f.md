

<a id='definition__r7rs__eq_3f'></a>

# `eq?` -- `r7rs` Definition


<a id='definition__r7rs__eq_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__eq_3f__extended-by'></a>

#### Extended by

 * [`equivalent-by-identity?`](../../vonuvoli/definitions/equivalent-by-identity_3f.md#definition__vonuvoli__equivalent-by-identity_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__eq_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any any) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__eq_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__eq_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__eq_3f__description'></a>

#### Description

> ````
> (eq? obj_1 obj_2)
> ````
> 
> A **predicate** is a procedure that always returns a boolean
> value (`#t` or `#f`).  An **equivalence predicate** is
> the computational analogue of a mathematical equivalence relation; it is
> symmetric, reflexive, and transitive.
> 
> Of the equivalence predicates
> described in this section, `eq?` is the finest or most
> discriminating, `equal?` is the coarsest, and `eqv?` is
> slightly less discriminating than `eq?`.
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

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

