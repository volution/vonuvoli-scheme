

<a id='export__r7rs__scheme_3a_cxr'></a>

# `scheme:cxr` -- `r7rs` Export


<a id='export__r7rs__scheme_3a_cxr__descriptor'></a>

#### Descriptor

````
(scheme cxr)
````


<a id='export__r7rs__scheme_3a_cxr__definitions'></a>

#### Definitions

 * [`caaar`](../../r7rs/definitions/caaar.md#definition__r7rs__caaar);
 * [`caadr`](../../r7rs/definitions/caadr.md#definition__r7rs__caadr);
 * [`cadar`](../../r7rs/definitions/cadar.md#definition__r7rs__cadar);
 * [`caddr`](../../r7rs/definitions/caddr.md#definition__r7rs__caddr);
 * [`cdaar`](../../r7rs/definitions/cdaar.md#definition__r7rs__cdaar);
 * [`cdadr`](../../r7rs/definitions/cdadr.md#definition__r7rs__cdadr);
 * [`cddar`](../../r7rs/definitions/cddar.md#definition__r7rs__cddar);
 * [`cdddr`](../../r7rs/definitions/cdddr.md#definition__r7rs__cdddr);
 * [`caaaar`](../../r7rs/definitions/caaaar.md#definition__r7rs__caaaar);
 * [`caaadr`](../../r7rs/definitions/caaadr.md#definition__r7rs__caaadr);
 * [`caadar`](../../r7rs/definitions/caadar.md#definition__r7rs__caadar);
 * [`caaddr`](../../r7rs/definitions/caaddr.md#definition__r7rs__caaddr);
 * [`cadaar`](../../r7rs/definitions/cadaar.md#definition__r7rs__cadaar);
 * [`cadadr`](../../r7rs/definitions/cadadr.md#definition__r7rs__cadadr);
 * [`caddar`](../../r7rs/definitions/caddar.md#definition__r7rs__caddar);
 * [`cadddr`](../../r7rs/definitions/cadddr.md#definition__r7rs__cadddr);
 * [`cdaaar`](../../r7rs/definitions/cdaaar.md#definition__r7rs__cdaaar);
 * [`cdaadr`](../../r7rs/definitions/cdaadr.md#definition__r7rs__cdaadr);
 * [`cdadar`](../../r7rs/definitions/cdadar.md#definition__r7rs__cdadar);
 * [`cdaddr`](../../r7rs/definitions/cdaddr.md#definition__r7rs__cdaddr);
 * [`cddaar`](../../r7rs/definitions/cddaar.md#definition__r7rs__cddaar);
 * [`cddadr`](../../r7rs/definitions/cddadr.md#definition__r7rs__cddadr);
 * [`cdddar`](../../r7rs/definitions/cdddar.md#definition__r7rs__cdddar);
 * [`cddddr`](../../r7rs/definitions/cddddr.md#definition__r7rs__cddddr);


<a id='export__r7rs__scheme_3a_cxr__description'></a>

#### Description

> ##### CxR Library
> 
> The `(scheme cxr)` library exports twenty-four procedures which
> are the compositions of from three to four `car` and `cdr`
> operations.  For example `caddar` could be defined by
> 
> ````
> (define caddar
>   (lambda (x) (car (cdr (cdr (car x))))))
> ````
> 
> The procedures `car` and `cdr` themselves and the four
> two-level compositions are included in the base library.  See
> section on pairs and lists.
> 
> ````
> caaaar                  caaadr
> caaar                   caadar
> caaddr                  caadr
> cadaar                  cadadr
> cadar                   caddar
> cadddr                  caddr
> cdaaar                  cdaadr
> cdaar                   cdadar
> cdaddr                  cdadr
> cddaar                  cddadr
> cddar                   cdddar
> cddddr                  cdddr
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='export__r7rs__scheme_3a_cxr__super-exports'></a>

#### Super-exports

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='export__r7rs__scheme_3a_cxr__categories'></a>

#### Categories

 * [`r7rs:libraries`](../../r7rs/categories/r7rs_3a_libraries.md#category__r7rs__r7rs_3a_libraries);


<a id='export__r7rs__scheme_3a_cxr__categories-recursive'></a>

#### Categories recursive

 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

