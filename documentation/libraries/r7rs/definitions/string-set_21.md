

<a id='definition__r7rs__string-set_21'></a>

# `string-set!` -- `r7rs` Definition


<a id='definition__r7rs__string-set_21__kind'></a>

#### Kind

`mutator!`;


<a id='definition__r7rs__string-set_21__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string range-offset character) -> (undefined))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(not vonuvoli)`
 * `((string range-offset character) -> (character))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * requires: `vonuvoli`


<a id='definition__r7rs__string-set_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string-set_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-set_21__description'></a>

#### Description

> ````
> (string-set! string k char)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `string`.
> 
> The `string-set!` procedure stores `char` in element `k` of `string`.
> There is no requirement for this procedure to execute in constant time.
> 
> ````
> (define (f) (make-string 3 #\*))
> (define (g) "***")
> (string-set! (f) 0 #\?)  ===>  #unspecified
> (string-set! (g) 0 #\?)  ===>  #error
> (string-set! (symbol->string 'immutable)
>              0
>              #\?)        ===>  #error
> ````
> 
> 
> **Note**:  There is no requirement for this procedure to execute in constant time.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string-set_21__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`character`](../../r7rs/types/character.md#type__r7rs__character);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__string-set_21__categories'></a>

#### Categories

 * [`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);


<a id='definition__r7rs__string-set_21__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

