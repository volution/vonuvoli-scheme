

<a id='type__r7rs__textual-port'></a>

# `textual-port` -- `r7rs` Types


#### Sub-types tree

* **[`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open)**:
  * **[`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open)**:
    * **[`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof)**;
  * **[`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open)**;
* **[`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed)**:
  * **[`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed)**;
  * **[`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed)**;
* **[`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port)**:
  * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open):
    * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
  * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
* **[`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port)**:
  * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
  * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);


#### Super-type

[`port`](../../r7rs/types/port.md#type__r7rs__port);


#### Sub-types

[`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open);
[`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed);
[`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port);
[`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port);


##### Sub-types recursive

[`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
[`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
[`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
[`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
[`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);


#### Referent definitions as input

[`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);


#### Referent definitions as input (recursive)

[`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
[`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);
[`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
[`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);
[`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
[`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);
[`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);

Note:  These definitions consume an input that is a super-type.


#### Predicate

```
|textual-port?|
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

