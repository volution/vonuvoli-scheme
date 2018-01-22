

(define (create-x n)
  (define result (make-vector n))
  (do ((i 0 (+ i 1)))
      ((>= i n) result)
    (vector-set! result i i)))

(define (create-y x)
  (let* ((n (vector-length x))
         (result (make-vector n)))
    (do ((i (- n 1) (- i 1)))
        ((< i 0) result)
      (vector-set! result i (vector-ref x i)))))

(define (try n)
  (vector-length (create-y (create-x n))))




(do
	(
		(timestamp-start (current-jiffy) timestamp-start)
		(loop 1000 (- loop 1))
	)
	((zero? loop)
		(display (round (/ (* (- (current-jiffy) timestamp-start) 1000) (jiffies-per-second))))
		(newline))
	
	(try 10000)
)

