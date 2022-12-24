

<a id='export__vonuvoli__vs_3a_io'></a>

# `vs:io` -- `vonuvoli` Export


<a id='export__vonuvoli__vs_3a_io__descriptor'></a>

#### Descriptor

````
(vonuvoli io)
````


<a id='export__vonuvoli__vs_3a_io__definitions'></a>

#### Definitions

 * [`port?`](../../vonuvoli/definitions/port_3f.md#definition__vonuvoli__port_3f);
 * [`input-port?`](../../vonuvoli/definitions/input-port_3f.md#definition__vonuvoli__input-port_3f);
 * [`output-port?`](../../vonuvoli/definitions/output-port_3f.md#definition__vonuvoli__output-port_3f);
 * [`binary-port?`](../../vonuvoli/definitions/binary-port_3f.md#definition__vonuvoli__binary-port_3f);
 * [`textual-port?`](../../vonuvoli/definitions/textual-port_3f.md#definition__vonuvoli__textual-port_3f);
 * [`binary-input-port?`](../../vonuvoli/definitions/binary-input-port_3f.md#definition__vonuvoli__binary-input-port_3f);
 * [`textual-input-port?`](../../vonuvoli/definitions/textual-input-port_3f.md#definition__vonuvoli__textual-input-port_3f);
 * [`binary-output-port?`](../../vonuvoli/definitions/binary-output-port_3f.md#definition__vonuvoli__binary-output-port_3f);
 * [`textual-output-port?`](../../vonuvoli/definitions/textual-output-port_3f.md#definition__vonuvoli__textual-output-port_3f);
 * [`eof-object?`](../../vonuvoli/definitions/eof-object_3f.md#definition__vonuvoli__eof-object_3f);
 * [`not-port?`](../../vonuvoli/definitions/not-port_3f.md#definition__vonuvoli__not-port_3f);
 * [`not-input-port?`](../../vonuvoli/definitions/not-input-port_3f.md#definition__vonuvoli__not-input-port_3f);
 * [`not-output-port?`](../../vonuvoli/definitions/not-output-port_3f.md#definition__vonuvoli__not-output-port_3f);
 * [`not-binary-port?`](../../vonuvoli/definitions/not-binary-port_3f.md#definition__vonuvoli__not-binary-port_3f);
 * [`not-textual-port?`](../../vonuvoli/definitions/not-textual-port_3f.md#definition__vonuvoli__not-textual-port_3f);
 * [`not-binary-input-port?`](../../vonuvoli/definitions/not-binary-input-port_3f.md#definition__vonuvoli__not-binary-input-port_3f);
 * [`not-textual-input-port?`](../../vonuvoli/definitions/not-textual-input-port_3f.md#definition__vonuvoli__not-textual-input-port_3f);
 * [`not-binary-output-port?`](../../vonuvoli/definitions/not-binary-output-port_3f.md#definition__vonuvoli__not-binary-output-port_3f);
 * [`not-textual-output-port?`](../../vonuvoli/definitions/not-textual-output-port_3f.md#definition__vonuvoli__not-textual-output-port_3f);
 * [`not-eof-object?`](../../vonuvoli/definitions/not-eof-object_3f.md#definition__vonuvoli__not-eof-object_3f);
 * [`current-input-port`](../../vonuvoli/definitions/current-input-port.md#definition__vonuvoli__current-input-port);
 * [`current-output-port`](../../vonuvoli/definitions/current-output-port.md#definition__vonuvoli__current-output-port);
 * [`current-error-port`](../../vonuvoli/definitions/current-error-port.md#definition__vonuvoli__current-error-port);
 * [`eof-object`](../../vonuvoli/definitions/eof-object.md#definition__vonuvoli__eof-object);
 * [`call-with-port`](../../vonuvoli/definitions/call-with-port.md#definition__vonuvoli__call-with-port);
 * [`input-port-open?`](../../vonuvoli/definitions/input-port-open_3f.md#definition__vonuvoli__input-port-open_3f);
 * [`output-port-open?`](../../vonuvoli/definitions/output-port-open_3f.md#definition__vonuvoli__output-port-open_3f);
 * [`close-port`](../../vonuvoli/definitions/close-port.md#definition__vonuvoli__close-port);
 * [`close-input-port`](../../vonuvoli/definitions/close-input-port.md#definition__vonuvoli__close-input-port);
 * [`close-output-port`](../../vonuvoli/definitions/close-output-port.md#definition__vonuvoli__close-output-port);
 * [`newline`](../../vonuvoli/definitions/newline.md#definition__vonuvoli__newline);
 * [`flush-output-port`](../../vonuvoli/definitions/flush-output-port.md#definition__vonuvoli__flush-output-port);


<a id='export__vonuvoli__vs_3a_io__definitions-recursive'></a>

#### Definitions recursive

 * [`open-input-bytevector`](../../vonuvoli/definitions/open-input-bytevector.md#definition__vonuvoli__open-input-bytevector);
 * [`open-input-string`](../../vonuvoli/definitions/open-input-string.md#definition__vonuvoli__open-input-string);
 * [`get-output-bytevector`](../../vonuvoli/definitions/get-output-bytevector.md#definition__vonuvoli__get-output-bytevector);
 * [`get-output-string`](../../vonuvoli/definitions/get-output-string.md#definition__vonuvoli__get-output-string);
 * [`open-binary-input-file`](../../vonuvoli/definitions/open-binary-input-file.md#definition__vonuvoli__open-binary-input-file);
 * [`open-binary-output-file`](../../vonuvoli/definitions/open-binary-output-file.md#definition__vonuvoli__open-binary-output-file);
 * [`open-input-file`](../../vonuvoli/definitions/open-input-file.md#definition__vonuvoli__open-input-file);
 * [`open-output-file`](../../vonuvoli/definitions/open-output-file.md#definition__vonuvoli__open-output-file);
 * [`port-descriptor`](../../vonuvoli/definitions/port-descriptor.md#definition__vonuvoli__port-descriptor);
 * [`port-descriptor-clone`](../../vonuvoli/definitions/port-descriptor-clone.md#definition__vonuvoli__port-descriptor-clone);
 * [`port-descriptor-ref`](../../vonuvoli/definitions/port-descriptor-ref.md#definition__vonuvoli__port-descriptor-ref);
 * [`port-descriptor-path`](../../vonuvoli/definitions/port-descriptor-path.md#definition__vonuvoli__port-descriptor-path);
 * [`port-temporary-release`](../../vonuvoli/definitions/port-temporary-release.md#definition__vonuvoli__port-temporary-release);
 * [`port-temporary-path`](../../vonuvoli/definitions/port-temporary-path.md#definition__vonuvoli__port-temporary-path);
 * [`call-with-binary-input-file`](../../vonuvoli/definitions/call-with-binary-input-file.md#definition__vonuvoli__call-with-binary-input-file);
 * [`call-with-binary-output-file`](../../vonuvoli/definitions/call-with-binary-output-file.md#definition__vonuvoli__call-with-binary-output-file);
 * [`call-with-input-file`](../../vonuvoli/definitions/call-with-input-file.md#definition__vonuvoli__call-with-input-file);
 * [`call-with-output-file`](../../vonuvoli/definitions/call-with-output-file.md#definition__vonuvoli__call-with-output-file);
 * [`with-binary-input-file`](../../vonuvoli/definitions/with-binary-input-file.md#definition__vonuvoli__with-binary-input-file);
 * [`with-binary-output-file`](../../vonuvoli/definitions/with-binary-output-file.md#definition__vonuvoli__with-binary-output-file);
 * [`with-input-from-file`](../../vonuvoli/definitions/with-input-from-file.md#definition__vonuvoli__with-input-from-file);
 * [`with-output-to-file`](../../vonuvoli/definitions/with-output-to-file.md#definition__vonuvoli__with-output-to-file);
 * [`port-descriptor-flag-ref`](../../vonuvoli/definitions/port-descriptor-flag-ref.md#definition__vonuvoli__port-descriptor-flag-ref);
 * [`port-descriptor-flag-set!`](../../vonuvoli/definitions/port-descriptor-flag-set_21.md#definition__vonuvoli__port-descriptor-flag-set_21);
 * [`open-output-bytevector`](../../vonuvoli/definitions/open-output-bytevector.md#definition__vonuvoli__open-output-bytevector);
 * [`open-output-string`](../../vonuvoli/definitions/open-output-string.md#definition__vonuvoli__open-output-string);
 * [`u8-ready?`](../../vonuvoli/definitions/u8-ready_3f.md#definition__vonuvoli__u8-ready_3f);
 * [`peek-u8`](../../vonuvoli/definitions/peek-u8.md#definition__vonuvoli__peek-u8);
 * [`read-u8`](../../vonuvoli/definitions/read-u8.md#definition__vonuvoli__read-u8);
 * [`char-ready?`](../../vonuvoli/definitions/char-ready_3f.md#definition__vonuvoli__char-ready_3f);
 * [`peek-char`](../../vonuvoli/definitions/peek-char.md#definition__vonuvoli__peek-char);
 * [`read-char`](../../vonuvoli/definitions/read-char.md#definition__vonuvoli__read-char);
 * [`read-bytevector!`](../../vonuvoli/definitions/read-bytevector_21.md#definition__vonuvoli__read-bytevector_21);
 * [`read-bytevector-append!`](../../vonuvoli/definitions/read-bytevector-append_21.md#definition__vonuvoli__read-bytevector-append_21);
 * [`read-bytevector`](../../vonuvoli/definitions/read-bytevector.md#definition__vonuvoli__read-bytevector);
 * [`read-bytevector-chunk`](../../vonuvoli/definitions/read-bytevector-chunk.md#definition__vonuvoli__read-bytevector-chunk);
 * [`read-bytevector-line`](../../vonuvoli/definitions/read-bytevector-line.md#definition__vonuvoli__read-bytevector-line);
 * [`read-bytevector-zero`](../../vonuvoli/definitions/read-bytevector-zero.md#definition__vonuvoli__read-bytevector-zero);
 * [`read-string-append!`](../../vonuvoli/definitions/read-string-append_21.md#definition__vonuvoli__read-string-append_21);
 * [`read-string`](../../vonuvoli/definitions/read-string.md#definition__vonuvoli__read-string);
 * [`read-string-chunk`](../../vonuvoli/definitions/read-string-chunk.md#definition__vonuvoli__read-string-chunk);
 * [`read-string-line`](../../vonuvoli/definitions/read-string-line.md#definition__vonuvoli__read-string-line);
 * [`read-string-zero`](../../vonuvoli/definitions/read-string-zero.md#definition__vonuvoli__read-string-zero);
 * [`read`](../../vonuvoli/definitions/read.md#definition__vonuvoli__read);
 * [`read-bytevector-fold`](../../vonuvoli/definitions/read-bytevector-fold.md#definition__vonuvoli__read-bytevector-fold);
 * [`read-bytevector-chunk-fold`](../../vonuvoli/definitions/read-bytevector-chunk-fold.md#definition__vonuvoli__read-bytevector-chunk-fold);
 * [`read-bytevector-line-fold`](../../vonuvoli/definitions/read-bytevector-line-fold.md#definition__vonuvoli__read-bytevector-line-fold);
 * [`read-bytevector-zero-fold`](../../vonuvoli/definitions/read-bytevector-zero-fold.md#definition__vonuvoli__read-bytevector-zero-fold);
 * [`read-string-fold`](../../vonuvoli/definitions/read-string-fold.md#definition__vonuvoli__read-string-fold);
 * [`read-string-chunk-fold`](../../vonuvoli/definitions/read-string-chunk-fold.md#definition__vonuvoli__read-string-chunk-fold);
 * [`read-string-line-fold`](../../vonuvoli/definitions/read-string-line-fold.md#definition__vonuvoli__read-string-line-fold);
 * [`read-string-zero-fold`](../../vonuvoli/definitions/read-string-zero-fold.md#definition__vonuvoli__read-string-zero-fold);
 * [`read-fold`](../../vonuvoli/definitions/read-fold.md#definition__vonuvoli__read-fold);
 * [`write-u8`](../../vonuvoli/definitions/write-u8.md#definition__vonuvoli__write-u8);
 * [`write-bytevector`](../../vonuvoli/definitions/write-bytevector.md#definition__vonuvoli__write-bytevector);
 * [`write-bytevector-line`](../../vonuvoli/definitions/write-bytevector-line.md#definition__vonuvoli__write-bytevector-line);
 * [`write-bytevector-zero`](../../vonuvoli/definitions/write-bytevector-zero.md#definition__vonuvoli__write-bytevector-zero);
 * [`write-char`](../../vonuvoli/definitions/write-char.md#definition__vonuvoli__write-char);
 * [`write-string`](../../vonuvoli/definitions/write-string.md#definition__vonuvoli__write-string);
 * [`write-string-line`](../../vonuvoli/definitions/write-string-line.md#definition__vonuvoli__write-string-line);
 * [`write-string-zero`](../../vonuvoli/definitions/write-string-zero.md#definition__vonuvoli__write-string-zero);
 * [`write`](../../vonuvoli/definitions/write.md#definition__vonuvoli__write);
 * [`write-shared`](../../vonuvoli/definitions/write-shared.md#definition__vonuvoli__write-shared);
 * [`write-simple`](../../vonuvoli/definitions/write-simple.md#definition__vonuvoli__write-simple);
 * [`display`](../../vonuvoli/definitions/display.md#definition__vonuvoli__display);
 * [`write-line`](../../vonuvoli/definitions/write-line.md#definition__vonuvoli__write-line);
 * [`display-line`](../../vonuvoli/definitions/display-line.md#definition__vonuvoli__display-line);
 * [`open-binary-temporary`](../../vonuvoli/definitions/open-binary-temporary.md#definition__vonuvoli__open-binary-temporary);
 * [`open-temporary`](../../vonuvoli/definitions/open-temporary.md#definition__vonuvoli__open-temporary);


<a id='export__vonuvoli__vs_3a_io__super-exports'></a>

#### Super-exports

 * [(none)](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports);


<a id='export__vonuvoli__vs_3a_io__sub-exports'></a>

#### Sub-exports

 * [`vs:io-open`](../../vonuvoli/exports/vs_3a_io-open.md#export__vonuvoli__vs_3a_io-open);
 * [`vs:io-strings`](../../vonuvoli/exports/vs_3a_io-strings.md#export__vonuvoli__vs_3a_io-strings);
 * [`vs:io-bytes`](../../vonuvoli/exports/vs_3a_io-bytes.md#export__vonuvoli__vs_3a_io-bytes);
 * [`vs:io-values`](../../vonuvoli/exports/vs_3a_io-values.md#export__vonuvoli__vs_3a_io-values);
 * [`vs:io-descriptors`](../../vonuvoli/exports/vs_3a_io-descriptors.md#export__vonuvoli__vs_3a_io-descriptors);
 * [`vs:io-temporary`](../../vonuvoli/exports/vs_3a_io-temporary.md#export__vonuvoli__vs_3a_io-temporary);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

