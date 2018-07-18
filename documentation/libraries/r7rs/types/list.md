

<a id='type__r7rs__list'></a>

# `list` -- `r7rs` Type


<a id='type__r7rs__list__sub-types-tree'></a>

#### Sub-types tree

* **[`list-not-null`](../../r7rs/types/list-not-null.md#type__r7rs__list-not-null)**:
  * **[`list-circular`](../../r7rs/types/list-circular.md#type__r7rs__list-circular)**;
  * **[`list-dotted-not-null`](../../r7rs/types/list-dotted-not-null.md#type__r7rs__list-dotted-not-null)**;
  * **[`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null)**;
  * **[`assoc-list-not-null`](../../r7rs/types/assoc-list-not-null.md#type__r7rs__assoc-list-not-null)**;
* **[`list-not-circular`](../../r7rs/types/list-not-circular.md#type__r7rs__list-not-circular)**:
  * **[`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted)**:
    * [`list-dotted-not-null`](../../r7rs/types/list-dotted-not-null.md#type__r7rs__list-dotted-not-null);
  * **[`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper)**:
    * [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
    * **[`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list)**:
      * ...


<a id='type__r7rs__list__super-types'></a>

#### Super-types

 * [(none)](../../r7rs/types/_index.md#toc__r7rs__types);


<a id='type__r7rs__list__sub-types'></a>

#### Sub-types

 * [`list-not-null`](../../r7rs/types/list-not-null.md#type__r7rs__list-not-null);
 * [`list-not-circular`](../../r7rs/types/list-not-circular.md#type__r7rs__list-not-circular);


<a id='type__r7rs__list__sub-types-recursive'></a>

##### Sub-types recursive

 * [`list-circular`](../../r7rs/types/list-circular.md#type__r7rs__list-circular);
 * [`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);
 * [`list-dotted-not-null`](../../r7rs/types/list-dotted-not-null.md#type__r7rs__list-dotted-not-null);
 * [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
 * [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
 * [`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list);
 * [`assoc-list-not-null`](../../r7rs/types/assoc-list-not-null.md#type__r7rs__assoc-list-not-null);


<a id='type__r7rs__list__referent-definitions-input'></a>

#### Referent definitions as input

 * [`append`](../../r7rs/definitions/append.md#definition__r7rs__append);
 * [`list-ref`](../../r7rs/definitions/list-ref.md#definition__r7rs__list-ref);
 * [`list-tail`](../../r7rs/definitions/list-tail.md#definition__r7rs__list-tail);
 * [`list-set!`](../../r7rs/definitions/list-set_21.md#definition__r7rs__list-set_21);
 * [`map`](../../r7rs/definitions/map.md#definition__r7rs__map);
 * [`for-each`](../../r7rs/definitions/for-each.md#definition__r7rs__for-each);
 * [`member`](../../r7rs/definitions/member.md#definition__r7rs__member);
 * [`memq`](../../r7rs/definitions/memq.md#definition__r7rs__memq);
 * [`memv`](../../r7rs/definitions/memv.md#definition__r7rs__memv);


<a id='type__r7rs__list__referent-definitions-output'></a>

#### Referent definitions as output

 * [`list-tail`](../../r7rs/definitions/list-tail.md#definition__r7rs__list-tail);


<a id='type__r7rs__list__referent-definitions-output-recursive'></a>

#### Referent definitions as output (recursive)

 * [`list-copy`](../../r7rs/definitions/list-copy.md#definition__r7rs__list-copy);
 * [`append`](../../r7rs/definitions/append.md#definition__r7rs__append);
 * [`list`](../../r7rs/definitions/list.md#definition__r7rs__list);
 * [`vector->list`](../../r7rs/definitions/vector-_3e_list.md#definition__r7rs__vector-_3e_list);
 * [`string->list`](../../r7rs/definitions/string-_3e_list.md#definition__r7rs__string-_3e_list);
 * [`error-object-irritants`](../../r7rs/definitions/error-object-irritants.md#definition__r7rs__error-object-irritants);
 * [`features`](../../r7rs/definitions/features.md#definition__r7rs__features);
 * [`make-list`](../../r7rs/definitions/make-list.md#definition__r7rs__make-list);
 * [`reverse`](../../r7rs/definitions/reverse.md#definition__r7rs__reverse);
 * [`command-line`](../../r7rs/definitions/command-line.md#definition__r7rs__command-line);
 * [`get-environment-variables`](../../r7rs/definitions/get-environment-variables.md#definition__r7rs__get-environment-variables);

Note:  These definitions produce an output that is a sub-type.


<a id='type__r7rs__list__predicate'></a>

#### Predicate

````
list?
````


<a id='type__r7rs__list__categories'></a>

#### Categories

 * [`r7rs:types-lists`](../../r7rs/categories/r7rs_3a_types-lists.md#category__r7rs__r7rs_3a_types-lists);


<a id='type__r7rs__list__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

