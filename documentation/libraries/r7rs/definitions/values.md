

<a id='definition__r7rs__values'></a>

# `values` -- `r7rs` Definition


<a id='definition__r7rs__values__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__values__implemented-by'></a>

#### Implemented by

 * [`values`](../../vonuvoli/definitions/values.md#definition__vonuvoli__values) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__values__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> ())`
   * inputs: none;
   * outputs: none;
 * `((any) -> (any))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * `((any |2...|) -> (any |2...|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` -- at least 2 times;
   * outputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` -- at least 2 times;


<a id='definition__r7rs__values__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__values__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__values__description'></a>

#### Description

> ````
> (values obj ...)
> ````
> 
> 
> Delivers all of its arguments to its continuation.
> The `values` procedure might be defined as follows:
> ````
> (define (values . things)
>   (call-with-current-continuation
>     (lambda (cont) (apply cont things))))
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__values__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

