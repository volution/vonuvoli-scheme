

<a id='type__r7rs__string-empty'></a>

# `string-empty` -- `r7rs` Type


<a id='type__r7rs__string-empty__super-types'></a>

#### Super-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='type__r7rs__string-empty__referent-definitions-input'></a>

#### Referent definitions as input

 * [`string-length`](../../r7rs/definitions/string-length.md#definition__r7rs__string-length);
 * [`string->symbol`](../../r7rs/definitions/string-_3e_symbol.md#definition__r7rs__string-_3e_symbol);
 * [`string->list`](../../r7rs/definitions/string-_3e_list.md#definition__r7rs__string-_3e_list);
 * [`string->vector`](../../r7rs/definitions/string-_3e_vector.md#definition__r7rs__string-_3e_vector);
 * [`string-upcase`](../../r7rs/definitions/string-upcase.md#definition__r7rs__string-upcase);
 * [`string-downcase`](../../r7rs/definitions/string-downcase.md#definition__r7rs__string-downcase);
 * [`string-foldcase`](../../r7rs/definitions/string-foldcase.md#definition__r7rs__string-foldcase);
 * [`string->utf8`](../../r7rs/definitions/string-_3e_utf8.md#definition__r7rs__string-_3e_utf8);


<a id='type__r7rs__string-empty__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`syntax-error`](../../r7rs/definitions/syntax-error.md#definition__r7rs__syntax-error);
 * [`string?`](../../r7rs/definitions/string_3f.md#definition__r7rs__string_3f);
 * [`string-append`](../../r7rs/definitions/string-append.md#definition__r7rs__string-append);
 * [`string-copy`](../../r7rs/definitions/string-copy.md#definition__r7rs__string-copy);
 * [`string-copy!`](../../r7rs/definitions/string-copy_21.md#definition__r7rs__string-copy_21);
 * [`string-fill!`](../../r7rs/definitions/string-fill_21.md#definition__r7rs__string-fill_21);
 * [`substring`](../../r7rs/definitions/substring.md#definition__r7rs__substring);
 * [`string-ref`](../../r7rs/definitions/string-ref.md#definition__r7rs__string-ref);
 * [`string-set!`](../../r7rs/definitions/string-set_21.md#definition__r7rs__string-set_21);
 * [`string=?`](../../r7rs/definitions/string_3d_3f.md#definition__r7rs__string_3d_3f);
 * [`string<?`](../../r7rs/definitions/string_3c_3f.md#definition__r7rs__string_3c_3f);
 * [`string>?`](../../r7rs/definitions/string_3e_3f.md#definition__r7rs__string_3e_3f);
 * [`string<=?`](../../r7rs/definitions/string_3c_3d_3f.md#definition__r7rs__string_3c_3d_3f);
 * [`string>=?`](../../r7rs/definitions/string_3e_3d_3f.md#definition__r7rs__string_3e_3d_3f);
 * [`string-ci=?`](../../r7rs/definitions/string-ci_3d_3f.md#definition__r7rs__string-ci_3d_3f);
 * [`string-ci<?`](../../r7rs/definitions/string-ci_3c_3f.md#definition__r7rs__string-ci_3c_3f);
 * [`string-ci>?`](../../r7rs/definitions/string-ci_3e_3f.md#definition__r7rs__string-ci_3e_3f);
 * [`string-ci<=?`](../../r7rs/definitions/string-ci_3c_3d_3f.md#definition__r7rs__string-ci_3c_3d_3f);
 * [`string-ci>=?`](../../r7rs/definitions/string-ci_3e_3d_3f.md#definition__r7rs__string-ci_3e_3d_3f);
 * [`string->number`](../../r7rs/definitions/string-_3e_number.md#definition__r7rs__string-_3e_number);
 * [`string-map`](../../r7rs/definitions/string-map.md#definition__r7rs__string-map);
 * [`string-for-each`](../../r7rs/definitions/string-for-each.md#definition__r7rs__string-for-each);
 * [`open-input-string`](../../r7rs/definitions/open-input-string.md#definition__r7rs__open-input-string);
 * [`write-string`](../../r7rs/definitions/write-string.md#definition__r7rs__write-string);
 * [`get-environment-variable`](../../r7rs/definitions/get-environment-variable.md#definition__r7rs__get-environment-variable);
 * [`error`](../../r7rs/definitions/error.md#definition__r7rs__error);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__string-empty__referent-definitions-output'></a>

#### Referent definitions as output

 * [`string`](../../r7rs/definitions/string.md#definition__r7rs__string);
 * [`make-string`](../../r7rs/definitions/make-string.md#definition__r7rs__make-string);
 * [`string-append`](../../r7rs/definitions/string-append.md#definition__r7rs__string-append);
 * [`list->string`](../../r7rs/definitions/list-_3e_string.md#definition__r7rs__list-_3e_string);
 * [`vector->string`](../../r7rs/definitions/vector-_3e_string.md#definition__r7rs__vector-_3e_string);
 * [`string-upcase`](../../r7rs/definitions/string-upcase.md#definition__r7rs__string-upcase);
 * [`string-downcase`](../../r7rs/definitions/string-downcase.md#definition__r7rs__string-downcase);
 * [`string-foldcase`](../../r7rs/definitions/string-foldcase.md#definition__r7rs__string-foldcase);
 * [`utf8->string`](../../r7rs/definitions/utf8-_3e_string.md#definition__r7rs__utf8-_3e_string);


<a id='type__r7rs__string-empty__predicate'></a>

#### Predicate

````
(lambda (value) (and (string? value) (zero? (string-length value))))
````


<a id='type__r7rs__string-empty__categories'></a>

#### Categories

 * [`types-miscellaneous`](../../r7rs/categories/types-miscellaneous.md#category__r7rs__types-miscellaneous);
 * [`types-constants`](../../r7rs/categories/types-constants.md#category__r7rs__types-constants);


<a id='type__r7rs__string-empty__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

