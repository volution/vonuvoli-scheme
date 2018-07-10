

<a id='type__r7rs__binary-input-port-open'></a>

# `binary-input-port-open` -- `r7rs` Types


#### Super-type

[`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port);
[`binary-port-open`](../../r7rs/types/binary-port-open.md#type__r7rs__binary-port-open);
[`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);


##### Super-types recursive

[`binary-port`](../../r7rs/types/binary-port.md#type__r7rs__binary-port);
[`port`](../../r7rs/types/port.md#type__r7rs__port);
[`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
[`port-open`](../../r7rs/types/port-open.md#type__r7rs__port-open);


#### Sub-types

[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);


#### Referent definitions as input

[`u8-ready?`](../../r7rs/definitions/u8-ready_3f.md#definition__r7rs__u8-ready_3f);
[`peek-u8`](../../r7rs/definitions/peek-u8.md#definition__r7rs__peek-u8);
[`read-u8`](../../r7rs/definitions/read-u8.md#definition__r7rs__read-u8);
[`read-bytevector`](../../r7rs/definitions/read-bytevector.md#definition__r7rs__read-bytevector);
[`read-bytevector!`](../../r7rs/definitions/read-bytevector_21.md#definition__r7rs__read-bytevector_21);


#### Referent definitions as input (recursive)

[`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
[`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);
[`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);
[`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
[`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);
[`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
[`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);
[`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);
[`close-port`](../../r7rs/definitions/close-port.md#definition__r7rs__close-port);
[`close-input-port`](../../r7rs/definitions/close-input-port.md#definition__r7rs__close-input-port);

Note:  These definitions consume an input that is a super-type.


#### Referent definitions as output

[`open-binary-input-file`](../../r7rs/definitions/open-binary-input-file.md#definition__r7rs__open-binary-input-file);


#### Predicate

```
(|lambda| (|value|) (|and| (|binary-port?| |value|) (|input-port?| |value|) (|input-port-open?| |value|)))
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

