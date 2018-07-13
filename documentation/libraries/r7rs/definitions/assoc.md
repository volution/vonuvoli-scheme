

<a id='definition__r7rs__assoc'></a>

# `assoc` -- `r7rs` Definitions


<a id='definition__r7rs__assoc__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__assoc__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any assoc-list) -> (list-or-false))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list);
   * output: a value of type [`list-or-false`](../../r7rs/types/list-or-false.md#type__r7rs__list-or-false);
 * `((any assoc-list procedure) -> (list-or-false))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list);
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`list-or-false`](../../r7rs/types/list-or-false.md#type__r7rs__list-or-false);


<a id='definition__r7rs__assoc__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__assoc__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__assoc__description'></a>

#### Description

> ````
> (assq obj alist)
> (assv obj alist)
> (assoc obj alist)
> (assoc obj alist compare)
> ````
> 
> 
> **Domain**:  It is an error if `alist` (for __association list__) is not a list of
> pairs.
> 
> These procedures find the first pair in `alist` whose car field is `obj`,
> and returns that pair.  If no pair in `alist` has `obj` as its
> car, then `#f` (not the empty list) is returned.  The `assq` procedure uses
> `eq?` to compare `obj` with the car fields of the pairs in `alist`,
> while `assv` uses `eqv?` and `assoc` uses `compare` if given
> and `equal?` otherwise.
> 
> ````
> (define e '((a 1) (b 2) (c 3)))
> (assq 'a e)               ===>  (a 1)
> (assq 'b e)               ===>  (b 2)
> (assq 'd e)               ===>  #f
> (assq (list 'a) '(((a)) ((b)) ((c))))
>                           ===>  #f
> (assoc (list 'a) '(((a)) ((b)) ((c))))
>                           ===>  ((a))
> (assoc 2.0 '((1 1) (2 4) (3 9)) =)
>                           ===> (2 4)
> (assq 5 '((2 3) (5 7) (11 13)))
>                           ===>  #unspecified
> (assv 5 '((2 3) (5 7) (11 13)))
>                           ===>  (5 7)
> ````
> 
> 
> **Rationale**:  Although they are often used as predicates,
> `memq`, `memv`, `member`, `assq`, `assv`, and `assoc` do not
> have question marks in their names because they return
> potentially useful values rather than just `#t` or `#f`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__assoc__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list);
 * [`list-or-false`](../../r7rs/types/list-or-false.md#type__r7rs__list-or-false);
 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);


<a id='definition__r7rs__assoc__categories'></a>

#### Categories

 * [`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);
 * [`vs:associations`](../../r7rs/categories/vs_3a_associations.md#category__r7rs__vs_3a_associations);


<a id='definition__r7rs__assoc__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

