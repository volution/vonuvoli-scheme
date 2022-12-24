

<a id='type__r7rs__port-closed'></a>

# `port-closed` -- `r7rs` Type


<a id='type__r7rs__port-closed__sub-types-tree'></a>

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


<a id='type__r7rs__port-closed__super-types'></a>

#### Super-types

 * [`port`](../../r7rs/types/port.md#type__r7rs__port);


<a id='type__r7rs__port-closed__sub-types'></a>

#### Sub-types

 * [`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);
 * [`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);
 * [`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed);
 * [`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed);


<a id='type__r7rs__port-closed__sub-types-recursive'></a>

##### Sub-types recursive

 * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
 * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
 * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
 * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);


<a id='type__r7rs__port-closed__predicate'></a>

#### Predicate

````
(lambda (value) (and (port? value) (not (or (input-port-open? value) (output-port-open? value)))))
````


<a id='type__r7rs__port-closed__categories'></a>

#### Categories

 * [`types-ports`](../../r7rs/categories/types-ports.md#category__r7rs__types-ports);


<a id='type__r7rs__port-closed__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

