

<a id='definition__r7rs__string-fill_21'></a>

# `string-fill!` -- `r7rs` Definition


<a id='definition__r7rs__string-fill_21__kind'></a>

#### Kind

`mutator!`;


<a id='definition__r7rs__string-fill_21__implemented-by'></a>

#### Implemented by

 * [`string-fill!`](../../vonuvoli/definitions/string-fill_21.md#definition__vonuvoli__string-fill_21) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__string-fill_21__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string character) -> (undefined))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((string character range-start) -> (undefined))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((string character range-start range-end) -> (undefined))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__string-fill_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string-fill_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-fill_21__description'></a>

#### Description

> ````
> (string-fill! string fill)
> (string-fill! string fill start)
> (string-fill! string fill start end)
> ````
> 
> 
> **Domain**:  It is an error if `fill` is not a character.
> 
> The `string-fill!` procedure stores `fill`
> in the elements of `string`
> between `start` and `end`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string-fill_21__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`character`](../../r7rs/types/character.md#type__r7rs__character);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

