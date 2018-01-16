

(define (ack m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        (else (ack (- m 1) (ack m (- n 1))))))




(do
	(
		(timestamp-start (current-jiffy) timestamp-start)
		(loop 1 (- loop 1))
	)
	((zero? loop)
		(display (round (/ (* (- (current-jiffy) timestamp-start) 1000) (jiffies-per-second))))
		(newline))
	
	(ack 3 8)
)

