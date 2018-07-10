

<a id='definition__r7rs__syntax-rules'></a>

# `syntax-rules` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Description

> ##### Pattern language
> 
> A `<transformer-spec>` has one of the following forms:
> 
> ````
> (syntax-rules (<literal> ...)
>     <syntax-rule> ...)
> (syntax-rules <ellipsis> (<literal> ...)
>     <syntax-rule> ...)
> _    ; auxiliary syntax
> ...    ; auxiliary syntax
> ````
> 
> **Syntax**:
> It is an error if any of the `<literal>`s, or the `<ellipsis>` in the second form,
> is not an identifier.
> It is also an error if
> `<syntax-rule>` is not of the form
> ````
> (<pattern> <template>)
> ````
> The `<pattern>` in a <syntax-rule> is a list `<pattern>`
> whose first element is an identifier.
> 
> A `<pattern>` is either an identifier, a constant, or one of the
> following
> ````
> (<pattern> ...)
> (<pattern> <pattern> ... . <pattern>)
> (<pattern> ... <pattern> <ellipsis> <pattern> ...)
> (<pattern> ... <pattern> <ellipsis> <pattern> ...
>   . <pattern>)
> #(<pattern> ...)
> #(<pattern> ... <pattern> <ellipsis> <pattern> ...)
> ````
> and a `<template>` is either an identifier, a constant, or one of the following
> ````
> (<element> ...)
> (<element> <element> ... . <template>)
> (<ellipsis> <template>)
> #(<element> ...)
> ````
> where an `<element>` is a `<template>` optionally
> followed by an `<ellipsis>`.
> An `<ellipsis>` is the identifier specified in the second form
> of `syntax-rules`, or the default identifier `...`
> (three consecutive periods) otherwise.
> 
> **Semantics**:
> An instance of `syntax-rules` produces a new macro
> transformer by specifying a sequence of hygienic rewrite rules.  A use
> of a macro whose keyword is associated with a transformer specified by
> `syntax-rules` is matched against the patterns contained in the
> `<syntax-rule>`s, beginning with the leftmost `<syntax-rule>`.
> When a match is found, the macro use is transcribed hygienically
> according to the template.
> 
> An identifier appearing within a `<pattern>` can be an underscore
> (`_`), a literal identifier listed in the list of `<literal>`s,
> or the `<ellipsis>`.
> All other identifiers appearing within a `<pattern>` are
> __pattern variables__.
> 
> The keyword at the beginning of the pattern in a
> `<syntax-rule>` is not involved in the matching and
> is considered neither a pattern variable nor a literal identifier.
> 
> Pattern variables match arbitrary input elements and
> are used to refer to elements of the input in the template.
> It is an error for the same pattern variable to appear more than once in a
> `<pattern>`.
> 
> Underscores also match arbitrary input elements but are not pattern variables
> and so cannot be used to refer to those elements.  If an underscore appears
> in the `<literal>`s list, then that takes precedence and
> underscores in the `<pattern>` match as literals.
> Multiple underscores can appear in a `<pattern>`.
> 
> Identifiers that appear in `(<literal> ...)` are
> interpreted as literal
> identifiers to be matched against corresponding elements of the input.
> An element in the input matches a literal identifier if and only if it is an
> identifier and either both its occurrence in the macro expression and its
> occurrence in the macro definition have the same lexical binding, or
> the two identifiers are the same and both have no lexical binding.
> 
> A subpattern followed by `<ellipsis>` can match zero or more elements of
> the input, unless `<ellipsis>` appears in the `<literal>`s, in which
> case it is matched as a literal.
> 
> More formally, an input expression `E` matches a pattern `P` if and only if:
> 
>   * `P` is an underscore (`_`).
> 
>   * `P` is a non-literal identifier; or
> 
>   * `P` is a literal identifier and `E` is an identifier with the same
>     binding; or
> 
>   * `P` is a list `(P_1 ... P_n)` and `E` is a
>     list of `n`
>     elements that match `P_1` through `P_n`, respectively; or
> 
>   * `P` is an improper list
>     `(P_1 P_2 ... P_n . P_n+1)`
>     and `E` is a list or
>     improper list of `n` or more elements that match `P_1` through `P_n`,
>     respectively, and whose `n`th tail matches `P_n+1`; or
> 
>   * `P` is of the form
>     `(P_1 ... P_k P_e <ellipsis> P_m+1 ... P_n)`
>     where `E` is
>     a proper list of `n` elements, the first `k` of which match
>     `P_1` through `P_k`, respectively,
>     whose next `m-k` elements each match `P_e`,
>     whose remaining `n-m` elements match `P_m+1` through `P_n`; or
> 
>   * `P` is of the form
>     `(P_1 ... P_k P_e <ellipsis> P_m+1 ... P_n . P_x)`
>     where `E` is
>     a list or improper list of `n` elements, the first `k` of which match
>     `P_1` through `P_k`,
>     whose next `m-k` elements each match `P_e`,
>     whose remaining `n-m` elements match `P_m+1` through `P_n`,
>     and whose `n`th and final cdr matches `P_x`; or
> 
>   * `P` is a vector of the form `#(P_1 ... P_n)`
>     and `E` is a vector
>     of `n` elements that match `P_1` through `P_n`; or
> 
>   * `P` is of the form
>     `#(P_1 ... P_k P_e <ellipsis> P_m+1 ... P_n)`
>     where `E` is a vector of `n`
>     elements the first `k` of which match `P_1` through `P_k`,
>     whose next `m-k` elements each match `P_e`,
>     and whose remaining `n-m` elements match `P_m+1` through `P_n`; or
> 
>   * `P` is a constant and `E` is equal to `P` in the sense of
>     the `equal?` procedure.
> 
> It is an error to use a macro keyword, within the scope of its
> binding, in an expression that does not match any of the patterns.
> 
> When a macro use is transcribed according to the template of the
> matching `<syntax-rule>`, pattern variables that occur in the
> template are replaced by the elements they match in the input.
> Pattern variables that occur in subpatterns followed by one or more
> instances of the identifier
> `<ellipsis>` are allowed only in subtemplates that are
> followed by as many instances of `<ellipsis>`.
> They are replaced in the
> output by all of the elements they match in the input, distributed as
> indicated.  It is an error if the output cannot be built up as
> specified.
> 
> Identifiers that appear in the template but are not pattern variables
> or the identifier
> `<ellipsis>` are inserted into the output as literal identifiers.  If a
> literal identifier is inserted as a free identifier then it refers to the
> binding of that identifier within whose scope the instance of
> `syntax-rules` appears.
> If a literal identifier is inserted as a bound identifier then it is
> in effect renamed to prevent inadvertent captures of free identifiers.
> 
> A template of the form
> `(<ellipsis> <template>)` is identical to `<template>`,
> except that
> ellipses within the template have no special meaning.
> That is, any ellipses contained within `<template>` are
> treated as ordinary identifiers.
> In particular, the template `(<ellipsis> <ellipsis>)` produces
> a single `<ellipsis>`.
> This allows syntactic abstractions to expand into code containing
> ellipses.
> 
> ````
> (define-syntax be-like-begin
>   (syntax-rules ()
>     ((be-like-begin name)
>      (define-syntax name
>        (syntax-rules ()
>          ((name expr (... ...))
>           (begin expr (... ...))))))))
> 
> (be-like-begin sequence)
> (sequence 1 2 3 4) ===> 4
> ````
> 
> As an example, if `let` and `cond` are defined as in
> section on derived expression types then they are hygienic (as required) and
> the following is not an error.
> 
> ````
> (let ((=> #f))
>   (cond (#t => 'ok)))           ===> ok
> ````
> 
> The macro transformer for `cond` recognizes `=>`
> as a local variable, and hence an expression, and not as the
> base identifier `=>`, which the macro transformer treats
> as a syntactic keyword.  Thus the example expands into
> 
> ````
> (let ((=> #f))
>   (if #t (begin => 'ok)))
> ````
> 
> instead of
> 
> ````
> (let ((=> #f))
>   (let ((temp #t))
>     (if temp ('ok temp))))
> ````
> 
> which would result in an invalid procedure call.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:syntaxes`](../../r7rs/categories/vs_3a_syntaxes.md#category__r7rs__vs_3a_syntaxes);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

