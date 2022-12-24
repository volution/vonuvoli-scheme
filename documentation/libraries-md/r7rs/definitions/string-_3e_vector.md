

<a id='definition__r7rs__string-_3e_vector'></a>

# `string->vector` -- `r7rs` Definition


<a id='definition__r7rs__string-_3e_vector__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__string-_3e_vector__implemented-by'></a>

#### Implemented by

 * [`string->vector`](../../vonuvoli/definitions/string-_3e_vector.md#definition__vonuvoli__string-_3e_vector) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__string-_3e_vector__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string-empty) -> (vector-empty))`
   * input: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
   * output: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * `((string-not-empty) -> (vector-not-empty))`
   * input: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
   * output: a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
 * `((string range-start) -> (vector))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * `((string range-start range-end) -> (vector))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);


<a id='definition__r7rs__string-_3e_vector__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string-_3e_vector__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-_3e_vector__description'></a>

#### Description

> ````
> (vector->string vector)
> (vector->string vector start)
> (vector->string vector start end)
> (string->vector string)
> (string->vector string start)
> (string->vector string start end)
> ````
> 
> 
> **Domain**:  It is an error if any element of `vector` between `start`
> and `end` is not a character.
> 
> The `vector->string` procedure returns a newly allocated string of the objects contained
> in the elements of `vector`
> between `start` and `end`.
> The `string->vector` procedure returns a newly
> created vector initialized to the elements of the string `string`
> between `start` and `end`.
> 
> In both procedures, order is preserved.
> 
> 
> ````
> (string->vector "ABC")          ===>   #(#\A #\B #\C)
> (vector->string #(#\1 #\2 #\3)  ===>   "123"
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string-_3e_vector__referenced-types'></a>

#### Referenced-types

 * [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

