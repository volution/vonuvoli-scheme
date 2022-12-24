

<a id='definition__r7rs__caar'></a>

# `caar` -- `r7rs` Definition


<a id='definition__r7rs__caar__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__caar__implemented-by'></a>

#### Implemented by

 * [`caar`](../../vonuvoli/definitions/caar.md#definition__vonuvoli__caar) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__caar__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any) -> (any))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__caar__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__caar__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__caar__description'></a>

#### Description

> ````
> (caar pair)
> (cadr pair)
> (cdar pair)
> (cddr pair)
> ````
> 
> 
> These procedures are compositions of `car` and `cdr` as follows:
> 
> ````
> (define (caar x) (car (car x)))
> (define (cadr x) (car (cdr x)))
> (define (cdar x) (cdr (car x)))
> (define (cddr x) (cdr (cdr x)))
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__caar__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

