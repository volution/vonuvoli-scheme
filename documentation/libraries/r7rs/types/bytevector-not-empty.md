

<a id='type__r7rs__bytevector-not-empty'></a>

# `bytevector-not-empty` -- `r7rs` Type


<a id='type__r7rs__bytevector-not-empty__super-types'></a>

#### Super-types

 * [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


<a id='type__r7rs__bytevector-not-empty__referent-definitions-input'></a>

#### Referent definitions as input

 * [`bytevector-length`](../../r7rs/definitions/bytevector-length.md#definition__r7rs__bytevector-length);
 * [`bytevector-u8-ref`](../../r7rs/definitions/bytevector-u8-ref.md#definition__r7rs__bytevector-u8-ref);
 * [`bytevector-u8-set!`](../../r7rs/definitions/bytevector-u8-set_21.md#definition__r7rs__bytevector-u8-set_21);
 * [`utf8->string`](../../r7rs/definitions/utf8-_3e_string.md#definition__r7rs__utf8-_3e_string);
 * [`read-bytevector!`](../../r7rs/definitions/read-bytevector_21.md#definition__r7rs__read-bytevector_21);


<a id='type__r7rs__bytevector-not-empty__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`bytevector?`](../../r7rs/definitions/bytevector_3f.md#definition__r7rs__bytevector_3f);
 * [`bytevector-append`](../../r7rs/definitions/bytevector-append.md#definition__r7rs__bytevector-append);
 * [`bytevector-copy`](../../r7rs/definitions/bytevector-copy.md#definition__r7rs__bytevector-copy);
 * [`bytevector-copy!`](../../r7rs/definitions/bytevector-copy_21.md#definition__r7rs__bytevector-copy_21);
 * [`open-input-bytevector`](../../r7rs/definitions/open-input-bytevector.md#definition__r7rs__open-input-bytevector);
 * [`write-bytevector`](../../r7rs/definitions/write-bytevector.md#definition__r7rs__write-bytevector);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__bytevector-not-empty__referent-definitions-output'></a>

#### Referent definitions as output

 * [`bytevector`](../../r7rs/definitions/bytevector.md#definition__r7rs__bytevector);
 * [`make-bytevector`](../../r7rs/definitions/make-bytevector.md#definition__r7rs__make-bytevector);
 * [`string->utf8`](../../r7rs/definitions/string-_3e_utf8.md#definition__r7rs__string-_3e_utf8);


<a id='type__r7rs__bytevector-not-empty__predicate'></a>

#### Predicate

````
(lambda (value) (and (bytevector? value) (not (zero? (bytevector-length value)))))
````


<a id='type__r7rs__bytevector-not-empty__categories'></a>

#### Categories

 * [`types-miscellaneous`](../../r7rs/categories/types-miscellaneous.md#category__r7rs__types-miscellaneous);


<a id='type__r7rs__bytevector-not-empty__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

