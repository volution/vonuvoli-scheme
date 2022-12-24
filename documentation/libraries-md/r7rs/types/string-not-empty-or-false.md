

<a id='type__r7rs__string-not-empty-or-false'></a>

# `string-not-empty-or-false` -- `r7rs` Type


<a id='type__r7rs__string-not-empty-or-false__super-types'></a>

#### Super-types

 * [`string-or-false`](../../r7rs/types/string-or-false.md#type__r7rs__string-or-false);


<a id='type__r7rs__string-not-empty-or-false__super-types-recursive'></a>

##### Super-types recursive

 * [`value-or-false`](../../r7rs/types/value-or-false.md#type__r7rs__value-or-false);


<a id='type__r7rs__string-not-empty-or-false__referent-definitions-output'></a>

#### Referent definitions as output

 * [`get-environment-variable`](../../r7rs/definitions/get-environment-variable.md#definition__r7rs__get-environment-variable);


<a id='type__r7rs__string-not-empty-or-false__predicate'></a>

#### Predicate

````
(lambda (value) (or (and (string? value) (not (zero? (string-length value)))) (false? value)))
````


<a id='type__r7rs__string-not-empty-or-false__categories'></a>

#### Categories

 * [`types-miscellaneous`](../../r7rs/categories/types-miscellaneous.md#category__r7rs__types-miscellaneous);


<a id='type__r7rs__string-not-empty-or-false__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

