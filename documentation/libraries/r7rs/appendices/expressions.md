

<a id='appendix__r7rs__expressions'></a>

# `r7rs` -- Expressions


#### Description

> Expression types are categorized as __primitive__ or __derived__.
> Primitive expression types include variables and procedure calls.
> Derived expression types are not semantically primitive, but can instead
> be defined as macros.
> Suitable syntax definitions of some of the derived expressions are
> given in section on derived expression types.
> 
> The procedures `force`, `promise?`, `make-promise`, and `make-parameter`
> are also described in this chapter because they are intimately associated
> with the `delay`, `delay-force`, and `parameterize` expression types.
> 
> 
> ##### Variable references
> 
> ````
> <variable>    syntax
> ````
> 
> An expression consisting of a variable
> (section on variables, syntactic keywords and regions) is a variable reference.  The value of
> the variable reference is the value stored in the location to which the
> variable is bound.  It is an error to reference an
> unbound variable.
> 
> ````
> (define x 28)
> x   ===>  28
> ````
> 
> 
> ##### Procedure calls
> 
> ````
> (<operator> <operand_1> ...)    syntax
> ````
> 
> A procedure call is written by enclosing in parentheses an
> expression for the procedure to be called followed by expressions for the arguments to be
> passed to it.  The operator and operand expressions are evaluated (in an
> unspecified order) and the resulting procedure is passed the resulting
> arguments.
> ````
> (+ 3 4)                          ===>  7
> ((if #f + *) 3 4)         ===>  12
> ````
> 
> The procedures in this document are available as the values of variables exported by the
> standard libraries.  For example, the addition and multiplication
> procedures in the above examples are the values of the variables `+`
> and `*` in the base library.  New procedures are created by evaluating `lambda` expressions
> (see section on `lambda`).
> 
> Procedure calls can return any number of values (see `values` in
> section on control features).
> Most of the procedures defined in this report return one
> value or, for procedures such as `apply`, pass on the values returned
> by a call to one of their arguments.
> Exceptions are noted in the individual descriptions.
> 
> **Note**:
> In contrast to other dialects of Lisp, the order of
> evaluation is unspecified, and the operator expression and the operand
> expressions are always evaluated with the same evaluation rules.
> 
> **Note**:
> Although the order of evaluation is otherwise unspecified, the effect of
> any concurrent evaluation of the operator and operand expressions is
> constrained to be consistent with some sequential order of evaluation.
> The order of evaluation may be chosen differently for each procedure call.
> 
> **Note**:
> In many dialects of Lisp, the empty list,
> `()`, is a legitimate expression evaluating to itself.  In Scheme, it is an error.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

