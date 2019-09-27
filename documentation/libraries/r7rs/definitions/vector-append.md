

<a id='definition__r7rs__vector-append'></a>

# `vector-append` -- `r7rs` Definition


<a id='definition__r7rs__vector-append__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__vector-append__implemented-by'></a>

#### Implemented by

 * [`vector-append`](../../vonuvoli/definitions/vector-append.md#definition__vonuvoli__vector-append) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__vector-append__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (vector-empty))`
   * inputs: none;
   * output: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * `((vector |1...|) -> (vector))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `...` -- at least one time;
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);


<a id='definition__r7rs__vector-append__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-append__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-append__description'></a>

#### Description

> ````
> (vector-append vector ...)
> ````
> 
> 
> Returns a newly allocated vector whose elements are the concatenation
> of the elements of the given vectors.
> 
> ````
> (vector-append #(a b c) #(d e f))  ===>  #(a b c d e f)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__vector-append__referenced-types'></a>

#### Referenced-types

 * [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

