

<a id='definition__r7rs__string_3c_3f'></a>

# `string<?` -- `r7rs` Definition


<a id='definition__r7rs__string_3c_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__string_3c_3f__implemented-by'></a>

#### Implemented by

 * [`string<?`](../../vonuvoli/definitions/string_3c_3f.md#definition__vonuvoli__string_3c_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__string_3c_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string |2...|) -> (boolean))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` -- at least 2 times;
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__string_3c_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string_3c_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string_3c_3f__description'></a>

#### Description

> ````
> (string<? string_1 string_2 string_3 ...)
> (string-ci<? string_1 string_2 string_3 ...)
> (string>? string_1 string_2 string_3 ...)
> (string-ci>? string_1 string_2 string_3 ...)
> (string<=? string_1 string_2 string_3 ...)
> (string-ci<=? string_1 string_2 string_3 ...)
> (string>=? string_1 string_2 string_3 ...)
> (string-ci>=? string_1 string_2 string_3 ...)
> ````
> 
> 
> These procedures return `#t` if their arguments are (respectively):
> monotonically increasing, monotonically decreasing,
> monotonically non-decreasing, or monotonically non-increasing.
> 
> These predicates are required to be transitive.
> 
> These procedures compare strings in an implementation-defined way.
> One approach is to make them the lexicographic extensions to strings of
> the corresponding orderings on characters.  In that case, `string<?`
> would be the lexicographic ordering on strings induced by the ordering
> `char<?` on characters, and if the two strings differ in length but
> are the same up to the length of the shorter string, the shorter string
> would be considered to be lexicographically less than the longer string.
> However, it is also permitted to use the natural ordering imposed by the
> implementation's internal representation of strings, or a more complex locale-specific
> ordering.
> 
> In all cases, a pair of strings must satisfy exactly one of
> `string<?`, `string=?`, and `string>?`, and must satisfy
> `string<=?` if and only if they do not satisfy `string>?` and
> `string>=?` if and only if they do not satisfy `string<?`.
> 
> The `-ci` procedures behave as if they applied
> `string-foldcase` to their arguments before invoking the corresponding
> procedures without  `-ci`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string_3c_3f__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

