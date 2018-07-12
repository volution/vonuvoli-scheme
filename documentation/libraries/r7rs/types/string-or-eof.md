

<a id='type__r7rs__string-or-eof'></a>

# `string-or-eof` -- `r7rs` Types


<a id='type__r7rs__string-or-eof__super-types'></a>

#### Super-types

 * [`value-or-eof`](../../r7rs/types/value-or-eof.md#type__r7rs__value-or-eof);


<a id='type__r7rs__string-or-eof__referent-definitions-output'></a>

#### Referent definitions as output

 * [`read-string`](../../r7rs/definitions/read-string.md#definition__r7rs__read-string);
 * [`read-line`](../../r7rs/definitions/read-line.md#definition__r7rs__read-line);


<a id='type__r7rs__string-or-eof__predicate'></a>

#### Predicate

````
(lambda (value) (or (string? value) (eof-object? value)))
````


<a id='type__r7rs__string-or-eof__categories'></a>

#### Categories

 * [`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);


<a id='type__r7rs__string-or-eof__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

