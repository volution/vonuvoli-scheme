

<a id='definition__r7rs__make-vector'></a>

# `make-vector` -- `r7rs` Definition


<a id='definition__r7rs__make-vector__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__make-vector__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((range-length-zero) -> (vector-empty))`
   * input: a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
   * output: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * `((range-length-zero any) -> (vector-empty))`
   * inputs:
     * a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * `((range-length-not-zero) -> (vector-not-empty))`
   * input: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
   * output: a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
 * `((range-length-not-zero any) -> (vector-not-empty))`
   * inputs:
     * a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);


<a id='definition__r7rs__make-vector__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__make-vector__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__make-vector__description'></a>

#### Description

> ````
> (make-vector k)
> (make-vector k fill)
> ````
> 
> 
> Returns a newly allocated vector of `k` elements.  If a second
> argument is given, then each element is initialized to `fill`.
> Otherwise the initial contents of each element is unspecified.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__make-vector__referenced-types'></a>

#### Referenced-types

 * [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
 * [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
 * [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);


<a id='definition__r7rs__make-vector__categories'></a>

#### Categories

 * [`vs:vectors`](../../vonuvoli/categories/vs_3a_vectors.md#category__vonuvoli__vs_3a_vectors);


<a id='definition__r7rs__make-vector__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

