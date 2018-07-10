

<a id='type__r7rs__string-or-eof'></a>

# `string-or-eof` -- `r7rs` Types


#### Super-type

[`value-or-eof`](../../r7rs/types/value-or-eof.md#type__r7rs__value-or-eof);


#### Referent definitions as output

[`read-string`](../../r7rs/definitions/read-string.md#definition__r7rs__read-string);
[`read-line`](../../r7rs/definitions/read-line.md#definition__r7rs__read-line);


#### Predicate

```
(|lambda| (|value|) (|or| (|string?| |value|) (|eof-object?| |value|)))
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

