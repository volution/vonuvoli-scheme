

<a id='type__r7rs__vector-empty'></a>

# `vector-empty` -- `r7rs` Type


<a id='type__r7rs__vector-empty__super-types'></a>

#### Super-types

 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);


<a id='type__r7rs__vector-empty__referent-definitions-input'></a>

#### Referent definitions as input

 * [`vector-length`](../../r7rs/definitions/vector-length.md#definition__r7rs__vector-length);
 * [`vector->list`](../../r7rs/definitions/vector-_3e_list.md#definition__r7rs__vector-_3e_list);
 * [`vector->string`](../../r7rs/definitions/vector-_3e_string.md#definition__r7rs__vector-_3e_string);


<a id='type__r7rs__vector-empty__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`vector?`](../../r7rs/definitions/vector_3f.md#definition__r7rs__vector_3f);
 * [`vector-append`](../../r7rs/definitions/vector-append.md#definition__r7rs__vector-append);
 * [`vector-copy`](../../r7rs/definitions/vector-copy.md#definition__r7rs__vector-copy);
 * [`vector-copy!`](../../r7rs/definitions/vector-copy_21.md#definition__r7rs__vector-copy_21);
 * [`vector-fill!`](../../r7rs/definitions/vector-fill_21.md#definition__r7rs__vector-fill_21);
 * [`vector-ref`](../../r7rs/definitions/vector-ref.md#definition__r7rs__vector-ref);
 * [`vector-set!`](../../r7rs/definitions/vector-set_21.md#definition__r7rs__vector-set_21);
 * [`vector-map`](../../r7rs/definitions/vector-map.md#definition__r7rs__vector-map);
 * [`vector-for-each`](../../r7rs/definitions/vector-for-each.md#definition__r7rs__vector-for-each);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__vector-empty__referent-definitions-output'></a>

#### Referent definitions as output

 * [`vector`](../../r7rs/definitions/vector.md#definition__r7rs__vector);
 * [`make-vector`](../../r7rs/definitions/make-vector.md#definition__r7rs__make-vector);
 * [`vector-append`](../../r7rs/definitions/vector-append.md#definition__r7rs__vector-append);
 * [`list->vector`](../../r7rs/definitions/list-_3e_vector.md#definition__r7rs__list-_3e_vector);
 * [`string->vector`](../../r7rs/definitions/string-_3e_vector.md#definition__r7rs__string-_3e_vector);


<a id='type__r7rs__vector-empty__predicate'></a>

#### Predicate

````
(lambda (value) (and (vector? value) (zero? (vector-length value))))
````


<a id='type__r7rs__vector-empty__categories'></a>

#### Categories

 * [`r7rs:types-miscellaneous`](../../r7rs/categories/r7rs_3a_types-miscellaneous.md#category__r7rs__r7rs_3a_types-miscellaneous);
 * [`r7rs:types-constants`](../../r7rs/categories/r7rs_3a_types-constants.md#category__r7rs__r7rs_3a_types-constants);


<a id='type__r7rs__vector-empty__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

