# EIN SOF Token

> **EMET — VALOA PIMEYTEEN** · [GAMMAWHITE.com](https://gammawhite.com) · [github.com/ninja666x/EIN-SOF-GAMMAWHITE.COM](https://github.com/ninja666x/EIN-SOF-GAMMAWHITE.COM)

Hajautettu talousprotokolla, joka yhdistää reaaliarvon, empatian ja hyvinvoinnin.
Ei spekulaatiota. Ei koronkiskontaa.

**Tila: v1.1 — prototyyppi / ensimmäinen versio. Ei auditoitu. Ei tuotantokäyttöön.**

## Sisältö

| Kansio | Mitä | Kieli |
|---|---|---|
| [`whitepaper/`](./whitepaper/) | White Paper v1.1 (`main.md` täysi, `main.typ` lyhyt) | Markdown / Typst |
| [`programs/ein-sof/`](./programs/ein-sof/src/lib.rs) | Solana-älykäs sopimus | Rust / Anchor 0.30 |
| [`tests/`](./tests/ein-sof.ts) | Anchor-testit | TypeScript |
| [`dna-cli/`](./dna-cli/) | DNA-sitoumus offline ("commit, don't reveal") | Rust |
| [`local-economy/`](./local-economy/) | 1000 agentin paikallistaloussimulaatio | Rust |
| [`vr-empathy/`](./vr-empathy/) | Voight-Kampff-empatiatesti | WebXR / Three.js |
| [`zkp/`](./zkp/) | Empatia-ZKP-piiri | Circom / snarkjs |
| [`lisp/`](./lisp/ein-sof.lisp) | Alkuperäinen prototyyppi | Common Lisp (SBCL) |
| [`docs/`](./docs/) | GitHub Pages -sivu (gammawhite.com) | Markdown |

## Pika-aloitus

```bash
make pdf-pandoc                                   # White paper PDF (pandoc + xelatex)
anchor build && anchor keys sync && anchor test   # Solana-ohjelma
cd dna-cli && cargo test --release && cd ..       # DNA-CLI
cd local-economy && cargo run --release && cd ..  # Simulaatio
cd vr-empathy && npm install && npm run dev       # VR (http://localhost:5173)
sbcl --load lisp/ein-sof.lisp --eval '(ein-sof:run-all-demos)' --quit
```

## Tunnetut puutteet (v1.1)

- `update_empathy`: kuka tahansa allekirjoittaja voi asettaa pisteet — oraakkeli-whitelist puuttuu.
- Program ID on paikkamerkki — aja `anchor keys sync`.
- VR-testin EDA/pupilli/mikroilme ovat satunnaisarvoja, kunnes anturit kytketään.
- Porttikieltotesti ei vielä aja bannausta loppuun (vaatii `admin_penalty`-instruktion).

## ⚠️ DNA

Raakaa DNA-dataa, avaimia, suoloja tai `commitment.txt`-tiedostoja **ei koskaan** commitoida tähän repoon.
`.gitignore` estää yleisimmät tiedostot.

## Lisenssi

© 2026 Tuomas Johannes Karjalainen. Kaikki oikeudet pidätetään. Ks. [LICENSE](./LICENSE).
