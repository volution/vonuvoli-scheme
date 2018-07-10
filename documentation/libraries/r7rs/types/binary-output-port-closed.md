

<a id='type__r7rs__binary-output-port-closed'></a>

# `binary-output-port-closed` -- `r7rs` Types


#### Super-type

[`binary-output-port`](../../r7rs/types/binary-output-port.md#type__r7rs__binary-output-port);
[`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed);
[`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);


##### Super-types recursive

[`binary-port`](../../r7rs/types/binary-port.md#type__r7rs__binary-port);
[`port`](../../r7rs/types/port.md#type__r7rs__port);
[`output-port`](../../r7rs/types/output-port.md#type__r7rs__output-port);
[`port-closed`](../../r7rs/types/port-closed.md#type__r7rs__port-closed);


#### Predicate

```
(|lambda| (|value|) (|and| (|binary-port?| |value|) (|output-port?| |value|) (|not| (|output-port-open?| |value|))))
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

