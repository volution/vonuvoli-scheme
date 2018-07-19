

<a id='definition__r7rs__list-ref'></a>

# `list-ref` -- `r7rs` Definition


<a id='definition__r7rs__list-ref__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__list-ref__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((list range-offset) -> (any))`
   * inputs:
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__list-ref__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__list-ref__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__list-ref__description'></a>

#### Description

> ````
> (list-ref list k)
> ````
> 
> 
> **Domain**:  The `list` argument can be circular, but
> it is an error if `list` has fewer than `k` elements.
> 
> Returns the `k`th element of `list`.  (This is the same
> as the car of `(list-tail list k)`.)
> 
> ````
> (list-ref '(a b c d) 2)           ===>  c
> (list-ref '(a b c d)
>     (exact (round 1.8)))          ===>  c
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__list-ref__referenced-types'></a>

#### Referenced-types

 * [`list`](../../r7rs/types/list.md#type__r7rs__list);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__list-ref__categories'></a>

#### Categories

 * [`vs:lists`](../../vonuvoli/categories/vs_3a_lists.md#category__vonuvoli__vs_3a_lists);


<a id='definition__r7rs__list-ref__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

