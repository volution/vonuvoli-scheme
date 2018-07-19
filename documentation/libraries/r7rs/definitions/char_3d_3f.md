

<a id='definition__r7rs__char_3d_3f'></a>

# `char=?` -- `r7rs` Definition


<a id='definition__r7rs__char_3d_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__char_3d_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((character) -> (true))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((character ...) -> (boolean))`
   * inputs:
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__char_3d_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__char_3d_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__char_3d_3f__description'></a>

#### Description

> ````
> (char=? char_1 char_2 char_3 ...)
> (char<? char_1 char_2 char_3 ...)
> (char>? char_1 char_2 char_3 ...)
> (char<=? char_1 char_2 char_3 ...)
> (char>=? char_1 char_2 char_3 ...)
> ````
> 
> 
> These procedures return `#t` if
> the results of passing their arguments to `char->integer`
> are respectively
> equal, monotonically increasing, monotonically decreasing,
> monotonically non-decreasing, or monotonically non-increasing.
> 
> These predicates are required to be transitive.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__char_3d_3f__referenced-types'></a>

#### Referenced-types

 * [`character`](../../r7rs/types/character.md#type__r7rs__character);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__char_3d_3f__categories'></a>

#### Categories

 * [`vs:characters`](../../vonuvoli/categories/vs_3a_characters.md#category__vonuvoli__vs_3a_characters);
 * [`vs:comparisons`](../../vonuvoli/categories/vs_3a_comparisons.md#category__vonuvoli__vs_3a_comparisons);
 * [`vs:equivalence`](../../vonuvoli/categories/vs_3a_equivalence.md#category__vonuvoli__vs_3a_equivalence);


<a id='definition__r7rs__char_3d_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

