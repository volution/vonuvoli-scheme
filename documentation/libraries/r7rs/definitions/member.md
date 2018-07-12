

<a id='definition__r7rs__member'></a>

# `member` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|any| |list|) |->| (|list-or-false|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
   * output: a value of type [`list-or-false`](../../r7rs/types/list-or-false.md#type__r7rs__list-or-false);
 * `((|any| |list| |procedure|) |->| (|list-or-false|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`list-or-false`](../../r7rs/types/list-or-false.md#type__r7rs__list-or-false);


#### Referenced types

[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`list`](../../r7rs/types/list.md#type__r7rs__list);
[`list-or-false`](../../r7rs/types/list-or-false.md#type__r7rs__list-or-false);
[`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);


#### Description

> ````
> (memq obj list)
> (memv obj list)
> (member obj list)
> (member obj list compare)
> ````
> 
> 
> These procedures return the first sublist of `list` whose car is
> `obj`, where the sublists of `list` are the non-empty lists
> returned by `(list-tail list k)` for `k` less
> than the length of `list`.  If
> `obj` does not occur in `list`, then `#f` (not the empty list) is
> returned.  The `memq` procedure uses `eq?` to compare `obj` with the elements of
> `list`, while `memv` uses `eqv?` and
> `member` uses `compare`, if given, and `equal?` otherwise.
> 
> ````
> (memq 'a '(a b c))              ===>  (a b c)
> (memq 'b '(a b c))              ===>  (b c)
> (memq 'a '(b c d))              ===>  #f
> (memq (list 'a) '(b (a) c))     ===>  #f
> (member (list 'a)
>         '(b (a) c))             ===>  ((a) c)
> (member "B"
>         '("a" "b" "c")
>         string-ci=?)            ===>  ("b" "c")
> (memq 101 '(100 101 102))       ===>  #unspecified
> (memv 101 '(100 101 102))       ===>  (101 102)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
