

<a id='definition__r7rs__set-car_21'></a>

# `set-car!` -- `r7rs` Definition


<a id='definition__r7rs__set-car_21__kind'></a>

#### Kind

`mutator!`;


<a id='definition__r7rs__set-car_21__extended-by'></a>

#### Extended by

 * [`set-car!`](../../vonuvoli/definitions/set-car_21.md#definition__vonuvoli__set-car_21) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__set-car_21__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((pair any) -> (undefined))`
   * inputs:
     * a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__set-car_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__set-car_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__set-car_21__description'></a>

#### Description

> ````
> (set-car! pair obj)
> ````
> 
> 
> Stores `obj` in the car field of `pair`.
> ````
> (define (f) (list 'not-a-constant-list))
> (define (g) '(constant-list))
> (set-car! (f) 3)             ===>  #unspecified
> (set-car! (g) 3)             ===>  #error
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__set-car_21__referenced-types'></a>

#### Referenced-types

 * [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

