



<a id='library__vonuvoli_r7rs'>

# `vonuvoli:r7rs` -- R7RS functionality with Vonuvoli-Scheme extensions


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----




<a id='toc__vonuvoli_r7rs__categories'>

## Categories

Quick list of categories:
* [`r7rs`](#category__vonuvoli_r7rs__r7rs): [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base), [`r7rs:case-lambda`](#category__vonuvoli_r7rs__r7rs_case-lambda), [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char), [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex), [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr), [`r7rs:eval`](#category__vonuvoli_r7rs__r7rs_eval), [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file), [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact), [`r7rs:lazy`](#category__vonuvoli_r7rs__r7rs_lazy), [`r7rs:load`](#category__vonuvoli_r7rs__r7rs_load), [`r7rs:process-context`](#category__vonuvoli_r7rs__r7rs_process-context), [`r7rs:r5rs`](#category__vonuvoli_r7rs__r7rs_r5rs), [`r7rs:read`](#category__vonuvoli_r7rs__r7rs_read), [`r7rs:repl`](#category__vonuvoli_r7rs__r7rs_repl), [`r7rs:time`](#category__vonuvoli_r7rs__r7rs_time), [`r7rs:write`](#category__vonuvoli_r7rs__r7rs_write), [`r7rs-x`](#category__vonuvoli_r7rs__r7rs-x);
* [`vs`](#category__vonuvoli_r7rs__vs): [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic), [`vs:associations`](#category__vonuvoli_r7rs__vs_associations), [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes), [`vs:booleans`](#category__vonuvoli_r7rs__vs_booleans), [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions), [`vs:globals`](#category__vonuvoli_r7rs__vs_globals), [`vs:file-system`](#category__vonuvoli_r7rs__vs_file-system), [`vs:characters`](#category__vonuvoli_r7rs__vs_characters), [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons), [`vs:compiler`](#category__vonuvoli_r7rs__vs_compiler), [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts), [`vs:continuations`](#category__vonuvoli_r7rs__vs_continuations), [`vs:control`](#category__vonuvoli_r7rs__vs_control), [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence), [`vs:errors`](#category__vonuvoli_r7rs__vs_errors), [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator), [`vs:functions`](#category__vonuvoli_r7rs__vs_functions), [`vs:lambda`](#category__vonuvoli_r7rs__vs_lambda), [`vs:lists`](#category__vonuvoli_r7rs__vs_lists), [`vs:loops`](#category__vonuvoli_r7rs__vs_loops), [`vs:modules`](#category__vonuvoli_r7rs__vs_modules), [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs), [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters), [`vs:ports`](#category__vonuvoli_r7rs__vs_ports), [`vs:promises`](#category__vonuvoli_r7rs__vs_promises), [`vs:quotation`](#category__vonuvoli_r7rs__vs_quotation), [`vs:records`](#category__vonuvoli_r7rs__vs_records), [`vs:strings`](#category__vonuvoli_r7rs__vs_strings), [`vs:symbols`](#category__vonuvoli_r7rs__vs_symbols), [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes), [`vs:system`](#category__vonuvoli_r7rs__vs_system), [`vs:types`](#category__vonuvoli_r7rs__vs_types), [`vs:unimplemented`](#category__vonuvoli_r7rs__vs_unimplemented), [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported), [`vs:values`](#category__vonuvoli_r7rs__vs_values), [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs'>

### Category `r7rs`

Contains the following sub-categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`r7rs:case-lambda`](#category__vonuvoli_r7rs__r7rs_case-lambda);
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`r7rs:eval`](#category__vonuvoli_r7rs__r7rs_eval);
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`r7rs:lazy`](#category__vonuvoli_r7rs__r7rs_lazy);
 * [`r7rs:load`](#category__vonuvoli_r7rs__r7rs_load);
 * [`r7rs:process-context`](#category__vonuvoli_r7rs__r7rs_process-context);
 * [`r7rs:r5rs`](#category__vonuvoli_r7rs__r7rs_r5rs);
 * [`r7rs:read`](#category__vonuvoli_r7rs__r7rs_read);
 * [`r7rs:repl`](#category__vonuvoli_r7rs__r7rs_repl);
 * [`r7rs:time`](#category__vonuvoli_r7rs__r7rs_time);
 * [`r7rs:write`](#category__vonuvoli_r7rs__r7rs_write);
 * [`r7rs-x`](#category__vonuvoli_r7rs__r7rs-x);


#### Description

> **FIXME!**



#### Types

 * [`any`](#value_kind__vonuvoli_r7rs__any);
 * [`null`](#value_kind__vonuvoli_r7rs__null);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);
 * [`number`](#value_kind__vonuvoli_r7rs__number);
 * [`symbol`](#value_kind__vonuvoli_r7rs__symbol);
 * [`character`](#value_kind__vonuvoli_r7rs__character);
 * [`string`](#value_kind__vonuvoli_r7rs__string);
 * [`bytevector`](#value_kind__vonuvoli_r7rs__bytevector);
 * [`vector`](#value_kind__vonuvoli_r7rs__vector);
 * [`pair`](#value_kind__vonuvoli_r7rs__pair);
 * [`port`](#value_kind__vonuvoli_r7rs__port);
 * [`eof-object`](#value_kind__vonuvoli_r7rs__eof-object);
 * [`procedure`](#value_kind__vonuvoli_r7rs__procedure);


#### Definitions

 * [`define-syntax`](#definition__vonuvoli_r7rs__define-syntax);
 * [`let-syntax`](#definition__vonuvoli_r7rs__let-syntax);
 * [`letrec-syntax`](#definition__vonuvoli_r7rs__letrec-syntax);
 * [`syntax-rules`](#definition__vonuvoli_r7rs__syntax-rules);
 * [`syntax-error`](#definition__vonuvoli_r7rs__syntax-error);
 * [`_`](#definition__vonuvoli_r7rs___);
 * [`...`](#definition__vonuvoli_r7rs_____);
 * [`=>`](#definition__vonuvoli_r7rs____);
 * [`else`](#definition__vonuvoli_r7rs__else);
 * [`quote`](#definition__vonuvoli_r7rs__quote);
 * [`quasiquote`](#definition__vonuvoli_r7rs__quasiquote);
 * [`unquote`](#definition__vonuvoli_r7rs__unquote);
 * [`unquote-splicing`](#definition__vonuvoli_r7rs__unquote-splicing);
 * [`lambda`](#definition__vonuvoli_r7rs__lambda);
 * [`case-lambda`](#definition__vonuvoli_r7rs__case-lambda);
 * [`define`](#definition__vonuvoli_r7rs__define);
 * [`let`](#definition__vonuvoli_r7rs__let);
 * [`let*`](#definition__vonuvoli_r7rs__let_);
 * [`letrec`](#definition__vonuvoli_r7rs__letrec);
 * [`letrec*`](#definition__vonuvoli_r7rs__letrec_);
 * [`set!`](#definition__vonuvoli_r7rs__set!);
 * [`define-values`](#definition__vonuvoli_r7rs__define-values);
 * [`let-values`](#definition__vonuvoli_r7rs__let-values);
 * [`let*-values`](#definition__vonuvoli_r7rs__let_-values);
 * [`define-record-type`](#definition__vonuvoli_r7rs__define-record-type);
 * [`begin`](#definition__vonuvoli_r7rs__begin);
 * [`and`](#definition__vonuvoli_r7rs__and);
 * [`or`](#definition__vonuvoli_r7rs__or);
 * [`if`](#definition__vonuvoli_r7rs__if);
 * [`unless`](#definition__vonuvoli_r7rs__unless);
 * [`when`](#definition__vonuvoli_r7rs__when);
 * [`cond`](#definition__vonuvoli_r7rs__cond);
 * [`case`](#definition__vonuvoli_r7rs__case);
 * [`do`](#definition__vonuvoli_r7rs__do);
 * [`eq?`](#definition__vonuvoli_r7rs__eq_);
 * [`eqv?`](#definition__vonuvoli_r7rs__eqv_);
 * [`equal?`](#definition__vonuvoli_r7rs__equal_);
 * [`not`](#definition__vonuvoli_r7rs__not);
 * [`boolean?`](#definition__vonuvoli_r7rs__boolean_);
 * [`boolean=?`](#definition__vonuvoli_r7rs__boolean__);
 * [`symbol?`](#definition__vonuvoli_r7rs__symbol_);
 * [`symbol=?`](#definition__vonuvoli_r7rs__symbol__);
 * [`number?`](#definition__vonuvoli_r7rs__number_);
 * [`integer?`](#definition__vonuvoli_r7rs__integer_);
 * [`real?`](#definition__vonuvoli_r7rs__real_);
 * [`rational?`](#definition__vonuvoli_r7rs__rational_);
 * [`complex?`](#definition__vonuvoli_r7rs__complex_);
 * [`exact?`](#definition__vonuvoli_r7rs__exact_);
 * [`inexact?`](#definition__vonuvoli_r7rs__inexact_);
 * [`exact-integer?`](#definition__vonuvoli_r7rs__exact-integer_);
 * [`zero?`](#definition__vonuvoli_r7rs__zero_);
 * [`positive?`](#definition__vonuvoli_r7rs__positive_);
 * [`odd?`](#definition__vonuvoli_r7rs__odd_);
 * [`even?`](#definition__vonuvoli_r7rs__even_);
 * [`=`](#definition__vonuvoli_r7rs___);
 * [`<`](#definition__vonuvoli_r7rs___);
 * [`>`](#definition__vonuvoli_r7rs___);
 * [`<=`](#definition__vonuvoli_r7rs____);
 * [`>=`](#definition__vonuvoli_r7rs____);
 * [`+`](#definition__vonuvoli_r7rs___);
 * [`-`](#definition__vonuvoli_r7rs__-);
 * [`*`](#definition__vonuvoli_r7rs___);
 * [`/`](#definition__vonuvoli_r7rs___);
 * [`abs`](#definition__vonuvoli_r7rs__abs);
 * [`floor/`](#definition__vonuvoli_r7rs__floor_);
 * [`floor-quotient`](#definition__vonuvoli_r7rs__floor-quotient);
 * [`floor-remainder`](#definition__vonuvoli_r7rs__floor-remainder);
 * [`truncate/`](#definition__vonuvoli_r7rs__truncate_);
 * [`truncate-quotient`](#definition__vonuvoli_r7rs__truncate-quotient);
 * [`truncate-remainder`](#definition__vonuvoli_r7rs__truncate-remainder);
 * [`floor`](#definition__vonuvoli_r7rs__floor);
 * [`ceiling`](#definition__vonuvoli_r7rs__ceiling);
 * [`truncate`](#definition__vonuvoli_r7rs__truncate);
 * [`round`](#definition__vonuvoli_r7rs__round);
 * [`min`](#definition__vonuvoli_r7rs__min);
 * [`max`](#definition__vonuvoli_r7rs__max);
 * [`gcd`](#definition__vonuvoli_r7rs__gcd);
 * [`lcm`](#definition__vonuvoli_r7rs__lcm);
 * [`expt`](#definition__vonuvoli_r7rs__expt);
 * [`square`](#definition__vonuvoli_r7rs__square);
 * [`exact-integer-sqrt`](#definition__vonuvoli_r7rs__exact-integer-sqrt);
 * [`rationalize`](#definition__vonuvoli_r7rs__rationalize);
 * [`numerator`](#definition__vonuvoli_r7rs__numerator);
 * [`denominator`](#definition__vonuvoli_r7rs__denominator);
 * [`inexact`](#definition__vonuvoli_r7rs__inexact);
 * [`exact`](#definition__vonuvoli_r7rs__exact);
 * [`make-rectangular`](#definition__vonuvoli_r7rs__make-rectangular);
 * [`real-part`](#definition__vonuvoli_r7rs__real-part);
 * [`imag-part`](#definition__vonuvoli_r7rs__imag-part);
 * [`make-polar`](#definition__vonuvoli_r7rs__make-polar);
 * [`magnitude`](#definition__vonuvoli_r7rs__magnitude);
 * [`angle`](#definition__vonuvoli_r7rs__angle);
 * [`sqrt`](#definition__vonuvoli_r7rs__sqrt);
 * [`exp`](#definition__vonuvoli_r7rs__exp);
 * [`log`](#definition__vonuvoli_r7rs__log);
 * [`sin`](#definition__vonuvoli_r7rs__sin);
 * [`cos`](#definition__vonuvoli_r7rs__cos);
 * [`tan`](#definition__vonuvoli_r7rs__tan);
 * [`asin`](#definition__vonuvoli_r7rs__asin);
 * [`acos`](#definition__vonuvoli_r7rs__acos);
 * [`atan`](#definition__vonuvoli_r7rs__atan);
 * [`finite?`](#definition__vonuvoli_r7rs__finite_);
 * [`infinite?`](#definition__vonuvoli_r7rs__infinite_);
 * [`nan?`](#definition__vonuvoli_r7rs__nan_);
 * [`pair?`](#definition__vonuvoli_r7rs__pair_);
 * [`cons`](#definition__vonuvoli_r7rs__cons);
 * [`car`](#definition__vonuvoli_r7rs__car);
 * [`cdr`](#definition__vonuvoli_r7rs__cdr);
 * [`set-car!`](#definition__vonuvoli_r7rs__set-car!);
 * [`set-cdr!`](#definition__vonuvoli_r7rs__set-cdr!);
 * [`caar`](#definition__vonuvoli_r7rs__caar);
 * [`cadr`](#definition__vonuvoli_r7rs__cadr);
 * [`cdar`](#definition__vonuvoli_r7rs__cdar);
 * [`cddr`](#definition__vonuvoli_r7rs__cddr);
 * [`caaar`](#definition__vonuvoli_r7rs__caaar);
 * [`caadr`](#definition__vonuvoli_r7rs__caadr);
 * [`cadar`](#definition__vonuvoli_r7rs__cadar);
 * [`caddr`](#definition__vonuvoli_r7rs__caddr);
 * [`cdaar`](#definition__vonuvoli_r7rs__cdaar);
 * [`cdadr`](#definition__vonuvoli_r7rs__cdadr);
 * [`cddar`](#definition__vonuvoli_r7rs__cddar);
 * [`cdddr`](#definition__vonuvoli_r7rs__cdddr);
 * [`caaaar`](#definition__vonuvoli_r7rs__caaaar);
 * [`caaadr`](#definition__vonuvoli_r7rs__caaadr);
 * [`caadar`](#definition__vonuvoli_r7rs__caadar);
 * [`caaddr`](#definition__vonuvoli_r7rs__caaddr);
 * [`cadaar`](#definition__vonuvoli_r7rs__cadaar);
 * [`cadadr`](#definition__vonuvoli_r7rs__cadadr);
 * [`caddar`](#definition__vonuvoli_r7rs__caddar);
 * [`cadddr`](#definition__vonuvoli_r7rs__cadddr);
 * [`cdaaar`](#definition__vonuvoli_r7rs__cdaaar);
 * [`cdaadr`](#definition__vonuvoli_r7rs__cdaadr);
 * [`cdadar`](#definition__vonuvoli_r7rs__cdadar);
 * [`cdaddr`](#definition__vonuvoli_r7rs__cdaddr);
 * [`cddaar`](#definition__vonuvoli_r7rs__cddaar);
 * [`cddadr`](#definition__vonuvoli_r7rs__cddadr);
 * [`cdddar`](#definition__vonuvoli_r7rs__cdddar);
 * [`cddddr`](#definition__vonuvoli_r7rs__cddddr);
 * [`null?`](#definition__vonuvoli_r7rs__null_);
 * [`list?`](#definition__vonuvoli_r7rs__list_);
 * [`list`](#definition__vonuvoli_r7rs__list);
 * [`make-list`](#definition__vonuvoli_r7rs__make-list);
 * [`length`](#definition__vonuvoli_r7rs__length);
 * [`append`](#definition__vonuvoli_r7rs__append);
 * [`list-copy`](#definition__vonuvoli_r7rs__list-copy);
 * [`reverse`](#definition__vonuvoli_r7rs__reverse);
 * [`list-ref`](#definition__vonuvoli_r7rs__list-ref);
 * [`list-tail`](#definition__vonuvoli_r7rs__list-tail);
 * [`list-set!`](#definition__vonuvoli_r7rs__list-set!);
 * [`map`](#definition__vonuvoli_r7rs__map);
 * [`for-each`](#definition__vonuvoli_r7rs__for-each);
 * [`member`](#definition__vonuvoli_r7rs__member);
 * [`memq`](#definition__vonuvoli_r7rs__memq);
 * [`memv`](#definition__vonuvoli_r7rs__memv);
 * [`assoc`](#definition__vonuvoli_r7rs__assoc);
 * [`assqc`](#definition__vonuvoli_r7rs__assqc);
 * [`assvc`](#definition__vonuvoli_r7rs__assvc);
 * [`vector?`](#definition__vonuvoli_r7rs__vector_);
 * [`vector`](#definition__vonuvoli_r7rs__vector);
 * [`make-vector`](#definition__vonuvoli_r7rs__make-vector);
 * [`vector-length`](#definition__vonuvoli_r7rs__vector-length);
 * [`vector-append`](#definition__vonuvoli_r7rs__vector-append);
 * [`vector-copy`](#definition__vonuvoli_r7rs__vector-copy);
 * [`vector-copy!`](#definition__vonuvoli_r7rs__vector-copy!);
 * [`vector-fill!`](#definition__vonuvoli_r7rs__vector-fill!);
 * [`vector-ref`](#definition__vonuvoli_r7rs__vector-ref);
 * [`vector-set!`](#definition__vonuvoli_r7rs__vector-set!);
 * [`vector->list`](#definition__vonuvoli_r7rs__vector-_list);
 * [`list->vector`](#definition__vonuvoli_r7rs__list-_vector);
 * [`vector-map`](#definition__vonuvoli_r7rs__vector-map);
 * [`vector-for-each`](#definition__vonuvoli_r7rs__vector-for-each);
 * [`string?`](#definition__vonuvoli_r7rs__string_);
 * [`string`](#definition__vonuvoli_r7rs__string);
 * [`make-string`](#definition__vonuvoli_r7rs__make-string);
 * [`string-length`](#definition__vonuvoli_r7rs__string-length);
 * [`string-append`](#definition__vonuvoli_r7rs__string-append);
 * [`string-copy`](#definition__vonuvoli_r7rs__string-copy);
 * [`string-copy!`](#definition__vonuvoli_r7rs__string-copy!);
 * [`string-fill!`](#definition__vonuvoli_r7rs__string-fill!);
 * [`substring`](#definition__vonuvoli_r7rs__substring);
 * [`string-ref`](#definition__vonuvoli_r7rs__string-ref);
 * [`string-set!`](#definition__vonuvoli_r7rs__string-set!);
 * [`string=?`](#definition__vonuvoli_r7rs__string__);
 * [`string<?`](#definition__vonuvoli_r7rs__string__);
 * [`string>?`](#definition__vonuvoli_r7rs__string__);
 * [`string<=?`](#definition__vonuvoli_r7rs__string___);
 * [`string>=?`](#definition__vonuvoli_r7rs__string___);
 * [`string-ci=?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci>?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`string-ci>=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`number->string`](#definition__vonuvoli_r7rs__number-_string);
 * [`string->number`](#definition__vonuvoli_r7rs__string-_number);
 * [`symbol->string`](#definition__vonuvoli_r7rs__symbol-_string);
 * [`string->symbol`](#definition__vonuvoli_r7rs__string-_symbol);
 * [`list->string`](#definition__vonuvoli_r7rs__list-_string);
 * [`string->list`](#definition__vonuvoli_r7rs__string-_list);
 * [`vector->string`](#definition__vonuvoli_r7rs__vector-_string);
 * [`string->vector`](#definition__vonuvoli_r7rs__string-_vector);
 * [`string-map`](#definition__vonuvoli_r7rs__string-map);
 * [`string-for-each`](#definition__vonuvoli_r7rs__string-for-each);
 * [`string-upcase`](#definition__vonuvoli_r7rs__string-upcase);
 * [`string-downcase`](#definition__vonuvoli_r7rs__string-downcase);
 * [`string-foldcase`](#definition__vonuvoli_r7rs__string-foldcase);
 * [`bytevector?`](#definition__vonuvoli_r7rs__bytevector_);
 * [`bytevector`](#definition__vonuvoli_r7rs__bytevector);
 * [`make-bytevector`](#definition__vonuvoli_r7rs__make-bytevector);
 * [`bytevector-length`](#definition__vonuvoli_r7rs__bytevector-length);
 * [`bytevector-append`](#definition__vonuvoli_r7rs__bytevector-append);
 * [`bytevector-copy`](#definition__vonuvoli_r7rs__bytevector-copy);
 * [`bytevector-copy!`](#definition__vonuvoli_r7rs__bytevector-copy!);
 * [`bytevector-u8-ref`](#definition__vonuvoli_r7rs__bytevector-u8-ref);
 * [`bytevector-u8-set!`](#definition__vonuvoli_r7rs__bytevector-u8-set!);
 * [`utf8->string`](#definition__vonuvoli_r7rs__utf8-_string);
 * [`string->utf8`](#definition__vonuvoli_r7rs__string-_utf8);
 * [`port?`](#definition__vonuvoli_r7rs__port_);
 * [`binary-port?`](#definition__vonuvoli_r7rs__binary-port_);
 * [`textual-port?`](#definition__vonuvoli_r7rs__textual-port_);
 * [`input-port?`](#definition__vonuvoli_r7rs__input-port_);
 * [`input-port-open?`](#definition__vonuvoli_r7rs__input-port-open_);
 * [`output-port?`](#definition__vonuvoli_r7rs__output-port_);
 * [`output-port-open?`](#definition__vonuvoli_r7rs__output-port-open_);
 * [`open-input-bytevector`](#definition__vonuvoli_r7rs__open-input-bytevector);
 * [`open-output-bytevector`](#definition__vonuvoli_r7rs__open-output-bytevector);
 * [`get-output-bytevector`](#definition__vonuvoli_r7rs__get-output-bytevector);
 * [`open-input-string`](#definition__vonuvoli_r7rs__open-input-string);
 * [`open-output-string`](#definition__vonuvoli_r7rs__open-output-string);
 * [`get-output-string`](#definition__vonuvoli_r7rs__get-output-string);
 * [`close-port`](#definition__vonuvoli_r7rs__close-port);
 * [`close-input-port`](#definition__vonuvoli_r7rs__close-input-port);
 * [`close-output-port`](#definition__vonuvoli_r7rs__close-output-port);
 * [`u8-ready?`](#definition__vonuvoli_r7rs__u8-ready_);
 * [`peek-u8`](#definition__vonuvoli_r7rs__peek-u8);
 * [`read-u8`](#definition__vonuvoli_r7rs__read-u8);
 * [`write-u8`](#definition__vonuvoli_r7rs__write-u8);
 * [`read-bytevector`](#definition__vonuvoli_r7rs__read-bytevector);
 * [`read-bytevector!`](#definition__vonuvoli_r7rs__read-bytevector!);
 * [`write-bytevector`](#definition__vonuvoli_r7rs__write-bytevector);
 * [`char-ready?`](#definition__vonuvoli_r7rs__char-ready_);
 * [`peek-char`](#definition__vonuvoli_r7rs__peek-char);
 * [`read-char`](#definition__vonuvoli_r7rs__read-char);
 * [`write-char`](#definition__vonuvoli_r7rs__write-char);
 * [`read-string`](#definition__vonuvoli_r7rs__read-string);
 * [`read-line`](#definition__vonuvoli_r7rs__read-line);
 * [`newline`](#definition__vonuvoli_r7rs__newline);
 * [`flush-output-port`](#definition__vonuvoli_r7rs__flush-output-port);
 * [`read`](#definition__vonuvoli_r7rs__read);
 * [`write`](#definition__vonuvoli_r7rs__write);
 * [`write-simple`](#definition__vonuvoli_r7rs__write-simple);
 * [`write-shared`](#definition__vonuvoli_r7rs__write-shared);
 * [`display`](#definition__vonuvoli_r7rs__display);
 * [`open-input-file`](#definition__vonuvoli_r7rs__open-input-file);
 * [`open-binary-input-file`](#definition__vonuvoli_r7rs__open-binary-input-file);
 * [`open-output-file`](#definition__vonuvoli_r7rs__open-output-file);
 * [`open-binary-output-file`](#definition__vonuvoli_r7rs__open-binary-output-file);
 * [`call-with-port`](#definition__vonuvoli_r7rs__call-with-port);
 * [`call-with-input-file`](#definition__vonuvoli_r7rs__call-with-input-file);
 * [`call-with-output-file`](#definition__vonuvoli_r7rs__call-with-output-file);
 * [`eof-object`](#definition__vonuvoli_r7rs__eof-object);
 * [`eof-object?`](#definition__vonuvoli_r7rs__eof-object_);
 * [`file-exists?`](#definition__vonuvoli_r7rs__file-exists_);
 * [`delete-file`](#definition__vonuvoli_r7rs__delete-file);
 * [`exit`](#definition__vonuvoli_r7rs__exit);
 * [`emergency-exit`](#definition__vonuvoli_r7rs__emergency-exit);
 * [`command-line`](#definition__vonuvoli_r7rs__command-line);
 * [`get-environment-variable`](#definition__vonuvoli_r7rs__get-environment-variable);
 * [`get-environment-variables`](#definition__vonuvoli_r7rs__get-environment-variables);
 * [`current-second`](#definition__vonuvoli_r7rs__current-second);
 * [`current-jiffy`](#definition__vonuvoli_r7rs__current-jiffy);
 * [`jiffies-per-second`](#definition__vonuvoli_r7rs__jiffies-per-second);
 * [`char?`](#definition__vonuvoli_r7rs__char_);
 * [`char=?`](#definition__vonuvoli_r7rs__char__);
 * [`char<?`](#definition__vonuvoli_r7rs__char__);
 * [`char>?`](#definition__vonuvoli_r7rs__char__);
 * [`char<=?`](#definition__vonuvoli_r7rs__char___);
 * [`char>=?`](#definition__vonuvoli_r7rs__char___);
 * [`char-ci=?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci>?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`char-ci>=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`char->integer`](#definition__vonuvoli_r7rs__char-_integer);
 * [`integer->char`](#definition__vonuvoli_r7rs__integer-_char);
 * [`digit-value`](#definition__vonuvoli_r7rs__digit-value);
 * [`char-alphabetic?`](#definition__vonuvoli_r7rs__char-alphabetic_);
 * [`char-upper-case?`](#definition__vonuvoli_r7rs__char-upper-case_);
 * [`char-lower-case?`](#definition__vonuvoli_r7rs__char-lower-case_);
 * [`char-numeric?`](#definition__vonuvoli_r7rs__char-numeric_);
 * [`char-whitespace?`](#definition__vonuvoli_r7rs__char-whitespace_);
 * [`char-upcase`](#definition__vonuvoli_r7rs__char-upcase);
 * [`char-downcase`](#definition__vonuvoli_r7rs__char-downcase);
 * [`char-foldcase`](#definition__vonuvoli_r7rs__char-foldcase);
 * [`procedure?`](#definition__vonuvoli_r7rs__procedure_);
 * [`apply`](#definition__vonuvoli_r7rs__apply);
 * [`values`](#definition__vonuvoli_r7rs__values);
 * [`call-with-values`](#definition__vonuvoli_r7rs__call-with-values);
 * [`error-object?`](#definition__vonuvoli_r7rs__error-object_);
 * [`read-error?`](#definition__vonuvoli_r7rs__read-error_);
 * [`file-error?`](#definition__vonuvoli_r7rs__file-error_);
 * [`error`](#definition__vonuvoli_r7rs__error);
 * [`error-object-message`](#definition__vonuvoli_r7rs__error-object-message);
 * [`error-object-irritants`](#definition__vonuvoli_r7rs__error-object-irritants);
 * [`guard`](#definition__vonuvoli_r7rs__guard);
 * [`with-exception-handler`](#definition__vonuvoli_r7rs__with-exception-handler);
 * [`raise`](#definition__vonuvoli_r7rs__raise);
 * [`raise-continuable`](#definition__vonuvoli_r7rs__raise-continuable);
 * [`parameterize`](#definition__vonuvoli_r7rs__parameterize);
 * [`make-parameter`](#definition__vonuvoli_r7rs__make-parameter);
 * [`current-input-port`](#definition__vonuvoli_r7rs__current-input-port);
 * [`current-output-port`](#definition__vonuvoli_r7rs__current-output-port);
 * [`current-error-port`](#definition__vonuvoli_r7rs__current-error-port);
 * [`with-input-from-file`](#definition__vonuvoli_r7rs__with-input-from-file);
 * [`with-output-from-file`](#definition__vonuvoli_r7rs__with-output-from-file);
 * [`delay`](#definition__vonuvoli_r7rs__delay);
 * [`delay-force`](#definition__vonuvoli_r7rs__delay-force);
 * [`promise?`](#definition__vonuvoli_r7rs__promise_);
 * [`make-promise`](#definition__vonuvoli_r7rs__make-promise);
 * [`force`](#definition__vonuvoli_r7rs__force);
 * [`eval`](#definition__vonuvoli_r7rs__eval);
 * [`environment`](#definition__vonuvoli_r7rs__environment);
 * [`interaction-environment`](#definition__vonuvoli_r7rs__interaction-environment);
 * [`scheme-report-environment`](#definition__vonuvoli_r7rs__scheme-report-environment);
 * [`null-environment`](#definition__vonuvoli_r7rs__null-environment);
 * [`call-with-current-continuation`](#definition__vonuvoli_r7rs__call-with-current-continuation);
 * [`dynamic-wind`](#definition__vonuvoli_r7rs__dynamic-wind);
 * [`cond-expand`](#definition__vonuvoli_r7rs__cond-expand);
 * [`features`](#definition__vonuvoli_r7rs__features);
 * [`include`](#definition__vonuvoli_r7rs__include);
 * [`include-ci`](#definition__vonuvoli_r7rs__include-ci);
 * [`import`](#definition__vonuvoli_r7rs__import);
 * [`load`](#definition__vonuvoli_r7rs__load);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_base'>

### Category `r7rs:base`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`define-syntax`](#definition__vonuvoli_r7rs__define-syntax);
 * [`let-syntax`](#definition__vonuvoli_r7rs__let-syntax);
 * [`letrec-syntax`](#definition__vonuvoli_r7rs__letrec-syntax);
 * [`syntax-rules`](#definition__vonuvoli_r7rs__syntax-rules);
 * [`syntax-error`](#definition__vonuvoli_r7rs__syntax-error);
 * [`_`](#definition__vonuvoli_r7rs___);
 * [`...`](#definition__vonuvoli_r7rs_____);
 * [`=>`](#definition__vonuvoli_r7rs____);
 * [`else`](#definition__vonuvoli_r7rs__else);
 * [`quote`](#definition__vonuvoli_r7rs__quote);
 * [`quasiquote`](#definition__vonuvoli_r7rs__quasiquote);
 * [`unquote`](#definition__vonuvoli_r7rs__unquote);
 * [`unquote-splicing`](#definition__vonuvoli_r7rs__unquote-splicing);
 * [`lambda`](#definition__vonuvoli_r7rs__lambda);
 * [`define`](#definition__vonuvoli_r7rs__define);
 * [`let`](#definition__vonuvoli_r7rs__let);
 * [`let*`](#definition__vonuvoli_r7rs__let_);
 * [`letrec`](#definition__vonuvoli_r7rs__letrec);
 * [`letrec*`](#definition__vonuvoli_r7rs__letrec_);
 * [`set!`](#definition__vonuvoli_r7rs__set!);
 * [`define-values`](#definition__vonuvoli_r7rs__define-values);
 * [`let-values`](#definition__vonuvoli_r7rs__let-values);
 * [`let*-values`](#definition__vonuvoli_r7rs__let_-values);
 * [`define-record-type`](#definition__vonuvoli_r7rs__define-record-type);
 * [`begin`](#definition__vonuvoli_r7rs__begin);
 * [`and`](#definition__vonuvoli_r7rs__and);
 * [`or`](#definition__vonuvoli_r7rs__or);
 * [`if`](#definition__vonuvoli_r7rs__if);
 * [`unless`](#definition__vonuvoli_r7rs__unless);
 * [`when`](#definition__vonuvoli_r7rs__when);
 * [`cond`](#definition__vonuvoli_r7rs__cond);
 * [`case`](#definition__vonuvoli_r7rs__case);
 * [`do`](#definition__vonuvoli_r7rs__do);
 * [`eq?`](#definition__vonuvoli_r7rs__eq_);
 * [`eqv?`](#definition__vonuvoli_r7rs__eqv_);
 * [`equal?`](#definition__vonuvoli_r7rs__equal_);
 * [`not`](#definition__vonuvoli_r7rs__not);
 * [`boolean?`](#definition__vonuvoli_r7rs__boolean_);
 * [`boolean=?`](#definition__vonuvoli_r7rs__boolean__);
 * [`symbol?`](#definition__vonuvoli_r7rs__symbol_);
 * [`symbol=?`](#definition__vonuvoli_r7rs__symbol__);
 * [`number?`](#definition__vonuvoli_r7rs__number_);
 * [`integer?`](#definition__vonuvoli_r7rs__integer_);
 * [`real?`](#definition__vonuvoli_r7rs__real_);
 * [`rational?`](#definition__vonuvoli_r7rs__rational_);
 * [`complex?`](#definition__vonuvoli_r7rs__complex_);
 * [`exact?`](#definition__vonuvoli_r7rs__exact_);
 * [`inexact?`](#definition__vonuvoli_r7rs__inexact_);
 * [`exact-integer?`](#definition__vonuvoli_r7rs__exact-integer_);
 * [`zero?`](#definition__vonuvoli_r7rs__zero_);
 * [`positive?`](#definition__vonuvoli_r7rs__positive_);
 * [`odd?`](#definition__vonuvoli_r7rs__odd_);
 * [`even?`](#definition__vonuvoli_r7rs__even_);
 * [`=`](#definition__vonuvoli_r7rs___);
 * [`<`](#definition__vonuvoli_r7rs___);
 * [`>`](#definition__vonuvoli_r7rs___);
 * [`<=`](#definition__vonuvoli_r7rs____);
 * [`>=`](#definition__vonuvoli_r7rs____);
 * [`+`](#definition__vonuvoli_r7rs___);
 * [`-`](#definition__vonuvoli_r7rs__-);
 * [`*`](#definition__vonuvoli_r7rs___);
 * [`/`](#definition__vonuvoli_r7rs___);
 * [`abs`](#definition__vonuvoli_r7rs__abs);
 * [`floor/`](#definition__vonuvoli_r7rs__floor_);
 * [`floor-quotient`](#definition__vonuvoli_r7rs__floor-quotient);
 * [`floor-remainder`](#definition__vonuvoli_r7rs__floor-remainder);
 * [`truncate/`](#definition__vonuvoli_r7rs__truncate_);
 * [`truncate-quotient`](#definition__vonuvoli_r7rs__truncate-quotient);
 * [`truncate-remainder`](#definition__vonuvoli_r7rs__truncate-remainder);
 * [`floor`](#definition__vonuvoli_r7rs__floor);
 * [`ceiling`](#definition__vonuvoli_r7rs__ceiling);
 * [`truncate`](#definition__vonuvoli_r7rs__truncate);
 * [`round`](#definition__vonuvoli_r7rs__round);
 * [`min`](#definition__vonuvoli_r7rs__min);
 * [`max`](#definition__vonuvoli_r7rs__max);
 * [`gcd`](#definition__vonuvoli_r7rs__gcd);
 * [`lcm`](#definition__vonuvoli_r7rs__lcm);
 * [`expt`](#definition__vonuvoli_r7rs__expt);
 * [`square`](#definition__vonuvoli_r7rs__square);
 * [`exact-integer-sqrt`](#definition__vonuvoli_r7rs__exact-integer-sqrt);
 * [`rationalize`](#definition__vonuvoli_r7rs__rationalize);
 * [`numerator`](#definition__vonuvoli_r7rs__numerator);
 * [`denominator`](#definition__vonuvoli_r7rs__denominator);
 * [`pair?`](#definition__vonuvoli_r7rs__pair_);
 * [`cons`](#definition__vonuvoli_r7rs__cons);
 * [`car`](#definition__vonuvoli_r7rs__car);
 * [`cdr`](#definition__vonuvoli_r7rs__cdr);
 * [`set-car!`](#definition__vonuvoli_r7rs__set-car!);
 * [`set-cdr!`](#definition__vonuvoli_r7rs__set-cdr!);
 * [`caar`](#definition__vonuvoli_r7rs__caar);
 * [`cadr`](#definition__vonuvoli_r7rs__cadr);
 * [`cdar`](#definition__vonuvoli_r7rs__cdar);
 * [`cddr`](#definition__vonuvoli_r7rs__cddr);
 * [`null?`](#definition__vonuvoli_r7rs__null_);
 * [`list?`](#definition__vonuvoli_r7rs__list_);
 * [`list`](#definition__vonuvoli_r7rs__list);
 * [`make-list`](#definition__vonuvoli_r7rs__make-list);
 * [`length`](#definition__vonuvoli_r7rs__length);
 * [`append`](#definition__vonuvoli_r7rs__append);
 * [`list-copy`](#definition__vonuvoli_r7rs__list-copy);
 * [`reverse`](#definition__vonuvoli_r7rs__reverse);
 * [`list-ref`](#definition__vonuvoli_r7rs__list-ref);
 * [`list-tail`](#definition__vonuvoli_r7rs__list-tail);
 * [`list-set!`](#definition__vonuvoli_r7rs__list-set!);
 * [`map`](#definition__vonuvoli_r7rs__map);
 * [`for-each`](#definition__vonuvoli_r7rs__for-each);
 * [`member`](#definition__vonuvoli_r7rs__member);
 * [`memq`](#definition__vonuvoli_r7rs__memq);
 * [`memv`](#definition__vonuvoli_r7rs__memv);
 * [`assoc`](#definition__vonuvoli_r7rs__assoc);
 * [`assqc`](#definition__vonuvoli_r7rs__assqc);
 * [`assvc`](#definition__vonuvoli_r7rs__assvc);
 * [`vector?`](#definition__vonuvoli_r7rs__vector_);
 * [`vector`](#definition__vonuvoli_r7rs__vector);
 * [`make-vector`](#definition__vonuvoli_r7rs__make-vector);
 * [`vector-length`](#definition__vonuvoli_r7rs__vector-length);
 * [`vector-append`](#definition__vonuvoli_r7rs__vector-append);
 * [`vector-copy`](#definition__vonuvoli_r7rs__vector-copy);
 * [`vector-copy!`](#definition__vonuvoli_r7rs__vector-copy!);
 * [`vector-fill!`](#definition__vonuvoli_r7rs__vector-fill!);
 * [`vector-ref`](#definition__vonuvoli_r7rs__vector-ref);
 * [`vector-set!`](#definition__vonuvoli_r7rs__vector-set!);
 * [`vector->list`](#definition__vonuvoli_r7rs__vector-_list);
 * [`list->vector`](#definition__vonuvoli_r7rs__list-_vector);
 * [`vector-map`](#definition__vonuvoli_r7rs__vector-map);
 * [`vector-for-each`](#definition__vonuvoli_r7rs__vector-for-each);
 * [`string?`](#definition__vonuvoli_r7rs__string_);
 * [`string`](#definition__vonuvoli_r7rs__string);
 * [`make-string`](#definition__vonuvoli_r7rs__make-string);
 * [`string-length`](#definition__vonuvoli_r7rs__string-length);
 * [`string-append`](#definition__vonuvoli_r7rs__string-append);
 * [`string-copy`](#definition__vonuvoli_r7rs__string-copy);
 * [`string-copy!`](#definition__vonuvoli_r7rs__string-copy!);
 * [`string-fill!`](#definition__vonuvoli_r7rs__string-fill!);
 * [`substring`](#definition__vonuvoli_r7rs__substring);
 * [`string-ref`](#definition__vonuvoli_r7rs__string-ref);
 * [`string-set!`](#definition__vonuvoli_r7rs__string-set!);
 * [`string=?`](#definition__vonuvoli_r7rs__string__);
 * [`string<?`](#definition__vonuvoli_r7rs__string__);
 * [`string>?`](#definition__vonuvoli_r7rs__string__);
 * [`string<=?`](#definition__vonuvoli_r7rs__string___);
 * [`string>=?`](#definition__vonuvoli_r7rs__string___);
 * [`number->string`](#definition__vonuvoli_r7rs__number-_string);
 * [`string->number`](#definition__vonuvoli_r7rs__string-_number);
 * [`symbol->string`](#definition__vonuvoli_r7rs__symbol-_string);
 * [`string->symbol`](#definition__vonuvoli_r7rs__string-_symbol);
 * [`list->string`](#definition__vonuvoli_r7rs__list-_string);
 * [`string->list`](#definition__vonuvoli_r7rs__string-_list);
 * [`vector->string`](#definition__vonuvoli_r7rs__vector-_string);
 * [`string->vector`](#definition__vonuvoli_r7rs__string-_vector);
 * [`string-map`](#definition__vonuvoli_r7rs__string-map);
 * [`string-for-each`](#definition__vonuvoli_r7rs__string-for-each);
 * [`bytevector?`](#definition__vonuvoli_r7rs__bytevector_);
 * [`bytevector`](#definition__vonuvoli_r7rs__bytevector);
 * [`make-bytevector`](#definition__vonuvoli_r7rs__make-bytevector);
 * [`bytevector-length`](#definition__vonuvoli_r7rs__bytevector-length);
 * [`bytevector-append`](#definition__vonuvoli_r7rs__bytevector-append);
 * [`bytevector-copy`](#definition__vonuvoli_r7rs__bytevector-copy);
 * [`bytevector-copy!`](#definition__vonuvoli_r7rs__bytevector-copy!);
 * [`bytevector-u8-ref`](#definition__vonuvoli_r7rs__bytevector-u8-ref);
 * [`bytevector-u8-set!`](#definition__vonuvoli_r7rs__bytevector-u8-set!);
 * [`utf8->string`](#definition__vonuvoli_r7rs__utf8-_string);
 * [`string->utf8`](#definition__vonuvoli_r7rs__string-_utf8);
 * [`port?`](#definition__vonuvoli_r7rs__port_);
 * [`binary-port?`](#definition__vonuvoli_r7rs__binary-port_);
 * [`textual-port?`](#definition__vonuvoli_r7rs__textual-port_);
 * [`input-port?`](#definition__vonuvoli_r7rs__input-port_);
 * [`input-port-open?`](#definition__vonuvoli_r7rs__input-port-open_);
 * [`output-port?`](#definition__vonuvoli_r7rs__output-port_);
 * [`output-port-open?`](#definition__vonuvoli_r7rs__output-port-open_);
 * [`open-input-bytevector`](#definition__vonuvoli_r7rs__open-input-bytevector);
 * [`open-output-bytevector`](#definition__vonuvoli_r7rs__open-output-bytevector);
 * [`get-output-bytevector`](#definition__vonuvoli_r7rs__get-output-bytevector);
 * [`open-input-string`](#definition__vonuvoli_r7rs__open-input-string);
 * [`open-output-string`](#definition__vonuvoli_r7rs__open-output-string);
 * [`get-output-string`](#definition__vonuvoli_r7rs__get-output-string);
 * [`close-port`](#definition__vonuvoli_r7rs__close-port);
 * [`close-input-port`](#definition__vonuvoli_r7rs__close-input-port);
 * [`close-output-port`](#definition__vonuvoli_r7rs__close-output-port);
 * [`u8-ready?`](#definition__vonuvoli_r7rs__u8-ready_);
 * [`peek-u8`](#definition__vonuvoli_r7rs__peek-u8);
 * [`read-u8`](#definition__vonuvoli_r7rs__read-u8);
 * [`write-u8`](#definition__vonuvoli_r7rs__write-u8);
 * [`read-bytevector`](#definition__vonuvoli_r7rs__read-bytevector);
 * [`read-bytevector!`](#definition__vonuvoli_r7rs__read-bytevector!);
 * [`write-bytevector`](#definition__vonuvoli_r7rs__write-bytevector);
 * [`char-ready?`](#definition__vonuvoli_r7rs__char-ready_);
 * [`peek-char`](#definition__vonuvoli_r7rs__peek-char);
 * [`read-char`](#definition__vonuvoli_r7rs__read-char);
 * [`write-char`](#definition__vonuvoli_r7rs__write-char);
 * [`read-string`](#definition__vonuvoli_r7rs__read-string);
 * [`read-line`](#definition__vonuvoli_r7rs__read-line);
 * [`newline`](#definition__vonuvoli_r7rs__newline);
 * [`flush-output-port`](#definition__vonuvoli_r7rs__flush-output-port);
 * [`call-with-port`](#definition__vonuvoli_r7rs__call-with-port);
 * [`eof-object`](#definition__vonuvoli_r7rs__eof-object);
 * [`eof-object?`](#definition__vonuvoli_r7rs__eof-object_);
 * [`char?`](#definition__vonuvoli_r7rs__char_);
 * [`char=?`](#definition__vonuvoli_r7rs__char__);
 * [`char<?`](#definition__vonuvoli_r7rs__char__);
 * [`char>?`](#definition__vonuvoli_r7rs__char__);
 * [`char<=?`](#definition__vonuvoli_r7rs__char___);
 * [`char>=?`](#definition__vonuvoli_r7rs__char___);
 * [`char->integer`](#definition__vonuvoli_r7rs__char-_integer);
 * [`integer->char`](#definition__vonuvoli_r7rs__integer-_char);
 * [`procedure?`](#definition__vonuvoli_r7rs__procedure_);
 * [`apply`](#definition__vonuvoli_r7rs__apply);
 * [`values`](#definition__vonuvoli_r7rs__values);
 * [`call-with-values`](#definition__vonuvoli_r7rs__call-with-values);
 * [`error-object?`](#definition__vonuvoli_r7rs__error-object_);
 * [`read-error?`](#definition__vonuvoli_r7rs__read-error_);
 * [`file-error?`](#definition__vonuvoli_r7rs__file-error_);
 * [`error`](#definition__vonuvoli_r7rs__error);
 * [`error-object-message`](#definition__vonuvoli_r7rs__error-object-message);
 * [`error-object-irritants`](#definition__vonuvoli_r7rs__error-object-irritants);
 * [`guard`](#definition__vonuvoli_r7rs__guard);
 * [`with-exception-handler`](#definition__vonuvoli_r7rs__with-exception-handler);
 * [`raise`](#definition__vonuvoli_r7rs__raise);
 * [`raise-continuable`](#definition__vonuvoli_r7rs__raise-continuable);
 * [`parameterize`](#definition__vonuvoli_r7rs__parameterize);
 * [`make-parameter`](#definition__vonuvoli_r7rs__make-parameter);
 * [`current-input-port`](#definition__vonuvoli_r7rs__current-input-port);
 * [`current-output-port`](#definition__vonuvoli_r7rs__current-output-port);
 * [`current-error-port`](#definition__vonuvoli_r7rs__current-error-port);
 * [`call-with-current-continuation`](#definition__vonuvoli_r7rs__call-with-current-continuation);
 * [`dynamic-wind`](#definition__vonuvoli_r7rs__dynamic-wind);
 * [`cond-expand`](#definition__vonuvoli_r7rs__cond-expand);
 * [`features`](#definition__vonuvoli_r7rs__features);
 * [`include`](#definition__vonuvoli_r7rs__include);
 * [`include-ci`](#definition__vonuvoli_r7rs__include-ci);
 * [`import`](#definition__vonuvoli_r7rs__import);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_case-lambda'>

### Category `r7rs:case-lambda`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`case-lambda`](#definition__vonuvoli_r7rs__case-lambda);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_char'>

### Category `r7rs:char`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`string-ci=?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci>?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`string-ci>=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`string-upcase`](#definition__vonuvoli_r7rs__string-upcase);
 * [`string-downcase`](#definition__vonuvoli_r7rs__string-downcase);
 * [`string-foldcase`](#definition__vonuvoli_r7rs__string-foldcase);
 * [`char-ci=?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci>?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`char-ci>=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`digit-value`](#definition__vonuvoli_r7rs__digit-value);
 * [`char-alphabetic?`](#definition__vonuvoli_r7rs__char-alphabetic_);
 * [`char-upper-case?`](#definition__vonuvoli_r7rs__char-upper-case_);
 * [`char-lower-case?`](#definition__vonuvoli_r7rs__char-lower-case_);
 * [`char-numeric?`](#definition__vonuvoli_r7rs__char-numeric_);
 * [`char-whitespace?`](#definition__vonuvoli_r7rs__char-whitespace_);
 * [`char-upcase`](#definition__vonuvoli_r7rs__char-upcase);
 * [`char-downcase`](#definition__vonuvoli_r7rs__char-downcase);
 * [`char-foldcase`](#definition__vonuvoli_r7rs__char-foldcase);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_complex'>

### Category `r7rs:complex`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`inexact`](#definition__vonuvoli_r7rs__inexact);
 * [`exact`](#definition__vonuvoli_r7rs__exact);
 * [`make-rectangular`](#definition__vonuvoli_r7rs__make-rectangular);
 * [`real-part`](#definition__vonuvoli_r7rs__real-part);
 * [`imag-part`](#definition__vonuvoli_r7rs__imag-part);
 * [`make-polar`](#definition__vonuvoli_r7rs__make-polar);
 * [`magnitude`](#definition__vonuvoli_r7rs__magnitude);
 * [`angle`](#definition__vonuvoli_r7rs__angle);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_cxr'>

### Category `r7rs:cxr`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`caaar`](#definition__vonuvoli_r7rs__caaar);
 * [`caadr`](#definition__vonuvoli_r7rs__caadr);
 * [`cadar`](#definition__vonuvoli_r7rs__cadar);
 * [`caddr`](#definition__vonuvoli_r7rs__caddr);
 * [`cdaar`](#definition__vonuvoli_r7rs__cdaar);
 * [`cdadr`](#definition__vonuvoli_r7rs__cdadr);
 * [`cddar`](#definition__vonuvoli_r7rs__cddar);
 * [`cdddr`](#definition__vonuvoli_r7rs__cdddr);
 * [`caaaar`](#definition__vonuvoli_r7rs__caaaar);
 * [`caaadr`](#definition__vonuvoli_r7rs__caaadr);
 * [`caadar`](#definition__vonuvoli_r7rs__caadar);
 * [`caaddr`](#definition__vonuvoli_r7rs__caaddr);
 * [`cadaar`](#definition__vonuvoli_r7rs__cadaar);
 * [`cadadr`](#definition__vonuvoli_r7rs__cadadr);
 * [`caddar`](#definition__vonuvoli_r7rs__caddar);
 * [`cadddr`](#definition__vonuvoli_r7rs__cadddr);
 * [`cdaaar`](#definition__vonuvoli_r7rs__cdaaar);
 * [`cdaadr`](#definition__vonuvoli_r7rs__cdaadr);
 * [`cdadar`](#definition__vonuvoli_r7rs__cdadar);
 * [`cdaddr`](#definition__vonuvoli_r7rs__cdaddr);
 * [`cddaar`](#definition__vonuvoli_r7rs__cddaar);
 * [`cddadr`](#definition__vonuvoli_r7rs__cddadr);
 * [`cdddar`](#definition__vonuvoli_r7rs__cdddar);
 * [`cddddr`](#definition__vonuvoli_r7rs__cddddr);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_eval'>

### Category `r7rs:eval`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`eval`](#definition__vonuvoli_r7rs__eval);
 * [`environment`](#definition__vonuvoli_r7rs__environment);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_file'>

### Category `r7rs:file`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`open-input-file`](#definition__vonuvoli_r7rs__open-input-file);
 * [`open-binary-input-file`](#definition__vonuvoli_r7rs__open-binary-input-file);
 * [`open-output-file`](#definition__vonuvoli_r7rs__open-output-file);
 * [`open-binary-output-file`](#definition__vonuvoli_r7rs__open-binary-output-file);
 * [`call-with-input-file`](#definition__vonuvoli_r7rs__call-with-input-file);
 * [`call-with-output-file`](#definition__vonuvoli_r7rs__call-with-output-file);
 * [`file-exists?`](#definition__vonuvoli_r7rs__file-exists_);
 * [`delete-file`](#definition__vonuvoli_r7rs__delete-file);
 * [`with-input-from-file`](#definition__vonuvoli_r7rs__with-input-from-file);
 * [`with-output-from-file`](#definition__vonuvoli_r7rs__with-output-from-file);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_inexact'>

### Category `r7rs:inexact`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`sqrt`](#definition__vonuvoli_r7rs__sqrt);
 * [`exp`](#definition__vonuvoli_r7rs__exp);
 * [`log`](#definition__vonuvoli_r7rs__log);
 * [`sin`](#definition__vonuvoli_r7rs__sin);
 * [`cos`](#definition__vonuvoli_r7rs__cos);
 * [`tan`](#definition__vonuvoli_r7rs__tan);
 * [`asin`](#definition__vonuvoli_r7rs__asin);
 * [`acos`](#definition__vonuvoli_r7rs__acos);
 * [`atan`](#definition__vonuvoli_r7rs__atan);
 * [`finite?`](#definition__vonuvoli_r7rs__finite_);
 * [`infinite?`](#definition__vonuvoli_r7rs__infinite_);
 * [`nan?`](#definition__vonuvoli_r7rs__nan_);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_lazy'>

### Category `r7rs:lazy`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`delay`](#definition__vonuvoli_r7rs__delay);
 * [`delay-force`](#definition__vonuvoli_r7rs__delay-force);
 * [`promise?`](#definition__vonuvoli_r7rs__promise_);
 * [`make-promise`](#definition__vonuvoli_r7rs__make-promise);
 * [`force`](#definition__vonuvoli_r7rs__force);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_load'>

### Category `r7rs:load`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`load`](#definition__vonuvoli_r7rs__load);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_process-context'>

### Category `r7rs:process-context`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`exit`](#definition__vonuvoli_r7rs__exit);
 * [`emergency-exit`](#definition__vonuvoli_r7rs__emergency-exit);
 * [`command-line`](#definition__vonuvoli_r7rs__command-line);
 * [`get-environment-variable`](#definition__vonuvoli_r7rs__get-environment-variable);
 * [`get-environment-variables`](#definition__vonuvoli_r7rs__get-environment-variables);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_r5rs'>

### Category `r7rs:r5rs`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`interaction-environment`](#definition__vonuvoli_r7rs__interaction-environment);
 * [`scheme-report-environment`](#definition__vonuvoli_r7rs__scheme-report-environment);
 * [`null-environment`](#definition__vonuvoli_r7rs__null-environment);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_read'>

### Category `r7rs:read`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`read`](#definition__vonuvoli_r7rs__read);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_repl'>

### Category `r7rs:repl`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`interaction-environment`](#definition__vonuvoli_r7rs__interaction-environment);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_time'>

### Category `r7rs:time`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`current-second`](#definition__vonuvoli_r7rs__current-second);
 * [`current-jiffy`](#definition__vonuvoli_r7rs__current-jiffy);
 * [`jiffies-per-second`](#definition__vonuvoli_r7rs__jiffies-per-second);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs_write'>

### Category `r7rs:write`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).


#### Description

> **FIXME!**



#### Definitions

 * [`write`](#definition__vonuvoli_r7rs__write);
 * [`write-simple`](#definition__vonuvoli_r7rs__write-simple);
 * [`write-shared`](#definition__vonuvoli_r7rs__write-shared);
 * [`display`](#definition__vonuvoli_r7rs__display);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs-x'>

### Category `r7rs-x`

Belongs to the super-category: [`r7rs`](#category__vonuvoli_r7rs__r7rs).

Contains the following sub-categories:
 * [`r7rs-x:types`](#category__vonuvoli_r7rs__r7rs-x_types);


#### Description

> **FIXME!**



#### Types

 * [`any`](#value_kind__vonuvoli_r7rs__any);
 * [`null`](#value_kind__vonuvoli_r7rs__null);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);
 * [`number`](#value_kind__vonuvoli_r7rs__number);
 * [`symbol`](#value_kind__vonuvoli_r7rs__symbol);
 * [`character`](#value_kind__vonuvoli_r7rs__character);
 * [`string`](#value_kind__vonuvoli_r7rs__string);
 * [`bytevector`](#value_kind__vonuvoli_r7rs__bytevector);
 * [`vector`](#value_kind__vonuvoli_r7rs__vector);
 * [`pair`](#value_kind__vonuvoli_r7rs__pair);
 * [`port`](#value_kind__vonuvoli_r7rs__port);
 * [`eof-object`](#value_kind__vonuvoli_r7rs__eof-object);
 * [`procedure`](#value_kind__vonuvoli_r7rs__procedure);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs-x_types'>

### Category `r7rs-x:types`

Belongs to the super-category: [`r7rs-x`](#category__vonuvoli_r7rs__r7rs-x).

Contains the following sub-categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**



#### Types

 * [`any`](#value_kind__vonuvoli_r7rs__any);
 * [`null`](#value_kind__vonuvoli_r7rs__null);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);
 * [`number`](#value_kind__vonuvoli_r7rs__number);
 * [`symbol`](#value_kind__vonuvoli_r7rs__symbol);
 * [`character`](#value_kind__vonuvoli_r7rs__character);
 * [`string`](#value_kind__vonuvoli_r7rs__string);
 * [`bytevector`](#value_kind__vonuvoli_r7rs__bytevector);
 * [`vector`](#value_kind__vonuvoli_r7rs__vector);
 * [`pair`](#value_kind__vonuvoli_r7rs__pair);
 * [`port`](#value_kind__vonuvoli_r7rs__port);
 * [`eof-object`](#value_kind__vonuvoli_r7rs__eof-object);
 * [`procedure`](#value_kind__vonuvoli_r7rs__procedure);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__r7rs-x_types-disjoint'>

### Category `r7rs-x:types-disjoint`

Belongs to the super-category: [`r7rs-x:types`](#category__vonuvoli_r7rs__r7rs-x_types).


#### Description

> **FIXME!**



#### Types

 * [`null`](#value_kind__vonuvoli_r7rs__null);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);
 * [`number`](#value_kind__vonuvoli_r7rs__number);
 * [`symbol`](#value_kind__vonuvoli_r7rs__symbol);
 * [`character`](#value_kind__vonuvoli_r7rs__character);
 * [`string`](#value_kind__vonuvoli_r7rs__string);
 * [`bytevector`](#value_kind__vonuvoli_r7rs__bytevector);
 * [`vector`](#value_kind__vonuvoli_r7rs__vector);
 * [`pair`](#value_kind__vonuvoli_r7rs__pair);
 * [`port`](#value_kind__vonuvoli_r7rs__port);
 * [`eof-object`](#value_kind__vonuvoli_r7rs__eof-object);
 * [`procedure`](#value_kind__vonuvoli_r7rs__procedure);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs'>

### Category `vs`

Contains the following sub-categories:
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:associations`](#category__vonuvoli_r7rs__vs_associations);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);
 * [`vs:booleans`](#category__vonuvoli_r7rs__vs_booleans);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);
 * [`vs:globals`](#category__vonuvoli_r7rs__vs_globals);
 * [`vs:file-system`](#category__vonuvoli_r7rs__vs_file-system);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:compiler`](#category__vonuvoli_r7rs__vs_compiler);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);
 * [`vs:continuations`](#category__vonuvoli_r7rs__vs_continuations);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:lambda`](#category__vonuvoli_r7rs__vs_lambda);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:loops`](#category__vonuvoli_r7rs__vs_loops);
 * [`vs:modules`](#category__vonuvoli_r7rs__vs_modules);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters);
 * [`vs:ports`](#category__vonuvoli_r7rs__vs_ports);
 * [`vs:promises`](#category__vonuvoli_r7rs__vs_promises);
 * [`vs:quotation`](#category__vonuvoli_r7rs__vs_quotation);
 * [`vs:records`](#category__vonuvoli_r7rs__vs_records);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:symbols`](#category__vonuvoli_r7rs__vs_symbols);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:system`](#category__vonuvoli_r7rs__vs_system);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);
 * [`vs:unimplemented`](#category__vonuvoli_r7rs__vs_unimplemented);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);
 * [`vs:values`](#category__vonuvoli_r7rs__vs_values);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**



#### Definitions

 * [`define-syntax`](#definition__vonuvoli_r7rs__define-syntax);
 * [`let-syntax`](#definition__vonuvoli_r7rs__let-syntax);
 * [`letrec-syntax`](#definition__vonuvoli_r7rs__letrec-syntax);
 * [`syntax-rules`](#definition__vonuvoli_r7rs__syntax-rules);
 * [`syntax-error`](#definition__vonuvoli_r7rs__syntax-error);
 * [`_`](#definition__vonuvoli_r7rs___);
 * [`...`](#definition__vonuvoli_r7rs_____);
 * [`=>`](#definition__vonuvoli_r7rs____);
 * [`else`](#definition__vonuvoli_r7rs__else);
 * [`quote`](#definition__vonuvoli_r7rs__quote);
 * [`quasiquote`](#definition__vonuvoli_r7rs__quasiquote);
 * [`unquote`](#definition__vonuvoli_r7rs__unquote);
 * [`unquote-splicing`](#definition__vonuvoli_r7rs__unquote-splicing);
 * [`lambda`](#definition__vonuvoli_r7rs__lambda);
 * [`case-lambda`](#definition__vonuvoli_r7rs__case-lambda);
 * [`define`](#definition__vonuvoli_r7rs__define);
 * [`let`](#definition__vonuvoli_r7rs__let);
 * [`let*`](#definition__vonuvoli_r7rs__let_);
 * [`letrec`](#definition__vonuvoli_r7rs__letrec);
 * [`letrec*`](#definition__vonuvoli_r7rs__letrec_);
 * [`set!`](#definition__vonuvoli_r7rs__set!);
 * [`define-values`](#definition__vonuvoli_r7rs__define-values);
 * [`let-values`](#definition__vonuvoli_r7rs__let-values);
 * [`let*-values`](#definition__vonuvoli_r7rs__let_-values);
 * [`define-record-type`](#definition__vonuvoli_r7rs__define-record-type);
 * [`begin`](#definition__vonuvoli_r7rs__begin);
 * [`and`](#definition__vonuvoli_r7rs__and);
 * [`or`](#definition__vonuvoli_r7rs__or);
 * [`if`](#definition__vonuvoli_r7rs__if);
 * [`unless`](#definition__vonuvoli_r7rs__unless);
 * [`when`](#definition__vonuvoli_r7rs__when);
 * [`cond`](#definition__vonuvoli_r7rs__cond);
 * [`case`](#definition__vonuvoli_r7rs__case);
 * [`do`](#definition__vonuvoli_r7rs__do);
 * [`eq?`](#definition__vonuvoli_r7rs__eq_);
 * [`eqv?`](#definition__vonuvoli_r7rs__eqv_);
 * [`equal?`](#definition__vonuvoli_r7rs__equal_);
 * [`boolean?`](#definition__vonuvoli_r7rs__boolean_);
 * [`boolean=?`](#definition__vonuvoli_r7rs__boolean__);
 * [`symbol?`](#definition__vonuvoli_r7rs__symbol_);
 * [`symbol=?`](#definition__vonuvoli_r7rs__symbol__);
 * [`number?`](#definition__vonuvoli_r7rs__number_);
 * [`integer?`](#definition__vonuvoli_r7rs__integer_);
 * [`real?`](#definition__vonuvoli_r7rs__real_);
 * [`rational?`](#definition__vonuvoli_r7rs__rational_);
 * [`complex?`](#definition__vonuvoli_r7rs__complex_);
 * [`exact?`](#definition__vonuvoli_r7rs__exact_);
 * [`inexact?`](#definition__vonuvoli_r7rs__inexact_);
 * [`exact-integer?`](#definition__vonuvoli_r7rs__exact-integer_);
 * [`zero?`](#definition__vonuvoli_r7rs__zero_);
 * [`positive?`](#definition__vonuvoli_r7rs__positive_);
 * [`odd?`](#definition__vonuvoli_r7rs__odd_);
 * [`even?`](#definition__vonuvoli_r7rs__even_);
 * [`=`](#definition__vonuvoli_r7rs___);
 * [`<`](#definition__vonuvoli_r7rs___);
 * [`>`](#definition__vonuvoli_r7rs___);
 * [`<=`](#definition__vonuvoli_r7rs____);
 * [`>=`](#definition__vonuvoli_r7rs____);
 * [`+`](#definition__vonuvoli_r7rs___);
 * [`-`](#definition__vonuvoli_r7rs__-);
 * [`*`](#definition__vonuvoli_r7rs___);
 * [`/`](#definition__vonuvoli_r7rs___);
 * [`abs`](#definition__vonuvoli_r7rs__abs);
 * [`floor/`](#definition__vonuvoli_r7rs__floor_);
 * [`floor-quotient`](#definition__vonuvoli_r7rs__floor-quotient);
 * [`floor-remainder`](#definition__vonuvoli_r7rs__floor-remainder);
 * [`truncate/`](#definition__vonuvoli_r7rs__truncate_);
 * [`truncate-quotient`](#definition__vonuvoli_r7rs__truncate-quotient);
 * [`truncate-remainder`](#definition__vonuvoli_r7rs__truncate-remainder);
 * [`floor`](#definition__vonuvoli_r7rs__floor);
 * [`ceiling`](#definition__vonuvoli_r7rs__ceiling);
 * [`truncate`](#definition__vonuvoli_r7rs__truncate);
 * [`round`](#definition__vonuvoli_r7rs__round);
 * [`min`](#definition__vonuvoli_r7rs__min);
 * [`max`](#definition__vonuvoli_r7rs__max);
 * [`gcd`](#definition__vonuvoli_r7rs__gcd);
 * [`lcm`](#definition__vonuvoli_r7rs__lcm);
 * [`expt`](#definition__vonuvoli_r7rs__expt);
 * [`square`](#definition__vonuvoli_r7rs__square);
 * [`exact-integer-sqrt`](#definition__vonuvoli_r7rs__exact-integer-sqrt);
 * [`rationalize`](#definition__vonuvoli_r7rs__rationalize);
 * [`numerator`](#definition__vonuvoli_r7rs__numerator);
 * [`denominator`](#definition__vonuvoli_r7rs__denominator);
 * [`inexact`](#definition__vonuvoli_r7rs__inexact);
 * [`exact`](#definition__vonuvoli_r7rs__exact);
 * [`make-rectangular`](#definition__vonuvoli_r7rs__make-rectangular);
 * [`real-part`](#definition__vonuvoli_r7rs__real-part);
 * [`imag-part`](#definition__vonuvoli_r7rs__imag-part);
 * [`make-polar`](#definition__vonuvoli_r7rs__make-polar);
 * [`magnitude`](#definition__vonuvoli_r7rs__magnitude);
 * [`angle`](#definition__vonuvoli_r7rs__angle);
 * [`sqrt`](#definition__vonuvoli_r7rs__sqrt);
 * [`exp`](#definition__vonuvoli_r7rs__exp);
 * [`log`](#definition__vonuvoli_r7rs__log);
 * [`sin`](#definition__vonuvoli_r7rs__sin);
 * [`cos`](#definition__vonuvoli_r7rs__cos);
 * [`tan`](#definition__vonuvoli_r7rs__tan);
 * [`asin`](#definition__vonuvoli_r7rs__asin);
 * [`acos`](#definition__vonuvoli_r7rs__acos);
 * [`atan`](#definition__vonuvoli_r7rs__atan);
 * [`finite?`](#definition__vonuvoli_r7rs__finite_);
 * [`infinite?`](#definition__vonuvoli_r7rs__infinite_);
 * [`nan?`](#definition__vonuvoli_r7rs__nan_);
 * [`pair?`](#definition__vonuvoli_r7rs__pair_);
 * [`cons`](#definition__vonuvoli_r7rs__cons);
 * [`car`](#definition__vonuvoli_r7rs__car);
 * [`cdr`](#definition__vonuvoli_r7rs__cdr);
 * [`set-car!`](#definition__vonuvoli_r7rs__set-car!);
 * [`set-cdr!`](#definition__vonuvoli_r7rs__set-cdr!);
 * [`caar`](#definition__vonuvoli_r7rs__caar);
 * [`cadr`](#definition__vonuvoli_r7rs__cadr);
 * [`cdar`](#definition__vonuvoli_r7rs__cdar);
 * [`cddr`](#definition__vonuvoli_r7rs__cddr);
 * [`caaar`](#definition__vonuvoli_r7rs__caaar);
 * [`caadr`](#definition__vonuvoli_r7rs__caadr);
 * [`cadar`](#definition__vonuvoli_r7rs__cadar);
 * [`caddr`](#definition__vonuvoli_r7rs__caddr);
 * [`cdaar`](#definition__vonuvoli_r7rs__cdaar);
 * [`cdadr`](#definition__vonuvoli_r7rs__cdadr);
 * [`cddar`](#definition__vonuvoli_r7rs__cddar);
 * [`cdddr`](#definition__vonuvoli_r7rs__cdddr);
 * [`caaaar`](#definition__vonuvoli_r7rs__caaaar);
 * [`caaadr`](#definition__vonuvoli_r7rs__caaadr);
 * [`caadar`](#definition__vonuvoli_r7rs__caadar);
 * [`caaddr`](#definition__vonuvoli_r7rs__caaddr);
 * [`cadaar`](#definition__vonuvoli_r7rs__cadaar);
 * [`cadadr`](#definition__vonuvoli_r7rs__cadadr);
 * [`caddar`](#definition__vonuvoli_r7rs__caddar);
 * [`cadddr`](#definition__vonuvoli_r7rs__cadddr);
 * [`cdaaar`](#definition__vonuvoli_r7rs__cdaaar);
 * [`cdaadr`](#definition__vonuvoli_r7rs__cdaadr);
 * [`cdadar`](#definition__vonuvoli_r7rs__cdadar);
 * [`cdaddr`](#definition__vonuvoli_r7rs__cdaddr);
 * [`cddaar`](#definition__vonuvoli_r7rs__cddaar);
 * [`cddadr`](#definition__vonuvoli_r7rs__cddadr);
 * [`cdddar`](#definition__vonuvoli_r7rs__cdddar);
 * [`cddddr`](#definition__vonuvoli_r7rs__cddddr);
 * [`null?`](#definition__vonuvoli_r7rs__null_);
 * [`list?`](#definition__vonuvoli_r7rs__list_);
 * [`list`](#definition__vonuvoli_r7rs__list);
 * [`make-list`](#definition__vonuvoli_r7rs__make-list);
 * [`length`](#definition__vonuvoli_r7rs__length);
 * [`append`](#definition__vonuvoli_r7rs__append);
 * [`list-copy`](#definition__vonuvoli_r7rs__list-copy);
 * [`reverse`](#definition__vonuvoli_r7rs__reverse);
 * [`list-ref`](#definition__vonuvoli_r7rs__list-ref);
 * [`list-tail`](#definition__vonuvoli_r7rs__list-tail);
 * [`list-set!`](#definition__vonuvoli_r7rs__list-set!);
 * [`map`](#definition__vonuvoli_r7rs__map);
 * [`for-each`](#definition__vonuvoli_r7rs__for-each);
 * [`member`](#definition__vonuvoli_r7rs__member);
 * [`memq`](#definition__vonuvoli_r7rs__memq);
 * [`memv`](#definition__vonuvoli_r7rs__memv);
 * [`assoc`](#definition__vonuvoli_r7rs__assoc);
 * [`assqc`](#definition__vonuvoli_r7rs__assqc);
 * [`assvc`](#definition__vonuvoli_r7rs__assvc);
 * [`vector?`](#definition__vonuvoli_r7rs__vector_);
 * [`vector`](#definition__vonuvoli_r7rs__vector);
 * [`make-vector`](#definition__vonuvoli_r7rs__make-vector);
 * [`vector-length`](#definition__vonuvoli_r7rs__vector-length);
 * [`vector-append`](#definition__vonuvoli_r7rs__vector-append);
 * [`vector-copy`](#definition__vonuvoli_r7rs__vector-copy);
 * [`vector-copy!`](#definition__vonuvoli_r7rs__vector-copy!);
 * [`vector-fill!`](#definition__vonuvoli_r7rs__vector-fill!);
 * [`vector-ref`](#definition__vonuvoli_r7rs__vector-ref);
 * [`vector-set!`](#definition__vonuvoli_r7rs__vector-set!);
 * [`vector->list`](#definition__vonuvoli_r7rs__vector-_list);
 * [`list->vector`](#definition__vonuvoli_r7rs__list-_vector);
 * [`vector-map`](#definition__vonuvoli_r7rs__vector-map);
 * [`vector-for-each`](#definition__vonuvoli_r7rs__vector-for-each);
 * [`string?`](#definition__vonuvoli_r7rs__string_);
 * [`string`](#definition__vonuvoli_r7rs__string);
 * [`make-string`](#definition__vonuvoli_r7rs__make-string);
 * [`string-length`](#definition__vonuvoli_r7rs__string-length);
 * [`string-append`](#definition__vonuvoli_r7rs__string-append);
 * [`string-copy`](#definition__vonuvoli_r7rs__string-copy);
 * [`string-copy!`](#definition__vonuvoli_r7rs__string-copy!);
 * [`string-fill!`](#definition__vonuvoli_r7rs__string-fill!);
 * [`substring`](#definition__vonuvoli_r7rs__substring);
 * [`string-ref`](#definition__vonuvoli_r7rs__string-ref);
 * [`string-set!`](#definition__vonuvoli_r7rs__string-set!);
 * [`string=?`](#definition__vonuvoli_r7rs__string__);
 * [`string<?`](#definition__vonuvoli_r7rs__string__);
 * [`string>?`](#definition__vonuvoli_r7rs__string__);
 * [`string<=?`](#definition__vonuvoli_r7rs__string___);
 * [`string>=?`](#definition__vonuvoli_r7rs__string___);
 * [`string-ci=?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci>?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`string-ci>=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`number->string`](#definition__vonuvoli_r7rs__number-_string);
 * [`string->number`](#definition__vonuvoli_r7rs__string-_number);
 * [`symbol->string`](#definition__vonuvoli_r7rs__symbol-_string);
 * [`string->symbol`](#definition__vonuvoli_r7rs__string-_symbol);
 * [`list->string`](#definition__vonuvoli_r7rs__list-_string);
 * [`string->list`](#definition__vonuvoli_r7rs__string-_list);
 * [`vector->string`](#definition__vonuvoli_r7rs__vector-_string);
 * [`string->vector`](#definition__vonuvoli_r7rs__string-_vector);
 * [`string-map`](#definition__vonuvoli_r7rs__string-map);
 * [`string-for-each`](#definition__vonuvoli_r7rs__string-for-each);
 * [`string-upcase`](#definition__vonuvoli_r7rs__string-upcase);
 * [`string-downcase`](#definition__vonuvoli_r7rs__string-downcase);
 * [`string-foldcase`](#definition__vonuvoli_r7rs__string-foldcase);
 * [`bytevector?`](#definition__vonuvoli_r7rs__bytevector_);
 * [`bytevector`](#definition__vonuvoli_r7rs__bytevector);
 * [`make-bytevector`](#definition__vonuvoli_r7rs__make-bytevector);
 * [`bytevector-length`](#definition__vonuvoli_r7rs__bytevector-length);
 * [`bytevector-append`](#definition__vonuvoli_r7rs__bytevector-append);
 * [`bytevector-copy`](#definition__vonuvoli_r7rs__bytevector-copy);
 * [`bytevector-copy!`](#definition__vonuvoli_r7rs__bytevector-copy!);
 * [`bytevector-u8-ref`](#definition__vonuvoli_r7rs__bytevector-u8-ref);
 * [`bytevector-u8-set!`](#definition__vonuvoli_r7rs__bytevector-u8-set!);
 * [`utf8->string`](#definition__vonuvoli_r7rs__utf8-_string);
 * [`string->utf8`](#definition__vonuvoli_r7rs__string-_utf8);
 * [`port?`](#definition__vonuvoli_r7rs__port_);
 * [`binary-port?`](#definition__vonuvoli_r7rs__binary-port_);
 * [`textual-port?`](#definition__vonuvoli_r7rs__textual-port_);
 * [`input-port?`](#definition__vonuvoli_r7rs__input-port_);
 * [`input-port-open?`](#definition__vonuvoli_r7rs__input-port-open_);
 * [`output-port?`](#definition__vonuvoli_r7rs__output-port_);
 * [`output-port-open?`](#definition__vonuvoli_r7rs__output-port-open_);
 * [`open-input-bytevector`](#definition__vonuvoli_r7rs__open-input-bytevector);
 * [`open-output-bytevector`](#definition__vonuvoli_r7rs__open-output-bytevector);
 * [`get-output-bytevector`](#definition__vonuvoli_r7rs__get-output-bytevector);
 * [`open-input-string`](#definition__vonuvoli_r7rs__open-input-string);
 * [`open-output-string`](#definition__vonuvoli_r7rs__open-output-string);
 * [`get-output-string`](#definition__vonuvoli_r7rs__get-output-string);
 * [`close-port`](#definition__vonuvoli_r7rs__close-port);
 * [`close-input-port`](#definition__vonuvoli_r7rs__close-input-port);
 * [`close-output-port`](#definition__vonuvoli_r7rs__close-output-port);
 * [`u8-ready?`](#definition__vonuvoli_r7rs__u8-ready_);
 * [`peek-u8`](#definition__vonuvoli_r7rs__peek-u8);
 * [`read-u8`](#definition__vonuvoli_r7rs__read-u8);
 * [`write-u8`](#definition__vonuvoli_r7rs__write-u8);
 * [`read-bytevector`](#definition__vonuvoli_r7rs__read-bytevector);
 * [`read-bytevector!`](#definition__vonuvoli_r7rs__read-bytevector!);
 * [`write-bytevector`](#definition__vonuvoli_r7rs__write-bytevector);
 * [`char-ready?`](#definition__vonuvoli_r7rs__char-ready_);
 * [`peek-char`](#definition__vonuvoli_r7rs__peek-char);
 * [`read-char`](#definition__vonuvoli_r7rs__read-char);
 * [`write-char`](#definition__vonuvoli_r7rs__write-char);
 * [`read-string`](#definition__vonuvoli_r7rs__read-string);
 * [`read-line`](#definition__vonuvoli_r7rs__read-line);
 * [`newline`](#definition__vonuvoli_r7rs__newline);
 * [`flush-output-port`](#definition__vonuvoli_r7rs__flush-output-port);
 * [`read`](#definition__vonuvoli_r7rs__read);
 * [`write`](#definition__vonuvoli_r7rs__write);
 * [`write-simple`](#definition__vonuvoli_r7rs__write-simple);
 * [`write-shared`](#definition__vonuvoli_r7rs__write-shared);
 * [`display`](#definition__vonuvoli_r7rs__display);
 * [`open-input-file`](#definition__vonuvoli_r7rs__open-input-file);
 * [`open-binary-input-file`](#definition__vonuvoli_r7rs__open-binary-input-file);
 * [`open-output-file`](#definition__vonuvoli_r7rs__open-output-file);
 * [`open-binary-output-file`](#definition__vonuvoli_r7rs__open-binary-output-file);
 * [`call-with-port`](#definition__vonuvoli_r7rs__call-with-port);
 * [`call-with-input-file`](#definition__vonuvoli_r7rs__call-with-input-file);
 * [`call-with-output-file`](#definition__vonuvoli_r7rs__call-with-output-file);
 * [`eof-object`](#definition__vonuvoli_r7rs__eof-object);
 * [`eof-object?`](#definition__vonuvoli_r7rs__eof-object_);
 * [`file-exists?`](#definition__vonuvoli_r7rs__file-exists_);
 * [`delete-file`](#definition__vonuvoli_r7rs__delete-file);
 * [`char?`](#definition__vonuvoli_r7rs__char_);
 * [`char=?`](#definition__vonuvoli_r7rs__char__);
 * [`char<?`](#definition__vonuvoli_r7rs__char__);
 * [`char>?`](#definition__vonuvoli_r7rs__char__);
 * [`char<=?`](#definition__vonuvoli_r7rs__char___);
 * [`char>=?`](#definition__vonuvoli_r7rs__char___);
 * [`char-ci=?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci>?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`char-ci>=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`char->integer`](#definition__vonuvoli_r7rs__char-_integer);
 * [`integer->char`](#definition__vonuvoli_r7rs__integer-_char);
 * [`digit-value`](#definition__vonuvoli_r7rs__digit-value);
 * [`char-alphabetic?`](#definition__vonuvoli_r7rs__char-alphabetic_);
 * [`char-upper-case?`](#definition__vonuvoli_r7rs__char-upper-case_);
 * [`char-lower-case?`](#definition__vonuvoli_r7rs__char-lower-case_);
 * [`char-numeric?`](#definition__vonuvoli_r7rs__char-numeric_);
 * [`char-whitespace?`](#definition__vonuvoli_r7rs__char-whitespace_);
 * [`char-upcase`](#definition__vonuvoli_r7rs__char-upcase);
 * [`char-downcase`](#definition__vonuvoli_r7rs__char-downcase);
 * [`char-foldcase`](#definition__vonuvoli_r7rs__char-foldcase);
 * [`procedure?`](#definition__vonuvoli_r7rs__procedure_);
 * [`apply`](#definition__vonuvoli_r7rs__apply);
 * [`values`](#definition__vonuvoli_r7rs__values);
 * [`call-with-values`](#definition__vonuvoli_r7rs__call-with-values);
 * [`error-object?`](#definition__vonuvoli_r7rs__error-object_);
 * [`read-error?`](#definition__vonuvoli_r7rs__read-error_);
 * [`file-error?`](#definition__vonuvoli_r7rs__file-error_);
 * [`error`](#definition__vonuvoli_r7rs__error);
 * [`error-object-message`](#definition__vonuvoli_r7rs__error-object-message);
 * [`error-object-irritants`](#definition__vonuvoli_r7rs__error-object-irritants);
 * [`guard`](#definition__vonuvoli_r7rs__guard);
 * [`with-exception-handler`](#definition__vonuvoli_r7rs__with-exception-handler);
 * [`raise`](#definition__vonuvoli_r7rs__raise);
 * [`raise-continuable`](#definition__vonuvoli_r7rs__raise-continuable);
 * [`parameterize`](#definition__vonuvoli_r7rs__parameterize);
 * [`make-parameter`](#definition__vonuvoli_r7rs__make-parameter);
 * [`current-input-port`](#definition__vonuvoli_r7rs__current-input-port);
 * [`current-output-port`](#definition__vonuvoli_r7rs__current-output-port);
 * [`current-error-port`](#definition__vonuvoli_r7rs__current-error-port);
 * [`with-input-from-file`](#definition__vonuvoli_r7rs__with-input-from-file);
 * [`with-output-from-file`](#definition__vonuvoli_r7rs__with-output-from-file);
 * [`delay`](#definition__vonuvoli_r7rs__delay);
 * [`delay-force`](#definition__vonuvoli_r7rs__delay-force);
 * [`promise?`](#definition__vonuvoli_r7rs__promise_);
 * [`make-promise`](#definition__vonuvoli_r7rs__make-promise);
 * [`force`](#definition__vonuvoli_r7rs__force);
 * [`eval`](#definition__vonuvoli_r7rs__eval);
 * [`environment`](#definition__vonuvoli_r7rs__environment);
 * [`interaction-environment`](#definition__vonuvoli_r7rs__interaction-environment);
 * [`scheme-report-environment`](#definition__vonuvoli_r7rs__scheme-report-environment);
 * [`null-environment`](#definition__vonuvoli_r7rs__null-environment);
 * [`call-with-current-continuation`](#definition__vonuvoli_r7rs__call-with-current-continuation);
 * [`dynamic-wind`](#definition__vonuvoli_r7rs__dynamic-wind);
 * [`cond-expand`](#definition__vonuvoli_r7rs__cond-expand);
 * [`features`](#definition__vonuvoli_r7rs__features);
 * [`include`](#definition__vonuvoli_r7rs__include);
 * [`include-ci`](#definition__vonuvoli_r7rs__include-ci);
 * [`import`](#definition__vonuvoli_r7rs__import);
 * [`load`](#definition__vonuvoli_r7rs__load);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_arithmetic'>

### Category `vs:arithmetic`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`number?`](#definition__vonuvoli_r7rs__number_);
 * [`integer?`](#definition__vonuvoli_r7rs__integer_);
 * [`real?`](#definition__vonuvoli_r7rs__real_);
 * [`rational?`](#definition__vonuvoli_r7rs__rational_);
 * [`complex?`](#definition__vonuvoli_r7rs__complex_);
 * [`exact?`](#definition__vonuvoli_r7rs__exact_);
 * [`inexact?`](#definition__vonuvoli_r7rs__inexact_);
 * [`exact-integer?`](#definition__vonuvoli_r7rs__exact-integer_);
 * [`zero?`](#definition__vonuvoli_r7rs__zero_);
 * [`positive?`](#definition__vonuvoli_r7rs__positive_);
 * [`odd?`](#definition__vonuvoli_r7rs__odd_);
 * [`even?`](#definition__vonuvoli_r7rs__even_);
 * [`=`](#definition__vonuvoli_r7rs___);
 * [`<`](#definition__vonuvoli_r7rs___);
 * [`>`](#definition__vonuvoli_r7rs___);
 * [`<=`](#definition__vonuvoli_r7rs____);
 * [`>=`](#definition__vonuvoli_r7rs____);
 * [`+`](#definition__vonuvoli_r7rs___);
 * [`-`](#definition__vonuvoli_r7rs__-);
 * [`*`](#definition__vonuvoli_r7rs___);
 * [`/`](#definition__vonuvoli_r7rs___);
 * [`abs`](#definition__vonuvoli_r7rs__abs);
 * [`floor/`](#definition__vonuvoli_r7rs__floor_);
 * [`floor-quotient`](#definition__vonuvoli_r7rs__floor-quotient);
 * [`floor-remainder`](#definition__vonuvoli_r7rs__floor-remainder);
 * [`truncate/`](#definition__vonuvoli_r7rs__truncate_);
 * [`truncate-quotient`](#definition__vonuvoli_r7rs__truncate-quotient);
 * [`truncate-remainder`](#definition__vonuvoli_r7rs__truncate-remainder);
 * [`floor`](#definition__vonuvoli_r7rs__floor);
 * [`ceiling`](#definition__vonuvoli_r7rs__ceiling);
 * [`truncate`](#definition__vonuvoli_r7rs__truncate);
 * [`round`](#definition__vonuvoli_r7rs__round);
 * [`min`](#definition__vonuvoli_r7rs__min);
 * [`max`](#definition__vonuvoli_r7rs__max);
 * [`gcd`](#definition__vonuvoli_r7rs__gcd);
 * [`lcm`](#definition__vonuvoli_r7rs__lcm);
 * [`expt`](#definition__vonuvoli_r7rs__expt);
 * [`square`](#definition__vonuvoli_r7rs__square);
 * [`exact-integer-sqrt`](#definition__vonuvoli_r7rs__exact-integer-sqrt);
 * [`rationalize`](#definition__vonuvoli_r7rs__rationalize);
 * [`numerator`](#definition__vonuvoli_r7rs__numerator);
 * [`denominator`](#definition__vonuvoli_r7rs__denominator);
 * [`inexact`](#definition__vonuvoli_r7rs__inexact);
 * [`exact`](#definition__vonuvoli_r7rs__exact);
 * [`make-rectangular`](#definition__vonuvoli_r7rs__make-rectangular);
 * [`real-part`](#definition__vonuvoli_r7rs__real-part);
 * [`imag-part`](#definition__vonuvoli_r7rs__imag-part);
 * [`make-polar`](#definition__vonuvoli_r7rs__make-polar);
 * [`magnitude`](#definition__vonuvoli_r7rs__magnitude);
 * [`angle`](#definition__vonuvoli_r7rs__angle);
 * [`sqrt`](#definition__vonuvoli_r7rs__sqrt);
 * [`exp`](#definition__vonuvoli_r7rs__exp);
 * [`log`](#definition__vonuvoli_r7rs__log);
 * [`sin`](#definition__vonuvoli_r7rs__sin);
 * [`cos`](#definition__vonuvoli_r7rs__cos);
 * [`tan`](#definition__vonuvoli_r7rs__tan);
 * [`asin`](#definition__vonuvoli_r7rs__asin);
 * [`acos`](#definition__vonuvoli_r7rs__acos);
 * [`atan`](#definition__vonuvoli_r7rs__atan);
 * [`finite?`](#definition__vonuvoli_r7rs__finite_);
 * [`infinite?`](#definition__vonuvoli_r7rs__infinite_);
 * [`nan?`](#definition__vonuvoli_r7rs__nan_);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_associations'>

### Category `vs:associations`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`assoc`](#definition__vonuvoli_r7rs__assoc);
 * [`assqc`](#definition__vonuvoli_r7rs__assqc);
 * [`assvc`](#definition__vonuvoli_r7rs__assvc);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_bytes'>

### Category `vs:bytes`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`bytevector?`](#definition__vonuvoli_r7rs__bytevector_);
 * [`bytevector`](#definition__vonuvoli_r7rs__bytevector);
 * [`make-bytevector`](#definition__vonuvoli_r7rs__make-bytevector);
 * [`bytevector-length`](#definition__vonuvoli_r7rs__bytevector-length);
 * [`bytevector-append`](#definition__vonuvoli_r7rs__bytevector-append);
 * [`bytevector-copy`](#definition__vonuvoli_r7rs__bytevector-copy);
 * [`bytevector-copy!`](#definition__vonuvoli_r7rs__bytevector-copy!);
 * [`bytevector-u8-ref`](#definition__vonuvoli_r7rs__bytevector-u8-ref);
 * [`bytevector-u8-set!`](#definition__vonuvoli_r7rs__bytevector-u8-set!);
 * [`utf8->string`](#definition__vonuvoli_r7rs__utf8-_string);
 * [`string->utf8`](#definition__vonuvoli_r7rs__string-_utf8);
 * [`open-input-bytevector`](#definition__vonuvoli_r7rs__open-input-bytevector);
 * [`open-output-bytevector`](#definition__vonuvoli_r7rs__open-output-bytevector);
 * [`get-output-bytevector`](#definition__vonuvoli_r7rs__get-output-bytevector);
 * [`u8-ready?`](#definition__vonuvoli_r7rs__u8-ready_);
 * [`peek-u8`](#definition__vonuvoli_r7rs__peek-u8);
 * [`read-u8`](#definition__vonuvoli_r7rs__read-u8);
 * [`write-u8`](#definition__vonuvoli_r7rs__write-u8);
 * [`read-bytevector`](#definition__vonuvoli_r7rs__read-bytevector);
 * [`read-bytevector!`](#definition__vonuvoli_r7rs__read-bytevector!);
 * [`write-bytevector`](#definition__vonuvoli_r7rs__write-bytevector);
 * [`newline`](#definition__vonuvoli_r7rs__newline);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_booleans'>

### Category `vs:booleans`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`boolean?`](#definition__vonuvoli_r7rs__boolean_);
 * [`boolean=?`](#definition__vonuvoli_r7rs__boolean__);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_conversions'>

### Category `vs:conversions`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`map`](#definition__vonuvoli_r7rs__map);
 * [`vector->list`](#definition__vonuvoli_r7rs__vector-_list);
 * [`list->vector`](#definition__vonuvoli_r7rs__list-_vector);
 * [`vector-map`](#definition__vonuvoli_r7rs__vector-map);
 * [`number->string`](#definition__vonuvoli_r7rs__number-_string);
 * [`string->number`](#definition__vonuvoli_r7rs__string-_number);
 * [`symbol->string`](#definition__vonuvoli_r7rs__symbol-_string);
 * [`string->symbol`](#definition__vonuvoli_r7rs__string-_symbol);
 * [`list->string`](#definition__vonuvoli_r7rs__list-_string);
 * [`string->list`](#definition__vonuvoli_r7rs__string-_list);
 * [`vector->string`](#definition__vonuvoli_r7rs__vector-_string);
 * [`string->vector`](#definition__vonuvoli_r7rs__string-_vector);
 * [`string-map`](#definition__vonuvoli_r7rs__string-map);
 * [`string-upcase`](#definition__vonuvoli_r7rs__string-upcase);
 * [`string-downcase`](#definition__vonuvoli_r7rs__string-downcase);
 * [`string-foldcase`](#definition__vonuvoli_r7rs__string-foldcase);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_globals'>

### Category `vs:globals`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`eof-object`](#definition__vonuvoli_r7rs__eof-object);
 * [`eof-object?`](#definition__vonuvoli_r7rs__eof-object_);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_file-system'>

### Category `vs:file-system`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`file-exists?`](#definition__vonuvoli_r7rs__file-exists_);
 * [`delete-file`](#definition__vonuvoli_r7rs__delete-file);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_characters'>

### Category `vs:characters`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`char-ready?`](#definition__vonuvoli_r7rs__char-ready_);
 * [`peek-char`](#definition__vonuvoli_r7rs__peek-char);
 * [`read-char`](#definition__vonuvoli_r7rs__read-char);
 * [`write-char`](#definition__vonuvoli_r7rs__write-char);
 * [`char?`](#definition__vonuvoli_r7rs__char_);
 * [`char=?`](#definition__vonuvoli_r7rs__char__);
 * [`char<?`](#definition__vonuvoli_r7rs__char__);
 * [`char>?`](#definition__vonuvoli_r7rs__char__);
 * [`char<=?`](#definition__vonuvoli_r7rs__char___);
 * [`char>=?`](#definition__vonuvoli_r7rs__char___);
 * [`char-ci=?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci>?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`char-ci>=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`char->integer`](#definition__vonuvoli_r7rs__char-_integer);
 * [`integer->char`](#definition__vonuvoli_r7rs__integer-_char);
 * [`digit-value`](#definition__vonuvoli_r7rs__digit-value);
 * [`char-alphabetic?`](#definition__vonuvoli_r7rs__char-alphabetic_);
 * [`char-upper-case?`](#definition__vonuvoli_r7rs__char-upper-case_);
 * [`char-lower-case?`](#definition__vonuvoli_r7rs__char-lower-case_);
 * [`char-numeric?`](#definition__vonuvoli_r7rs__char-numeric_);
 * [`char-whitespace?`](#definition__vonuvoli_r7rs__char-whitespace_);
 * [`char-upcase`](#definition__vonuvoli_r7rs__char-upcase);
 * [`char-downcase`](#definition__vonuvoli_r7rs__char-downcase);
 * [`char-foldcase`](#definition__vonuvoli_r7rs__char-foldcase);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_comparisons'>

### Category `vs:comparisons`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`boolean=?`](#definition__vonuvoli_r7rs__boolean__);
 * [`symbol=?`](#definition__vonuvoli_r7rs__symbol__);
 * [`=`](#definition__vonuvoli_r7rs___);
 * [`<`](#definition__vonuvoli_r7rs___);
 * [`>`](#definition__vonuvoli_r7rs___);
 * [`<=`](#definition__vonuvoli_r7rs____);
 * [`>=`](#definition__vonuvoli_r7rs____);
 * [`string=?`](#definition__vonuvoli_r7rs__string__);
 * [`string<?`](#definition__vonuvoli_r7rs__string__);
 * [`string>?`](#definition__vonuvoli_r7rs__string__);
 * [`string<=?`](#definition__vonuvoli_r7rs__string___);
 * [`string>=?`](#definition__vonuvoli_r7rs__string___);
 * [`string-ci=?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci>?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`string-ci>=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`char=?`](#definition__vonuvoli_r7rs__char__);
 * [`char<?`](#definition__vonuvoli_r7rs__char__);
 * [`char>?`](#definition__vonuvoli_r7rs__char__);
 * [`char<=?`](#definition__vonuvoli_r7rs__char___);
 * [`char>=?`](#definition__vonuvoli_r7rs__char___);
 * [`char-ci=?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci>?`](#definition__vonuvoli_r7rs__char-ci__);
 * [`char-ci<=?`](#definition__vonuvoli_r7rs__char-ci___);
 * [`char-ci>=?`](#definition__vonuvoli_r7rs__char-ci___);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_compiler'>

### Category `vs:compiler`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`cond-expand`](#definition__vonuvoli_r7rs__cond-expand);
 * [`features`](#definition__vonuvoli_r7rs__features);
 * [`include`](#definition__vonuvoli_r7rs__include);
 * [`include-ci`](#definition__vonuvoli_r7rs__include-ci);
 * [`import`](#definition__vonuvoli_r7rs__import);
 * [`load`](#definition__vonuvoli_r7rs__load);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_contexts'>

### Category `vs:contexts`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`define`](#definition__vonuvoli_r7rs__define);
 * [`let`](#definition__vonuvoli_r7rs__let);
 * [`let*`](#definition__vonuvoli_r7rs__let_);
 * [`letrec`](#definition__vonuvoli_r7rs__letrec);
 * [`letrec*`](#definition__vonuvoli_r7rs__letrec_);
 * [`set!`](#definition__vonuvoli_r7rs__set!);
 * [`define-values`](#definition__vonuvoli_r7rs__define-values);
 * [`let-values`](#definition__vonuvoli_r7rs__let-values);
 * [`let*-values`](#definition__vonuvoli_r7rs__let_-values);
 * [`define-record-type`](#definition__vonuvoli_r7rs__define-record-type);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_continuations'>

### Category `vs:continuations`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`call-with-current-continuation`](#definition__vonuvoli_r7rs__call-with-current-continuation);
 * [`dynamic-wind`](#definition__vonuvoli_r7rs__dynamic-wind);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_control'>

### Category `vs:control`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`begin`](#definition__vonuvoli_r7rs__begin);
 * [`and`](#definition__vonuvoli_r7rs__and);
 * [`or`](#definition__vonuvoli_r7rs__or);
 * [`if`](#definition__vonuvoli_r7rs__if);
 * [`unless`](#definition__vonuvoli_r7rs__unless);
 * [`when`](#definition__vonuvoli_r7rs__when);
 * [`cond`](#definition__vonuvoli_r7rs__cond);
 * [`case`](#definition__vonuvoli_r7rs__case);
 * [`do`](#definition__vonuvoli_r7rs__do);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_equivalence'>

### Category `vs:equivalence`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Description

> A **predicate** is a procedure that always returns a boolean
> value (`#t` or `#f`).  An **equivalence predicate** is
> the computational analogue of a mathematical equivalence relation; it is
> symmetric, reflexive, and transitive.
> 
> Of the equivalence predicates
> described in this section, `eq?` is the finest or most
> discriminating, `equal?` is the coarsest, and `eqv?` is
> slightly less discriminating than `eq?`.



#### Definitions

 * [`eq?`](#definition__vonuvoli_r7rs__eq_);
 * [`eqv?`](#definition__vonuvoli_r7rs__eqv_);
 * [`equal?`](#definition__vonuvoli_r7rs__equal_);
 * [`boolean=?`](#definition__vonuvoli_r7rs__boolean__);
 * [`symbol=?`](#definition__vonuvoli_r7rs__symbol__);
 * [`string=?`](#definition__vonuvoli_r7rs__string__);
 * [`string<?`](#definition__vonuvoli_r7rs__string__);
 * [`string>?`](#definition__vonuvoli_r7rs__string__);
 * [`string<=?`](#definition__vonuvoli_r7rs__string___);
 * [`string>=?`](#definition__vonuvoli_r7rs__string___);
 * [`string-ci=?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`char=?`](#definition__vonuvoli_r7rs__char__);
 * [`char-ci=?`](#definition__vonuvoli_r7rs__char-ci__);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_errors'>

### Category `vs:errors`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`error-object?`](#definition__vonuvoli_r7rs__error-object_);
 * [`read-error?`](#definition__vonuvoli_r7rs__read-error_);
 * [`file-error?`](#definition__vonuvoli_r7rs__file-error_);
 * [`error`](#definition__vonuvoli_r7rs__error);
 * [`error-object-message`](#definition__vonuvoli_r7rs__error-object-message);
 * [`error-object-irritants`](#definition__vonuvoli_r7rs__error-object-irritants);
 * [`guard`](#definition__vonuvoli_r7rs__guard);
 * [`with-exception-handler`](#definition__vonuvoli_r7rs__with-exception-handler);
 * [`raise`](#definition__vonuvoli_r7rs__raise);
 * [`raise-continuable`](#definition__vonuvoli_r7rs__raise-continuable);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_evaluator'>

### Category `vs:evaluator`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`guard`](#definition__vonuvoli_r7rs__guard);
 * [`with-exception-handler`](#definition__vonuvoli_r7rs__with-exception-handler);
 * [`raise`](#definition__vonuvoli_r7rs__raise);
 * [`raise-continuable`](#definition__vonuvoli_r7rs__raise-continuable);
 * [`delay`](#definition__vonuvoli_r7rs__delay);
 * [`delay-force`](#definition__vonuvoli_r7rs__delay-force);
 * [`promise?`](#definition__vonuvoli_r7rs__promise_);
 * [`make-promise`](#definition__vonuvoli_r7rs__make-promise);
 * [`force`](#definition__vonuvoli_r7rs__force);
 * [`eval`](#definition__vonuvoli_r7rs__eval);
 * [`environment`](#definition__vonuvoli_r7rs__environment);
 * [`interaction-environment`](#definition__vonuvoli_r7rs__interaction-environment);
 * [`scheme-report-environment`](#definition__vonuvoli_r7rs__scheme-report-environment);
 * [`null-environment`](#definition__vonuvoli_r7rs__null-environment);
 * [`features`](#definition__vonuvoli_r7rs__features);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_functions'>

### Category `vs:functions`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`map`](#definition__vonuvoli_r7rs__map);
 * [`for-each`](#definition__vonuvoli_r7rs__for-each);
 * [`vector-map`](#definition__vonuvoli_r7rs__vector-map);
 * [`vector-for-each`](#definition__vonuvoli_r7rs__vector-for-each);
 * [`string-map`](#definition__vonuvoli_r7rs__string-map);
 * [`string-for-each`](#definition__vonuvoli_r7rs__string-for-each);
 * [`call-with-port`](#definition__vonuvoli_r7rs__call-with-port);
 * [`call-with-input-file`](#definition__vonuvoli_r7rs__call-with-input-file);
 * [`call-with-output-file`](#definition__vonuvoli_r7rs__call-with-output-file);
 * [`procedure?`](#definition__vonuvoli_r7rs__procedure_);
 * [`apply`](#definition__vonuvoli_r7rs__apply);
 * [`values`](#definition__vonuvoli_r7rs__values);
 * [`call-with-values`](#definition__vonuvoli_r7rs__call-with-values);
 * [`with-input-from-file`](#definition__vonuvoli_r7rs__with-input-from-file);
 * [`with-output-from-file`](#definition__vonuvoli_r7rs__with-output-from-file);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_lambda'>

### Category `vs:lambda`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`lambda`](#definition__vonuvoli_r7rs__lambda);
 * [`case-lambda`](#definition__vonuvoli_r7rs__case-lambda);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_lists'>

### Category `vs:lists`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`pair?`](#definition__vonuvoli_r7rs__pair_);
 * [`cons`](#definition__vonuvoli_r7rs__cons);
 * [`car`](#definition__vonuvoli_r7rs__car);
 * [`cdr`](#definition__vonuvoli_r7rs__cdr);
 * [`set-car!`](#definition__vonuvoli_r7rs__set-car!);
 * [`set-cdr!`](#definition__vonuvoli_r7rs__set-cdr!);
 * [`caar`](#definition__vonuvoli_r7rs__caar);
 * [`cadr`](#definition__vonuvoli_r7rs__cadr);
 * [`cdar`](#definition__vonuvoli_r7rs__cdar);
 * [`cddr`](#definition__vonuvoli_r7rs__cddr);
 * [`caaar`](#definition__vonuvoli_r7rs__caaar);
 * [`caadr`](#definition__vonuvoli_r7rs__caadr);
 * [`cadar`](#definition__vonuvoli_r7rs__cadar);
 * [`caddr`](#definition__vonuvoli_r7rs__caddr);
 * [`cdaar`](#definition__vonuvoli_r7rs__cdaar);
 * [`cdadr`](#definition__vonuvoli_r7rs__cdadr);
 * [`cddar`](#definition__vonuvoli_r7rs__cddar);
 * [`cdddr`](#definition__vonuvoli_r7rs__cdddr);
 * [`caaaar`](#definition__vonuvoli_r7rs__caaaar);
 * [`caaadr`](#definition__vonuvoli_r7rs__caaadr);
 * [`caadar`](#definition__vonuvoli_r7rs__caadar);
 * [`caaddr`](#definition__vonuvoli_r7rs__caaddr);
 * [`cadaar`](#definition__vonuvoli_r7rs__cadaar);
 * [`cadadr`](#definition__vonuvoli_r7rs__cadadr);
 * [`caddar`](#definition__vonuvoli_r7rs__caddar);
 * [`cadddr`](#definition__vonuvoli_r7rs__cadddr);
 * [`cdaaar`](#definition__vonuvoli_r7rs__cdaaar);
 * [`cdaadr`](#definition__vonuvoli_r7rs__cdaadr);
 * [`cdadar`](#definition__vonuvoli_r7rs__cdadar);
 * [`cdaddr`](#definition__vonuvoli_r7rs__cdaddr);
 * [`cddaar`](#definition__vonuvoli_r7rs__cddaar);
 * [`cddadr`](#definition__vonuvoli_r7rs__cddadr);
 * [`cdddar`](#definition__vonuvoli_r7rs__cdddar);
 * [`cddddr`](#definition__vonuvoli_r7rs__cddddr);
 * [`null?`](#definition__vonuvoli_r7rs__null_);
 * [`list?`](#definition__vonuvoli_r7rs__list_);
 * [`list`](#definition__vonuvoli_r7rs__list);
 * [`make-list`](#definition__vonuvoli_r7rs__make-list);
 * [`length`](#definition__vonuvoli_r7rs__length);
 * [`append`](#definition__vonuvoli_r7rs__append);
 * [`list-copy`](#definition__vonuvoli_r7rs__list-copy);
 * [`reverse`](#definition__vonuvoli_r7rs__reverse);
 * [`list-ref`](#definition__vonuvoli_r7rs__list-ref);
 * [`list-tail`](#definition__vonuvoli_r7rs__list-tail);
 * [`list-set!`](#definition__vonuvoli_r7rs__list-set!);
 * [`map`](#definition__vonuvoli_r7rs__map);
 * [`for-each`](#definition__vonuvoli_r7rs__for-each);
 * [`member`](#definition__vonuvoli_r7rs__member);
 * [`memq`](#definition__vonuvoli_r7rs__memq);
 * [`memv`](#definition__vonuvoli_r7rs__memv);
 * [`assoc`](#definition__vonuvoli_r7rs__assoc);
 * [`assqc`](#definition__vonuvoli_r7rs__assqc);
 * [`assvc`](#definition__vonuvoli_r7rs__assvc);
 * [`vector->list`](#definition__vonuvoli_r7rs__vector-_list);
 * [`list->vector`](#definition__vonuvoli_r7rs__list-_vector);
 * [`list->string`](#definition__vonuvoli_r7rs__list-_string);
 * [`string->list`](#definition__vonuvoli_r7rs__string-_list);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_loops'>

### Category `vs:loops`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`do`](#definition__vonuvoli_r7rs__do);
 * [`map`](#definition__vonuvoli_r7rs__map);
 * [`for-each`](#definition__vonuvoli_r7rs__for-each);
 * [`vector-map`](#definition__vonuvoli_r7rs__vector-map);
 * [`vector-for-each`](#definition__vonuvoli_r7rs__vector-for-each);
 * [`string-map`](#definition__vonuvoli_r7rs__string-map);
 * [`string-for-each`](#definition__vonuvoli_r7rs__string-for-each);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_modules'>

### Category `vs:modules`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_pairs'>

### Category `vs:pairs`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`pair?`](#definition__vonuvoli_r7rs__pair_);
 * [`cons`](#definition__vonuvoli_r7rs__cons);
 * [`car`](#definition__vonuvoli_r7rs__car);
 * [`cdr`](#definition__vonuvoli_r7rs__cdr);
 * [`set-car!`](#definition__vonuvoli_r7rs__set-car!);
 * [`set-cdr!`](#definition__vonuvoli_r7rs__set-cdr!);
 * [`caar`](#definition__vonuvoli_r7rs__caar);
 * [`cadr`](#definition__vonuvoli_r7rs__cadr);
 * [`cdar`](#definition__vonuvoli_r7rs__cdar);
 * [`cddr`](#definition__vonuvoli_r7rs__cddr);
 * [`caaar`](#definition__vonuvoli_r7rs__caaar);
 * [`caadr`](#definition__vonuvoli_r7rs__caadr);
 * [`cadar`](#definition__vonuvoli_r7rs__cadar);
 * [`caddr`](#definition__vonuvoli_r7rs__caddr);
 * [`cdaar`](#definition__vonuvoli_r7rs__cdaar);
 * [`cdadr`](#definition__vonuvoli_r7rs__cdadr);
 * [`cddar`](#definition__vonuvoli_r7rs__cddar);
 * [`cdddr`](#definition__vonuvoli_r7rs__cdddr);
 * [`caaaar`](#definition__vonuvoli_r7rs__caaaar);
 * [`caaadr`](#definition__vonuvoli_r7rs__caaadr);
 * [`caadar`](#definition__vonuvoli_r7rs__caadar);
 * [`caaddr`](#definition__vonuvoli_r7rs__caaddr);
 * [`cadaar`](#definition__vonuvoli_r7rs__cadaar);
 * [`cadadr`](#definition__vonuvoli_r7rs__cadadr);
 * [`caddar`](#definition__vonuvoli_r7rs__caddar);
 * [`cadddr`](#definition__vonuvoli_r7rs__cadddr);
 * [`cdaaar`](#definition__vonuvoli_r7rs__cdaaar);
 * [`cdaadr`](#definition__vonuvoli_r7rs__cdaadr);
 * [`cdadar`](#definition__vonuvoli_r7rs__cdadar);
 * [`cdaddr`](#definition__vonuvoli_r7rs__cdaddr);
 * [`cddaar`](#definition__vonuvoli_r7rs__cddaar);
 * [`cddadr`](#definition__vonuvoli_r7rs__cddadr);
 * [`cdddar`](#definition__vonuvoli_r7rs__cdddar);
 * [`cddddr`](#definition__vonuvoli_r7rs__cddddr);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_parameters'>

### Category `vs:parameters`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`parameterize`](#definition__vonuvoli_r7rs__parameterize);
 * [`make-parameter`](#definition__vonuvoli_r7rs__make-parameter);
 * [`current-input-port`](#definition__vonuvoli_r7rs__current-input-port);
 * [`current-output-port`](#definition__vonuvoli_r7rs__current-output-port);
 * [`current-error-port`](#definition__vonuvoli_r7rs__current-error-port);
 * [`with-input-from-file`](#definition__vonuvoli_r7rs__with-input-from-file);
 * [`with-output-from-file`](#definition__vonuvoli_r7rs__with-output-from-file);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_ports'>

### Category `vs:ports`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).

Contains the following sub-categories:
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);
 * [`vs:ports:values`](#category__vonuvoli_r7rs__vs_ports_values);


#### Definitions

 * [`port?`](#definition__vonuvoli_r7rs__port_);
 * [`binary-port?`](#definition__vonuvoli_r7rs__binary-port_);
 * [`textual-port?`](#definition__vonuvoli_r7rs__textual-port_);
 * [`input-port?`](#definition__vonuvoli_r7rs__input-port_);
 * [`input-port-open?`](#definition__vonuvoli_r7rs__input-port-open_);
 * [`output-port?`](#definition__vonuvoli_r7rs__output-port_);
 * [`output-port-open?`](#definition__vonuvoli_r7rs__output-port-open_);
 * [`open-input-bytevector`](#definition__vonuvoli_r7rs__open-input-bytevector);
 * [`open-output-bytevector`](#definition__vonuvoli_r7rs__open-output-bytevector);
 * [`get-output-bytevector`](#definition__vonuvoli_r7rs__get-output-bytevector);
 * [`open-input-string`](#definition__vonuvoli_r7rs__open-input-string);
 * [`open-output-string`](#definition__vonuvoli_r7rs__open-output-string);
 * [`get-output-string`](#definition__vonuvoli_r7rs__get-output-string);
 * [`close-port`](#definition__vonuvoli_r7rs__close-port);
 * [`close-input-port`](#definition__vonuvoli_r7rs__close-input-port);
 * [`close-output-port`](#definition__vonuvoli_r7rs__close-output-port);
 * [`u8-ready?`](#definition__vonuvoli_r7rs__u8-ready_);
 * [`peek-u8`](#definition__vonuvoli_r7rs__peek-u8);
 * [`read-u8`](#definition__vonuvoli_r7rs__read-u8);
 * [`write-u8`](#definition__vonuvoli_r7rs__write-u8);
 * [`read-bytevector`](#definition__vonuvoli_r7rs__read-bytevector);
 * [`read-bytevector!`](#definition__vonuvoli_r7rs__read-bytevector!);
 * [`write-bytevector`](#definition__vonuvoli_r7rs__write-bytevector);
 * [`char-ready?`](#definition__vonuvoli_r7rs__char-ready_);
 * [`peek-char`](#definition__vonuvoli_r7rs__peek-char);
 * [`read-char`](#definition__vonuvoli_r7rs__read-char);
 * [`write-char`](#definition__vonuvoli_r7rs__write-char);
 * [`read-string`](#definition__vonuvoli_r7rs__read-string);
 * [`read-line`](#definition__vonuvoli_r7rs__read-line);
 * [`newline`](#definition__vonuvoli_r7rs__newline);
 * [`flush-output-port`](#definition__vonuvoli_r7rs__flush-output-port);
 * [`read`](#definition__vonuvoli_r7rs__read);
 * [`write`](#definition__vonuvoli_r7rs__write);
 * [`write-simple`](#definition__vonuvoli_r7rs__write-simple);
 * [`write-shared`](#definition__vonuvoli_r7rs__write-shared);
 * [`display`](#definition__vonuvoli_r7rs__display);
 * [`open-input-file`](#definition__vonuvoli_r7rs__open-input-file);
 * [`open-binary-input-file`](#definition__vonuvoli_r7rs__open-binary-input-file);
 * [`open-output-file`](#definition__vonuvoli_r7rs__open-output-file);
 * [`open-binary-output-file`](#definition__vonuvoli_r7rs__open-binary-output-file);
 * [`call-with-port`](#definition__vonuvoli_r7rs__call-with-port);
 * [`call-with-input-file`](#definition__vonuvoli_r7rs__call-with-input-file);
 * [`call-with-output-file`](#definition__vonuvoli_r7rs__call-with-output-file);
 * [`eof-object`](#definition__vonuvoli_r7rs__eof-object);
 * [`eof-object?`](#definition__vonuvoli_r7rs__eof-object_);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_ports_input'>

### Category `vs:ports:input`

Belongs to the super-category: [`vs:ports`](#category__vonuvoli_r7rs__vs_ports).


#### Definitions

 * [`input-port?`](#definition__vonuvoli_r7rs__input-port_);
 * [`input-port-open?`](#definition__vonuvoli_r7rs__input-port-open_);
 * [`open-input-bytevector`](#definition__vonuvoli_r7rs__open-input-bytevector);
 * [`open-input-string`](#definition__vonuvoli_r7rs__open-input-string);
 * [`close-input-port`](#definition__vonuvoli_r7rs__close-input-port);
 * [`u8-ready?`](#definition__vonuvoli_r7rs__u8-ready_);
 * [`peek-u8`](#definition__vonuvoli_r7rs__peek-u8);
 * [`read-u8`](#definition__vonuvoli_r7rs__read-u8);
 * [`read-bytevector`](#definition__vonuvoli_r7rs__read-bytevector);
 * [`read-bytevector!`](#definition__vonuvoli_r7rs__read-bytevector!);
 * [`char-ready?`](#definition__vonuvoli_r7rs__char-ready_);
 * [`peek-char`](#definition__vonuvoli_r7rs__peek-char);
 * [`read-char`](#definition__vonuvoli_r7rs__read-char);
 * [`read-string`](#definition__vonuvoli_r7rs__read-string);
 * [`read-line`](#definition__vonuvoli_r7rs__read-line);
 * [`read`](#definition__vonuvoli_r7rs__read);
 * [`open-input-file`](#definition__vonuvoli_r7rs__open-input-file);
 * [`open-binary-input-file`](#definition__vonuvoli_r7rs__open-binary-input-file);
 * [`call-with-input-file`](#definition__vonuvoli_r7rs__call-with-input-file);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_ports_output'>

### Category `vs:ports:output`

Belongs to the super-category: [`vs:ports`](#category__vonuvoli_r7rs__vs_ports).


#### Definitions

 * [`output-port?`](#definition__vonuvoli_r7rs__output-port_);
 * [`output-port-open?`](#definition__vonuvoli_r7rs__output-port-open_);
 * [`open-output-bytevector`](#definition__vonuvoli_r7rs__open-output-bytevector);
 * [`get-output-bytevector`](#definition__vonuvoli_r7rs__get-output-bytevector);
 * [`open-output-string`](#definition__vonuvoli_r7rs__open-output-string);
 * [`get-output-string`](#definition__vonuvoli_r7rs__get-output-string);
 * [`close-output-port`](#definition__vonuvoli_r7rs__close-output-port);
 * [`write-u8`](#definition__vonuvoli_r7rs__write-u8);
 * [`write-bytevector`](#definition__vonuvoli_r7rs__write-bytevector);
 * [`write-char`](#definition__vonuvoli_r7rs__write-char);
 * [`newline`](#definition__vonuvoli_r7rs__newline);
 * [`flush-output-port`](#definition__vonuvoli_r7rs__flush-output-port);
 * [`write`](#definition__vonuvoli_r7rs__write);
 * [`write-simple`](#definition__vonuvoli_r7rs__write-simple);
 * [`write-shared`](#definition__vonuvoli_r7rs__write-shared);
 * [`display`](#definition__vonuvoli_r7rs__display);
 * [`open-output-file`](#definition__vonuvoli_r7rs__open-output-file);
 * [`open-binary-output-file`](#definition__vonuvoli_r7rs__open-binary-output-file);
 * [`call-with-output-file`](#definition__vonuvoli_r7rs__call-with-output-file);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_ports_open'>

### Category `vs:ports:open`

Belongs to the super-category: [`vs:ports`](#category__vonuvoli_r7rs__vs_ports).


#### Definitions

 * [`input-port-open?`](#definition__vonuvoli_r7rs__input-port-open_);
 * [`output-port-open?`](#definition__vonuvoli_r7rs__output-port-open_);
 * [`open-input-bytevector`](#definition__vonuvoli_r7rs__open-input-bytevector);
 * [`open-output-bytevector`](#definition__vonuvoli_r7rs__open-output-bytevector);
 * [`open-input-string`](#definition__vonuvoli_r7rs__open-input-string);
 * [`open-output-string`](#definition__vonuvoli_r7rs__open-output-string);
 * [`open-input-file`](#definition__vonuvoli_r7rs__open-input-file);
 * [`open-binary-input-file`](#definition__vonuvoli_r7rs__open-binary-input-file);
 * [`open-output-file`](#definition__vonuvoli_r7rs__open-output-file);
 * [`open-binary-output-file`](#definition__vonuvoli_r7rs__open-binary-output-file);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_ports_values'>

### Category `vs:ports:values`

Belongs to the super-category: [`vs:ports`](#category__vonuvoli_r7rs__vs_ports).


#### Definitions

 * [`read`](#definition__vonuvoli_r7rs__read);
 * [`write`](#definition__vonuvoli_r7rs__write);
 * [`write-simple`](#definition__vonuvoli_r7rs__write-simple);
 * [`write-shared`](#definition__vonuvoli_r7rs__write-shared);
 * [`display`](#definition__vonuvoli_r7rs__display);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_promises'>

### Category `vs:promises`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`delay`](#definition__vonuvoli_r7rs__delay);
 * [`delay-force`](#definition__vonuvoli_r7rs__delay-force);
 * [`promise?`](#definition__vonuvoli_r7rs__promise_);
 * [`make-promise`](#definition__vonuvoli_r7rs__make-promise);
 * [`force`](#definition__vonuvoli_r7rs__force);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_quotation'>

### Category `vs:quotation`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`quote`](#definition__vonuvoli_r7rs__quote);
 * [`quasiquote`](#definition__vonuvoli_r7rs__quasiquote);
 * [`unquote`](#definition__vonuvoli_r7rs__unquote);
 * [`unquote-splicing`](#definition__vonuvoli_r7rs__unquote-splicing);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_records'>

### Category `vs:records`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`define-record-type`](#definition__vonuvoli_r7rs__define-record-type);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_strings'>

### Category `vs:strings`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`string?`](#definition__vonuvoli_r7rs__string_);
 * [`string`](#definition__vonuvoli_r7rs__string);
 * [`make-string`](#definition__vonuvoli_r7rs__make-string);
 * [`string-length`](#definition__vonuvoli_r7rs__string-length);
 * [`string-append`](#definition__vonuvoli_r7rs__string-append);
 * [`string-copy`](#definition__vonuvoli_r7rs__string-copy);
 * [`string-copy!`](#definition__vonuvoli_r7rs__string-copy!);
 * [`string-fill!`](#definition__vonuvoli_r7rs__string-fill!);
 * [`substring`](#definition__vonuvoli_r7rs__substring);
 * [`string-ref`](#definition__vonuvoli_r7rs__string-ref);
 * [`string-set!`](#definition__vonuvoli_r7rs__string-set!);
 * [`string=?`](#definition__vonuvoli_r7rs__string__);
 * [`string<?`](#definition__vonuvoli_r7rs__string__);
 * [`string>?`](#definition__vonuvoli_r7rs__string__);
 * [`string<=?`](#definition__vonuvoli_r7rs__string___);
 * [`string>=?`](#definition__vonuvoli_r7rs__string___);
 * [`string-ci=?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci>?`](#definition__vonuvoli_r7rs__string-ci__);
 * [`string-ci<=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`string-ci>=?`](#definition__vonuvoli_r7rs__string-ci___);
 * [`number->string`](#definition__vonuvoli_r7rs__number-_string);
 * [`string->number`](#definition__vonuvoli_r7rs__string-_number);
 * [`symbol->string`](#definition__vonuvoli_r7rs__symbol-_string);
 * [`string->symbol`](#definition__vonuvoli_r7rs__string-_symbol);
 * [`list->string`](#definition__vonuvoli_r7rs__list-_string);
 * [`string->list`](#definition__vonuvoli_r7rs__string-_list);
 * [`vector->string`](#definition__vonuvoli_r7rs__vector-_string);
 * [`string->vector`](#definition__vonuvoli_r7rs__string-_vector);
 * [`string-map`](#definition__vonuvoli_r7rs__string-map);
 * [`string-for-each`](#definition__vonuvoli_r7rs__string-for-each);
 * [`string-upcase`](#definition__vonuvoli_r7rs__string-upcase);
 * [`string-downcase`](#definition__vonuvoli_r7rs__string-downcase);
 * [`string-foldcase`](#definition__vonuvoli_r7rs__string-foldcase);
 * [`utf8->string`](#definition__vonuvoli_r7rs__utf8-_string);
 * [`string->utf8`](#definition__vonuvoli_r7rs__string-_utf8);
 * [`open-input-string`](#definition__vonuvoli_r7rs__open-input-string);
 * [`open-output-string`](#definition__vonuvoli_r7rs__open-output-string);
 * [`get-output-string`](#definition__vonuvoli_r7rs__get-output-string);
 * [`char-ready?`](#definition__vonuvoli_r7rs__char-ready_);
 * [`peek-char`](#definition__vonuvoli_r7rs__peek-char);
 * [`read-char`](#definition__vonuvoli_r7rs__read-char);
 * [`write-char`](#definition__vonuvoli_r7rs__write-char);
 * [`read-string`](#definition__vonuvoli_r7rs__read-string);
 * [`read-line`](#definition__vonuvoli_r7rs__read-line);
 * [`newline`](#definition__vonuvoli_r7rs__newline);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_symbols'>

### Category `vs:symbols`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`symbol?`](#definition__vonuvoli_r7rs__symbol_);
 * [`symbol=?`](#definition__vonuvoli_r7rs__symbol__);
 * [`symbol->string`](#definition__vonuvoli_r7rs__symbol-_string);
 * [`string->symbol`](#definition__vonuvoli_r7rs__string-_symbol);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_syntaxes'>

### Category `vs:syntaxes`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`define-syntax`](#definition__vonuvoli_r7rs__define-syntax);
 * [`let-syntax`](#definition__vonuvoli_r7rs__let-syntax);
 * [`letrec-syntax`](#definition__vonuvoli_r7rs__letrec-syntax);
 * [`syntax-rules`](#definition__vonuvoli_r7rs__syntax-rules);
 * [`syntax-error`](#definition__vonuvoli_r7rs__syntax-error);
 * [`_`](#definition__vonuvoli_r7rs___);
 * [`...`](#definition__vonuvoli_r7rs_____);
 * [`=>`](#definition__vonuvoli_r7rs____);
 * [`else`](#definition__vonuvoli_r7rs__else);
 * [`quote`](#definition__vonuvoli_r7rs__quote);
 * [`quasiquote`](#definition__vonuvoli_r7rs__quasiquote);
 * [`unquote`](#definition__vonuvoli_r7rs__unquote);
 * [`unquote-splicing`](#definition__vonuvoli_r7rs__unquote-splicing);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_system'>

### Category `vs:system`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_types'>

### Category `vs:types`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`boolean?`](#definition__vonuvoli_r7rs__boolean_);
 * [`symbol?`](#definition__vonuvoli_r7rs__symbol_);
 * [`number?`](#definition__vonuvoli_r7rs__number_);
 * [`integer?`](#definition__vonuvoli_r7rs__integer_);
 * [`real?`](#definition__vonuvoli_r7rs__real_);
 * [`rational?`](#definition__vonuvoli_r7rs__rational_);
 * [`complex?`](#definition__vonuvoli_r7rs__complex_);
 * [`exact?`](#definition__vonuvoli_r7rs__exact_);
 * [`inexact?`](#definition__vonuvoli_r7rs__inexact_);
 * [`exact-integer?`](#definition__vonuvoli_r7rs__exact-integer_);
 * [`pair?`](#definition__vonuvoli_r7rs__pair_);
 * [`null?`](#definition__vonuvoli_r7rs__null_);
 * [`list?`](#definition__vonuvoli_r7rs__list_);
 * [`vector?`](#definition__vonuvoli_r7rs__vector_);
 * [`string?`](#definition__vonuvoli_r7rs__string_);
 * [`port?`](#definition__vonuvoli_r7rs__port_);
 * [`char?`](#definition__vonuvoli_r7rs__char_);
 * [`procedure?`](#definition__vonuvoli_r7rs__procedure_);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_unimplemented'>

### Category `vs:unimplemented`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_unsupported'>

### Category `vs:unsupported`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`define-syntax`](#definition__vonuvoli_r7rs__define-syntax);
 * [`let-syntax`](#definition__vonuvoli_r7rs__let-syntax);
 * [`letrec-syntax`](#definition__vonuvoli_r7rs__letrec-syntax);
 * [`syntax-rules`](#definition__vonuvoli_r7rs__syntax-rules);
 * [`syntax-error`](#definition__vonuvoli_r7rs__syntax-error);
 * [`rationalize`](#definition__vonuvoli_r7rs__rationalize);
 * [`numerator`](#definition__vonuvoli_r7rs__numerator);
 * [`denominator`](#definition__vonuvoli_r7rs__denominator);
 * [`make-rectangular`](#definition__vonuvoli_r7rs__make-rectangular);
 * [`real-part`](#definition__vonuvoli_r7rs__real-part);
 * [`imag-part`](#definition__vonuvoli_r7rs__imag-part);
 * [`make-polar`](#definition__vonuvoli_r7rs__make-polar);
 * [`magnitude`](#definition__vonuvoli_r7rs__magnitude);
 * [`angle`](#definition__vonuvoli_r7rs__angle);
 * [`raise-continuable`](#definition__vonuvoli_r7rs__raise-continuable);
 * [`eval`](#definition__vonuvoli_r7rs__eval);
 * [`environment`](#definition__vonuvoli_r7rs__environment);
 * [`interaction-environment`](#definition__vonuvoli_r7rs__interaction-environment);
 * [`scheme-report-environment`](#definition__vonuvoli_r7rs__scheme-report-environment);
 * [`null-environment`](#definition__vonuvoli_r7rs__null-environment);
 * [`call-with-current-continuation`](#definition__vonuvoli_r7rs__call-with-current-continuation);
 * [`dynamic-wind`](#definition__vonuvoli_r7rs__dynamic-wind);
 * [`cond-expand`](#definition__vonuvoli_r7rs__cond-expand);
 * [`features`](#definition__vonuvoli_r7rs__features);
 * [`include`](#definition__vonuvoli_r7rs__include);
 * [`include-ci`](#definition__vonuvoli_r7rs__include-ci);
 * [`import`](#definition__vonuvoli_r7rs__import);
 * [`load`](#definition__vonuvoli_r7rs__load);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_values'>

### Category `vs:values`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`define-values`](#definition__vonuvoli_r7rs__define-values);
 * [`let-values`](#definition__vonuvoli_r7rs__let-values);
 * [`let*-values`](#definition__vonuvoli_r7rs__let_-values);
 * [`values`](#definition__vonuvoli_r7rs__values);
 * [`call-with-values`](#definition__vonuvoli_r7rs__call-with-values);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='category__vonuvoli_r7rs__vs_vectors'>

### Category `vs:vectors`

Belongs to the super-category: [`vs`](#category__vonuvoli_r7rs__vs).


#### Definitions

 * [`vector?`](#definition__vonuvoli_r7rs__vector_);
 * [`vector`](#definition__vonuvoli_r7rs__vector);
 * [`make-vector`](#definition__vonuvoli_r7rs__make-vector);
 * [`vector-length`](#definition__vonuvoli_r7rs__vector-length);
 * [`vector-append`](#definition__vonuvoli_r7rs__vector-append);
 * [`vector-copy`](#definition__vonuvoli_r7rs__vector-copy);
 * [`vector-copy!`](#definition__vonuvoli_r7rs__vector-copy!);
 * [`vector-fill!`](#definition__vonuvoli_r7rs__vector-fill!);
 * [`vector-ref`](#definition__vonuvoli_r7rs__vector-ref);
 * [`vector-set!`](#definition__vonuvoli_r7rs__vector-set!);
 * [`vector->list`](#definition__vonuvoli_r7rs__vector-_list);
 * [`list->vector`](#definition__vonuvoli_r7rs__list-_vector);
 * [`vector-map`](#definition__vonuvoli_r7rs__vector-map);
 * [`vector-for-each`](#definition__vonuvoli_r7rs__vector-for-each);
 * [`vector->string`](#definition__vonuvoli_r7rs__vector-_string);
 * [`string->vector`](#definition__vonuvoli_r7rs__string-_vector);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----




<a id='toc__vonuvoli_r7rs__types'>

## Types

Quick list of types:
* [`any`](#value_kind__vonuvoli_r7rs__any): [`null`](#value_kind__vonuvoli_r7rs__null), [`boolean`](#value_kind__vonuvoli_r7rs__boolean), [`number`](#value_kind__vonuvoli_r7rs__number), [`symbol`](#value_kind__vonuvoli_r7rs__symbol), [`character`](#value_kind__vonuvoli_r7rs__character), [`string`](#value_kind__vonuvoli_r7rs__string), [`bytevector`](#value_kind__vonuvoli_r7rs__bytevector), [`vector`](#value_kind__vonuvoli_r7rs__vector), [`pair`](#value_kind__vonuvoli_r7rs__pair), [`port`](#value_kind__vonuvoli_r7rs__port), [`eof-object`](#value_kind__vonuvoli_r7rs__eof-object), [`procedure`](#value_kind__vonuvoli_r7rs__procedure);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__any'>

### Type `any`

Contains the following sub-types:
 * [`null`](#value_kind__vonuvoli_r7rs__null);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);
 * [`number`](#value_kind__vonuvoli_r7rs__number);
 * [`symbol`](#value_kind__vonuvoli_r7rs__symbol);
 * [`character`](#value_kind__vonuvoli_r7rs__character);
 * [`string`](#value_kind__vonuvoli_r7rs__string);
 * [`bytevector`](#value_kind__vonuvoli_r7rs__bytevector);
 * [`vector`](#value_kind__vonuvoli_r7rs__vector);
 * [`pair`](#value_kind__vonuvoli_r7rs__pair);
 * [`port`](#value_kind__vonuvoli_r7rs__port);
 * [`eof-object`](#value_kind__vonuvoli_r7rs__eof-object);
 * [`procedure`](#value_kind__vonuvoli_r7rs__procedure);

Belongs to the following categories:
 * [`r7rs-x:types`](#category__vonuvoli_r7rs__r7rs-x_types);


#### Description

> **FIXME!**



#### Referent definitions

 * [`syntax-error`](#definition__vonuvoli_r7rs__syntax-error);
 * [`quote`](#definition__vonuvoli_r7rs__quote);
 * [`quasiquote`](#definition__vonuvoli_r7rs__quasiquote);
 * [`unquote`](#definition__vonuvoli_r7rs__unquote);
 * [`unquote-splicing`](#definition__vonuvoli_r7rs__unquote-splicing);
 * [`case`](#definition__vonuvoli_r7rs__case);
 * [`eq?`](#definition__vonuvoli_r7rs__eq_);
 * [`eqv?`](#definition__vonuvoli_r7rs__eqv_);
 * [`equal?`](#definition__vonuvoli_r7rs__equal_);
 * [`not`](#definition__vonuvoli_r7rs__not);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__null'>

### Type `null`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|null?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__boolean'>

### Type `boolean`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|boolean?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**



#### Referent definitions

 * [`eq?`](#definition__vonuvoli_r7rs__eq_);
 * [`eqv?`](#definition__vonuvoli_r7rs__eqv_);
 * [`equal?`](#definition__vonuvoli_r7rs__equal_);
 * [`not`](#definition__vonuvoli_r7rs__not);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__number'>

### Type `number`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|number?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__symbol'>

### Type `symbol`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|symbol?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__character'>

### Type `character`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|char?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__string'>

### Type `string`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|string?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**



#### Referent definitions

 * [`syntax-error`](#definition__vonuvoli_r7rs__syntax-error);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__bytevector'>

### Type `bytevector`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|bytevector?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__vector'>

### Type `vector`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|vector?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__pair'>

### Type `pair`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|pair?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__port'>

### Type `port`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|port?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__eof-object'>

### Type `eof-object`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|eof-object?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='value_kind__vonuvoli_r7rs__procedure'>

### Type `procedure`

Belongs to the super-type: [`any`](#value_kind__vonuvoli_r7rs__any).

Verified by the folowing predicate:
```
|procedure?|
```

Belongs to the following categories:
 * [`r7rs-x:types-disjoint`](#category__vonuvoli_r7rs__r7rs-x_types-disjoint);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----




<a id='toc__vonuvoli_r7rs__definitions'>

## Definitions

Quick list of definitions:
* [`define-syntax`](#definition__vonuvoli_r7rs__define-syntax);
* [`let-syntax`](#definition__vonuvoli_r7rs__let-syntax);
* [`letrec-syntax`](#definition__vonuvoli_r7rs__letrec-syntax);
* [`syntax-rules`](#definition__vonuvoli_r7rs__syntax-rules);
* [`syntax-error`](#definition__vonuvoli_r7rs__syntax-error);
* [`_`](#definition__vonuvoli_r7rs___);
* [`...`](#definition__vonuvoli_r7rs_____);
* [`=>`](#definition__vonuvoli_r7rs____);
* [`else`](#definition__vonuvoli_r7rs__else);
* [`quote`](#definition__vonuvoli_r7rs__quote);
* [`quasiquote`](#definition__vonuvoli_r7rs__quasiquote);
* [`unquote`](#definition__vonuvoli_r7rs__unquote);
* [`unquote-splicing`](#definition__vonuvoli_r7rs__unquote-splicing);
* [`lambda`](#definition__vonuvoli_r7rs__lambda);
* [`case-lambda`](#definition__vonuvoli_r7rs__case-lambda);
* [`define`](#definition__vonuvoli_r7rs__define);
* [`let`](#definition__vonuvoli_r7rs__let);
* [`let*`](#definition__vonuvoli_r7rs__let_);
* [`letrec`](#definition__vonuvoli_r7rs__letrec);
* [`letrec*`](#definition__vonuvoli_r7rs__letrec_);
* [`set!`](#definition__vonuvoli_r7rs__set!);
* [`define-values`](#definition__vonuvoli_r7rs__define-values);
* [`let-values`](#definition__vonuvoli_r7rs__let-values);
* [`let*-values`](#definition__vonuvoli_r7rs__let_-values);
* [`define-record-type`](#definition__vonuvoli_r7rs__define-record-type);
* [`begin`](#definition__vonuvoli_r7rs__begin);
* [`and`](#definition__vonuvoli_r7rs__and);
* [`or`](#definition__vonuvoli_r7rs__or);
* [`if`](#definition__vonuvoli_r7rs__if);
* [`unless`](#definition__vonuvoli_r7rs__unless);
* [`when`](#definition__vonuvoli_r7rs__when);
* [`cond`](#definition__vonuvoli_r7rs__cond);
* [`case`](#definition__vonuvoli_r7rs__case);
* [`do`](#definition__vonuvoli_r7rs__do);
* [`eq?`](#definition__vonuvoli_r7rs__eq_);
* [`eqv?`](#definition__vonuvoli_r7rs__eqv_);
* [`equal?`](#definition__vonuvoli_r7rs__equal_);
* [`not`](#definition__vonuvoli_r7rs__not);
* [`boolean?`](#definition__vonuvoli_r7rs__boolean_);
* [`boolean=?`](#definition__vonuvoli_r7rs__boolean__);
* [`symbol?`](#definition__vonuvoli_r7rs__symbol_);
* [`symbol=?`](#definition__vonuvoli_r7rs__symbol__);
* [`number?`](#definition__vonuvoli_r7rs__number_);
* [`integer?`](#definition__vonuvoli_r7rs__integer_);
* [`real?`](#definition__vonuvoli_r7rs__real_);
* [`rational?`](#definition__vonuvoli_r7rs__rational_);
* [`complex?`](#definition__vonuvoli_r7rs__complex_);
* [`exact?`](#definition__vonuvoli_r7rs__exact_);
* [`inexact?`](#definition__vonuvoli_r7rs__inexact_);
* [`exact-integer?`](#definition__vonuvoli_r7rs__exact-integer_);
* [`zero?`](#definition__vonuvoli_r7rs__zero_);
* [`positive?`](#definition__vonuvoli_r7rs__positive_);
* [`odd?`](#definition__vonuvoli_r7rs__odd_);
* [`even?`](#definition__vonuvoli_r7rs__even_);
* [`=`](#definition__vonuvoli_r7rs___);
* [`<`](#definition__vonuvoli_r7rs___);
* [`>`](#definition__vonuvoli_r7rs___);
* [`<=`](#definition__vonuvoli_r7rs____);
* [`>=`](#definition__vonuvoli_r7rs____);
* [`+`](#definition__vonuvoli_r7rs___);
* [`-`](#definition__vonuvoli_r7rs__-);
* [`*`](#definition__vonuvoli_r7rs___);
* [`/`](#definition__vonuvoli_r7rs___);
* [`abs`](#definition__vonuvoli_r7rs__abs);
* [`floor/`](#definition__vonuvoli_r7rs__floor_);
* [`floor-quotient`](#definition__vonuvoli_r7rs__floor-quotient);
* [`floor-remainder`](#definition__vonuvoli_r7rs__floor-remainder);
* [`truncate/`](#definition__vonuvoli_r7rs__truncate_);
* [`truncate-quotient`](#definition__vonuvoli_r7rs__truncate-quotient);
* [`truncate-remainder`](#definition__vonuvoli_r7rs__truncate-remainder);
* [`floor`](#definition__vonuvoli_r7rs__floor);
* [`ceiling`](#definition__vonuvoli_r7rs__ceiling);
* [`truncate`](#definition__vonuvoli_r7rs__truncate);
* [`round`](#definition__vonuvoli_r7rs__round);
* [`min`](#definition__vonuvoli_r7rs__min);
* [`max`](#definition__vonuvoli_r7rs__max);
* [`gcd`](#definition__vonuvoli_r7rs__gcd);
* [`lcm`](#definition__vonuvoli_r7rs__lcm);
* [`expt`](#definition__vonuvoli_r7rs__expt);
* [`square`](#definition__vonuvoli_r7rs__square);
* [`exact-integer-sqrt`](#definition__vonuvoli_r7rs__exact-integer-sqrt);
* [`rationalize`](#definition__vonuvoli_r7rs__rationalize);
* [`numerator`](#definition__vonuvoli_r7rs__numerator);
* [`denominator`](#definition__vonuvoli_r7rs__denominator);
* [`inexact`](#definition__vonuvoli_r7rs__inexact);
* [`exact`](#definition__vonuvoli_r7rs__exact);
* [`make-rectangular`](#definition__vonuvoli_r7rs__make-rectangular);
* [`real-part`](#definition__vonuvoli_r7rs__real-part);
* [`imag-part`](#definition__vonuvoli_r7rs__imag-part);
* [`make-polar`](#definition__vonuvoli_r7rs__make-polar);
* [`magnitude`](#definition__vonuvoli_r7rs__magnitude);
* [`angle`](#definition__vonuvoli_r7rs__angle);
* [`sqrt`](#definition__vonuvoli_r7rs__sqrt);
* [`exp`](#definition__vonuvoli_r7rs__exp);
* [`log`](#definition__vonuvoli_r7rs__log);
* [`sin`](#definition__vonuvoli_r7rs__sin);
* [`cos`](#definition__vonuvoli_r7rs__cos);
* [`tan`](#definition__vonuvoli_r7rs__tan);
* [`asin`](#definition__vonuvoli_r7rs__asin);
* [`acos`](#definition__vonuvoli_r7rs__acos);
* [`atan`](#definition__vonuvoli_r7rs__atan);
* [`finite?`](#definition__vonuvoli_r7rs__finite_);
* [`infinite?`](#definition__vonuvoli_r7rs__infinite_);
* [`nan?`](#definition__vonuvoli_r7rs__nan_);
* [`pair?`](#definition__vonuvoli_r7rs__pair_);
* [`cons`](#definition__vonuvoli_r7rs__cons);
* [`car`](#definition__vonuvoli_r7rs__car);
* [`cdr`](#definition__vonuvoli_r7rs__cdr);
* [`set-car!`](#definition__vonuvoli_r7rs__set-car!);
* [`set-cdr!`](#definition__vonuvoli_r7rs__set-cdr!);
* [`caar`](#definition__vonuvoli_r7rs__caar);
* [`cadr`](#definition__vonuvoli_r7rs__cadr);
* [`cdar`](#definition__vonuvoli_r7rs__cdar);
* [`cddr`](#definition__vonuvoli_r7rs__cddr);
* [`caaar`](#definition__vonuvoli_r7rs__caaar);
* [`caadr`](#definition__vonuvoli_r7rs__caadr);
* [`cadar`](#definition__vonuvoli_r7rs__cadar);
* [`caddr`](#definition__vonuvoli_r7rs__caddr);
* [`cdaar`](#definition__vonuvoli_r7rs__cdaar);
* [`cdadr`](#definition__vonuvoli_r7rs__cdadr);
* [`cddar`](#definition__vonuvoli_r7rs__cddar);
* [`cdddr`](#definition__vonuvoli_r7rs__cdddr);
* [`caaaar`](#definition__vonuvoli_r7rs__caaaar);
* [`caaadr`](#definition__vonuvoli_r7rs__caaadr);
* [`caadar`](#definition__vonuvoli_r7rs__caadar);
* [`caaddr`](#definition__vonuvoli_r7rs__caaddr);
* [`cadaar`](#definition__vonuvoli_r7rs__cadaar);
* [`cadadr`](#definition__vonuvoli_r7rs__cadadr);
* [`caddar`](#definition__vonuvoli_r7rs__caddar);
* [`cadddr`](#definition__vonuvoli_r7rs__cadddr);
* [`cdaaar`](#definition__vonuvoli_r7rs__cdaaar);
* [`cdaadr`](#definition__vonuvoli_r7rs__cdaadr);
* [`cdadar`](#definition__vonuvoli_r7rs__cdadar);
* [`cdaddr`](#definition__vonuvoli_r7rs__cdaddr);
* [`cddaar`](#definition__vonuvoli_r7rs__cddaar);
* [`cddadr`](#definition__vonuvoli_r7rs__cddadr);
* [`cdddar`](#definition__vonuvoli_r7rs__cdddar);
* [`cddddr`](#definition__vonuvoli_r7rs__cddddr);
* [`null?`](#definition__vonuvoli_r7rs__null_);
* [`list?`](#definition__vonuvoli_r7rs__list_);
* [`list`](#definition__vonuvoli_r7rs__list);
* [`make-list`](#definition__vonuvoli_r7rs__make-list);
* [`length`](#definition__vonuvoli_r7rs__length);
* [`append`](#definition__vonuvoli_r7rs__append);
* [`list-copy`](#definition__vonuvoli_r7rs__list-copy);
* [`reverse`](#definition__vonuvoli_r7rs__reverse);
* [`list-ref`](#definition__vonuvoli_r7rs__list-ref);
* [`list-tail`](#definition__vonuvoli_r7rs__list-tail);
* [`list-set!`](#definition__vonuvoli_r7rs__list-set!);
* [`map`](#definition__vonuvoli_r7rs__map);
* [`for-each`](#definition__vonuvoli_r7rs__for-each);
* [`member`](#definition__vonuvoli_r7rs__member);
* [`memq`](#definition__vonuvoli_r7rs__memq);
* [`memv`](#definition__vonuvoli_r7rs__memv);
* [`assoc`](#definition__vonuvoli_r7rs__assoc);
* [`assqc`](#definition__vonuvoli_r7rs__assqc);
* [`assvc`](#definition__vonuvoli_r7rs__assvc);
* [`vector?`](#definition__vonuvoli_r7rs__vector_);
* [`vector`](#definition__vonuvoli_r7rs__vector);
* [`make-vector`](#definition__vonuvoli_r7rs__make-vector);
* [`vector-length`](#definition__vonuvoli_r7rs__vector-length);
* [`vector-append`](#definition__vonuvoli_r7rs__vector-append);
* [`vector-copy`](#definition__vonuvoli_r7rs__vector-copy);
* [`vector-copy!`](#definition__vonuvoli_r7rs__vector-copy!);
* [`vector-fill!`](#definition__vonuvoli_r7rs__vector-fill!);
* [`vector-ref`](#definition__vonuvoli_r7rs__vector-ref);
* [`vector-set!`](#definition__vonuvoli_r7rs__vector-set!);
* [`vector->list`](#definition__vonuvoli_r7rs__vector-_list);
* [`list->vector`](#definition__vonuvoli_r7rs__list-_vector);
* [`vector-map`](#definition__vonuvoli_r7rs__vector-map);
* [`vector-for-each`](#definition__vonuvoli_r7rs__vector-for-each);
* [`string?`](#definition__vonuvoli_r7rs__string_);
* [`string`](#definition__vonuvoli_r7rs__string);
* [`make-string`](#definition__vonuvoli_r7rs__make-string);
* [`string-length`](#definition__vonuvoli_r7rs__string-length);
* [`string-append`](#definition__vonuvoli_r7rs__string-append);
* [`string-copy`](#definition__vonuvoli_r7rs__string-copy);
* [`string-copy!`](#definition__vonuvoli_r7rs__string-copy!);
* [`string-fill!`](#definition__vonuvoli_r7rs__string-fill!);
* [`substring`](#definition__vonuvoli_r7rs__substring);
* [`string-ref`](#definition__vonuvoli_r7rs__string-ref);
* [`string-set!`](#definition__vonuvoli_r7rs__string-set!);
* [`string=?`](#definition__vonuvoli_r7rs__string__);
* [`string<?`](#definition__vonuvoli_r7rs__string__);
* [`string>?`](#definition__vonuvoli_r7rs__string__);
* [`string<=?`](#definition__vonuvoli_r7rs__string___);
* [`string>=?`](#definition__vonuvoli_r7rs__string___);
* [`string-ci=?`](#definition__vonuvoli_r7rs__string-ci__);
* [`string-ci<?`](#definition__vonuvoli_r7rs__string-ci__);
* [`string-ci>?`](#definition__vonuvoli_r7rs__string-ci__);
* [`string-ci<=?`](#definition__vonuvoli_r7rs__string-ci___);
* [`string-ci>=?`](#definition__vonuvoli_r7rs__string-ci___);
* [`number->string`](#definition__vonuvoli_r7rs__number-_string);
* [`string->number`](#definition__vonuvoli_r7rs__string-_number);
* [`symbol->string`](#definition__vonuvoli_r7rs__symbol-_string);
* [`string->symbol`](#definition__vonuvoli_r7rs__string-_symbol);
* [`list->string`](#definition__vonuvoli_r7rs__list-_string);
* [`string->list`](#definition__vonuvoli_r7rs__string-_list);
* [`vector->string`](#definition__vonuvoli_r7rs__vector-_string);
* [`string->vector`](#definition__vonuvoli_r7rs__string-_vector);
* [`string-map`](#definition__vonuvoli_r7rs__string-map);
* [`string-for-each`](#definition__vonuvoli_r7rs__string-for-each);
* [`string-upcase`](#definition__vonuvoli_r7rs__string-upcase);
* [`string-downcase`](#definition__vonuvoli_r7rs__string-downcase);
* [`string-foldcase`](#definition__vonuvoli_r7rs__string-foldcase);
* [`bytevector?`](#definition__vonuvoli_r7rs__bytevector_);
* [`bytevector`](#definition__vonuvoli_r7rs__bytevector);
* [`make-bytevector`](#definition__vonuvoli_r7rs__make-bytevector);
* [`bytevector-length`](#definition__vonuvoli_r7rs__bytevector-length);
* [`bytevector-append`](#definition__vonuvoli_r7rs__bytevector-append);
* [`bytevector-copy`](#definition__vonuvoli_r7rs__bytevector-copy);
* [`bytevector-copy!`](#definition__vonuvoli_r7rs__bytevector-copy!);
* [`bytevector-u8-ref`](#definition__vonuvoli_r7rs__bytevector-u8-ref);
* [`bytevector-u8-set!`](#definition__vonuvoli_r7rs__bytevector-u8-set!);
* [`utf8->string`](#definition__vonuvoli_r7rs__utf8-_string);
* [`string->utf8`](#definition__vonuvoli_r7rs__string-_utf8);
* [`port?`](#definition__vonuvoli_r7rs__port_);
* [`binary-port?`](#definition__vonuvoli_r7rs__binary-port_);
* [`textual-port?`](#definition__vonuvoli_r7rs__textual-port_);
* [`input-port?`](#definition__vonuvoli_r7rs__input-port_);
* [`input-port-open?`](#definition__vonuvoli_r7rs__input-port-open_);
* [`output-port?`](#definition__vonuvoli_r7rs__output-port_);
* [`output-port-open?`](#definition__vonuvoli_r7rs__output-port-open_);
* [`open-input-bytevector`](#definition__vonuvoli_r7rs__open-input-bytevector);
* [`open-output-bytevector`](#definition__vonuvoli_r7rs__open-output-bytevector);
* [`get-output-bytevector`](#definition__vonuvoli_r7rs__get-output-bytevector);
* [`open-input-string`](#definition__vonuvoli_r7rs__open-input-string);
* [`open-output-string`](#definition__vonuvoli_r7rs__open-output-string);
* [`get-output-string`](#definition__vonuvoli_r7rs__get-output-string);
* [`close-port`](#definition__vonuvoli_r7rs__close-port);
* [`close-input-port`](#definition__vonuvoli_r7rs__close-input-port);
* [`close-output-port`](#definition__vonuvoli_r7rs__close-output-port);
* [`u8-ready?`](#definition__vonuvoli_r7rs__u8-ready_);
* [`peek-u8`](#definition__vonuvoli_r7rs__peek-u8);
* [`read-u8`](#definition__vonuvoli_r7rs__read-u8);
* [`write-u8`](#definition__vonuvoli_r7rs__write-u8);
* [`read-bytevector`](#definition__vonuvoli_r7rs__read-bytevector);
* [`read-bytevector!`](#definition__vonuvoli_r7rs__read-bytevector!);
* [`write-bytevector`](#definition__vonuvoli_r7rs__write-bytevector);
* [`char-ready?`](#definition__vonuvoli_r7rs__char-ready_);
* [`peek-char`](#definition__vonuvoli_r7rs__peek-char);
* [`read-char`](#definition__vonuvoli_r7rs__read-char);
* [`write-char`](#definition__vonuvoli_r7rs__write-char);
* [`read-string`](#definition__vonuvoli_r7rs__read-string);
* [`read-line`](#definition__vonuvoli_r7rs__read-line);
* [`newline`](#definition__vonuvoli_r7rs__newline);
* [`flush-output-port`](#definition__vonuvoli_r7rs__flush-output-port);
* [`read`](#definition__vonuvoli_r7rs__read);
* [`write`](#definition__vonuvoli_r7rs__write);
* [`write-simple`](#definition__vonuvoli_r7rs__write-simple);
* [`write-shared`](#definition__vonuvoli_r7rs__write-shared);
* [`display`](#definition__vonuvoli_r7rs__display);
* [`open-input-file`](#definition__vonuvoli_r7rs__open-input-file);
* [`open-binary-input-file`](#definition__vonuvoli_r7rs__open-binary-input-file);
* [`open-output-file`](#definition__vonuvoli_r7rs__open-output-file);
* [`open-binary-output-file`](#definition__vonuvoli_r7rs__open-binary-output-file);
* [`call-with-port`](#definition__vonuvoli_r7rs__call-with-port);
* [`call-with-input-file`](#definition__vonuvoli_r7rs__call-with-input-file);
* [`call-with-output-file`](#definition__vonuvoli_r7rs__call-with-output-file);
* [`eof-object`](#definition__vonuvoli_r7rs__eof-object);
* [`eof-object?`](#definition__vonuvoli_r7rs__eof-object_);
* [`file-exists?`](#definition__vonuvoli_r7rs__file-exists_);
* [`delete-file`](#definition__vonuvoli_r7rs__delete-file);
* [`exit`](#definition__vonuvoli_r7rs__exit);
* [`emergency-exit`](#definition__vonuvoli_r7rs__emergency-exit);
* [`command-line`](#definition__vonuvoli_r7rs__command-line);
* [`get-environment-variable`](#definition__vonuvoli_r7rs__get-environment-variable);
* [`get-environment-variables`](#definition__vonuvoli_r7rs__get-environment-variables);
* [`current-second`](#definition__vonuvoli_r7rs__current-second);
* [`current-jiffy`](#definition__vonuvoli_r7rs__current-jiffy);
* [`jiffies-per-second`](#definition__vonuvoli_r7rs__jiffies-per-second);
* [`char?`](#definition__vonuvoli_r7rs__char_);
* [`char=?`](#definition__vonuvoli_r7rs__char__);
* [`char<?`](#definition__vonuvoli_r7rs__char__);
* [`char>?`](#definition__vonuvoli_r7rs__char__);
* [`char<=?`](#definition__vonuvoli_r7rs__char___);
* [`char>=?`](#definition__vonuvoli_r7rs__char___);
* [`char-ci=?`](#definition__vonuvoli_r7rs__char-ci__);
* [`char-ci<?`](#definition__vonuvoli_r7rs__char-ci__);
* [`char-ci>?`](#definition__vonuvoli_r7rs__char-ci__);
* [`char-ci<=?`](#definition__vonuvoli_r7rs__char-ci___);
* [`char-ci>=?`](#definition__vonuvoli_r7rs__char-ci___);
* [`char->integer`](#definition__vonuvoli_r7rs__char-_integer);
* [`integer->char`](#definition__vonuvoli_r7rs__integer-_char);
* [`digit-value`](#definition__vonuvoli_r7rs__digit-value);
* [`char-alphabetic?`](#definition__vonuvoli_r7rs__char-alphabetic_);
* [`char-upper-case?`](#definition__vonuvoli_r7rs__char-upper-case_);
* [`char-lower-case?`](#definition__vonuvoli_r7rs__char-lower-case_);
* [`char-numeric?`](#definition__vonuvoli_r7rs__char-numeric_);
* [`char-whitespace?`](#definition__vonuvoli_r7rs__char-whitespace_);
* [`char-upcase`](#definition__vonuvoli_r7rs__char-upcase);
* [`char-downcase`](#definition__vonuvoli_r7rs__char-downcase);
* [`char-foldcase`](#definition__vonuvoli_r7rs__char-foldcase);
* [`procedure?`](#definition__vonuvoli_r7rs__procedure_);
* [`apply`](#definition__vonuvoli_r7rs__apply);
* [`values`](#definition__vonuvoli_r7rs__values);
* [`call-with-values`](#definition__vonuvoli_r7rs__call-with-values);
* [`error-object?`](#definition__vonuvoli_r7rs__error-object_);
* [`read-error?`](#definition__vonuvoli_r7rs__read-error_);
* [`file-error?`](#definition__vonuvoli_r7rs__file-error_);
* [`error`](#definition__vonuvoli_r7rs__error);
* [`error-object-message`](#definition__vonuvoli_r7rs__error-object-message);
* [`error-object-irritants`](#definition__vonuvoli_r7rs__error-object-irritants);
* [`guard`](#definition__vonuvoli_r7rs__guard);
* [`with-exception-handler`](#definition__vonuvoli_r7rs__with-exception-handler);
* [`raise`](#definition__vonuvoli_r7rs__raise);
* [`raise-continuable`](#definition__vonuvoli_r7rs__raise-continuable);
* [`parameterize`](#definition__vonuvoli_r7rs__parameterize);
* [`make-parameter`](#definition__vonuvoli_r7rs__make-parameter);
* [`current-input-port`](#definition__vonuvoli_r7rs__current-input-port);
* [`current-output-port`](#definition__vonuvoli_r7rs__current-output-port);
* [`current-error-port`](#definition__vonuvoli_r7rs__current-error-port);
* [`with-input-from-file`](#definition__vonuvoli_r7rs__with-input-from-file);
* [`with-output-from-file`](#definition__vonuvoli_r7rs__with-output-from-file);
* [`delay`](#definition__vonuvoli_r7rs__delay);
* [`delay-force`](#definition__vonuvoli_r7rs__delay-force);
* [`promise?`](#definition__vonuvoli_r7rs__promise_);
* [`make-promise`](#definition__vonuvoli_r7rs__make-promise);
* [`force`](#definition__vonuvoli_r7rs__force);
* [`eval`](#definition__vonuvoli_r7rs__eval);
* [`environment`](#definition__vonuvoli_r7rs__environment);
* [`interaction-environment`](#definition__vonuvoli_r7rs__interaction-environment);
* [`scheme-report-environment`](#definition__vonuvoli_r7rs__scheme-report-environment);
* [`null-environment`](#definition__vonuvoli_r7rs__null-environment);
* [`call-with-current-continuation`](#definition__vonuvoli_r7rs__call-with-current-continuation);
* [`dynamic-wind`](#definition__vonuvoli_r7rs__dynamic-wind);
* [`cond-expand`](#definition__vonuvoli_r7rs__cond-expand);
* [`features`](#definition__vonuvoli_r7rs__features);
* [`include`](#definition__vonuvoli_r7rs__include);
* [`include-ci`](#definition__vonuvoli_r7rs__include-ci);
* [`import`](#definition__vonuvoli_r7rs__import);
* [`load`](#definition__vonuvoli_r7rs__load);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__define-syntax'>

### Definition `define-syntax`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `keyword`: identifier;

Syntax variants:
 * `(|_| |keyword| |@syntax-transformer|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__let-syntax'>

### Definition `let-syntax`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `keyword`: identifier;
 * `syntaxes`: pattern with variants:
   * `()`;
   * `((|keyword| |@syntax-transformer|) |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |syntaxes|)`
 * `(|_| |syntaxes| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__letrec-syntax'>

### Definition `letrec-syntax`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `keyword`: identifier;
 * `syntaxes`: pattern with variants:
   * `()`;
   * `((|keyword| |@syntax-transformer|) |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |syntaxes|)`
 * `(|_| |syntaxes| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__syntax-rules'>

### Definition `syntax-rules`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__syntax-error'>

### Definition `syntax-error`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `message`: value of type [string](#value_kind__vonuvoli_r7rs__string);
 * `argument`: value of type [any](#value_kind__vonuvoli_r7rs__any);

Syntax variants:
 * `(|_| |message|)`
 * `(|_| |message| |argument| |...|)`


#### Referenced types

 * [`string`](#value_kind__vonuvoli_r7rs__string);
 * [`any`](#value_kind__vonuvoli_r7rs__any);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs___'>

### Definition `_`

Has the following kind: `auxiliary-syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs_____'>

### Definition `...`

Has the following kind: `auxiliary-syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs____'>

### Definition `=>`

Has the following kind: `auxiliary-syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__else'>

### Definition `else`

Has the following kind: `auxiliary-syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__quote'>

### Definition `quote`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:quotation`](#category__vonuvoli_r7rs__vs_quotation);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `token`: value of type [any](#value_kind__vonuvoli_r7rs__any);

Syntax variants:
 * `(|_| |token|)`


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__quasiquote'>

### Definition `quasiquote`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:quotation`](#category__vonuvoli_r7rs__vs_quotation);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `token`: value of type [any](#value_kind__vonuvoli_r7rs__any);

Syntax variants:
 * `(|_| |token|)`


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__unquote'>

### Definition `unquote`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:quotation`](#category__vonuvoli_r7rs__vs_quotation);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `token`: value of type [any](#value_kind__vonuvoli_r7rs__any);

Syntax variants:
 * `(|_| |token|)`


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__unquote-splicing'>

### Definition `unquote-splicing`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:syntaxes`](#category__vonuvoli_r7rs__vs_syntaxes);
 * [`vs:quotation`](#category__vonuvoli_r7rs__vs_quotation);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `token`: value of type [any](#value_kind__vonuvoli_r7rs__any);

Syntax variants:
 * `(|_| |token|)`


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__lambda'>

### Definition `lambda`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lambda`](#category__vonuvoli_r7rs__vs_lambda);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `argument`: identifier;
 * `argument-rest`: identifier;
 * `arguments`: pattern with variants:
   * `()`;
   * `(|argument| |...|)`;
   * `(|argument| |...| . |argument-rest|)`;
   * `|argument-rest|`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |arguments| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__case-lambda'>

### Definition `case-lambda`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:case-lambda`](#category__vonuvoli_r7rs__r7rs_case-lambda);
 * [`vs:lambda`](#category__vonuvoli_r7rs__vs_lambda);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `argument`: identifier;
 * `argument-rest`: identifier;
 * `arguments`: pattern with variants:
   * `()`;
   * `(|argument| |...|)`;
   * `(|argument| |...| . |argument-rest|)`;
   * `|argument-rest|`;
 * `expression`: expression;

Syntax variants:
 * `(|_| (|arguments| |expression|) |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__define'>

### Definition `define`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `argument`: identifier;
 * `argument-rest`: identifier;
 * `expression`: expression;

Syntax variants:
 * `(|_| |variable| |expression|)`
 * `(|_| (|variable|) |expression| |...|)`
 * `(|_| (|variable| |argument| |...|) |expression| |...|)`
 * `(|_| (|variable| |argument| |...| . |argument-rest|) |expression| |...|)`
 * `(|_| (|variable| . |argument-rest|) |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__let'>

### Definition `let`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `function`: identifier;
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `(|variable| |initializer|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |bindings|)`
 * `(|_| |bindings| |expression| |...|)`
 * `(|_| |function| |bindings| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__let_'>

### Definition `let*`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `(|variable| |initializer|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |bindings|)`
 * `(|_| |bindings| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__letrec'>

### Definition `letrec`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `(|variable| |initializer|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |bindings|)`
 * `(|_| |bindings| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__letrec_'>

### Definition `letrec*`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `(|variable| |initializer|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |bindings|)`
 * `(|_| |bindings| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__set!'>

### Definition `set!`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `expression`: expression;

Syntax variants:
 * `(|_| |variable| |expression|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__define-values'>

### Definition `define-values`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);
 * [`vs:values`](#category__vonuvoli_r7rs__vs_values);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `expression`: expression;

Syntax variants:
 * `(|_| (|variable| |...|) |expression|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__let-values'>

### Definition `let-values`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);
 * [`vs:values`](#category__vonuvoli_r7rs__vs_values);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `((|variable| |...|) |initializer|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |bindings|)`
 * `(|_| |bindings| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__let_-values'>

### Definition `let*-values`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);
 * [`vs:values`](#category__vonuvoli_r7rs__vs_values);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `((|variable| |...|) |initializer|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |bindings|)`
 * `(|_| |bindings| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__define-record-type'>

### Definition `define-record-type`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:contexts`](#category__vonuvoli_r7rs__vs_contexts);
 * [`vs:records`](#category__vonuvoli_r7rs__vs_records);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `type-identifier`: identifier;
 * `constructor-identifier`: identifier;
 * `predicate-identifier`: identifier;
 * `field-identifier`: identifier;
 * `field-accessor-identifier`: identifier;
 * `field-mutator-identifier`: identifier;
 * `constructor-descriptor`: pattern with variants:
   * `|constructor-identifier|`;
   * `(|constructor-identifier| |field-identifier| |...|)`;
 * `field-descriptor`: pattern with variants:
   * `(|field-identifier| |field-accessor-identifier|)`;
   * `(|field-identifier| |field-accessor-identifier| |field-mutator-identifier|)`;

Syntax variants:
 * `(|_| |type-identifier| |constructor-descriptor| |predicate-identifier| |field-descriptor| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__begin'>

### Definition `begin`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(|_|)`
 * `(|_| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__and'>

### Definition `and`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(|_|)`
 * `(|_| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__or'>

### Definition `or`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(|_|)`
 * `(|_| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__if'>

### Definition `if`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `condition`: expression;
 * `then-expression`: expression;
 * `else-expression`: expression;

Syntax variants:
 * `(|_| |condition| |then-expression|)`
 * `(|_| |condition| |then-expression| |else-expression|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__unless'>

### Definition `unless`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `condition`: expression;
 * `then-expression`: expression;

Syntax variants:
 * `(|_| |condition| |then-expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__when'>

### Definition `when`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `condition`: expression;
 * `then-expression`: expression;

Syntax variants:
 * `(|_| |condition| |then-expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cond'>

### Definition `cond`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `else`: literal;
 * `condition`: expression;
 * `then-expression`: expression;
 * `clause`: pattern with variants:
   * `(|condition|)`;
   * `(|condition| |then-expression| |...|)`;
   * `(|else|)`;
   * `(|else| |then-expression| |...|)`;

Syntax variants:
 * `(|_|)`
 * `(|_| |clause| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__case'>

### Definition `case`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `else`: literal;
 * `value`: expression;
 * `variant`: value of type [any](#value_kind__vonuvoli_r7rs__any);
 * `then-expression`: expression;
 * `clause`: pattern with variants:
   * `((|variant| |...|))`;
   * `((|variant| |...|) |then-expression| |...|)`;
   * `(|else|)`;
   * `(|else| |then-expression| |...|)`;

Syntax variants:
 * `(|_| |value|)`
 * `(|_| |value| |clause| |...|)`


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__do'>

### Definition `do`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:control`](#category__vonuvoli_r7rs__vs_control);
 * [`vs:loops`](#category__vonuvoli_r7rs__vs_loops);


#### Description

> **FIXME!**


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

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__eq_'>

### Definition `eq?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> The `eq?` procedure is similar to `eqv?` except that in some cases it is
> capable of discerning distinctions finer than those detectable by
> `eqv?`.  It must always return `#f` when `eqv?` also
> would, but may return `#f` in some cases where `eqv?` would return `#t`.
> 
> On symbols, booleans, the empty list, pairs, and records,
> and also on non-empty
> strings, vectors, and bytevectors, `eq?` and `eqv?` are guaranteed to have the same
> behavior.  On procedures, `eq?` must return true if the arguments' location
> tags are equal.  On numbers and characters, `eq?`'s behavior is
> implementation-dependent, but it will always return either true or
> false.  On empty strings, empty vectors, and empty bytevectors, `eq?` may also behave
> differently from `eqv?`.
> 
> 
> ````
> (eq? 'a 'a)                     ===>  #t
> (eq? '(a) '(a))                 ===>  #unspecified
> (eq? (list 'a) (list 'a))       ===>  #f
> (eq? "a" "a")                   ===>  #unspecified
> (eq? "" "")                     ===>  #unspecified
> (eq? '() '())                   ===>  #t
> (eq? 2 2)                       ===>  #unspecified
> (eq? #\A #\A)                   ===>  #unspecified
> (eq? car car)                   ===>  #t
> (let ((n (+ 2 3)))
>   (eq? n n))                    ===>  #unspecified
> (let ((x '(a)))
>   (eq? x x))                    ===>  #t
> (let ((x '#()))
>   (eq? x x))                    ===>  #t
> (let ((p (lambda (x) x)))
>   (eq? p p))                    ===>  #t
> ````
> 
> 
> **Rationale**:  It will usually be possible to implement `eq?` much
> more efficiently than `eqv?`, for example, as a simple pointer
> comparison instead of as some more complicated operation.  One reason is
> that it is not always possible to compute `eqv?` of two numbers in
> constant time, whereas `eq?` implemented as pointer comparison will
> always finish in constant time.


#### Procedure signature


Procedure variants:
 * `((|any| |...|) |->| (|boolean|))`
   * inputs:
     * a value type [`any`](#value_kind__vonuvoli_r7rs__any);
     * `...` (i.e. variadic);
   * outputs:
     * a value type [`boolean`](#value_kind__vonuvoli_r7rs__boolean);


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__eqv_'>

### Definition `eqv?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> The `eqv?` procedure defines a useful equivalence relation on objects.
> Briefly, it returns `#t` if `obj-1` and `obj-2` are
> normally regarded as the same object.  This relation is left slightly
> open to interpretation, but the following partial specification of
> `eqv?` holds for all implementations of Scheme.
> 
> 
> The `eqv?` procedure returns `#t` if:
> 
>   * `obj-1` and `obj-2` are both `#t` or both `#f`.
> 
>   * `obj-1` and `obj-2` are both symbols and are the same
> symbol according to the `symbol=?` procedure.
> 
>   * `obj-1` and `obj-2` are both exact numbers and
> are numerically equal (in the sense of `=`).
> 
>   * `obj-1` and `obj-2` are both inexact numbers such that
> they are numerically equal (in the sense of `=`)
> and they yield the same results (in the sense of `eqv?`)
> when passed as arguments to any other procedure
> that can be defined as a finite composition of Scheme's standard
> arithmetic procedures, provided it does not result in a `NaN` value.
> 
>   * `obj-1` and `obj-2` are both characters and are the same
> character according to the `char=?` procedure.
> 
>   * `obj-1` and `obj-2` are both the empty list.
> 
>   * `obj-1` and `obj-2` are pairs, vectors, bytevectors, records,
> or strings that denote the same location in the store.
> 
>   * `obj-1` and `obj-2` are procedures whose location tags are
> equal.
> 
> 
> The `eqv?` procedure returns `#f` if:
> 
>   * `obj-1` and `obj-2` are of different types.
> 
>   * one of `obj-1` and `obj-2` is `#t` but the other is
> `#f`.
> 
>   * `obj-1` and `obj-2` are symbols but are not the same
> symbol according to the `symbol=?` procedure.
> 
>   * one of `obj-1` and `obj-2` is an exact number but the other
> is an inexact number.
> 
>   * `obj-1` and `obj-2` are both exact numbers and
> are numerically unequal (in the sense of `=`).
> 
>   * `obj-1` and `obj-2` are both inexact numbers such that either
> they are numerically unequal (in the sense of `=`),
> or they do not yield the same results (in the sense of `eqv?`)
> when passed as arguments to any other procedure
> that can be defined as a finite composition of Scheme's standard
> arithmetic procedures, provided it does not result in a `NaN` value.
> As an exception, the behavior of `eqv?` is unspecified
> when both `obj-1` and `obj-2` are `NaN`.
> 
>   * `obj-1` and `obj-2` are characters for which the `char=?`
> procedure returns `#f`.
> 
>   * one of `obj-1` and `obj-2` is the empty list but the other
> is not.
> 
>   * `obj-1` and `obj-2` are pairs, vectors, bytevectors, records,
> or strings that denote distinct locations.
> 
>   * `obj-1` and `obj-2` are procedures that would behave differently
> (return different values or have different side effects) for some arguments.
> 
> 
> ````
> (eqv? 'a 'a)                     ===>  #t
> (eqv? 'a 'b)                     ===>  #f
> (eqv? 2 2)                       ===>  #t
> (eqv? 2 2.0)                     ===>  #f
> (eqv? '() '())                   ===>  #t
> (eqv? 100000000 100000000)       ===>  #t
> (eqv? 0.0 +nan.0)                ===>  #f
> (eqv? (cons 1 2) (cons 1 2))     ===>  #f
> (eqv? (lambda () 1)
>       (lambda () 2))             ===>  #f
> (let ((p (lambda (x) x)))
>   (eqv? p p))                    ===>  #t
> (eqv? #f 'nil)                   ===>  #f
> ````
> 
> 
> The following examples illustrate cases in which the above rules do
> not fully specify the behavior of `eqv?`.  All that can be said
> about such cases is that the value returned by `eqv?` must be a
> boolean.
> 
> ````
> (eqv? "" "")             ===>  #unspecified
> (eqv? '#() '#())         ===>  #unspecified
> (eqv? (lambda (x) x)
>       (lambda (x) x))    ===>  #unspecified
> (eqv? (lambda (x) x)
>       (lambda (y) y))    ===>  #unspecified
> (eqv? 1.0e0 1.0f0)       ===>  #unspecified
> (eqv? +nan.0 +nan.0)     ===>  #unspecified
> ````
> 
> Note that `(eqv? 0.0 -0.0)` will return `#f` if negative zero
> is distinguished, and `#t` if negative zero is not distinguished.
> 
> 
> Since it is an error to modify constant objects (those returned by
> literal expressions), implementations may
> share structure between constants where appropriate.  Thus
> the value of `eqv?` on constants is sometimes
> implementation-dependent.
> 
> ````
> (eqv? '(a) '(a))                 ===>  #unspecified
> (eqv? "a" "a")                   ===>  #unspecified
> (eqv? '(b) (cdr '(a b)))         ===>  #unspecified
> (let ((x '(a)))
>   (eqv? x x))                    ===>  #t
> ````
> 
> 
> The above definition of `eqv?` allows implementations latitude in
> their treatment of procedures and literals:  implementations may
> either detect or fail to detect that two procedures or two literals
> are equivalent to each other, and can decide whether or not to
> merge representations of equivalent objects by using the same pointer or
> bit pattern to represent both.
> 
> **Note**:  If inexact numbers are represented as IEEE binary floating-point numbers,
> then an implementation of `eqv?` that simply compares equal-sized
> inexact numbers for bitwise equality is correct by the above definition.


#### Procedure signature


Procedure variants:
 * `((|any| |...|) |->| (|boolean|))`
   * inputs:
     * a value type [`any`](#value_kind__vonuvoli_r7rs__any);
     * `...` (i.e. variadic);
   * outputs:
     * a value type [`boolean`](#value_kind__vonuvoli_r7rs__boolean);


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__equal_'>

### Definition `equal?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> The `equal?` procedure, when applied to pairs, vectors, strings and
> bytevectors, recursively compares them, returning `#t` when the
> unfoldings of its arguments into (possibly infinite) trees are equal
> (in the sense of `equal?`)
> as ordered trees, and `#f` otherwise.  It returns the same as
> `eqv?` when applied to booleans, symbols, numbers, characters,
> ports, procedures, and the empty list.  If two objects are `eqv?`,
> they must be `equal?` as well.  In all other cases, `equal?`
> may return either `#t` or `#f`.
> 
> Note that records are `equal?` if their record types are the same
> and their correspondingly named fields are `equal?`.
> 
> Even if its arguments are
> circular data structures, `equal?` must always terminate.
> 
> 
> ````
> (equal? 'a 'a)                  ===>  #t
> (equal? '(a) '(a))              ===>  #t
> (equal? '(a (b) c)
>         '(a (b) c))             ===>  #t
> (equal? "abc" "abc")            ===>  #t
> (equal? 2 2)                    ===>  #t
> (equal? (make-vector 5 'a)
>         (make-vector 5 'a))     ===>  #t
> (equal? '#1=(a b . #1#)
>         '#2=(a b a b . #2#))    ===>  #t
> (equal? (lambda (x) x)
>         (lambda (y) y))         ===>  #unspecified
> ````
> 
> 
> **Note**:  A rule of thumb is that objects are generally `equal?` if they print
> the same.


#### Procedure signature


Procedure variants:
 * `((|any| |...|) |->| (|boolean|))`
   * inputs:
     * a value type [`any`](#value_kind__vonuvoli_r7rs__any);
     * `...` (i.e. variadic);
   * outputs:
     * a value type [`boolean`](#value_kind__vonuvoli_r7rs__boolean);


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__not'>

### Definition `not`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);


#### Description

> **FIXME!**


#### Procedure signature


Procedure variants:
 * `((|any|) |->| (|boolean|))`
   * inputs:
     * a value type [`any`](#value_kind__vonuvoli_r7rs__any);
   * outputs:
     * a value type [`boolean`](#value_kind__vonuvoli_r7rs__boolean);


#### Referenced types

 * [`any`](#value_kind__vonuvoli_r7rs__any);
 * [`boolean`](#value_kind__vonuvoli_r7rs__boolean);

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__boolean_'>

### Definition `boolean?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:booleans`](#category__vonuvoli_r7rs__vs_booleans);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__boolean__'>

### Definition `boolean=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:booleans`](#category__vonuvoli_r7rs__vs_booleans);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__symbol_'>

### Definition `symbol?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:symbols`](#category__vonuvoli_r7rs__vs_symbols);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__symbol__'>

### Definition `symbol=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:symbols`](#category__vonuvoli_r7rs__vs_symbols);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__number_'>

### Definition `number?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__integer_'>

### Definition `integer?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__real_'>

### Definition `real?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__rational_'>

### Definition `rational?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__complex_'>

### Definition `complex?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__exact_'>

### Definition `exact?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__inexact_'>

### Definition `inexact?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__exact-integer_'>

### Definition `exact-integer?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__zero_'>

### Definition `zero?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__positive_'>

### Definition `positive?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__odd_'>

### Definition `odd?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__even_'>

### Definition `even?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs___'>

### Definition `=`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs___'>

### Definition `<`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs___'>

### Definition `>`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs____'>

### Definition `<=`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs____'>

### Definition `>=`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs___'>

### Definition `+`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__-'>

### Definition `-`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs___'>

### Definition `*`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs___'>

### Definition `/`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__abs'>

### Definition `abs`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__floor_'>

### Definition `floor/`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__floor-quotient'>

### Definition `floor-quotient`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__floor-remainder'>

### Definition `floor-remainder`

Has the following kind: `procedure`.

With the following aliases:
 * `modulo`;

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__truncate_'>

### Definition `truncate/`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__truncate-quotient'>

### Definition `truncate-quotient`

Has the following kind: `procedure`.

With the following aliases:
 * `quotient`;

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__truncate-remainder'>

### Definition `truncate-remainder`

Has the following kind: `procedure`.

With the following aliases:
 * `remainder`;

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__floor'>

### Definition `floor`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__ceiling'>

### Definition `ceiling`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__truncate'>

### Definition `truncate`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__round'>

### Definition `round`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__min'>

### Definition `min`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__max'>

### Definition `max`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__gcd'>

### Definition `gcd`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__lcm'>

### Definition `lcm`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__expt'>

### Definition `expt`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__square'>

### Definition `square`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__exact-integer-sqrt'>

### Definition `exact-integer-sqrt`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__rationalize'>

### Definition `rationalize`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__numerator'>

### Definition `numerator`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__denominator'>

### Definition `denominator`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__inexact'>

### Definition `inexact`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__exact'>

### Definition `exact`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__make-rectangular'>

### Definition `make-rectangular`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__real-part'>

### Definition `real-part`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__imag-part'>

### Definition `imag-part`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__make-polar'>

### Definition `make-polar`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__magnitude'>

### Definition `magnitude`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__angle'>

### Definition `angle`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:complex`](#category__vonuvoli_r7rs__r7rs_complex);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__sqrt'>

### Definition `sqrt`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__exp'>

### Definition `exp`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__log'>

### Definition `log`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__sin'>

### Definition `sin`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cos'>

### Definition `cos`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__tan'>

### Definition `tan`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__asin'>

### Definition `asin`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__acos'>

### Definition `acos`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__atan'>

### Definition `atan`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__finite_'>

### Definition `finite?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__infinite_'>

### Definition `infinite?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__nan_'>

### Definition `nan?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:inexact`](#category__vonuvoli_r7rs__r7rs_inexact);
 * [`vs:arithmetic`](#category__vonuvoli_r7rs__vs_arithmetic);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__pair_'>

### Definition `pair?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cons'>

### Definition `cons`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__car'>

### Definition `car`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdr'>

### Definition `cdr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__set-car!'>

### Definition `set-car!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__set-cdr!'>

### Definition `set-cdr!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caar'>

### Definition `caar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cadr'>

### Definition `cadr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdar'>

### Definition `cdar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cddr'>

### Definition `cddr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caaar'>

### Definition `caaar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caadr'>

### Definition `caadr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cadar'>

### Definition `cadar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caddr'>

### Definition `caddr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdaar'>

### Definition `cdaar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdadr'>

### Definition `cdadr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cddar'>

### Definition `cddar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdddr'>

### Definition `cdddr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caaaar'>

### Definition `caaaar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caaadr'>

### Definition `caaadr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caadar'>

### Definition `caadar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caaddr'>

### Definition `caaddr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cadaar'>

### Definition `cadaar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cadadr'>

### Definition `cadadr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__caddar'>

### Definition `caddar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cadddr'>

### Definition `cadddr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdaaar'>

### Definition `cdaaar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdaadr'>

### Definition `cdaadr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdadar'>

### Definition `cdadar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdaddr'>

### Definition `cdaddr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cddaar'>

### Definition `cddaar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cddadr'>

### Definition `cddadr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cdddar'>

### Definition `cdddar`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cddddr'>

### Definition `cddddr`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:cxr`](#category__vonuvoli_r7rs__r7rs_cxr);
 * [`vs:pairs`](#category__vonuvoli_r7rs__vs_pairs);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__null_'>

### Definition `null?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__list_'>

### Definition `list?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__list'>

### Definition `list`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__make-list'>

### Definition `make-list`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__length'>

### Definition `length`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__append'>

### Definition `append`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__list-copy'>

### Definition `list-copy`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__reverse'>

### Definition `reverse`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__list-ref'>

### Definition `list-ref`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__list-tail'>

### Definition `list-tail`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__list-set!'>

### Definition `list-set!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__map'>

### Definition `map`

Has the following kind: `functor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);
 * [`vs:loops`](#category__vonuvoli_r7rs__vs_loops);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__for-each'>

### Definition `for-each`

Has the following kind: `functor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:loops`](#category__vonuvoli_r7rs__vs_loops);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__member'>

### Definition `member`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__memq'>

### Definition `memq`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__memv'>

### Definition `memv`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__assoc'>

### Definition `assoc`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:associations`](#category__vonuvoli_r7rs__vs_associations);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__assqc'>

### Definition `assqc`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:associations`](#category__vonuvoli_r7rs__vs_associations);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__assvc'>

### Definition `assvc`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:associations`](#category__vonuvoli_r7rs__vs_associations);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector_'>

### Definition `vector?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector'>

### Definition `vector`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__make-vector'>

### Definition `make-vector`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-length'>

### Definition `vector-length`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-append'>

### Definition `vector-append`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-copy'>

### Definition `vector-copy`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-copy!'>

### Definition `vector-copy!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-fill!'>

### Definition `vector-fill!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-ref'>

### Definition `vector-ref`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-set!'>

### Definition `vector-set!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-_list'>

### Definition `vector->list`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__list-_vector'>

### Definition `list->vector`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-map'>

### Definition `vector-map`

Has the following kind: `functor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);
 * [`vs:loops`](#category__vonuvoli_r7rs__vs_loops);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-for-each'>

### Definition `vector-for-each`

Has the following kind: `functor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:loops`](#category__vonuvoli_r7rs__vs_loops);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string_'>

### Definition `string?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string'>

### Definition `string`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__make-string'>

### Definition `make-string`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-length'>

### Definition `string-length`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-append'>

### Definition `string-append`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-copy'>

### Definition `string-copy`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-copy!'>

### Definition `string-copy!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-fill!'>

### Definition `string-fill!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__substring'>

### Definition `substring`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-ref'>

### Definition `string-ref`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-set!'>

### Definition `string-set!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string__'>

### Definition `string=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string__'>

### Definition `string<?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string__'>

### Definition `string>?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string___'>

### Definition `string<=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string___'>

### Definition `string>=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-ci__'>

### Definition `string-ci=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-ci__'>

### Definition `string-ci<?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-ci__'>

### Definition `string-ci>?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-ci___'>

### Definition `string-ci<=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-ci___'>

### Definition `string-ci>=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__number-_string'>

### Definition `number->string`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-_number'>

### Definition `string->number`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__symbol-_string'>

### Definition `symbol->string`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:symbols`](#category__vonuvoli_r7rs__vs_symbols);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-_symbol'>

### Definition `string->symbol`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:symbols`](#category__vonuvoli_r7rs__vs_symbols);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__list-_string'>

### Definition `list->string`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-_list'>

### Definition `string->list`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:lists`](#category__vonuvoli_r7rs__vs_lists);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__vector-_string'>

### Definition `vector->string`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-_vector'>

### Definition `string->vector`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:vectors`](#category__vonuvoli_r7rs__vs_vectors);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-map'>

### Definition `string-map`

Has the following kind: `functor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);
 * [`vs:loops`](#category__vonuvoli_r7rs__vs_loops);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-for-each'>

### Definition `string-for-each`

Has the following kind: `functor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:loops`](#category__vonuvoli_r7rs__vs_loops);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-upcase'>

### Definition `string-upcase`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-downcase'>

### Definition `string-downcase`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-foldcase'>

### Definition `string-foldcase`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:conversions`](#category__vonuvoli_r7rs__vs_conversions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__bytevector_'>

### Definition `bytevector?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__bytevector'>

### Definition `bytevector`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__make-bytevector'>

### Definition `make-bytevector`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__bytevector-length'>

### Definition `bytevector-length`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__bytevector-append'>

### Definition `bytevector-append`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__bytevector-copy'>

### Definition `bytevector-copy`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__bytevector-copy!'>

### Definition `bytevector-copy!`

Has the following kind: `procedure!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__bytevector-u8-ref'>

### Definition `bytevector-u8-ref`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__bytevector-u8-set!'>

### Definition `bytevector-u8-set!`

Has the following kind: `mutator!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__utf8-_string'>

### Definition `utf8->string`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__string-_utf8'>

### Definition `string->utf8`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__port_'>

### Definition `port?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports`](#category__vonuvoli_r7rs__vs_ports);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__binary-port_'>

### Definition `binary-port?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports`](#category__vonuvoli_r7rs__vs_ports);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__textual-port_'>

### Definition `textual-port?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports`](#category__vonuvoli_r7rs__vs_ports);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__input-port_'>

### Definition `input-port?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__input-port-open_'>

### Definition `input-port-open?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__output-port_'>

### Definition `output-port?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__output-port-open_'>

### Definition `output-port-open?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__open-input-bytevector'>

### Definition `open-input-bytevector`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__open-output-bytevector'>

### Definition `open-output-bytevector`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__get-output-bytevector'>

### Definition `get-output-bytevector`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__open-input-string'>

### Definition `open-input-string`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__open-output-string'>

### Definition `open-output-string`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__get-output-string'>

### Definition `get-output-string`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__close-port'>

### Definition `close-port`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports`](#category__vonuvoli_r7rs__vs_ports);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__close-input-port'>

### Definition `close-input-port`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__close-output-port'>

### Definition `close-output-port`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__u8-ready_'>

### Definition `u8-ready?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__peek-u8'>

### Definition `peek-u8`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__read-u8'>

### Definition `read-u8`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__write-u8'>

### Definition `write-u8`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__read-bytevector'>

### Definition `read-bytevector`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__read-bytevector!'>

### Definition `read-bytevector!`

Has the following kind: `procedure!`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__write-bytevector'>

### Definition `write-bytevector`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-ready_'>

### Definition `char-ready?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__peek-char'>

### Definition `peek-char`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__read-char'>

### Definition `read-char`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__write-char'>

### Definition `write-char`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__read-string'>

### Definition `read-string`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__read-line'>

### Definition `read-line`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__newline'>

### Definition `newline`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:bytes`](#category__vonuvoli_r7rs__vs_bytes);
 * [`vs:strings`](#category__vonuvoli_r7rs__vs_strings);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__flush-output-port'>

### Definition `flush-output-port`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__read'>

### Definition `read`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:read`](#category__vonuvoli_r7rs__r7rs_read);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:ports:values`](#category__vonuvoli_r7rs__vs_ports_values);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__write'>

### Definition `write`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:write`](#category__vonuvoli_r7rs__r7rs_write);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:values`](#category__vonuvoli_r7rs__vs_ports_values);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__write-simple'>

### Definition `write-simple`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:write`](#category__vonuvoli_r7rs__r7rs_write);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:values`](#category__vonuvoli_r7rs__vs_ports_values);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__write-shared'>

### Definition `write-shared`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:write`](#category__vonuvoli_r7rs__r7rs_write);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:values`](#category__vonuvoli_r7rs__vs_ports_values);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__display'>

### Definition `display`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:write`](#category__vonuvoli_r7rs__r7rs_write);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:values`](#category__vonuvoli_r7rs__vs_ports_values);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__open-input-file'>

### Definition `open-input-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__open-binary-input-file'>

### Definition `open-binary-input-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__open-output-file'>

### Definition `open-output-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__open-binary-output-file'>

### Definition `open-binary-output-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:ports:open`](#category__vonuvoli_r7rs__vs_ports_open);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__call-with-port'>

### Definition `call-with-port`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports`](#category__vonuvoli_r7rs__vs_ports);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__call-with-input-file'>

### Definition `call-with-input-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:ports:input`](#category__vonuvoli_r7rs__vs_ports_input);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__call-with-output-file'>

### Definition `call-with-output-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:ports:output`](#category__vonuvoli_r7rs__vs_ports_output);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__eof-object'>

### Definition `eof-object`

Has the following kind: `constant`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports`](#category__vonuvoli_r7rs__vs_ports);
 * [`vs:globals`](#category__vonuvoli_r7rs__vs_globals);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__eof-object_'>

### Definition `eof-object?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:ports`](#category__vonuvoli_r7rs__vs_ports);
 * [`vs:globals`](#category__vonuvoli_r7rs__vs_globals);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__file-exists_'>

### Definition `file-exists?`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:file-system`](#category__vonuvoli_r7rs__vs_file-system);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__delete-file'>

### Definition `delete-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:file-system`](#category__vonuvoli_r7rs__vs_file-system);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__exit'>

### Definition `exit`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:process-context`](#category__vonuvoli_r7rs__r7rs_process-context);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__emergency-exit'>

### Definition `emergency-exit`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:process-context`](#category__vonuvoli_r7rs__r7rs_process-context);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__command-line'>

### Definition `command-line`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:process-context`](#category__vonuvoli_r7rs__r7rs_process-context);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__get-environment-variable'>

### Definition `get-environment-variable`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:process-context`](#category__vonuvoli_r7rs__r7rs_process-context);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__get-environment-variables'>

### Definition `get-environment-variables`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:process-context`](#category__vonuvoli_r7rs__r7rs_process-context);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__current-second'>

### Definition `current-second`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:time`](#category__vonuvoli_r7rs__r7rs_time);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__current-jiffy'>

### Definition `current-jiffy`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:time`](#category__vonuvoli_r7rs__r7rs_time);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__jiffies-per-second'>

### Definition `jiffies-per-second`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:time`](#category__vonuvoli_r7rs__r7rs_time);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char_'>

### Definition `char?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char__'>

### Definition `char=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char__'>

### Definition `char<?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char__'>

### Definition `char>?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char___'>

### Definition `char<=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char___'>

### Definition `char>=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-ci__'>

### Definition `char-ci=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);
 * [`vs:equivalence`](#category__vonuvoli_r7rs__vs_equivalence);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-ci__'>

### Definition `char-ci<?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-ci__'>

### Definition `char-ci>?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-ci___'>

### Definition `char-ci<=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-ci___'>

### Definition `char-ci>=?`

Has the following kind: `comparator`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);
 * [`vs:comparisons`](#category__vonuvoli_r7rs__vs_comparisons);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-_integer'>

### Definition `char->integer`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__integer-_char'>

### Definition `integer->char`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__digit-value'>

### Definition `digit-value`

Has the following kind: `converter`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-alphabetic_'>

### Definition `char-alphabetic?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-upper-case_'>

### Definition `char-upper-case?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-lower-case_'>

### Definition `char-lower-case?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-numeric_'>

### Definition `char-numeric?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-whitespace_'>

### Definition `char-whitespace?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-upcase'>

### Definition `char-upcase`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-downcase'>

### Definition `char-downcase`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__char-foldcase'>

### Definition `char-foldcase`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:char`](#category__vonuvoli_r7rs__r7rs_char);
 * [`vs:characters`](#category__vonuvoli_r7rs__vs_characters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__procedure_'>

### Definition `procedure?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:types`](#category__vonuvoli_r7rs__vs_types);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__apply'>

### Definition `apply`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__values'>

### Definition `values`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:values`](#category__vonuvoli_r7rs__vs_values);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__call-with-values'>

### Definition `call-with-values`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);
 * [`vs:values`](#category__vonuvoli_r7rs__vs_values);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__error-object_'>

### Definition `error-object?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__read-error_'>

### Definition `read-error?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__file-error_'>

### Definition `file-error?`

Has the following kind: `predicate`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__error'>

### Definition `error`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__error-object-message'>

### Definition `error-object-message`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__error-object-irritants'>

### Definition `error-object-irritants`

Has the following kind: `accessor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__guard'>

### Definition `guard`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `variable`: identifier;
 * `else`: literal;
 * `clause-condition`: expression;
 * `clause-expression`: expression;
 * `clause`: pattern with variants:
   * `(|clause-condition|)`;
   * `(|clause-condition| |clause-expression| |...|)`;
   * `(|else| |clause-expression| |...|)`;
 * `guarded-expression`: expression;

Syntax variants:
 * `(|_| (|variable| |clause| |...|) |guarded-expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__with-exception-handler'>

### Definition `with-exception-handler`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__raise'>

### Definition `raise`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__raise-continuable'>

### Definition `raise-continuable`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:errors`](#category__vonuvoli_r7rs__vs_errors);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__parameterize'>

### Definition `parameterize`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `parameter`: expression;
 * `initializer`: expression;
 * `parameters`: pattern with variants:
   * `()`;
   * `((|parameter| |initializer|) |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |parameters|)`
 * `(|_| |parameters| |expression| |...|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__make-parameter'>

### Definition `make-parameter`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__current-input-port'>

### Definition `current-input-port`

Has the following kind: `parameter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__current-output-port'>

### Definition `current-output-port`

Has the following kind: `parameter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__current-error-port'>

### Definition `current-error-port`

Has the following kind: `parameter`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__with-input-from-file'>

### Definition `with-input-from-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__with-output-from-file'>

### Definition `with-output-from-file`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:file`](#category__vonuvoli_r7rs__r7rs_file);
 * [`vs:parameters`](#category__vonuvoli_r7rs__vs_parameters);
 * [`vs:functions`](#category__vonuvoli_r7rs__vs_functions);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__delay'>

### Definition `delay`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:lazy`](#category__vonuvoli_r7rs__r7rs_lazy);
 * [`vs:promises`](#category__vonuvoli_r7rs__vs_promises);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(|_| |expression|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__delay-force'>

### Definition `delay-force`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:lazy`](#category__vonuvoli_r7rs__r7rs_lazy);
 * [`vs:promises`](#category__vonuvoli_r7rs__vs_promises);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);


#### Description

> **FIXME!**


#### Syntax signature


Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(|_| |expression|)`

Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__promise_'>

### Definition `promise?`

Has the following kind: `type-predicate`.

Belongs to the following categories:
 * [`r7rs:lazy`](#category__vonuvoli_r7rs__r7rs_lazy);
 * [`vs:promises`](#category__vonuvoli_r7rs__vs_promises);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__make-promise'>

### Definition `make-promise`

Has the following kind: `constructor`.

Belongs to the following categories:
 * [`r7rs:lazy`](#category__vonuvoli_r7rs__r7rs_lazy);
 * [`vs:promises`](#category__vonuvoli_r7rs__vs_promises);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__force'>

### Definition `force`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:lazy`](#category__vonuvoli_r7rs__r7rs_lazy);
 * [`vs:promises`](#category__vonuvoli_r7rs__vs_promises);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__eval'>

### Definition `eval`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:eval`](#category__vonuvoli_r7rs__r7rs_eval);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__environment'>

### Definition `environment`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:eval`](#category__vonuvoli_r7rs__r7rs_eval);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__interaction-environment'>

### Definition `interaction-environment`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:r5rs`](#category__vonuvoli_r7rs__r7rs_r5rs);
 * [`r7rs:repl`](#category__vonuvoli_r7rs__r7rs_repl);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__scheme-report-environment'>

### Definition `scheme-report-environment`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:r5rs`](#category__vonuvoli_r7rs__r7rs_r5rs);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__null-environment'>

### Definition `null-environment`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:r5rs`](#category__vonuvoli_r7rs__r7rs_r5rs);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__call-with-current-continuation'>

### Definition `call-with-current-continuation`

Has the following kind: `procedure`.

With the following aliases:
 * `call/cc`;

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:continuations`](#category__vonuvoli_r7rs__vs_continuations);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__dynamic-wind'>

### Definition `dynamic-wind`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:continuations`](#category__vonuvoli_r7rs__vs_continuations);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__cond-expand'>

### Definition `cond-expand`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:compiler`](#category__vonuvoli_r7rs__vs_compiler);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__features'>

### Definition `features`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:evaluator`](#category__vonuvoli_r7rs__vs_evaluator);
 * [`vs:compiler`](#category__vonuvoli_r7rs__vs_compiler);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__include'>

### Definition `include`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:compiler`](#category__vonuvoli_r7rs__vs_compiler);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__include-ci'>

### Definition `include-ci`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:compiler`](#category__vonuvoli_r7rs__vs_compiler);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__import'>

### Definition `import`

Has the following kind: `syntax`.

Belongs to the following categories:
 * [`r7rs:base`](#category__vonuvoli_r7rs__r7rs_base);
 * [`vs:compiler`](#category__vonuvoli_r7rs__vs_compiler);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----



<a id='definition__vonuvoli_r7rs__load'>

### Definition `load`

Has the following kind: `procedure`.

Belongs to the following categories:
 * [`r7rs:load`](#category__vonuvoli_r7rs__r7rs_load);
 * [`vs:compiler`](#category__vonuvoli_r7rs__vs_compiler);
 * [`vs:unsupported`](#category__vonuvoli_r7rs__vs_unsupported);


#### Description

> **FIXME!**


Goto: [library](#library__vonuvoli_r7rs), [categories](#toc__vonuvoli_r7rs__categories), [types](#toc__vonuvoli_r7rs__types), [definitions](#toc__vonuvoli_r7rs__definitions).

----

