

<a id='definition__r7rs__cond-expand'></a>

# `cond-expand` -- `r7rs` Definition


<a id='definition__r7rs__cond-expand__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__cond-expand__implemented-by'></a>

#### Implemented by

 * [`cond-expand`](../../vonuvoli/definitions/cond-expand.md#definition__vonuvoli__cond-expand) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__cond-expand__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__cond-expand__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__cond-expand__description'></a>

#### Description

> ````
> (cond-expand <ce-clause_1> <ce-clause_2> ...)
> ````
> 
> 
> **Syntax**:
> The `cond-expand` expression type
> provides a way to statically
> expand different expressions depending on the
> implementation.  A
> `<ce-clause>` takes the following form:
> ````
> (<feature-requirement> <expression> ...)
> ````
> 
> The last clause can be an "else clause", which has the form:
> ````
> (else <expression> ...)
> ````
> 
> A `<feature-requirement>` takes one of the following forms:
> 
>   * `<feature-identifier>`
>   * `(library <library-name>)`
>   * `(and <feature-requirement> ...)`
>   * `(or <feature-requirement> ...)`
>   * `(not <feature-requirement>)`
> 
> **Semantics**:
> Each implementation maintains a list of feature identifiers which are
> present, as well as a list of libraries which can be imported.  The
> value of a `<feature-requirement>` is determined by replacing
> each `<feature-identifier>` and `(library <library-name>)`
> on the implementation's lists with `#t`, and all other feature
> identifiers and library names with `#f`, then evaluating the
> resulting expression as a Scheme boolean expression under the normal
> interpretation of `and`, `or`, and `not`.
> 
> A `cond-expand` is then expanded by evaluating the
> `<feature-requirement>`s of successive `<ce-clause>`s
> in order until one of them returns `#t`.  When a true clause is
> found, the corresponding `<expression>`s are expanded to a
> `begin`, and the remaining clauses are ignored.
> If none of the `<feature-requirement>`s evaluate to `#t`, then
> if there is an else clause, its `<expression>`s are
> included.  Otherwise, the behavior of the `cond-expand` is unspecified.
> Unlike `cond`, `cond-expand` does not depend on the value
> of any variables.
> 
> The exact features provided are implementation-defined, but for
> portability a core set of features is given in
> appendix on standard feature identifiers.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

