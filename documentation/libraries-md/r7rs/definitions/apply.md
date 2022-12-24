

<a id='definition__r7rs__apply'></a>

# `apply` -- `r7rs` Definition


<a id='definition__r7rs__apply__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__apply__implemented-by'></a>

#### Implemented by

 * [`apply`](../../vonuvoli/definitions/apply.md#definition__vonuvoli__apply) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__apply__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((procedure-0) -> (any))`
   * input: a value of type [`procedure-0`](../../r7rs/types/procedure-0.md#type__r7rs__procedure-0);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * `((procedure &variadic any &trailing list-proper) -> (any))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * (variadic) a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * (variadic -- none, or any number of times;)
     * (trailing) a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__apply__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__apply__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__apply__description'></a>

#### Description

> ````
> (apply proc arg_1 ... args)
> ````
> 
> 
> The `apply` procedure calls `proc` with the elements of the list
> `(append (list arg_1 ...) args)` as the actual
> arguments.
> 
> ````
> (apply + (list 3 4))              ===>  7
> 
> (define compose
>   (lambda (f g)
>     (lambda args
>       (f (apply g args)))))
> 
> ((compose sqrt *) 12 75)              ===>  30
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__apply__referenced-types'></a>

#### Referenced-types

 * [`procedure-0`](../../r7rs/types/procedure-0.md#type__r7rs__procedure-0);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

