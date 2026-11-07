---
title: "EIN SOF TOKEN — White Paper v1.1"
subtitle: "Reaaliarvoon sidottu, empaattinen talousprotokolla"
author: "Tuomas Johannes Karjalainen (s. 1971-09-07 13:30 UTC / 15:30 Turku, 070971-119X)"
date: "2026 — Turku, Suomi"
lang: fi-FI
---

> *"Golem on savesta tehty olento, joka herää eloon, kun sen otsaan kirjoitetaan totuuden sana (emet). EIN SOF on Golem, jonka otsassa lukee: rehellisyys, empatia, reaaliarvo."*

© 2026 Tuomas Johannes Karjalainen. Kaikki oikeudet pidätetään. GAMMAWHITE.com — EMET — VALOA PIMEYTEEN

# 1. TIIVISTELMÄ

EIN SOF Token (E-SOF) on hajautettu talousprotokolla, joka yhdistää:

Reaaliarvoon sidotun tokenin (RWA: vilja, energia, asunnot, palvelut)

Biometrisen empatiatestin (Voight-Kampff, "Proof of Empathy")

Bilateraalisen voitonjaon (50/50 voitto, 100 % tappio sijoittajille)

Demurragen (negatiivinen korko, estää hamstraamisen)

DNA-sitoumuksen (yksisuuntainen SHA-256-tiiviste, ei raakaa DNA:ta)

Ydinajatus: Talous, jossa arvo syntyy työstä, hyvinvoinnista ja aidosta empatiasta — ei spekulaatiosta, koronkiskonnasta (usury) tai psykopaattisesta vallankäytöstä.

Teknologia: Solana (nopeus) + ZKP (yksityisyys) + SBT (identiteetti) + IPFS (tallennus).

Kohdeyleisö: Paikallistaloudet, osuuskunnat, reilun kaupan verkostot, kehitysyhteisöt — kaikki, jotka haluavat vaihtaa ilman pankkia ja ilman riistoa.

# 2. ONGELMAN MÄÄRITTELY

## 2.1 Nykyisen kryptotalouden ongelmat

| Ongelma | Kuvaus | Seuraus |
|---|---|---|
| Spekulaatio | Tokenit ilman reaaliarvoa | Kuplat, romahdukset |
| Exit scamit | Perustajat karkaavat rahojen kanssa | Sijoittajat menettävät kaiken |
| Passiivinen riisto | Staking, yield farming | Arvo siirtyy työltä pääomalle |
| Psykopaattinen hallinto | Äänioikeus kaikille | Yhteisöä vahingoittavat päätökset |
| Yksityisyyden puute | Kaikki julkista | Doxxaus, kiristys |
| Regulaation epäselvyys | Utility vs. security | Oikeusjutut (esim. SEC v. Ripple) |

## 2.2 Perinteisen rahan ongelmat

Velkavetoinen kasvu: Raha syntyy velasta → pakko kasvaa ikuisesti.

Korko (usury): Raha kasvaa ilman työtä → eriarvoisuus.

Inflaatio: Säästäminen on mahdotonta → kulutus pakollista.

Pankkien valta: Välimiehet kontrolloivat kaikkea.

## 2.3 EIN SOF:n vastaus

Jokaiselle ongelmalle on tekninen ratkaisu, ei poliittinen lupaus.

# 3. RATKAISUN YLEISKUVA

## 3.1 Kolme pilaria

```
┌──────────────────────────────────────────────────────┐
│                    EIN SOF                           │
├──────────────┬──────────────────┬────────────────────┤
│   TOTUUS     │    EMPATIA       │    HYVINVOINTI     │
│  (DNA/SBT)   │ (Voight-Kampff)  │   (Biometria)      │
├──────────────┼──────────────────┼────────────────────┤
│  Identiteetti│  Portinvartija   │  Arvon kerroin     │
│  Ei sock     │  Ei psykopaatteja│  Ei loppuunpaloa   │
│  puppeteja   │  verkostoon      │                    │
└──────────────┴──────────────────┴────────────────────┘
```

## 3.2 Miten arvo syntyy

```
Arvo = Työ × Hyvinvointi × Empatia
```

Työ: Mitattu energiana (kalorit) ja tuotoksena (RWA).

Hyvinvointi: Biometrinen kerroin (HRV, stressi).

Empatia: Yhteisön luottamus (SBT-pisteet).

## 3.3 Mitä EIN SOF EI ole

❌ Ei spekulatiivinen krypto (ei "to the moon")

❌ Ei keskuspankki (ei rahapoliittista valtaa)

❌ Ei pasifistinen utopia (portit ovat tiukat)

❌ Ei salaliitto (kaikki koodi on avointa)

# 4. TEKNINEN ARKKITEHTUURI

## 4.1 Kerrosmalli

```
┌─────────────────────────────────────────────────────┐
│  L6: SOVELLUS — Mobiili / VR / Web                  │
├─────────────────────────────────────────────────────┤
│  L5: KÄYTTÖLIITTYMÄ — Empatiatesti, kauppa, riidat  │
├─────────────────────────────────────────────────────┤
│  L4: ÄLYKÄS SOPIMUS — Solana (Anchor, Rust)         │
│      • Escrow  • Voitonjako  • Demurrage            │
│      • Riidat  • SBT  • Transfer Hook               │
├─────────────────────────────────────────────────────┤
│  L3: IDENTITEETTI — ZKP + SBT + DNA-sitoumus        │
├─────────────────────────────────────────────────────┤
│  L2: ORAKKELI — Biometria, RWA-todennus, laatu      │
├─────────────────────────────────────────────────────┤
│  L1: TALLENNUS — IPFS / Arweave (salattu)           │
├─────────────────────────────────────────────────────┤
│  L0: LOHKOKETJU — Solana (mainnet / devnet)         │
└─────────────────────────────────────────────────────┘
```

## 4.2 Tokenstandardi

E-SOF käyttää SPL Token-2022 -standardia, laajennettuna:

| Ominaisuus | Toteutus | Tarkoitus |
|---|---|---|
| Transfer Hook | Estää siirrot, jos empatia < 0.2 | Portinvartija |
| Permanent Delegate | Demurrage-veloitus | Rahan kierto |
| Non-Transferable | SBT-maineet | Identiteetti |
| Confidential Transfer | ZKP | Yksityisyys |
| Interest-Bearing | Ei (kielletty) | Estää usury |

## 4.3 Tietovirta: kauppa

```
1. Ostaja selaa markkinapaikkaa (RWA-listaus)
2. Ostaja lukitsee E-SOF-varat escrow-tilille (älykäs sopimus)
3. Myyjä toimittaa tuotteen (toimitus todistetaan videolla)
4. Ostaja kuittaa vastaanoton (tai reklamoi 48 h kuluessa)
5. Älykäs sopimus vapauttaa varat myyjälle
6. Molemmat saavat +1 mainepisteen
```

# 5. DNA-INTEGRAATIO — "COMMIT, DON'T REVEAL"

## 5.1 Periaate

Raaka DNA ei koskaan mene lohkoketjuun. Vain yksisuuntainen tiiviste.

```
[23andMe .txt — SINUN LAITTEELLASI]
              │
              ▼
     SHA-256(raw_dna + salt)
              │
              ▼
   dna_commitment = 32 tavua  ◄── TÄMÄ menee lohkoketjuun
              │
              ▼
   [Salattu blob: AES-256-GCM]
              │
              ▼
   [IPFS / Arweave — salattuna, avain vain sinulla]
```

## 5.2 Miksi tämä on turvallista

| Uhka | Miten estetään |
|---|---|
| DNA vuotaa lohkoketjusta | Sitä ei ole siellä — vain hash |
| Kvanttitietokone murtaa | SHA-256 kestää toistaiseksi; migraatio SHA-3:een |
| 23andMe vuotaa | Raaka data pysyy sinulla, ei 23andMe:llä |
| Sukulaisten yksityisyys | Hash ei paljasta sukulaisuutta |
| GDPR Artikla 9 | Ketjuun vain suolattu hash (pseudonymisointi), ks. 10.3 |

## 5.3 Kolme tilaa

| Tila | Lohkoketjussa | Kuka voi todentaa | Suositus |
|---|---|---|---|
| Yksityinen | Vain dna_commitment | Vain sinä (ZKP) | ✅ Oletus |
| Osittainen | commitment + zk_proof | Solmut ZKP:llä | ✅ Kauppa |
| Julkinen | commitment + julkinen avain | Kuka tahansa | ⚠️ Ei suositella |

## 5.4 DNA:n rooli empatiatestissä

DNA ei mittaa empatiaa. Se on ankkuri: todistaa, että "tämä on sama henkilö, joka läpäisi testin vuonna X". Tämä estää:

Sock puppet -hyökkäykset: Yksi ihminen = yksi DNA = yksi identiteetti.

Identiteettivarkaudet: Empatiapisteitä ei voi siirtää toiselle.

Useat tilit: Sybil-hyökkäys estyy.

## 5.5 DNA-CLI (Rust)

Katso Liite A: täydellinen työkalu, jolla lasket sitoumuksen offline-tilassa.

# 6. EMPATIATESTI (VOIGHT-KAMPFF)

## 6.1 Filosofia

Philip K. Dickin Do Androids Dream of Electric Sheep? -romaanissa (ja Blade Runner -elokuvassa) Voight-Kampff-testi mittaa empatiaa erottamaan ihmiset androideista. EIN SOF:ssä testi erottaa empaattiset ihmiset psykopaateista — ei kirjaimellisesti, vaan toiminnallisesti: verkostoon pääsee vain, jos kykenee aitoon myötätuntoon.

## 6.2 Testin rakenne

20–30 skenaariota, jaettu kategorioihin:

| Kategoria | Esimerkki | Mitä mitataan |
|---|---|---|
| Kärsimys | Lapsi itkee kylmässä | Myötätunto |
| Epäoikeudenmukaisuus | Petos, vallankäyttö | Moraalinen närkästys |
| Altruismi | Auttaminen ilman palkkiota | Empaattinen toimintahalu |
| Julmuus | Eläimen kidutus | Inho, järkytys |
| Kuolema | Läheisen menetys | Suru |

## 6.3 Biometrinen pisteytys

```
empatia = Σ (w_i × signaali_i) − w_ind × välinpitämättömyys
```

| Signaali | Paino | Mittaus |
|---|---|---|
| HRV-lasku | 0.30 | Sykevälivaihtelu |
| Ihokonduktanssi | 0.25 | Hikoilu (EDA) |
| Pupillien laajeneminen | 0.20 | Eye tracking |
| Mikroilmeet | 0.15 | Kasvojentunnistus |
| Välinpitämättömyys | −0.10 | Ei reaktiota |

## 6.4 Luokittelu

| Pisteet | Luokka | Oikeudet |
|---|---|---|
| 0.0–0.2 | :rejected | Ei pääsyä |
| 0.2–0.5 | :restricted | Kauppa, ei äänioikeutta |
| 0.5–0.8 | :accepted | Täysi kauppa, ei äänioikeutta |
| 0.8–1.0 | :node | Äänioikeus, escrow-solmu |

## 6.5 Yksityisyys: ZKP

Lohkoketjuun ei koskaan kirjata raakabiometriaa. Sen sijaan:

```circom
template EmpathyProof() {
    signal input empathy;      // 0..10000
    signal input threshold;    // 0..10000
    signal input salt;         // satunnaisuus
    signal output ok;          // 1 jos empathy >= threshold
    component gte = GreaterEqThan(14);
    gte.in[0] <== empathy;
    gte.in[1] <== threshold;
    ok <== gte.out;
}
```

Todistus todistaa: "Empatiani on ≥ 0.5" paljastamatta tarkkaa lukua tai biometriaa.

# 7. TOKENOMIA

## 7.1 Tokenin luonne

E-SOF on:

| Ominaisuus | Arvo | Perustelu |
|---|---|---|
| Reaaliarvosidottu | 1 E-SOF = X kg viljaa | Ei spekulaatiota |
| Vanheneva (demurrage) | λ = 1 % / kk | Rahan kierto |
| Ei korkoa | 0 % APY | Ei usury |
| Ei siirrettävä (SBT) | Maine | Identiteetti |
| Rajoitettu siirto | Transfer Hook | Portinvartija |

## 7.2 Demurrage

```
arvo(t) = nimellisarvo × e^(−λt),  λ = 0.01 / kk
```

12 kuukauden jälkeen: 100 E-SOF → 88.69 E-SOF.

Miksi? Koska fyysinen vilja homehtuu, energia kuluu, asunto kuluu. Raha, joka ei kierrä, on kuollutta painolastia. Demurrage pakottaa kulutukseen tai lainaamiseen.

## 7.3 Bilateraalinen voitonjako

| Tilanne | Työntekijät | Sijoittajat |
|---|---|---|
| Voitto | 50 % | 50 % |
| Tappio | 0 % | 100 % (alaskirjaus) |

Juridisesti: Tämä on pääomasijoitus (equity), ei velka. Sijoittaja on yrittäjäkumppani, ei velkoja.

## 7.4 Hyvinvointikerroin

```
wellbeing = (HRV_current / HRV_baseline) × (1 − stress)
```

Vaikutus: Jos työntekijän hyvinvointi laskee, järjestelmä automaattisesti:

Alentaa työkuormaa

Nostaa palkkiota kompensaationa

Ilmoittaa solmulle (jos ongelma jatkuu)

## 7.5 Emissio

E-SOF:ää ei "louhita" eikä "paineta". Sitä luodaan vain reaalivarantoa vastaan:

```
1 kg viljaa → 1 E-SOF (varastotodistus)
1 kWh energiaa → 1 E-SOF
1 m² asuntoa / kk → 1 E-SOF
```

Seuraus: Token määrä on sidottu reaalitalouteen. Ei inflaatiota, ei deflaatiota — vain vakautta.

## 7.6 Vertailu

| Ominaisuus | Bitcoin | Ethereum | E-SOF |
|---|---|---|---|
| Reaaliarvo | ❌ | ❌ | ✅ |
| Demurrage | ❌ | ❌ | ✅ |
| Empatiatesti | ❌ | ❌ | ✅ |
| Ei korkoa | ✅ | ❌ | ✅ |
| DNA-ankkuri | ❌ | ❌ | ✅ |
| Yksityisyys (ZKP) | ⚠️ | ⚠️ | ✅ |

# 8. HALLINTO (GOVERNANCE)

## 8.1 Äänioikeus

Vain :node-luokituksen identiteetit (empatia ≥ 0.8) voivat äänestää.

Miksi? Koska psykopaatti, joka äänestää yhteisön sääntöjä, tuhoaa yhteisön. Empatia on pätevyysvaatimus, kuten ajokortti.

## 8.2 Päätöksenteko

| Päätös | Kynnys | Aika |
|---|---|---|
| Protokollapäivitys | 2/3 solmuista | 7 vrk |
| Uusi RWA-luokka | 2/3 solmuista | 14 vrk |
| Solmun erottaminen | 3/4 solmuista | 48 h |
| Hätäpysäytys | 1/2 solmuista | 1 h |

## 8.3 Veto-oikeus

Kenellä tahansa :accepted-luokituksen omaavalla on oikeus nostaa reklamaatio mistä tahansa päätöksestä. Reklamaatio käsitellään satunnaisessa 5 solmun tuomaristossa.

## 8.4 Läpinäkyvyys

Kaikki äänestykset, päätökset ja perustelut kirjataan lohkoketjuun. Ei salaisia kokouksia.

# 9. RIIDANRATKAISU

## 9.1 Prosessi

```
1. Ostaja tekee reklamaation (48 h kuluessa)
        │
        ▼
2. Myyjällä 48 h aikaa vastata
        │
        ▼
3. Jos erimielisyys → Satunnainen 5 solmun tuomaristo
        │
        ▼
4. Tuomaristo katsoo videot/kuvat (IPFS)
        │
        ▼
5. Äänestys: :buyer / :seller / :tied
        │
        ▼
6. Päätös kirjataan SBT:hen (maine)
        │
        ▼
7. Häviäjä menettää mainetta, voittaja saa hyvityksen
```

## 9.2 Mainepisteet

| Tapahtuma | Vaikutus |
|---|---|
| Onnistunut kauppa | +1 |
| Reklamaatio (häviö) | −5 |
| Reklamaatio (voitto) | +2 |
| Exit scam -yritys | −100 (porttikielto) |
| Väärä reklamaatio | −3 |

## 9.3 Porttikielto

Kun maine ≤ −100:

Identiteetti merkitään banned = true

Kaupankäynti estetään (Transfer Hook)

Äänioikeus poistetaan

Pysyvä (ei palautusta)

# 10. TURVALLISUUS JA YKSITYISYYS

## 10.1 Uhkamalli

| Uhka | Miten estetään |
|---|---|
| Sybil-hyökkäys | DNA-sitoumus (yksi ihminen = yksi identiteetti) |
| Sock puppet | SBT sidottu DNA:han |
| Psykopaatti verkostossa | Empatiatesti (portti) |
| Exit scam | Maine + reklamaatiot + escrow |
| DNA-vuoto | Commit-don't-reveal |
| Kvanttihyökkäys | SHA-256 → SHA-3 migraatio (2027) |
| 51 % hyökkäys | Solana (ei PoW) + maine-pohjainen konsensus |
| Oraakkelimanipulaatio | Useita riippumattomia oraakkeleita |

## 10.2 Yksityisyys

| Tieto | Missä | Kuka näkee |
|---|---|---|
| Raaka DNA | Sinun laitteellasi | Vain sinä |
| DNA-sitoumus | Lohkoketju | Kaikki (hash) |
| Biometria | Laitteesi (ZKP) | Ei kukaan |
| Empatiapisteet | Lohkoketju (ZKP) | Vain tulos (≥ threshold) |
| Kauppahistoria | Lohkoketju | Julkinen (tai ZKP) |
| Maine | Lohkoketju | Julkinen |

## 10.3 GDPR-yhteensopivuus

Oikeus tulla unohdetuksi: Suolattu DNA-sitoumus on pseudonymisoitua tietoa, jota GDPR (johdanto-osan kappale 26) pitää edelleen henkilötietona. Lohkoketjusta ei voi poistaa mitään, joten käyttäjä voi hävittää suolan (salt): ilman sitä sitoumusta ei voi enää yhdistää DNA:han. Raaka data poistetaan laitteelta. Ratkaisu on arvioitava juristin kanssa ennen käyttöönottoa.

Dataminimointi: Vain hash, ei raakadataa.

Suostumus: Käyttäjä valitsee tason (yksityinen / osittainen / julkinen).

# 11. OIKEUDELLISET NÄKÖKOHDAT

## 11.1 Onko E-SOF arvopaperi?

Howeyn testi (SEC v. W.J. Howey Co., 1946):

Sijoitus rahaa? → Kyllä (jos ostat E-SOF:ää odottaen tuottoa)

Yhteinen yritys? → Kyllä (verkosto)

Odotus voitoista muiden ponnisteluista? → Ei (E-SOF ei tuota korkoa)

Johtopäätös: E-SOF on hyödyketoken (utility token), ei arvopaperi — edellyttäen, että:

Sitä markkinoidaan vaihdon välineenä, ei sijoituksena.

Sitä ei luvata tuottoa.

Se on sidottu reaaliarvoon.

## 11.2 WTT Gigawatt -tapaus (vertailu)

Tapauksessa Giga Watt myi WTT-tokeneita, jotka oikeuttivat sähkönkäyttöön. Sijoittajat nostivat kanteen, koska tokeneita markkinoitiin sijoituksina. E-SOF välttää tämän olemalla eksplisiittisesti hyödyke, ei sijoitus.

## 11.3 Regulaatiostrategia

| Vaihe | Toimenpide |
|---|---|
| V0.5 | Testnet, ei julkista myyntiä |
| V1.0 | Rajoitettu käyttöönotto (osuuskunnat) |
| V2.0 | Reg D 506(c) -poikkeus (akkreditoidut sijoittajat) |
| V3.0 | Reg A+ (laajempi yleisö) |

Ei ICO:ta. Ei julkista tokenmyyntiä. Vain reaaliarvoon sidottu käyttöönotto.

# 12. TIEKARTTA

| Vaihe | Aikataulu | Tavoite | Teknologia |
|---|---|---|---|
| V0.1 | Q1 2026 | Lisp-prototyyppi | SBCL |
| V0.5 | Q2 2026 | Solana devnet | Anchor/Rust |
| V0.8 | Q3 2026 | VR-empatiatesti | WebXR |
| V1.0 | Q4 2026 | Mainnet-beta, paikallistaloudet | Solana mainnet |
| V1.5 | Q1 2027 | Riidanratkaisu, ZKP | Circom |
| V2.0 | Q2 2027 | Globaali laajennus | Multi-chain |
| V3.0 | 2028 | Täysi hajautus | DAO |

# 13. TEKNISET LIITTEET

## Liite A: DNA-CLI (Rust)

Täydellinen työkalu DNA-sitoumuksen laskemiseen offline-tilassa. Lähdekoodi: `dna-cli/src/main.rs`, testit: `dna-cli/tests/integration.rs`.

```bash
# 1. Laske sitoumus 23andMe-datasta (offline!)
dna-cli commit -i ~/genome.txt -o ~/commitment.txt

# 2. Salaa raakadata (valinnainen)
openssl rand -hex 32 > ~/dna.key
dna-cli encrypt -i ~/genome.txt -o ~/genome.enc -k ~/dna.key

# 3. Poista raakadata (GDPR)
shred -u ~/genome.txt

# 4. Myöhemmin: tarkista sitoumus
dna-cli verify -i ~/genome.txt -c ~/commitment.txt
```

Lohkoketjuun menee: commitment (32 tavua). Ei koskaan: raakadata, salt, avain.

## Liite B: Anchor-ohjelma

Lähdekoodi: `programs/ein-sof/src/lib.rs`, testit: `tests/ein-sof.ts`.

## Liite C: Lisp-prototyyppi

Lähdekoodi: `lisp/ein-sof.lisp` (11 moduulia, SBCL-yhteensopiva).

## Liite D: ZKP-piiri

Lähdekoodi: `zkp/empathy.circom` (Circom 2.1) ja `zkp/prove.ts` (snarkjs).

# 14. VIITTEET JA SANASTO

## 14.1 Viitteet

Nakamoto, S. (2008). Bitcoin: A Peer-to-Peer Electronic Cash System.

Buterin, V. (2014). Ethereum White Paper.

Gesell, S. (1916). Die natürliche Wirtschaftsordnung (vapaaraha/demurrage).

Dick, P.K. (1968). Do Androids Dream of Electric Sheep? (Voight-Kampff).

SEC v. W.J. Howey Co., 328 U.S. 293 (1946).

Solana Foundation. (2024). SPL Token-2022 Documentation.

Groth, J. (2016). On the Size of Pairing-Based Non-interactive Arguments (Groth16).

GDPR, Regulation (EU) 2016/679, Artikla 9.

## 14.2 Sanasto

| Termi | Määritelmä |
|---|---|
| RWA | Real World Asset — reaalimaailman omaisuus |
| SBT | Soulbound Token — siirtämätön identiteettitoken |
| ZKP | Zero-Knowledge Proof — nollatietotodistus |
| Demurrage | Negatiivinen korko, rahan vanheneminen |
| Usury | Koronkiskonta (kielletty E-SOF:ssa) |
| Escrow | Turvavarasto, kolmannen osapuolen hallussa |
| Oraakkeli | Off-chain-datan tuoja lohkoketjuun |
| Sybil-hyökkäys | Useiden väärien identiteettien luominen |
| Voight-Kampff | Empatiatesti (Blade Runner) |

# LOPPUSANAT

EIN SOF ei ole utopia. Se on tekninen spesifikaatio sille, miltä talous näyttäisi, jos se rakennettaisiin uudelleen rehellisyyden, empatian ja reaaliarvon varaan.

Se ei vaadi vallankumousta. Se vaatii koodin kirjoittamista, testaamista ja käyttöönottoa — yksi paikallistalous kerrallaan.

Golem herää eloon, kun sen otsaan kirjoitetaan emet (totuus). EIN SOF:n otsassa lukee:

```
empatia ≥ 0.5
arvo = työ × hyvinvointi
ei korkoa, ei riistoa
```

Tervetuloa rakentamaan.


---

Copyright © 2026 Tuomas Johannes Karjalainen (s. 1971-09-07 13:30 UTC / 15:30 Turku, 070971-119X) · Turku, Suomi · Kaikki oikeudet pidätetään.
