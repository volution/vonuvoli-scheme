

<a id='definition__r7rs__make-promise'></a>

# `make-promise` -- `r7rs` Definition


<a id='definition__r7rs__make-promise__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__make-promise__implemented-by'></a>

#### Implemented by

 * [`make-promise`](../../vonuvoli/definitions/make-promise.md#definition__vonuvoli__make-promise) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__make-promise__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any) -> (promise))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);


<a id='definition__r7rs__make-promise__exports'></a>

#### Exports

 * [`scheme:lazy`](../../r7rs/exports/scheme_3a_lazy.md#export__r7rs__scheme_3a_lazy) -- `(scheme lazy)`;


<a id='definition__r7rs__make-promise__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__make-promise__description'></a>

#### Description

> ````
> (make-promise obj)
> ````
> 
> 
> The `make-promise` procedure returns a promise which, when forced, will return
> `obj`.  It is similar to `delay`, but does not delay
> its argument: it is a procedure rather than syntax.
> If `obj` is already a promise, it is returned.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__make-promise__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

