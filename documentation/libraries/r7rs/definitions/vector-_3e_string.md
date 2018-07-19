

<a id='definition__r7rs__vector-_3e_string'></a>

# `vector->string` -- `r7rs` Definition


<a id='definition__r7rs__vector-_3e_string__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__vector-_3e_string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((vector-empty) -> (string-empty))`
   * input: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((vector-not-empty) -> (string-not-empty))`
   * input: a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * `((vector range-start) -> (string))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * `((vector range-start range-end) -> (string))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='definition__r7rs__vector-_3e_string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-_3e_string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-_3e_string__description'></a>

#### Description

> Please refer to [`string->vector`](../../r7rs/definitions/string-_3e_vector.md#definition__r7rs__string-_3e_vector).


<a id='definition__r7rs__vector-_3e_string__referenced-types'></a>

#### Referenced-types

 * [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
 * [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


<a id='definition__r7rs__vector-_3e_string__categories'></a>

#### Categories

 * [`vs:strings`](../../vonuvoli/categories/vs_3a_strings.md#category__vonuvoli__vs_3a_strings);
 * [`vs:vectors`](../../vonuvoli/categories/vs_3a_vectors.md#category__vonuvoli__vs_3a_vectors);
 * [`vs:conversions`](../../vonuvoli/categories/vs_3a_conversions.md#category__vonuvoli__vs_3a_conversions);


<a id='definition__r7rs__vector-_3e_string__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

