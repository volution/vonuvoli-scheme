

(define (write-integers)
	(loop
		(write (random-i64))
		(write-char #\newline)))

(define (write-reals)
	(loop
		(write (random-f64))
		(write-char #\newline)))


(define (write-characters)
	(loop
		(write-char (random-char))
		(write-char #\newline)))

(define (write-characters-ascii)
	(loop
		(write-char (random-char-ascii))
		(write-char #\newline)))

(define (write-characters-ascii-alphabetic)
	(loop
		(write-char (random-char-ascii-alphabetic))
		(write-char #\newline)))


(define (write-bytes-1)
	(loop
		(write-u8 (random-u8))))


(define (write-bytes-buffer size)
	(define buffer (bytevector->mutable (make-bytevector size)))
	(loop
		(random-bytevector-fill! buffer)
		(write-bytevector buffer)))

(define (write-bytes-permutation)
	(define buffer (bytevector->mutable (random-bytevector-permutation)))
	(loop
		(random-bytevector-shuffle! buffer)
		(write-bytevector buffer)))




(case 'write-characters-ascii-alphabetic
	
	((write-integers) (write-integers))
	((write-reals) (write-reals))
	
	((write-characters) (write-characters))
	((write-characters-ascii) (write-characters-ascii))
	((write-characters-ascii-alphabetic) (write-characters-ascii-alphabetic))
	
	((write-bytes-1) (write-bytes-1))
	((write-bytes-buffer-1-kib) (write-bytes-buffer (* 1 1024)))
	((write-bytes-buffer-128-kib) (write-bytes-buffer (* 128 1024)))
	((write-bytes-buffer-1-mib) (write-bytes-buffer (* 1 1024 1024)))
	((write-bytes-buffer-4-mib) (write-bytes-buffer (* 4 1024 1024)))
	((write-bytes-buffer-16-mib) (write-bytes-buffer (* 16 1024 1024)))
	((write-bytes-buffer-128-mib) (write-bytes-buffer (* 128 1024 1024)))
	((write-bytes-permutation) (write-bytes-permutation))
	
	(else (error "e80849ad")))

