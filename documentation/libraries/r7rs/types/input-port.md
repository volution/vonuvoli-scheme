

<a id='type__r7rs__input-port'></a>

# `input-port` -- `r7rs` Type


<a id='type__r7rs__input-port__sub-types-tree'></a>

#### Sub-types tree

* **[`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open)**:
  * **[`input-port-eof`](../../r7rs/types/input-port-eof.md#type__r7rs__input-port-eof)**:
    * **[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof)**;
    * **[`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof)**;
  * **[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open)**:
    * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
  * **[`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open)**:
    * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
* **[`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed)**:
  * **[`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed)**;
  * **[`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed)**;
* **[`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port)**:
  * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open):
    * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
  * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
* **[`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port)**:
  * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open):
    * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
  * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
* **[`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port)**;
* **[`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port)**;


<a id='type__r7rs__input-port__super-types'></a>

#### Super-types

 * [`port`](../../r7rs/types/port.md#type__r7rs__port);


<a id='type__r7rs__input-port__sub-types'></a>

#### Sub-types

 * [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
 * [`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);
 * [`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port);
 * [`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port);
 * [`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port);
 * [`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port);


<a id='type__r7rs__input-port__sub-types-recursive'></a>

##### Sub-types recursive

 * [`input-port-eof`](../../r7rs/types/input-port-eof.md#type__r7rs__input-port-eof);
 * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
 * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
 * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
 * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
 * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
 * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);


<a id='type__r7rs__input-port__referent-definitions-input'></a>

#### Referent definitions as input

 * [`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
 * [`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);


<a id='type__r7rs__input-port__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
 * [`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);
 * [`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);
 * [`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
 * [`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);
 * [`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__input-port__referent-definitions-output'></a>

#### Referent definitions as output

 * [`current-input-port`](../../r7rs/definitions/current-input-port.md#definition__r7rs__current-input-port);


<a id='type__r7rs__input-port__referent-definitions-output-recursive'></a>

#### Referent definitions as output (recursive)

 * [`open-binary-input-file`](../../r7rs/definitions/open-binary-input-file.md#definition__r7rs__open-binary-input-file);
 * [`open-input-file`](../../r7rs/definitions/open-input-file.md#definition__r7rs__open-input-file);
 * [`open-input-bytevector`](../../r7rs/definitions/open-input-bytevector.md#definition__r7rs__open-input-bytevector);
 * [`open-input-string`](../../r7rs/definitions/open-input-string.md#definition__r7rs__open-input-string);

Note:  These definitions produce an output that is a sub-type.


<a id='type__r7rs__input-port__predicate'></a>

#### Predicate

````
input-port?
````


<a id='type__r7rs__input-port__description'></a>

#### Description

> For details please refer to [`port`](../../r7rs/types/port.md#type__r7rs__port).
> 
> If `port` is omitted from any input procedure, it defaults to the
> value returned by `(current-input-port)`.
> It is an error to attempt an input operation on a closed port.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__input-port__categories'></a>

#### Categories

 * [`types-ports`](../../r7rs/categories/types-ports.md#category__r7rs__types-ports);


<a id='type__r7rs__input-port__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

