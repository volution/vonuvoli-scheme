

<a id='definition__r7rs__list'></a>

# `list` -- `r7rs` Definition


<a id='definition__r7rs__list__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__list__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (null))`
   * inputs: none;
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((any ...) -> (list-proper))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


<a id='definition__r7rs__list__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__list__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__list__description'></a>

#### Description

> ````
> (list obj ...)
> ````
> 
> 
> Returns a newly allocated list of its arguments.
> 
> ````
> (list 'a (+ 3 4) 'c)            ===>  (a 7 c)
> (list)                          ===>  ()
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__list__referenced-types'></a>

#### Referenced-types

 * [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


<a id='definition__r7rs__list__categories'></a>

#### Categories

 * [`vs:lists`](../../vonuvoli/categories/vs_3a_lists.md#category__vonuvoli__vs_3a_lists);


<a id='definition__r7rs__list__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

