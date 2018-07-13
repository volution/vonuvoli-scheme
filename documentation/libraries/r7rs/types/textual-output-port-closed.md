

<a id='type__r7rs__textual-output-port-closed'></a>

# `textual-output-port-closed` -- `r7rs` Types


<a id='type__r7rs__textual-output-port-closed__super-types'></a>

#### Super-types

 * [`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port);
 * [`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed);
 * [`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);


<a id='type__r7rs__textual-output-port-closed__super-types-recursive'></a>

##### Super-types recursive

 * [`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`output-port`](../../r7rs/types/output-port.md#type__r7rs__output-port);
 * [`port-closed`](../../r7rs/types/port-closed.md#type__r7rs__port-closed);


<a id='type__r7rs__textual-output-port-closed__predicate'></a>

#### Predicate

````
(lambda (value) (and (textual-port? value) (output-port? value) (not (output-port-open? value))))
````


<a id='type__r7rs__textual-output-port-closed__categories'></a>

#### Categories

 * [`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);


<a id='type__r7rs__textual-output-port-closed__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

