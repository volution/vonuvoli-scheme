

<a id='definition__r7rs__ZZZZ__3d'></a>

# `=` -- `r7rs` Definitions


<a id='definition__r7rs__ZZZZ__3d__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__ZZZZ__3d__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-not-nan) -> (true))`
   * input: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((number-not-nan ...) -> (boolean))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * `((number ...) -> (false))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` (i.e. variadic);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__ZZZZ__3d__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__ZZZZ__3d__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__ZZZZ__3d__description'></a>

#### Description

> ````
> (= z_1 z_2 z_3 ...)
> (< x_1 x_2 x_3 ...)
> (> x_1 x_2 x_3 ...)
> (<= x_1 x_2 x_3 ...)
> (>= x_1 x_2 x_3 ...)
> ````
> 
> 
> These procedures return `#t` if their arguments are (respectively):
> equal, monotonically increasing, monotonically decreasing,
> monotonically non-decreasing, or monotonically non-increasing,
> and `#f` otherwise.
> If any of the arguments are `+nan.0`, all the predicates return `#f`.
> They do not distinguish between inexact zero and inexact negative zero.
> 
> These predicates are required to be transitive.
> 
> **Note**:  The implementation approach
> of converting all arguments to inexact numbers
> if any argument is inexact is not transitive.  For example, let
> `big` be `(expt 2 1000)`, and assume that `big` is exact and that
> inexact numbers are represented by 64-bit IEEE binary floating point numbers.
> Then `(= (- big 1) (inexact big))` and
> `(= (inexact big) (+ big 1))` would both be true with this approach,
> because of the limitations of IEEE
> representations of large integers, whereas `(= (- big 1) (+ big 1))`
> is false.  Converting inexact values to exact numbers that are the same (in the sense of `=`) to them will avoid
> this problem, though special care must be taken with infinities.
> 
> 
> **Note**:  While it is not an error to compare __inexact__ numbers using these
> predicates, the results are unreliable because a small inaccuracy
> can affect the result; this is especially true of `=` and `zero?`.
> When in doubt, consult a numerical analyst.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__ZZZZ__3d__referenced-types'></a>

#### Referenced-types

 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__ZZZZ__3d__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);
 * [`vs:comparisons`](../../r7rs/categories/vs_3a_comparisons.md#category__r7rs__vs_3a_comparisons);


<a id='definition__r7rs__ZZZZ__3d__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

