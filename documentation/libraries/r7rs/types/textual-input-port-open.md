

<a id='type__r7rs__textual-input-port-open'></a>

# `textual-input-port-open` -- `r7rs` Type


<a id='type__r7rs__textual-input-port-open__super-types'></a>

#### Super-types

 * [`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port);
 * [`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open);
 * [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);


<a id='type__r7rs__textual-input-port-open__super-types-recursive'></a>

##### Super-types recursive

 * [`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
 * [`port-open`](../../r7rs/types/port-open.md#type__r7rs__port-open);


<a id='type__r7rs__textual-input-port-open__sub-types'></a>

#### Sub-types

 * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);


<a id='type__r7rs__textual-input-port-open__referent-definitions-input'></a>

#### Referent definitions as input

 * [`char-ready?`](../../r7rs/definitions/char-ready_3f.md#definition__r7rs__char-ready_3f);
 * [`peek-char`](../../r7rs/definitions/peek-char.md#definition__r7rs__peek-char);
 * [`read-char`](../../r7rs/definitions/read-char.md#definition__r7rs__read-char);
 * [`read-string`](../../r7rs/definitions/read-string.md#definition__r7rs__read-string);
 * [`read-line`](../../r7rs/definitions/read-line.md#definition__r7rs__read-line);
 * [`read`](../../r7rs/definitions/read.md#definition__r7rs__read);


<a id='type__r7rs__textual-input-port-open__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
 * [`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);
 * [`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);
 * [`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
 * [`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);
 * [`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
 * [`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);
 * [`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);
 * [`close-port`](../../r7rs/definitions/close-port.md#definition__r7rs__close-port);
 * [`close-input-port`](../../r7rs/definitions/close-input-port.md#definition__r7rs__close-input-port);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__textual-input-port-open__referent-definitions-output'></a>

#### Referent definitions as output

 * [`open-input-file`](../../r7rs/definitions/open-input-file.md#definition__r7rs__open-input-file);


<a id='type__r7rs__textual-input-port-open__predicate'></a>

#### Predicate

````
(lambda (value) (and (textual-port? value) (input-port? value) (input-port-open? value)))
````


<a id='type__r7rs__textual-input-port-open__categories'></a>

#### Categories

 * [`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);


<a id='type__r7rs__textual-input-port-open__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

