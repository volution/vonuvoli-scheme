

<a id='definition__r7rs__vector-set_21'></a>

# `vector-set!` -- `r7rs` Definition


<a id='definition__r7rs__vector-set_21__kind'></a>

#### Kind

`mutator!`;


<a id='definition__r7rs__vector-set_21__extended-by'></a>

#### Extended by

 * [`vector-set!`](../../vonuvoli/definitions/vector-set_21.md#definition__vonuvoli__vector-set_21) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__vector-set_21__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((vector-not-empty range-offset any) -> (undefined))`
   * inputs:
     * a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__vector-set_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-set_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-set_21__description'></a>

#### Description

> ````
> (vector-set! vector k obj)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `vector`.
> 
> The `vector-set!` procedure stores `obj` in element `k` of `vector`.
> ````
> (let ((vec (vector 0 '(2 2 2 2) "Anna")))
>   (vector-set! vec 1 '("Sue" "Sue"))
>   vec)      ===>  #(0 ("Sue" "Sue") "Anna")
> 
> (vector-set! '#(0 1 2) 1 "doe")  ===>  #error  ; constant vector
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__vector-set_21__referenced-types'></a>

#### Referenced-types

 * [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

