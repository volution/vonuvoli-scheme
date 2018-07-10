

<a id='type__r7rs__port-closed'></a>

# `port-closed` -- `r7rs` Types


#### Sub-types tree

* **[`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed)**:
  * **[`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed)**;
  * **[`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed)**;
* **[`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed)**:
  * **[`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed)**;
  * **[`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed)**;
* **[`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed)**:
  * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
  * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
* **[`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed)**:
  * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
  * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);


#### Super-type

[`port`](../../r7rs/types/port.md#type__r7rs__port);


#### Sub-types

[`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);
[`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);
[`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed);
[`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed);


##### Sub-types recursive

[`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
[`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
[`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
[`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);


#### Predicate

```
(|lambda| (|value|) (|and| (|port?| |value|) (|not| (|or| (|input-port-open?| |value|) (|output-port-open?| |value|)))))
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

