

<a id='type__r7rs__vector'></a>

# `vector` -- `r7rs` Type


<a id='type__r7rs__vector__super-types'></a>

#### Super-types

 * [(none)](../../r7rs/types/_index.md#toc__r7rs__types);


<a id='type__r7rs__vector__sub-types'></a>

#### Sub-types

 * [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);


<a id='type__r7rs__vector__referent-definitions-input'></a>

#### Referent definitions as input

 * [`vector?`](../../r7rs/definitions/vector_3f.md#definition__r7rs__vector_3f);
 * [`vector-append`](../../r7rs/definitions/vector-append.md#definition__r7rs__vector-append);
 * [`vector-copy`](../../r7rs/definitions/vector-copy.md#definition__r7rs__vector-copy);
 * [`vector-copy!`](../../r7rs/definitions/vector-copy_21.md#definition__r7rs__vector-copy_21);
 * [`vector-fill!`](../../r7rs/definitions/vector-fill_21.md#definition__r7rs__vector-fill_21);
 * [`vector-ref`](../../r7rs/definitions/vector-ref.md#definition__r7rs__vector-ref);
 * [`vector-set!`](../../r7rs/definitions/vector-set_21.md#definition__r7rs__vector-set_21);
 * [`vector->list`](../../r7rs/definitions/vector-_3e_list.md#definition__r7rs__vector-_3e_list);
 * [`vector-map`](../../r7rs/definitions/vector-map.md#definition__r7rs__vector-map);
 * [`vector-for-each`](../../r7rs/definitions/vector-for-each.md#definition__r7rs__vector-for-each);
 * [`vector->string`](../../r7rs/definitions/vector-_3e_string.md#definition__r7rs__vector-_3e_string);


<a id='type__r7rs__vector__referent-definitions-output'></a>

#### Referent definitions as output

 * [`vector-append`](../../r7rs/definitions/vector-append.md#definition__r7rs__vector-append);
 * [`vector-copy`](../../r7rs/definitions/vector-copy.md#definition__r7rs__vector-copy);
 * [`string->vector`](../../r7rs/definitions/string-_3e_vector.md#definition__r7rs__string-_3e_vector);


<a id='type__r7rs__vector__referent-definitions-output-recursive'></a>

#### Referent definitions as output (recursive)

 * [`vector`](../../r7rs/definitions/vector.md#definition__r7rs__vector);
 * [`make-vector`](../../r7rs/definitions/make-vector.md#definition__r7rs__make-vector);
 * [`list->vector`](../../r7rs/definitions/list-_3e_vector.md#definition__r7rs__list-_3e_vector);

Note:  These definitions produce an output that is a sub-type.


<a id='type__r7rs__vector__predicate'></a>

#### Predicate

````
vector?
````


<a id='type__r7rs__vector__description'></a>

#### Description

> Vectors are heterogeneous structures whose elements are indexed
> by integers.  A vector typically occupies less space than a list
> of the same length, and the average time needed to access a randomly
> chosen element is typically less for the vector than for the list.
> 
> The __length__ of a vector is the number of elements that it
> contains.  This number is a non-negative integer that is fixed when the
> vector is created.  The __valid indexes__ of a
> vector are the exact non-negative integers less than the length of the
> vector.  The first element in a vector is indexed by zero, and the last
> element is indexed by one less than the length of the vector.
> 
> Vectors are written using the notation `#(obj ...)`.
> For example, a vector of length `3` containing the number `0` in element
> `0`, the list `(2 2 2 2)` in element `1`, and the string `"Anna"` in
> element `2` can be written as follows:
> 
> ````
> #(0 (2 2 2 2) "Anna")
> ````
> 
> Vector constants are self-evaluating, so they do not need to be quoted in programs.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__vector__categories'></a>

#### Categories

 * [`r7rs:types-disjoint`](../../r7rs/categories/r7rs_3a_types-disjoint.md#category__r7rs__r7rs_3a_types-disjoint);


<a id='type__r7rs__vector__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

