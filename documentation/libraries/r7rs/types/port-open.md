

<a id='type__r7rs__port-open'></a>

# `port-open` -- `r7rs` Types


<a id='type__r7rs__port-open__sub-types-tree'></a>

#### Sub-types tree

* **[`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open)**:
  * **[`input-port-eof`](../../r7rs/types/input-port-eof.md#type__r7rs__input-port-eof)**:
    * **[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof)**;
    * **[`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof)**;
  * **[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open)**:
    * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
  * **[`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open)**:
    * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
* **[`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open)**:
  * **[`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open)**;
  * **[`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open)**;
* **[`binary-port-open`](../../r7rs/types/binary-port-open.md#type__r7rs__binary-port-open)**:
  * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open):
    * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
  * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
* **[`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open)**:
  * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open):
    * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
  * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);


<a id='type__r7rs__port-open__super-types'></a>

#### Super-types

 * [`port`](../../r7rs/types/port.md#type__r7rs__port);


<a id='type__r7rs__port-open__sub-types'></a>

#### Sub-types

 * [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
 * [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);
 * [`binary-port-open`](../../r7rs/types/binary-port-open.md#type__r7rs__binary-port-open);
 * [`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open);


<a id='type__r7rs__port-open__sub-types-recursive'></a>

##### Sub-types recursive

 * [`input-port-eof`](../../r7rs/types/input-port-eof.md#type__r7rs__input-port-eof);
 * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
 * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
 * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
 * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
 * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
 * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);


<a id='type__r7rs__port-open__predicate'></a>

#### Predicate

````
(lambda (value) (and (port? value) (or (input-port-open? value) (output-port-open? value))))
````


<a id='type__r7rs__port-open__categories'></a>

#### Categories

 * [`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);


<a id='type__r7rs__port-open__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

