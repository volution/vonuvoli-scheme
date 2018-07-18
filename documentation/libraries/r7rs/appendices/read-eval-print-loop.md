

<a id='appendix__r7rs__read-eval-print-loop'></a>

# REPL (read-eval-print-loop)


<a id='appendix__r7rs__read-eval-print-loop__description'></a>

#### Description

> Implementations may provide an interactive session called a
> __REPL__ (Read-Eval-Print Loop), where import declarations,
> expressions and definitions can be
> entered and evaluated one at a time.  For convenience and ease of use,
> the global Scheme environment in a REPL
> must not be empty, but must start out with at least the bindings provided by the
> base library.  This library includes the core syntax of Scheme
> and generally useful procedures that manipulate data.  For example, the
> variable `abs` is bound to a
> procedure of one argument that computes the absolute value of a
> number, and the variable `+` is bound to a procedure that computes
> sums.  The full list of `(scheme base)` bindings can be found in
> sequence on standard libraries.
> 
> Implementations may provide an initial REPL environment
> which behaves as if all possible variables are bound to locations, most of
> which contain unspecified values.  Top level REPL definitions in
> such an implementation are truly equivalent to assignments,
> unless the identifier is defined as a syntax keyword.
> 
> An implementation may provide a mode of operation in which the REPL
> reads its input from a file.  Such a file is not, in general, the same
> as a program, because it can contain import declarations in places other than
> the beginning.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

