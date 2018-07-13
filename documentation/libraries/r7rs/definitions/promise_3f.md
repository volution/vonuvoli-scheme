

<a id='definition__r7rs__promise_3f'></a>

# `promise?` -- `r7rs` Definitions


<a id='definition__r7rs__promise_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__promise_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((promise) -> (true))`
   * input: a value of type [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any ...) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


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
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__promise_3f__categories'></a>

#### Categories

 * [`vs:promises`](../../r7rs/categories/vs_3a_promises.md#category__r7rs__vs_3a_promises);
 * [`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);


<a id='definition__r7rs__promise_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

