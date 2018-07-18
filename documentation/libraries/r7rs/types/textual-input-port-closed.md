

<a id='type__r7rs__textual-input-port-closed'></a>

# `textual-input-port-closed` -- `r7rs` Type


<a id='type__r7rs__textual-input-port-closed__super-types'></a>

#### Super-types

 * [`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port);
 * [`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed);
 * [`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);


<a id='type__r7rs__textual-input-port-closed__super-types-recursive'></a>

##### Super-types recursive

 * [`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
 * [`port-closed`](../../r7rs/types/port-closed.md#type__r7rs__port-closed);


<a id='type__r7rs__textual-input-port-closed__predicate'></a>

#### Predicate

````
(lambda (value) (and (textual-port? value) (input-port? value) (not (input-port-open? value))))
````


<a id='type__r7rs__textual-input-port-closed__categories'></a>

#### Categories

 * [`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);


<a id='type__r7rs__textual-input-port-closed__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

