// EIN SOF White Paper v1.1 — Typst
// Copyright © 2026 Tuomas Johannes Karjalainen (s. 7.9.1971 klo 8:30, 070971-119X)

#set document(
  title: "EIN SOF TOKEN — White Paper v1.1",
  author: "Tuomas Johannes Karjalainen",
  keywords: ("RWA", "empathy", "ZKP", "SBT", "demurrage"),
)

#set page(
  paper: "a4",
  margin: (x: 2.5cm, y: 2.5cm),
  numbering: "1",
  header: align(right)[_EIN SOF — GAMMAWHITE.com_],
  footer: align(center)[#counter(page).display()],
)

#set text(font: ("DejaVu Serif", "Linux Libertine"), size: 11pt, lang: "fi")
#set heading(numbering: "1.1")
#set par(justify: true, leading: 0.65em)

#show heading.where(level: 1): it => {
  pagebreak(weak: true)
  v(1em)
  text(size: 20pt, weight: "bold", it)
  v(0.5em)
  line(length: 100%, stroke: 0.5pt)
  v(0.5em)
}

// --- Kansilehti ---
#align(center + horizon)[
  #text(size: 32pt, weight: "bold")[EIN SOF]
  #v(0.3em)
  #text(size: 18pt)[Reaaliarvoon sidottu, empaattinen talousprotokolla]
  #v(2em)
  #text(size: 14pt)[_White Paper v1.1_]
  #v(3em)
  #text(size: 12pt)[
    Tuomas Johannes Karjalainen \
    _s. 7.9.1971 klo 8:30 · 070971-119X · Turku, Suomi_
  ]
  #v(3em)
  #text(size: 10pt, style: "italic")[
    © 2026 Tuomas Johannes Karjalainen. Kaikki oikeudet pidätetään.
  ]
  #v(1em)
  #text(size: 10pt)[GAMMAWHITE.com — _EMET — VALOA PIMEYTEEN_]
]

// --- Sisältö ---
#outline(title: "Sisällysluettelo", depth: 2)

= Tiivistelmä

EIN SOF Token (E-SOF) on hajautettu talousprotokolla, joka yhdistää
reaaliarvoon sidotun tokenin, biometrisen empatiatestin (Voight-Kampff),
bilateraalisen voitonjaon, demurragen ja DNA-sitoumuksen. Tavoitteena
on talous, jossa arvo syntyy työstä, hyvinvoinnista ja aidosta empatiasta
— ei spekulaatiosta, koronkiskonnasta (usury) tai psykopaattisesta
vallankäytöstä.

= Ongelman määrittely

Nykyinen kryptotalous kärsii spekulaatiosta, exit scameista ja
passiivisesta riistosta. Perinteinen raha puolestaan perustuu
velkavetoiseen kasvuun ja korkoon. EIN SOF vastaa molempiin
teknisin ratkaisuin.

= Tekninen arkkitehtuuri

== Kerrosmalli

#table(
  columns: (auto, 1fr),
  [*Kerros*], [*Sisältö*],
  [L6], [Sovellus — Mobiili / VR / Web],
  [L5], [Käyttöliittymä — Empatiatesti, kauppa, riidat],
  [L4], [Älykäs sopimus — Solana (Anchor, Rust)],
  [L3], [Identiteetti — ZKP + SBT + DNA-sitoumus],
  [L2], [Oraakkeli — Biometria, RWA-todennus],
  [L1], [Tallennus — IPFS / Arweave (salattu)],
  [L0], [Lohkoketju — Solana],
)

== DNA-integraatio — "Commit, don't reveal"

Raaka DNA *ei koskaan* mene lohkoketjuun. Vain yksisuuntainen
tiiviste:

$ "dna_commitment" = "SHA-256"("raw_dna" + "salt") $

= Tokenomia

== Demurrage

$ "arvo"(t) = "nominal" dot e^(-lambda t), quad lambda = 0.01\/"kk" $

== Bilateraalinen voitonjako

#table(
  columns: (auto, auto, auto),
  [*Tilanne*], [*Työntekijät*], [*Sijoittajat*],
  [Voitto], [50 %], [50 %],
  [Tappio], [0 %], [100 %],
)

= Yhteenveto

EIN SOF ei ole utopia vaan tekninen spesifikaatio. Se ei vaadi
vallankumousta — se vaatii koodin kirjoittamista.

#align(center)[
  #v(2em)
  #text(size: 12pt, style: "italic")[
    _Golem herää eloon, kun sen otsaan kirjoitetaan EMET._
  ]
]
