

<a id='definition__r7rs__call-with-values'></a>

# `call-with-values` -- `r7rs` Definitions


<a id='definition__r7rs__call-with-values__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__call-with-values__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((producer . procedure) (consumer . procedure)) -> (any))`
   * inputs:
     * `producer` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * `consumer` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__call-with-values__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__call-with-values__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__call-with-values__description'></a>

#### Description

> ````
> (call-with-values producer consumer)
> ````
> 
> 
> Calls its `producer` argument with no arguments and
> a continuation that, when passed some values, calls the
> `consumer` procedure with those values as arguments.
> The continuation for the call to `consumer` is the
> continuation of the call to `call-with-values`.
> 
> ````
> (call-with-values (lambda () (values 4 5))
>                   (lambda (a b) b))
>                                                    ===>  5
> 
> (call-with-values * -)                             ===>  -1
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__call-with-values__referenced-types'></a>

#### Referenced-types

 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__call-with-values__categories'></a>

#### Categories

 * [`vs:functions`](../../r7rs/categories/vs_3a_functions.md#category__r7rs__vs_3a_functions);
 * [`vs:values`](../../r7rs/categories/vs_3a_values.md#category__r7rs__vs_3a_values);


<a id='definition__r7rs__call-with-values__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

