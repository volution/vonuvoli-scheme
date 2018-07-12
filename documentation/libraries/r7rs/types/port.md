

<a id='type__r7rs__port'></a>

# `port` -- `r7rs` Types


<a id='type__r7rs__port__sub-types-tree'></a>

#### Sub-types tree

* **[`port-open`](../../r7rs/types/port-open.md#type__r7rs__port-open)**:
  * **[`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open)**:
    * **[`input-port-eof`](../../r7rs/types/input-port-eof.md#type__r7rs__input-port-eof)**:
      * ...
    * **[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open)**:
      * ...
    * **[`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open)**:
      * ...
  * **[`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open)**:
    * **[`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open)**;
    * **[`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open)**;
  * **[`binary-port-open`](../../r7rs/types/binary-port-open.md#type__r7rs__binary-port-open)**:
    * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open):
      * ...
    * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
  * **[`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open)**:
    * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open):
      * ...
    * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
* **[`port-closed`](../../r7rs/types/port-closed.md#type__r7rs__port-closed)**:
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
* **[`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port)**:
  * [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open):
    * [`input-port-eof`](../../r7rs/types/input-port-eof.md#type__r7rs__input-port-eof):
      * ...
    * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open):
      * ...
    * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open):
      * ...
  * [`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed):
    * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
    * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
  * **[`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port)**:
    * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open):
      * ...
    * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
  * **[`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port)**:
    * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open):
      * ...
    * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
  * **[`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port)**;
  * **[`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port)**;
* **[`output-port`](../../r7rs/types/output-port.md#type__r7rs__output-port)**:
  * [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open):
    * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
    * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
  * [`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed):
    * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
    * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);
  * **[`binary-output-port`](../../r7rs/types/binary-output-port.md#type__r7rs__binary-output-port)**:
    * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
    * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
  * **[`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port)**:
    * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
    * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);
  * **[`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port)**;
  * **[`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port)**;
* **[`binary-port`](../../r7rs/types/binary-port.md#type__r7rs__binary-port)**:
  * [`binary-port-open`](../../r7rs/types/binary-port-open.md#type__r7rs__binary-port-open):
    * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open):
      * ...
    * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
  * [`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed):
    * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
    * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
  * [`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port):
    * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open):
      * ...
    * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
  * [`binary-output-port`](../../r7rs/types/binary-output-port.md#type__r7rs__binary-output-port):
    * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
    * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
* **[`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port)**:
  * [`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open):
    * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open):
      * ...
    * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
  * [`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed):
    * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
    * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);
  * [`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port):
    * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open):
      * ...
    * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
  * [`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port):
    * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
    * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);
* **[`bytevector-port`](../../r7rs/types/bytevector-port.md#type__r7rs__bytevector-port)**:
  * [`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port);
  * [`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port);
* **[`string-port`](../../r7rs/types/string-port.md#type__r7rs__string-port)**:
  * [`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port);
  * [`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port);


<a id='type__r7rs__port__super-types'></a>

#### Super-types

 * [(none)](../../r7rs/types/_index.md#toc__r7rs__types);


<a id='type__r7rs__port__sub-types'></a>

#### Sub-types

 * [`port-open`](../../r7rs/types/port-open.md#type__r7rs__port-open);
 * [`port-closed`](../../r7rs/types/port-closed.md#type__r7rs__port-closed);
 * [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
 * [`output-port`](../../r7rs/types/output-port.md#type__r7rs__output-port);
 * [`binary-port`](../../r7rs/types/binary-port.md#type__r7rs__binary-port);
 * [`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
 * [`bytevector-port`](../../r7rs/types/bytevector-port.md#type__r7rs__bytevector-port);
 * [`string-port`](../../r7rs/types/string-port.md#type__r7rs__string-port);


<a id='type__r7rs__port__sub-types-recursive'></a>

##### Sub-types recursive

 * [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
 * [`input-port-eof`](../../r7rs/types/input-port-eof.md#type__r7rs__input-port-eof);
 * [`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);
 * [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);
 * [`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);
 * [`binary-port-open`](../../r7rs/types/binary-port-open.md#type__r7rs__binary-port-open);
 * [`binary-port-closed`](../../r7rs/types/binary-port-closed.md#type__r7rs__binary-port-closed);
 * [`binary-input-port`](../../r7rs/types/binary-input-port.md#type__r7rs__binary-input-port);
 * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
 * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
 * [`binary-input-port-closed`](../../r7rs/types/binary-input-port-closed.md#type__r7rs__binary-input-port-closed);
 * [`binary-output-port`](../../r7rs/types/binary-output-port.md#type__r7rs__binary-output-port);
 * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
 * [`binary-output-port-closed`](../../r7rs/types/binary-output-port-closed.md#type__r7rs__binary-output-port-closed);
 * [`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open);
 * [`textual-port-closed`](../../r7rs/types/textual-port-closed.md#type__r7rs__textual-port-closed);
 * [`textual-input-port`](../../r7rs/types/textual-input-port.md#type__r7rs__textual-input-port);
 * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
 * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
 * [`textual-input-port-closed`](../../r7rs/types/textual-input-port-closed.md#type__r7rs__textual-input-port-closed);
 * [`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port);
 * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
 * [`textual-output-port-closed`](../../r7rs/types/textual-output-port-closed.md#type__r7rs__textual-output-port-closed);
 * [`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port);
 * [`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port);
 * [`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port);
 * [`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port);


<a id='type__r7rs__port__referent-definitions-input'></a>

#### Referent definitions as input

 * [`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
 * [`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);
 * [`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);
 * [`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
 * [`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);
 * [`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
 * [`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);
 * [`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);


<a id='type__r7rs__port__predicate'></a>

#### Predicate

````
port?
````


<a id='type__r7rs__port__description'></a>

#### Description

> Ports represent input and output devices.  To Scheme, an input port is
> a Scheme object that can deliver data upon command, while an output
> port is a Scheme object that can accept data.
> Whether the input and output port types are disjoint is
> implementation-dependent.
> 
> Different __port types__ operate on different data.  Scheme
> implementations are required to support __textual ports__
> and __binary ports__, but may also provide other port types.
> 
> A textual port supports reading or writing of individual characters
> from or to a backing store containing characters
> using `read-char` and `write-char` below, and it supports operations
> defined in terms of characters, such as `read` and `write`.
> 
> A binary port supports reading or writing of individual bytes from
> or to a backing store containing bytes using `read-u8` and
> `write-u8` below, as well as operations defined in terms of bytes.
> Whether the textual and binary port types are disjoint is
> implementation-dependent.
> 
> Ports can be used to access files, devices, and similar things on the host
> system on which the Scheme program is running.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__port__categories'></a>

#### Categories

 * [`r7rs:types-disjoint`](../../r7rs/categories/r7rs_3a_types-disjoint.md#category__r7rs__r7rs_3a_types-disjoint);
 * [`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);


<a id='type__r7rs__port__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

