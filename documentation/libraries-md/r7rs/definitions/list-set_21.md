

<a id='definition__r7rs__list-set_21'></a>

# `list-set!` -- `r7rs` Definition


<a id='definition__r7rs__list-set_21__kind'></a>

#### Kind

`mutator!`;


<a id='definition__r7rs__list-set_21__extended-by'></a>

#### Extended by

 * [`list-set-car!`](../../vonuvoli/definitions/list-set-car_21.md#definition__vonuvoli__list-set-car_21) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__list-set_21__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((list-not-null range-offset any) -> (undefined))`
   * inputs:
     * a value of type [`list-not-null`](../../r7rs/types/list-not-null.md#type__r7rs__list-not-null);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__list-set_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__list-set_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__list-set_21__description'></a>

#### Description

> ````
> (list-set! list k obj)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `list`.
> 
> The `list-set!` procedure stores `obj` in element `k` of `list`.
> 
> ````
> (let ((ls (list 'one 'two 'five!)))
>   (list-set! ls 2 'three)
>   ls)      ===>  (one two three)
> 
> (list-set! '(0 1 2) 1 "oops")  ===>  #error  ; constant list
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__list-set_21__referenced-types'></a>

#### Referenced-types

 * [`list-not-null`](../../r7rs/types/list-not-null.md#type__r7rs__list-not-null);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

