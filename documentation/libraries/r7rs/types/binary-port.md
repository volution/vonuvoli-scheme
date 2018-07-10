

<a id='type__r7rs__binary-port'></a>

# `binary-port` -- `r7rs` Types


#### Sub-types tree

* **[`binary-port-open`](../../r7rs/types/binary-port-open.md#type__r7rs__binary-port-open)**:
  * **[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open)**:
    * **[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof)**;
  * **[`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open)**;
* **[`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed)**:
  * **[`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed)**;
  * **[`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed)**;
* **[`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port)**:
  * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open):
    * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
  * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
* **[`binary-output-port`](../../r7rs/types/binary-output-port.md#type__r7rs__binary-output-port)**:
  * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
  * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);


#### Super-type

[`port`](../../r7rs/types/port.md#type__r7rs__port);


#### Sub-types

[`binary-port-open`](../../r7rs/types/binary-port-open.md#type__r7rs__binary-port-open);
[`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed);
[`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port);
[`binary-output-port`](../../r7rs/types/binary-output-port.md#type__r7rs__binary-output-port);


##### Sub-types recursive

[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
[`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
[`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
[`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);


#### Referent definitions as input

[`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);


#### Referent definitions as input (recursive)

[`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
[`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);
[`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
[`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);
[`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
[`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);
[`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);

Note:  These definitions consume an input that is a super-type.


#### Predicate

```
|binary-port?|
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

