

<a id='definition__r7rs__string-map'></a>

# `string-map` -- `r7rs` Definition


<a id='definition__r7rs__string-map__kind'></a>

#### Kind

`functor`;


<a id='definition__r7rs__string-map__implemented-by'></a>

#### Implemented by

 * [`string-map`](../../vonuvoli/definitions/string-map.md#definition__vonuvoli__string-map) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__string-map__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((map-procedure string |1...|) -> (any))`
   * inputs:
     * a value of type [`map-procedure`](../../r7rs/types/map-procedure.md#type__r7rs__map-procedure);
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` -- at least one time;
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__string-map__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string-map__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-map__description'></a>

#### Description

> ````
> (string-map proc string_1 string_2 ...)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not
> accept as many arguments as there are `string`s
> and return a single character.
> 
> The `string-map` procedure applies `proc` element-wise to the elements of the
> `string`s and returns a string of the results, in order.
> If more than one `string` is given and not all strings have the same length,
> `string-map` terminates when the shortest string runs out.
> The dynamic order in which `proc` is applied to the elements of the
> `string`s is unspecified.
> If multiple returns occur from `string-map`,
> the values returned by earlier returns are not mutated.
> 
> ````
> (string-map char-foldcase "AbdEgH") ===>  "abdegh"
> 
> (string-map
>  (lambda (c)
>    (integer->char (+ 1 (char->integer c))))
>  "HAL")                ===>  "IBM"
> 
> (string-map
>  (lambda (c k)
>    ((if (eqv? k #\u) char-upcase char-downcase)
>     c))
>  "studlycaps xxx"
>  "ululululul")   ===>   "StUdLyCaPs"
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string-map__referenced-types'></a>

#### Referenced-types

 * [`map-procedure`](../../r7rs/types/map-procedure.md#type__r7rs__map-procedure);
 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

