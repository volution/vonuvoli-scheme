

<a id='definition__r7rs__cons'></a>

# `cons` -- `r7rs` Definition


<a id='definition__r7rs__cons__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__cons__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any any) -> (pair))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);


<a id='definition__r7rs__cons__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__cons__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__cons__description'></a>

#### Description

> ````
> (cons obj_1 obj_2)
> ````
> 
> 
> Returns a newly allocated pair whose car is `obj_1` and whose cdr is
> `obj_2`.  The pair is guaranteed to be different (in the sense of
> `eqv?`) from every existing object.
> 
> ````
> (cons 'a '())           ===>  (a)
> (cons '(a) '(b c d))    ===>  ((a) b c d)
> (cons "a" '(b c))       ===>  ("a" b c)
> (cons 'a 3)             ===>  (a . 3)
> (cons '(a b) 'c)        ===>  ((a b) . c)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__cons__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);


<a id='definition__r7rs__cons__categories'></a>

#### Categories

 * [`vs:pairs`](../../vonuvoli/categories/vs_3a_pairs.md#category__vonuvoli__vs_3a_pairs);
 * [`vs:lists`](../../vonuvoli/categories/vs_3a_lists.md#category__vonuvoli__vs_3a_lists);


<a id='definition__r7rs__cons__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

