

<a id='definition__r7rs__string-map'></a>

# `string-map` -- `r7rs` Definition


<a id='definition__r7rs__string-map__kind'></a>

#### Kind

`functor`;


<a id='definition__r7rs__string-map__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((procedure string ...) -> (any))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` (i.e. variadic);
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

 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__string-map__categories'></a>

#### Categories

 * [`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
 * [`vs:functions`](../../r7rs/categories/vs_3a_functions.md#category__r7rs__vs_3a_functions);
 * [`vs:conversions`](../../r7rs/categories/vs_3a_conversions.md#category__r7rs__vs_3a_conversions);
 * [`vs:loops`](../../r7rs/categories/vs_3a_loops.md#category__r7rs__vs_3a_loops);


<a id='definition__r7rs__string-map__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

