

<a id='definition__r7rs__caaar'></a>

# `caaar` -- `r7rs` Definition


<a id='definition__r7rs__caaar__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__caaar__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any) -> (any))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__caaar__exports'></a>

#### Exports

 * [`scheme:cxr`](../../r7rs/exports/scheme_3a_cxr.md#export__r7rs__scheme_3a_cxr) -- `(scheme cxr)`;


<a id='definition__r7rs__caaar__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__caaar__description'></a>

#### Description

> ````
> (caaar pair)
> (caadr pair)
> ...
> (cdddar pair)
> (cddddr pair)
> ````
> 
> 
> These twenty-four procedures are further compositions of `car` and `cdr`
> on the same principles.
> For example, `caddr` could be defined by:
> 
> ````
> (define caddr (lambda (x) (car (cdr (cdr x)))))
> ````
> 
> Arbitrary compositions up to four deep are provided.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__caaar__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__caaar__categories'></a>

#### Categories

 * [`vs:pairs`](../../r7rs/categories/vs_3a_pairs.md#category__r7rs__vs_3a_pairs);
 * [`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);


<a id='definition__r7rs__caaar__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

