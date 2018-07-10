

<a id='type__r7rs__input-port-closed'></a>

# `input-port-closed` -- `r7rs` Types


#### Super-type

[`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
[`port-closed`](../../r7rs/types/port-closed.md#type__r7rs__port-closed);


##### Super-types recursive

[`port`](../../r7rs/types/port.md#type__r7rs__port);


#### Sub-types

[`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
[`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);


#### Referent definitions as input

[`close-port`](../../r7rs/definitions/close-port.md#definition__r7rs__close-port);
[`close-input-port`](../../r7rs/definitions/close-input-port.md#definition__r7rs__close-input-port);


#### Referent definitions as input (recursive)

[`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
[`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);
[`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);
[`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
[`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);
[`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
[`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);
[`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);

Note:  These definitions consume an input that is a super-type.


#### Predicate

```
(|lambda| (|value|) (|and| (|input-port?| |value|) (|not| (|input-port-open?| |value|))))
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

