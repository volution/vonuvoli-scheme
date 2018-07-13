

<a id='type__r7rs__pair'></a>

# `pair` -- `r7rs` Types


<a id='type__r7rs__pair__super-types'></a>

#### Super-types

 * [(none)](../../r7rs/types/_index.md#toc__r7rs__types);


<a id='type__r7rs__pair__referent-definitions-input'></a>

#### Referent definitions as input

 * [`pair?`](../../r7rs/definitions/pair_3f.md#definition__r7rs__pair_3f);
 * [`car`](../../r7rs/definitions/car.md#definition__r7rs__car);
 * [`cdr`](../../r7rs/definitions/cdr.md#definition__r7rs__cdr);
 * [`set-car!`](../../r7rs/definitions/set-car_21.md#definition__r7rs__set-car_21);
 * [`set-cdr!`](../../r7rs/definitions/set-cdr_21.md#definition__r7rs__set-cdr_21);
 * [`null?`](../../r7rs/definitions/null_3f.md#definition__r7rs__null_3f);


<a id='type__r7rs__pair__referent-definitions-output'></a>

#### Referent definitions as output

 * [`cons`](../../r7rs/definitions/cons.md#definition__r7rs__cons);


<a id='type__r7rs__pair__predicate'></a>

#### Predicate

````
pair?
````


<a id='type__r7rs__pair__description'></a>

#### Description

> A __pair__ (sometimes called a __dotted pair__) is a
> record structure with two fields called the car and cdr fields (for
> historical reasons).  Pairs are created by the procedure `cons`.
> The car and cdr fields are accessed by the procedures `car` and
> `cdr`.  The car and cdr fields are assigned by the procedures
> `set-car!` and `set-cdr!`.
> 
> Pairs are used primarily to represent lists.  A __list__ can
> be defined recursively as either the __empty list__ or a pair whose
> cdr is a list.  More precisely, the set of lists is defined as the smallest
> set `X` such that:
> 
>   * The empty list is in `X`.
>   * If `list` is in `X`, then any pair whose cdr field contains
>       `list` is also in `X`.
> 
> The objects in the car fields of successive pairs of a list are the
> elements of the list.  For example, a two-element list is a pair whose car
> is the first element and whose cdr is a pair whose car is the second element
> and whose cdr is the empty list.  The length of a list is the number of
> elements, which is the same as the number of pairs.
> 
> The __empty list__ is a special object of its own type.
> It is not a pair, it has no elements, and its length is zero.
> 
> **Note**:  The above definitions imply that all lists have finite length and are
> terminated by the empty list.
> 
> 
> The most general notation (external representation) for Scheme pairs is
> the __dotted__ notation `(c_1 . c_2)` where
> `c_1` is the value of the car field and `c_2` is the value of the
> cdr field.  For example `(4 . 5)` is a pair whose car is `4` and whose
> cdr is `5`.  Note that `(4 . 5)` is the external representation of a
> pair, not an expression that evaluates to a pair.
> 
> A more streamlined notation can be used for lists: the elements of the
> list are simply enclosed in parentheses and separated by spaces.  The
> __empty list__ is written `()`.  For example,
> 
> ````
> (a b c d e)
> ````
> 
> and
> 
> ````
> (a . (b . (c . (d . (e . ())))))
> ````
> 
> are equivalent notations for a list of symbols.
> 
> A chain of pairs not ending in the empty list is called an
> __improper list__.  Note that an improper list is not a list.
> The list and dotted notations can be combined to represent
> improper lists:
> 
> ````
> (a b c . d)
> ````
> 
> is equivalent to
> 
> ````
> (a . (b . (c . d)))
> ````
> 
> Whether a given pair is a list depends upon what is stored in the cdr
> field.  When the `set-cdr!` procedure is used, an object can be a
> list one moment and not the next:
> 
> ````
> (define x (list 'a 'b 'c))
> (define y x)
> y                       ===>  (a b c)
> (list? y)               ===>  #t
> (set-cdr! x 4)          ===>  #unspecified
> x                       ===>  (a . 4)
> (eqv? x y)              ===>  #t
> y                       ===>  (a . 4)
> (list? y)               ===>  #f
> (set-cdr! x x)          ===>  #unspecified
> (list? x)               ===>  #f
> ````
> 
> Within literal expressions and representations of objects read by the
> `read` procedure, the forms `'` (quote), `’` (backquote), `,` (comma), and
> `,@` (comma and at-sign) denote two-element lists whose first elements are
> the symbols `quote`, `quasiquote`, `unquote`, and
> `unquote-splicing`, respectively.  The second element in each case
> is `<datum>`.  This convention is supported so that arbitrary Scheme
> programs can be represented as lists.
> That is, according to Scheme's grammar, every
> `<expression>` is also a `<datum>` (see section on external representations).
> Among other things, this permits the use of the `read` procedure to
> parse Scheme programs.  See section on external representation.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__pair__categories'></a>

#### Categories

 * [`r7rs:types-disjoint`](../../r7rs/categories/r7rs_3a_types-disjoint.md#category__r7rs__r7rs_3a_types-disjoint);
 * [`r7rs:types-lists`](../../r7rs/categories/r7rs_3a_types-lists.md#category__r7rs__r7rs_3a_types-lists);


<a id='type__r7rs__pair__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

