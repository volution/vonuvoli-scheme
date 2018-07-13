

<a id='type__r7rs__output-port'></a>

# `output-port` -- `r7rs` Types


<a id='type__r7rs__output-port__sub-types-tree'></a>

#### Sub-types tree

* **[`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open)**:
  * **[`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open)**;
  * **[`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open)**;
* **[`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed)**:
  * **[`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed)**;
  * **[`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed)**;
* **[`binary-output-port`](../../r7rs/types/binary-output-port.md#type__r7rs__binary-output-port)**:
  * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
  * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
* **[`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port)**:
  * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
  * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);
* **[`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port)**;
* **[`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port)**;


<a id='type__r7rs__output-port__super-types'></a>

#### Super-types

 * [`port`](../../r7rs/types/port.md#type__r7rs__port);


<a id='type__r7rs__output-port__sub-types'></a>

#### Sub-types

 * [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);
 * [`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);
 * [`binary-output-port`](../../r7rs/types/binary-output-port.md#type__r7rs__binary-output-port);
 * [`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port);
 * [`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port);
 * [`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port);


<a id='type__r7rs__output-port__sub-types-recursive'></a>

##### Sub-types recursive

 * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
 * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
 * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
 * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);


<a id='type__r7rs__output-port__referent-definitions-input'></a>

#### Referent definitions as input

 * [`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
 * [`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);


<a id='type__r7rs__output-port__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
 * [`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);
 * [`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);
 * [`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
 * [`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);
 * [`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__output-port__referent-definitions-output'></a>

#### Referent definitions as output

 * [`current-output-port`](../../r7rs/definitions/current-output-port.md#definition__r7rs__current-output-port);
 * [`current-error-port`](../../r7rs/definitions/current-error-port.md#definition__r7rs__current-error-port);


<a id='type__r7rs__output-port__referent-definitions-output-recursive'></a>

#### Referent definitions as output (recursive)

 * [`open-binary-output-file`](../../r7rs/definitions/open-binary-output-file.md#definition__r7rs__open-binary-output-file);
 * [`open-output-file`](../../r7rs/definitions/open-output-file.md#definition__r7rs__open-output-file);
 * [`open-output-bytevector`](../../r7rs/definitions/open-output-bytevector.md#definition__r7rs__open-output-bytevector);
 * [`open-output-string`](../../r7rs/definitions/open-output-string.md#definition__r7rs__open-output-string);

Note:  These definitions produce an output that is a sub-type.


<a id='type__r7rs__output-port__predicate'></a>

#### Predicate

````
output-port?
````


<a id='type__r7rs__output-port__description'></a>

#### Description

> For details please refer to [`port`](../../r7rs/types/port.md#type__r7rs__port).
> 
> If `port` is omitted from any output procedure, it defaults to the
> value returned by `(current-output-port)`.
> It is an error to attempt an output operation on a closed port.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__output-port__categories'></a>

#### Categories

 * [`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);


<a id='type__r7rs__output-port__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

