

<a id='definition__r7rs__jiffies-per-second'></a>

# `jiffies-per-second` -- `r7rs` Definitions


<a id='definition__r7rs__jiffies-per-second__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__jiffies-per-second__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (timestamp-jiffy))`
   * inputs: none;
   * output: a value of type [`timestamp-jiffy`](../../r7rs/types/timestamp-jiffy.md#type__r7rs__timestamp-jiffy);


<a id='definition__r7rs__jiffies-per-second__exports'></a>

#### Exports

 * [`scheme:time`](../../r7rs/exports/scheme_3a_time.md#export__r7rs__scheme_3a_time);


<a id='definition__r7rs__jiffies-per-second__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__jiffies-per-second__description'></a>

#### Description

> ````
> (jiffies-per-second)
> ````
> 
> 
> Returns an exact integer representing the number of jiffies per SI
> second. This value is an implementation-specified constant.
> 
> ````
> (define (time-length)
>   (let ((list (make-list 100000))
>         (start (current-jiffy)))
>     (length list)
>     (/ (- (current-jiffy) start)
>        (jiffies-per-second))))
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__jiffies-per-second__referenced-types'></a>

#### Referenced-types

 * [`timestamp-jiffy`](../../r7rs/types/timestamp-jiffy.md#type__r7rs__timestamp-jiffy);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

