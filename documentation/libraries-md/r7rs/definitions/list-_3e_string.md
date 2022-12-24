

<a id='definition__r7rs__list-_3e_string'></a>

# `list->string` -- `r7rs` Definition


<a id='definition__r7rs__list-_3e_string__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__list-_3e_string__implemented-by'></a>

#### Implemented by

 * [`list->string`](../../vonuvoli/definitions/list-_3e_string.md#definition__vonuvoli__list-_3e_string) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__list-_3e_string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((null) -> (string-empty))`
   * input: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((list-proper-not-null) -> (string-not-empty))`
   * input: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);


<a id='definition__r7rs__list-_3e_string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__list-_3e_string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__list-_3e_string__description'></a>

#### Description

> Please refer to [`string->list`](../../r7rs/definitions/string-_3e_list.md#definition__r7rs__string-_3e_list).


<a id='definition__r7rs__list-_3e_string__referenced-types'></a>

#### Referenced-types

 * [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
 * [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

