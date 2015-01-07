
(define (assoc item l)
  (cond ((null? l) #f) 
        ((equal? item (caar l)) (car l))
        (else (assoc item (cdr l)))
  )
)

(define (equal? a b)
  (cond ((and (null? a) (null? b)) #t)
        ((or (null? a) (null? b)) #f) 
        ((or (and (symbol? a) (symbol? b)) (and (number? a) (number? b))) (eq? a b))
        ((or (symbol? a) (symbol? b)) (eq? a b))
        ((and (pair? a) (pair? b)) (and (equal? (car a) (car b)) (equal? (cdr a) (cdr b))))
  )
)

(define (append a b)
  (cond ((null? a) b)
        (else (cons (car a) (append (cdr a) b)))
  )
)

(define (map f L)
  (cond ((null? L) '())
        (else (cons (f (car L)) (map f (cdr L))))
  )
)

(define (not b)
  (if b #f #t)
)

(define (or a b)
  (if a #t b))

(define (and a b)
  (if a b #f))

(define (caar L)
  (car(car L))
)

(define (cddr L)
  (cdr(cdr L))
)

(define (cadr L)
  (car(cdr L))
)

(define (cdar L)
  (cdr(car L))
)

(define (cadar L)
  (car(cdar L))
)

(define (caddr L)
 (car(cddr L))
)

(define (cadddr L)
  (car(cdr(cddr L)))
)



      