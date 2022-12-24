

<a id='definition__r7rs__list-_3e_vector'></a>

# `list->vector` -- `r7rs` Definition


<a id='definition__r7rs__list-_3e_vector__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__list-_3e_vector__implemented-by'></a>

#### Implemented by

 * [`list->vector`](../../vonuvoli/definitions/list-_3e_vector.md#definition__vonuvoli__list-_3e_vector) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__list-_3e_vector__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((null) -> (vector-empty))`
   * input: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
   * output: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * `((list-proper-not-null) -> (vector-not-empty))`
   * input: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
   * output: a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);


<a id='definition__r7rs__list-_3e_vector__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__list-_3e_vector__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__list-_3e_vector__description'></a>

#### Description

> Please refer to [`vector->list`](../../r7rs/definitions/vector-_3e_list.md#definition__r7rs__vector-_3e_list).


<a id='definition__r7rs__list-_3e_vector__referenced-types'></a>

#### Referenced-types

 * [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
 * [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

