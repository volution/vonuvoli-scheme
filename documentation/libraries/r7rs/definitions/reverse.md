

<a id='definition__r7rs__reverse'></a>

# `reverse` -- `r7rs` Definition


<a id='definition__r7rs__reverse__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__reverse__implemented-by'></a>

#### Implemented by

 * [`reverse`](../../vonuvoli/definitions/reverse.md#definition__vonuvoli__reverse) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__reverse__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((null) -> (null))`
   * input: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((list-proper-not-null) -> (list-proper-not-null))`
   * input: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
   * output: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);


<a id='definition__r7rs__reverse__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__reverse__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__reverse__description'></a>

#### Description

> ````
> (reverse list)
> ````
> 
> 
> Returns a newly allocated list consisting of the elements of `list`
> in reverse order.
> 
> ````
> (reverse '(a b c))              ===>  (c b a)
> (reverse '(a (b c) d (e (f))))  ===>  ((e (f)) d (b c) a)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__reverse__referenced-types'></a>

#### Referenced-types

 * [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

