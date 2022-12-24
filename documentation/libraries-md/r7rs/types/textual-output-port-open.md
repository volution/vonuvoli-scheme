

<a id='type__r7rs__textual-output-port-open'></a>

# `textual-output-port-open` -- `r7rs` Type


<a id='type__r7rs__textual-output-port-open__super-types'></a>

#### Super-types

 * [`textual-output-port`](../../r7rs/types/textual-output-port.md#type__r7rs__textual-output-port);
 * [`textual-port-open`](../../r7rs/types/textual-port-open.md#type__r7rs__textual-port-open);
 * [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);


<a id='type__r7rs__textual-output-port-open__super-types-recursive'></a>

##### Super-types recursive

 * [`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`output-port`](../../r7rs/types/output-port.md#type__r7rs__output-port);
 * [`port-open`](../../r7rs/types/port-open.md#type__r7rs__port-open);


<a id='type__r7rs__textual-output-port-open__referent-definitions-input'></a>

#### Referent definitions as input

 * [`write-char`](../../r7rs/definitions/write-char.md#definition__r7rs__write-char);
 * [`write-string`](../../r7rs/definitions/write-string.md#definition__r7rs__write-string);
 * [`write`](../../r7rs/definitions/write.md#definition__r7rs__write);
 * [`write-simple`](../../r7rs/definitions/write-simple.md#definition__r7rs__write-simple);
 * [`write-shared`](../../r7rs/definitions/write-shared.md#definition__r7rs__write-shared);
 * [`display`](../../r7rs/definitions/display.md#definition__r7rs__display);


<a id='type__r7rs__textual-output-port-open__referent-definitions-input-recursive'></a>

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
 * [`close-output-port`](../../r7rs/definitions/close-output-port.md#definition__r7rs__close-output-port);
 * [`newline`](../../r7rs/definitions/newline.md#definition__r7rs__newline);
 * [`flush-output-port`](../../r7rs/definitions/flush-output-port.md#definition__r7rs__flush-output-port);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__textual-output-port-open__referent-definitions-output'></a>

#### Referent definitions as output

 * [`open-output-file`](../../r7rs/definitions/open-output-file.md#definition__r7rs__open-output-file);


<a id='type__r7rs__textual-output-port-open__predicate'></a>

#### Predicate

````
(lambda (value) (and (textual-port? value) (output-port? value) (output-port-open? value)))
````


<a id='type__r7rs__textual-output-port-open__categories'></a>

#### Categories

 * [`types-ports`](../../r7rs/categories/types-ports.md#category__r7rs__types-ports);


<a id='type__r7rs__textual-output-port-open__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

