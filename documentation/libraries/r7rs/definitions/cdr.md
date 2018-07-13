

<a id='definition__r7rs__cdr'></a>

# `cdr` -- `r7rs` Definitions


<a id='definition__r7rs__cdr__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__cdr__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((pair) -> (any))`
   * input: a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__cdr__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__cdr__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__cdr__description'></a>

#### Description

> ````
> (cdr pair)
> ````
> 
> 
> Returns the contents of the cdr field of `pair`.
> Note that it is an error to take the cdr of the empty list.
> 
> ````
> (cdr '((a) b c d))      ===>  (b c d)
> (cdr '(1 . 2))          ===>  2
> (cdr '())               ===>  #error
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__cdr__referenced-types'></a>

#### Referenced-types

 * [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__cdr__categories'></a>

#### Categories

 * [`vs:pairs`](../../r7rs/categories/vs_3a_pairs.md#category__r7rs__vs_3a_pairs);
 * [`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);


<a id='definition__r7rs__cdr__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

