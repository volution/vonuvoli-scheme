

<a id='type__r7rs__binary-input-port-closed'></a>

# `binary-input-port-closed` -- `r7rs` Types


#### Super-type

[`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port);
[`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed);
[`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);


##### Super-types recursive

[`binary-port`](../../r7rs/types/binary-port.md#type__r7rs__binary-port);
[`port`](../../r7rs/types/port.md#type__r7rs__port);
[`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
[`port-closed`](../../r7rs/types/port-closed.md#type__r7rs__port-closed);


#### Predicate

```
(|lambda| (|value|) (|and| (|binary-port?| |value|) (|input-port?| |value|) (|not| (|input-port-open?| |value|))))
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

