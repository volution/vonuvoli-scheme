

<a id='definition__r7rs__car'></a>

# `car` -- `r7rs` Definition


<a id='definition__r7rs__car__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__car__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((pair) -> (any))`
   * input: a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__car__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__car__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__car__description'></a>

#### Description

> ````
> (car pair)
> ````
> 
> 
> Returns the contents of the car field of `pair`.  Note that it is an
> error to take the car of the __empty list__.
> 
> ````
> (car '(a b c))          ===>  a
> (car '((a) b c d))      ===>  (a)
> (car '(1 . 2))          ===>  1
> (car '())               ===>  #error
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__car__referenced-types'></a>

#### Referenced-types

 * [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__car__categories'></a>

#### Categories

 * [`vs:pairs`](../../vonuvoli/categories/vs_3a_pairs.md#category__vonuvoli__vs_3a_pairs);
 * [`vs:lists`](../../vonuvoli/categories/vs_3a_lists.md#category__vonuvoli__vs_3a_lists);


<a id='definition__r7rs__car__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

