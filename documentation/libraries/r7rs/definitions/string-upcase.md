

<a id='definition__r7rs__string-upcase'></a>

# `string-upcase` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|string-empty|) |->| (|string-empty|))`
   * input: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((|string-not-empty|) |->| (|string-not-empty|))`
   * input: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);


#### Referenced types

[`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
[`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);


#### Description

> ````
> (string-upcase string)
> (string-downcase string)
> (string-foldcase string)
> ````
> 
> 
> These procedures apply the Unicode full string uppercasing, lowercasing,
> and case-folding algorithms to their arguments and return the result.
> In certain cases, the result differs in length from the argument.
> If the result is equal to the argument in the sense of `string=?`, the argument may be returned.
> Note that language-sensitive mappings and foldings are not used.
> 
> The __Unicode Standard__ prescribes special treatment of the Greek letter
> `$\Sigma$`, whose normal lower-case form is `$\sigma$` but which becomes
> `$\varsigma$` at the end of a word.  See __UAX #29__ (part of
> the __Unicode Standard__) for details.  However, implementations of
> `string-downcase` are not required to provide this behavior, and may
> choose to change `$\Sigma$` to `$\sigma$` in all cases.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:char`](../../r7rs/categories/r7rs_3a_char.md#category__r7rs__r7rs_3a_char);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
[`vs:conversions`](../../r7rs/categories/vs_3a_conversions.md#category__r7rs__vs_3a_conversions);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

