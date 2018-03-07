
(define (write-integers)
	(loop
		(write (random-i64))
		(write-char #\newline)))

(define (write-reals)
	(loop
		(write (random-f64))
		(write-char #\newline)))

(define (write-bytes)
	(loop
		(write-u8 (random-i64 256))))


;(write-integers)
;(write-reals)
(write-bytes)

