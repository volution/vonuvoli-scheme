

<a id='type__r7rs__binary-port-closed'></a>

# `binary-port-closed` -- `r7rs` Type


<a id='type__r7rs__binary-port-closed__super-types'></a>

#### Super-types

 * [`binary-port`](../../r7rs/types/binary-port.md#type__r7rs__binary-port);
 * [`port-closed`](../../r7rs/types/port-closed.md#type__r7rs__port-closed);


<a id='type__r7rs__binary-port-closed__super-types-recursive'></a>

##### Super-types recursive

 * [`port`](../../r7rs/types/port.md#type__r7rs__port);


<a id='type__r7rs__binary-port-closed__sub-types'></a>

#### Sub-types

 * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
 * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);


<a id='type__r7rs__binary-port-closed__predicate'></a>

#### Predicate

````
(lambda (value) (and (binary-port? value) (not (or (input-port-open? value) (output-port-open? value)))))
````


<a id='type__r7rs__binary-port-closed__categories'></a>

#### Categories

 * [`types-ports`](../../r7rs/categories/types-ports.md#category__r7rs__types-ports);


<a id='type__r7rs__binary-port-closed__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

