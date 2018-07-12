

<a id='definition__r7rs__not'></a>

# `not` -- `r7rs` Definitions


<a id='definition__r7rs__not__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__not__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((true) -> (false))`
   * input: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((false) -> (true))`
   * input: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__not__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__not__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__not__description'></a>

#### Description

> ````
> (not obj)
> ````
> 
> 
> The `not` procedure returns `#t` if `obj` is false, and returns
> `#f` otherwise.
> 
> ````
> (not #t)         ===>  #f
> (not 3)          ===>  #f
> (not (list 3))   ===>  #f
> (not #f)         ===>  #t
> (not '())        ===>  #f
> (not (list))     ===>  #f
> (not 'nil)       ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__not__referenced-types'></a>

#### Referenced-types

 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__not__categories'></a>

#### Categories

 * [`vs:booleans`](../../r7rs/categories/vs_3a_booleans.md#category__r7rs__vs_3a_booleans);


<a id='definition__r7rs__not__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

