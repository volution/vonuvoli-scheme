

<a id='definition__r7rs__jiffies-per-second'></a>

# `jiffies-per-second` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|timestamp-jiffy|))`
   * inputs: none;
   * output: a value of type [`timestamp-jiffy`](../../r7rs/types/timestamp-jiffy.md#type__r7rs__timestamp-jiffy);


#### Referenced types

[`timestamp-jiffy`](../../r7rs/types/timestamp-jiffy.md#type__r7rs__timestamp-jiffy);


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


#### Categories

[`r7rs:time`](../../r7rs/categories/r7rs_3a_time.md#category__r7rs__r7rs_3a_time);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

