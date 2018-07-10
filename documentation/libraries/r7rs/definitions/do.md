

<a id='definition__r7rs__do'></a>

# `do` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `binding-variable`: identifier;
 * `binding-initializer`: expression;
 * `binding-updater`: expression;
 * `binding`: pattern with variants:
   * `(|binding-variable| |binding-initializer|)`;
   * `(|binding-variable| |binding-initializer| |binding-updater|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `exit-test`: expression;
 * `exit-expression`: expression;
 * `exit-clause`: pattern with variants:
   * `(|exit-test|)`;
   * `(|exit-test| |exit-expression|)`;
 * `iteration-expression`: expression;

Syntax variants:
 * `(|_| |bindings| |exit-clause|)`
 * `(|_| |bindings| |exit-clause| |iteration-expression| |...|)`


#### Description

> ````
> (do ((<variable_1> <init_1> <step_1>)
>      ...)
>     (<test> <expression> ...)
>   <command> ...)
> ````
> 
> **Syntax**:
> All of `<init>`, `<step>`, `<test>`, and `<command>`
> are expressions.
> 
> **Semantics**:
> A `do` expression is an iteration construct.  It specifies a set of variables to
> be bound, how they are to be initialized at the start, and how they are
> to be updated on each iteration.  When a termination condition is met,
> the loop exits after evaluating the `<expression>`s.
> 
> A `do` expression is evaluated as follows:
> The `<init>` expressions are evaluated (in some unspecified order),
> the `<variable>`s are bound to fresh locations, the results of the
> `<init>` expressions are stored in the bindings of the
> `<variable>`s, and then the iteration phase begins.
> 
> Each iteration begins by evaluating `<test>`; if the result is
> false (see section on booleans), then the `<command>`
> expressions are evaluated in order for effect, the `<step>`
> expressions are evaluated in some unspecified order, the
> `<variable>`s are bound to fresh locations, the results of the
> `<step>`s are stored in the bindings of the
> `<variable>`s, and the next iteration begins.
> 
> If `<test>` evaluates to a true value, then the
> `<expression>`s are evaluated from left to right and the values of
> the last `<expression>` are returned.  If no `<expression>`s
> are present, then the value of the `do` expression is unspecified.
> 
> The region of the binding of a `<variable>`
> consists of the entire `do` expression except for the `<init>`s.
> It is an error for a `<variable>` to appear more than once in the
> list of `do` variables.
> 
> A `<step>` can be omitted, in which case the effect is the
> same as if `(<variable> <init> <variable>)` had
> been written instead of `(<variable> <init>)`.
> 
> ````
> (do ((vec (make-vector 5))
>      (i 0 (+ i 1)))
>     ((= i 5) vec)
>   (vector-set! vec i i))          ===>  #(0 1 2 3 4)
> 
> (let ((x '(1 3 5 7 9)))
>   (do ((x x (cdr x))
>        (sum 0 (+ sum (car x))))
>       ((null? x) sum)))             ===>  25
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:control`](../../r7rs/categories/vs_3a_control.md#category__r7rs__vs_3a_control);
[`vs:loops`](../../r7rs/categories/vs_3a_loops.md#category__r7rs__vs_3a_loops);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

