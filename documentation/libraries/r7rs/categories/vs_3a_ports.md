

<a id='category__r7rs__vs_3a_ports'></a>

# `vs:ports` -- `r7rs` Categories


<a id='category__r7rs__vs_3a_ports__definitions'></a>

#### Definitions

 * [`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f);
 * [`binary-port?`](../../r7rs/definitions/binary-port_3f.md#definition__r7rs__binary-port_3f);
 * [`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f);
 * [`close-port`](../../r7rs/definitions/close-port.md#definition__r7rs__close-port);
 * [`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);
 * [`eof-object`](../../r7rs/definitions/eof-object.md#definition__r7rs__eof-object);
 * [`eof-object?`](../../r7rs/definitions/eof-object_3f.md#definition__r7rs__eof-object_3f);


<a id='category__r7rs__vs_3a_ports__definitions-recursive'></a>

#### Definitions recursive

 * [`input-port?`](../../r7rs/definitions/input-port_3f.md#definition__r7rs__input-port_3f);
 * [`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f);
 * [`output-port?`](../../r7rs/definitions/output-port_3f.md#definition__r7rs__output-port_3f);
 * [`output-port-open?`](../../r7rs/definitions/output-port-open_3f.md#definition__r7rs__output-port-open_3f);
 * [`open-input-bytevector`](../../r7rs/definitions/open-input-bytevector.md#definition__r7rs__open-input-bytevector);
 * [`open-output-bytevector`](../../r7rs/definitions/open-output-bytevector.md#definition__r7rs__open-output-bytevector);
 * [`get-output-bytevector`](../../r7rs/definitions/get-output-bytevector.md#definition__r7rs__get-output-bytevector);
 * [`open-input-string`](../../r7rs/definitions/open-input-string.md#definition__r7rs__open-input-string);
 * [`open-output-string`](../../r7rs/definitions/open-output-string.md#definition__r7rs__open-output-string);
 * [`get-output-string`](../../r7rs/definitions/get-output-string.md#definition__r7rs__get-output-string);
 * [`close-input-port`](../../r7rs/definitions/close-input-port.md#definition__r7rs__close-input-port);
 * [`close-output-port`](../../r7rs/definitions/close-output-port.md#definition__r7rs__close-output-port);
 * [`u8-ready?`](../../r7rs/definitions/u8-ready_3f.md#definition__r7rs__u8-ready_3f);
 * [`peek-u8`](../../r7rs/definitions/peek-u8.md#definition__r7rs__peek-u8);
 * [`read-u8`](../../r7rs/definitions/read-u8.md#definition__r7rs__read-u8);
 * [`write-u8`](../../r7rs/definitions/write-u8.md#definition__r7rs__write-u8);
 * [`read-bytevector`](../../r7rs/definitions/read-bytevector.md#definition__r7rs__read-bytevector);
 * [`read-bytevector!`](../../r7rs/definitions/read-bytevector_21.md#definition__r7rs__read-bytevector_21);
 * [`write-bytevector`](../../r7rs/definitions/write-bytevector.md#definition__r7rs__write-bytevector);
 * [`char-ready?`](../../r7rs/definitions/char-ready_3f.md#definition__r7rs__char-ready_3f);
 * [`peek-char`](../../r7rs/definitions/peek-char.md#definition__r7rs__peek-char);
 * [`read-char`](../../r7rs/definitions/read-char.md#definition__r7rs__read-char);
 * [`write-char`](../../r7rs/definitions/write-char.md#definition__r7rs__write-char);
 * [`read-string`](../../r7rs/definitions/read-string.md#definition__r7rs__read-string);
 * [`write-string`](../../r7rs/definitions/write-string.md#definition__r7rs__write-string);
 * [`read-line`](../../r7rs/definitions/read-line.md#definition__r7rs__read-line);
 * [`newline`](../../r7rs/definitions/newline.md#definition__r7rs__newline);
 * [`flush-output-port`](../../r7rs/definitions/flush-output-port.md#definition__r7rs__flush-output-port);
 * [`read`](../../r7rs/definitions/read.md#definition__r7rs__read);
 * [`write`](../../r7rs/definitions/write.md#definition__r7rs__write);
 * [`write-simple`](../../r7rs/definitions/write-simple.md#definition__r7rs__write-simple);
 * [`write-shared`](../../r7rs/definitions/write-shared.md#definition__r7rs__write-shared);
 * [`display`](../../r7rs/definitions/display.md#definition__r7rs__display);
 * [`open-input-file`](../../r7rs/definitions/open-input-file.md#definition__r7rs__open-input-file);
 * [`open-binary-input-file`](../../r7rs/definitions/open-binary-input-file.md#definition__r7rs__open-binary-input-file);
 * [`open-output-file`](../../r7rs/definitions/open-output-file.md#definition__r7rs__open-output-file);
 * [`open-binary-output-file`](../../r7rs/definitions/open-binary-output-file.md#definition__r7rs__open-binary-output-file);
 * [`call-with-input-file`](../../r7rs/definitions/call-with-input-file.md#definition__r7rs__call-with-input-file);
 * [`call-with-output-file`](../../r7rs/definitions/call-with-output-file.md#definition__r7rs__call-with-output-file);


<a id='category__r7rs__vs_3a_ports__super-categories'></a>

#### Super-categories

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);


<a id='category__r7rs__vs_3a_ports__sub-categories'></a>

#### Sub-categories

 * [`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
 * [`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);
 * [`vs:ports:open`](../../r7rs/categories/vs_3a_ports_3a_open.md#category__r7rs__vs_3a_ports_3a_open);
 * [`vs:ports:values`](../../r7rs/categories/vs_3a_ports_3a_values.md#category__r7rs__vs_3a_ports_3a_values);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

