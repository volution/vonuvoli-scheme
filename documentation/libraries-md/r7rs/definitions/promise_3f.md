

<a id='definition__r7rs__promise_3f'></a>

# `promise?` -- `r7rs` Definition


<a id='definition__r7rs__promise_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__promise_3f__extended-by'></a>

#### Extended by

 * [`promise?`](../../vonuvoli/definitions/promise_3f.md#definition__vonuvoli__promise_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__promise_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((promise) -> (true))`
   * input: a value of type [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__promise_3f__exports'></a>

#### Exports

 * [`scheme:lazy`](../../r7rs/exports/scheme_3a_lazy.md#export__r7rs__scheme_3a_lazy) -- `(scheme lazy)`;


<a id='definition__r7rs__promise_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__promise_3f__description'></a>

#### Description

> ````
> (promise? obj)
> ````
> 
> 
> The `promise?` procedure returns
> `#t` if its argument is a promise, and `#f` otherwise.  Note
> that promises are not necessarily disjoint from other Scheme types such
> as procedures.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__promise_3f__referenced-types'></a>

#### Referenced-types

 * [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

