;;;; ============================================================
;;;; EIN SOF TOKEN — Complete Implementation
;;;; Copyright © 2026 Tuomas Johannes Karjalainen (s. 1971-09-07 13:30 UTC / 15:30 Turku, 070971-119X)
;;;; Common Lisp (SBCL-yhteensopiva)
;;;; ============================================================

(defpackage :ein-sof
  (:use :cl)
  (:shadow #:identity)   ; CL:IDENTITY on lukittu SBCL:ssä
  (:export #:run-all-demos
           #:repl))

(in-package :ein-sof)

;;;; ============================================================
;;;; 1. APUFUNKTIOT
;;;; ============================================================

(defun now () (get-universal-time))

(defun months-between (t1 t2)
  (/ (- t2 t1) (* 30 24 3600.0)))

(defun clamp (x lo hi)
  (max lo (min hi x)))

;;;; ============================================================
;;;; 2. IDENTITEETTI & MAINE (SBT)
;;;; ============================================================

(defstruct identity
  "Käyttäjän pysyvä identiteetti (Soulbound Token)."
  (id            nil :type symbol)
  (empathy-score 0.0 :type float)   ; 0.0–1.0
  (reputation    0   :type integer) ; mainepisteet
  (created       (now) :type integer)
  (banned        nil :type boolean))

(defun classify-empathy (score)
  (cond ((< score 0.2)  :rejected)
        ((< score 0.5)  :restricted)
        ((< score 0.8)  :accepted)
        (t              :node)))

(defun can-trade-p (identity)
  "Vain :accepted tai :node voivat käydä kauppaa."
  (and (not (identity-banned identity))
       (member (classify-empathy (identity-empathy-score identity))
               '(:accepted :node))))

(defun can-vote-p (identity)
  "Vain :node voi äänestää."
  (and (not (identity-banned identity))
       (eq (classify-empathy (identity-empathy-score identity)) :node)))

(defun adjust-reputation (identity delta)
  (incf (identity-reputation identity) delta)
  (when (<= (identity-reputation identity) -100)
    (setf (identity-banned identity) t)))

;;;; ============================================================
;;;; 3. EMPATIATESTI (VOIGHT-KAMPFF)
;;;; ============================================================

(defstruct biometric-reading
  (hrv-drop         0.0 :type float)
  (skin-conductance 0.0 :type float)
  (pupil-dilation   0.0 :type float)
  (micro-expression 0.0 :type float)
  (indifference     0.0 :type float))

(defparameter *empathy-weights*
  '((:hrv . 0.30) (:skin . 0.25) (:pupil . 0.20)
    (:expression . 0.15) (:indifference . -0.10)))

(defun score-scenario (r)
  (let ((w *empathy-weights*))
    (clamp (+ (* (biometric-reading-hrv-drop r)
                 (cdr (assoc :hrv w)))
              (* (biometric-reading-skin-conductance r)
                 (cdr (assoc :skin w)))
              (* (biometric-reading-pupil-dilation r)
                 (cdr (assoc :pupil w)))
              (* (biometric-reading-micro-expression r)
                 (cdr (assoc :expression w)))
              (* (biometric-reading-indifference r)
                 (cdr (assoc :indifference w))))
           0.0 1.0)))

(defun run-voight-kampff (readings)
  (let* ((scores (mapcar #'score-scenario readings))
         (avg    (if scores (/ (reduce #'+ scores) (length scores)) 0.0)))
    (list :average-score  (float avg)
          :classification (classify-empathy avg)
          :per-scenario   scores)))

(defun apply-empathy-test (identity readings)
  "Päivittää identiteetin empatiapisteen testin perusteella."
  (let* ((result (run-voight-kampff readings))
         (score  (getf result :average-score)))
    (setf (identity-empathy-score identity) score)
    result))

;;;; ============================================================
;;;; 4. TOKEN & DEMURRAGE
;;;; ============================================================

(defstruct token
  (nominal 1.0 :type float)
  (created (now) :type integer)
  (lambda  0.01 :type float))

(defun token-value (token &optional (at (now)))
  (* (token-nominal token)
     (exp (- (* (token-lambda token)
                (months-between (token-created token) at))))))

(defun apply-demurrage (token &optional (at (now)))
  (setf (token-nominal token) (token-value token at)))

;;;; ============================================================
;;;; 5. TYÖNTEKIJÄT & SIJOITTAJAT
;;;; ============================================================

(defstruct worker
  (id nil :type symbol)
  (identity nil)
  (hrv-baseline 1.0 :type float)
  (hrv-current  1.0 :type float)
  (stress       0.0 :type float))

(defstruct investor
  (id nil :type symbol)
  (identity nil)
  (capital 0.0 :type float))

(defun worker-wellbeing (w)
  (* (/ (worker-hrv-current w)
        (max 0.001 (worker-hrv-baseline w)))
     (- 1.0 (worker-stress w))))

(defun wellness-multiplier (workers)
  (if workers
      (/ (reduce #'+ (mapcar #'worker-wellbeing workers))
         (length workers))
      1.0))

;;;; ============================================================
;;;; 6. BILATERAALINEN VOITONJAKO
;;;; ============================================================

(defun split-profit (profit workers investors)
  (cond
    ((>= profit 0.0)
     (let ((half (/ profit 2.0)))
       (list :workers   half
             :investors half
             :per-worker (/ half (max 1 (length workers)))
             :loss-bearer :none)))
    (t
     (list :workers   0.0
           :investors profit
           :per-worker 0.0
           :loss-bearer :investors))))

(defun settle-period (revenue costs workers investors)
  (let* ((profit (- revenue costs))
         (wm     (wellness-multiplier workers))
         (adj    (* profit wm))
         (split  (split-profit adj workers investors)))
    (list :profit profit :wellness wm
          :adjusted-profit adj :split split)))

;;;; ============================================================
;;;; 7. SIMULOITU SOLANA-KERROS
;;;; ============================================================

(defstruct tx
  (from nil :type symbol)
  (to   nil :type symbol)
  (amount 0.0 :type float)
  (kind :transfer :type symbol)
  (timestamp (now) :type integer)
  (signature "" :type string))

(defvar *ledger* nil "Lista tehdyistä transaktioista.")
(defvar *tx-counter* 0)

(defun make-signature ()
  (format nil "SIG-~8,'0X" (incf *tx-counter*)))

(defun submit-tx (from to amount &key (kind :transfer))
  "Simuloi Solana-transaktiota. Palauttaa tx-rakenteen tai NIL."
  (let ((tx (make-tx :from from :to to :amount amount
                     :kind kind :signature (make-signature))))
    (push tx *ledger*)
    tx))

(defun ledger-balance (id)
  "Laskee saldon simulaatiolokista."
  (reduce #'+ *ledger*
          :key (lambda (tx)
                 (cond ((eq (tx-from tx) id) (- (tx-amount tx)))
                       ((eq (tx-to tx) id)   (tx-amount tx))
                       (t 0.0)))
          :initial-value 0.0))

;;;; ============================================================
;;;; 8. REKLAMAATIO- & RIIDANRATKAISU
;;;; ============================================================

(defstruct dispute
  (id 0 :type integer)
  (buyer nil)
  (seller nil)
  (amount 0.0 :type float)
  (reason "" :type string)
  (status :open :type symbol)   ; :open :resolved-buyer :resolved-seller
  (votes-for-buyer 0 :type integer)
  (votes-for-seller 0 :type integer)
  (created (now) :type integer))

(defvar *dispute-counter* 0)
(defvar *disputes* nil)

(defun file-dispute (buyer seller amount reason)
  (let ((d (make-dispute :id (incf *dispute-counter*)
                         :buyer buyer :seller seller
                         :amount amount :reason reason)))
    (push d *disputes*)
    (format t "~&[DISPUTE ~D] ~A vs ~A: ~A (~,2f E-SOF)~%"
            (dispute-id d) buyer seller reason amount)
    d))

(defun vote-dispute (d voter side)
  "SOLMU-identiteetti äänestää. SIDE on :buyer tai :seller."
  (unless (can-vote-p voter)
    (error "Vain :node-luokituksen identiteetti voi äänestää."))
  (ecase side
    (:buyer  (incf (dispute-votes-for-buyer d)))
    (:seller (incf (dispute-votes-for-seller d))))
  d)

(defun resolve-dispute (d)
  "Ratkaisee riidan äänten perusteella ja päivittää maineet."
  (let ((buyer  (dispute-buyer d))
        (seller (dispute-seller d)))
    (cond
      ((> (dispute-votes-for-buyer d) (dispute-votes-for-seller d))
       (setf (dispute-status d) :resolved-buyer)
       (adjust-reputation buyer  +2)
       (adjust-reputation seller -5)
       (format t "~&[RESOLVED] Ostaja voitti. ~A +2, ~A -5.~%" buyer seller))
      ((> (dispute-votes-for-seller d) (dispute-votes-for-buyer d))
       (setf (dispute-status d) :resolved-seller)
       (adjust-reputation buyer  -5)
       (adjust-reputation seller +2)
       (format t "~&[RESOLVED] Myyjä voitti. ~A -5, ~A +2.~%" buyer seller))
      (t
       (format t "~&[RESOLVED] Tasapeli. Ei muutoksia.~%")))
    d))

;;;; ============================================================
;;;; 9. KORKEAN TASON KAUPPASKENAARIO
;;;; ============================================================

(defun trade (buyer seller amount item)
  "Suorittaa kaupan: escrow → toimitus → vapautus."
  (unless (can-trade-p (worker-identity buyer))
    (error "Ostaja ei voi käydä kauppaa (empatia/banni)."))
  (unless (can-trade-p (worker-identity seller))
    (error "Myyjä ei voi käydä kauppaa (empatia/banni)."))
  (format t "~&[TRADE] ~A ostaa ~A:lta ~A (~,2f E-SOF)~%"
          (worker-id buyer) (worker-id seller) item amount)
  (submit-tx (worker-id buyer) (worker-id seller) amount :kind :escrow)
  (format t "~&[ESCROW] Varat lukittu. Odotetaan toimitusta...~%")
  (submit-tx (worker-id buyer) (worker-id seller) amount :kind :release)
  (adjust-reputation (worker-identity buyer)  +1)
  (adjust-reputation (worker-identity seller) +1)
  (format t "~&[OK] Kauppa suoritettu. Maine +1 molemmille.~%"))

;;;; ============================================================
;;;; 10. DEMOT
;;;; ============================================================

(defun demo-identities ()
  (let* ((good-readings (list (make-biometric-reading
                                :hrv-drop 0.8 :skin-conductance 0.7
                                :pupil-dilation 0.6 :micro-expression 0.7
                                :indifference 0.0)
                              (make-biometric-reading
                                :hrv-drop 0.9 :skin-conductance 0.8
                                :pupil-dilation 0.7 :micro-expression 0.8
                                :indifference 0.0)))
         (bad-readings  (list (make-biometric-reading
                                :hrv-drop 0.05 :skin-conductance 0.0
                                :pupil-dilation 0.0 :micro-expression 0.0
                                :indifference 1.0)
                              (make-biometric-reading
                                :hrv-drop 0.1 :skin-conductance 0.05
                                :pupil-dilation 0.05 :micro-expression 0.0
                                :indifference 0.95)))
         (alice (make-identity :id :alice))
         (bob   (make-identity :id :bob))
         (eve   (make-identity :id :eve)))
    (format t "~&=== EMPATIATESTIT ===~%")
    (print (apply-empathy-test alice good-readings))
    (print (apply-empathy-test bob   good-readings))
    (print (apply-empathy-test eve   bad-readings))
    (list :alice alice :bob bob :eve eve)))

(defun demo-economy ()
  (format t "~&~%=== TALOUSMALLI ===~%")
  (let* ((w1 (make-worker :id :alice :hrv-baseline 1.0
                          :hrv-current 0.95 :stress 0.1))
         (w2 (make-worker :id :bob   :hrv-baseline 1.0
                          :hrv-current 0.80 :stress 0.3))
         (i1 (make-investor :id :inv-a :capital 10000.0))
         (tok (make-token :nominal 100.0)))
    (format t "~&Tokenin arvo ajan myötä:~%")
    (loop for m from 0 to 12 by 3
          do (format t "  Kuukausi ~2d: ~,4f~%"
                     m (token-value tok (+ (token-created tok)
                                           (* m 30 24 3600)))))
    (format t "~&Voitto 1000:~%  ~A~%"
            (settle-period 2000 1000 (list w1 w2) (list i1)))
    (format t "~&Tappio -500:~%  ~A~%"
            (settle-period 500 1000 (list w1 w2) (list i1)))))

(defun demo-dispute ()
  (format t "~&~%=== RIITATAPAUS ===~%")
  (let* ((ids (demo-identities))
         (alice (getf ids :alice))
         (bob   (getf ids :bob))
         (eve   (getf ids :eve))
         (nodes (list alice bob eve)))
    (let ((d (file-dispute alice bob 50.0 "Vilja oli homeessa.")))
      ;; Kolme solmua äänestävät (vain :node-luokitus)
      (dolist (n nodes)
        (when (can-vote-p n)
          (vote-dispute d n (if (eq (identity-id n) :eve) :seller :buyer))))
      (resolve-dispute d)
      (format t "~&Lopputulos: ~A~%" (dispute-status d))
      (format t "Alice maine: ~D, Bob maine: ~D, Eve maine: ~D~%"
              (identity-reputation alice)
              (identity-reputation bob)
              (identity-reputation eve)))))

(defun demo-trade ()
  (format t "~&~%=== KAUPPA ===~%")
  (let* ((ids (demo-identities))
         (alice (getf ids :alice))
         (bob   (getf ids :bob))
         (wa (make-worker :id :alice :identity alice))
         (wb (make-worker :id :bob   :identity bob)))
    (trade wa wb 25.0 "10 kg luomuvehnää")
    (format t "~&Ledger-saldo Alice: ~,2f, Bob: ~,2f~%"
            (ledger-balance :alice) (ledger-balance :bob))))

(defun run-all-demos ()
  (demo-identities)
  (demo-economy)
  (demo-dispute)
  (demo-trade)
  (format t "~&~%=== KAIKKI DEMOT AJETTU ===~%"))

;;;; ============================================================
;;;; 11. REPL
;;;; ============================================================

(defun repl ()
  (format t "~&EIN SOF REPL. Komennot: demo, quit.~%")
  (loop
    (format t "~&e-sof> ")
    (finish-output)
    (let ((line (read-line nil :eof)))
      (cond ((or (eq line :eof) (string= line "quit")) (return))
            ((string= line "demo") (run-all-demos))
            (t (format t "Tuntematon komento.~%"))))))
