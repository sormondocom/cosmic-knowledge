# Celestial Numerology Analyzer

A Rust terminal application for analyzing words and phrases through ten numerological and gematria
traditions, exploring Enochian angelology, navigating world cosmological systems, and generating
sacred-frequency WAV files.

---

## At-a-Glance

### Quick-reference: what the app can do

| Want to… | How |
|---|---|
| Analyze a word across all ten systems | `cargo run` → option **1** |
| Select a subset of numerology systems | option **1** → enter system numbers at the prompt |
| Browse or look up an Aethyr | `cargo run -- --aethyr ZAX` or `cargo run -- --aethyr 10` |
| Translate a word into Enochian letter names | option **2** → sub-option **4** |
| Browse the 19 Enochian Keys | option **2** → sub-option **5** |
| Export all Solfeggio frequencies as WAV | `cargo run -- --export-all` |
| Export a single frequency interactively | option **3** → pick a frequency |
| Create a custom binaural beat | option **3** → sub-option **11** |
| Explore Chinese cosmology | option **4** → sub-option **1** |
| Explore African traditions | option **4** → sub-option **2** |
| Run the Psi–RNG experiment | option **5** |
| Configure range and delay for Psi–RNG | option **5** → enter settings at the prompts |
| Skip the intro animation | `cargo run -- --fast` |
| Run silently (no audio) | `cargo run -- --silent` |
| Run all unit tests | `cargo test` |
| See full help | `cargo run -- --help` |

### Systems at a glance

| Module | System | Culture / Tradition | Key feature |
|---|---|---|---|
| `numerology/hebrew.rs` | Hebrew Gematria | Kabbalistic / Jewish | Mispar Hechrachi, non-linear values |
| `numerology/pythagorean.rs` | Pythagorean | Western / New Age | Mod-9 alphabetic cycle |
| `numerology/chaldean.rs` | Chaldean | Mesopotamian (modern) | No letter assigned 9; compound numbers |
| `numerology/greek.rs` | Greek Isopsephy | Neoplatonic / Hellenistic | Classical Greek numeric alphabet |
| `numerology/agrippan.rs` | Agrippan | Renaissance Hermeticism | Barrett/Agrippa English extension |
| `numerology/ordinal.rs` | Simple Ordinal | Modern English | A=1 … Z=26 |
| `numerology/ordinal.rs` | Reverse Ordinal | Modern English | A=26 … Z=1 |
| `numerology/abjad.rs` | Abjad | Arabic / Islamic | Semitic abjad numerals |
| `enochian/alphabet.rs` | Enochian Ordinal | Dee–Kelley (1582–1587) | Positional 1–21; most historically defensible |
| `enochian/alphabet.rs` | Enochian G.D. | Golden Dawn (19th c.) | Hebrew-mapped values; Mathers/Regardie retrofit |
| `cosmology/chinese.rs` | Nine Star Ki | East Asian | Solar-year natal star calculation |
| `cosmology/chinese.rs` | Wu Xing | Chinese | Five-element cycle |
| `cosmology/chinese.rs` | Ba Gua | Chinese / I Ching | Eight trigrams |
| `cosmology/african.rs` | Yoruba Ifá | West African | 256 Odù; divination corpus |
| `cosmology/african.rs` | Akan Day Names | Ghanaian | Birth-day soul name system |
| `cosmology/african.rs` | Kemetic Numbers | Ancient Egyptian | Sacred numerical symbolism |
| `audio.rs` | Solfeggio / Binaural | Modern esoteric | WAV export; binaural beat synthesis |
| `rng.rs` | Psi–RNG Experiment | Experimental parapsychology | RDRAND hardware TRNG; configurable range and delay |

**Output formats:** interactive terminal · plain-text reports (`exports/*.txt`) ·
HTML reports · PDF reports · WAV audio (`exports/*.wav`)

---

## Overview

The Celestial Numerology Analyzer is a single-binary Rust application built around a modular
library of gematria and cosmological traditions. It lets users:

- Compute letter-number values under ten distinct systems simultaneously and compare results.
- Drill into Enochian angelology — browse the alphabet, the 30 Aethyrs, and the 19 Angelic Calls as
  recorded in John Dee's manuscripts.
- Access Chinese and African cosmological frameworks (Nine Star Ki natal charts, Wu Xing, Ba Gua,
  Yoruba Ifá Odù, Akan day-name souls, Kemetic number symbolism).
- Export pure-tone Solfeggio frequencies and binaural-beat WAV files for meditative use.
- Save session results to timestamped plain-text files or send formatted HTML reports to a system
  printer.

The application is structured so that all domain logic lives in library modules; `main.rs` is a
thin dispatcher of session loops. Each tradition is isolated in its own source file, making it
straightforward to locate, audit, or extend individual systems.

---

## Numerology Systems

The `numerology/` module aggregates ten letter-to-number mapping systems. Shared utilities —
`digital_root`, `meaning_of`, `angelic_message`, `master_numbers_message`,
`check_special_sequences`, and `get_calculation_breakdown` — live in `numerology/mod.rs`.

The function `numerology(word: &str)` returns all ten results in a stable order:
Hebrew Gematria, Pythagorean, Chaldean, Greek Isopsephy, Agrippan, Simple Ordinal, Reverse
Ordinal, Abjad, Enochian Ordinal, Enochian G.D.

### Hebrew Gematria

**Source file:** `src/numerology/hebrew.rs`
**Tradition:** Kabbalistic / Jewish mysticism
**Method:** Mispar Hechrachi (absolute value). Each letter carries a fixed weight derived from
its position in the Hebrew alphabet: units (Aleph–Tet: 1–9), tens (Yod–Tsadi: 10–90), hundreds
(Qoph–Tav: 100–400). The Latin-letter mapping used here is a transliteration convention from
Western occultism; classical Hebrew gematria operates on Hebrew script directly.

**Letter values (Latin transliteration):**

| Letter | Value | Letter | Value | Letter | Value |
|--------|-------|--------|-------|--------|-------|
| A | 1 | J | 10 | S | 200 |
| B | 2 | K | 20 | T | 300 |
| C | 3 | L | 30 | U | 400 |
| D | 4 | M | 40 | V | 500 |
| E | 5 | N | 50 | W | 600 |
| F | 6 | O | 60 | X | 700 |
| G | 7 | P | 70 | Y | 800 |
| H | 8 | Q | 80 | Z | 900 |
| I | 9 | R | 100 | | |

**Key source texts:**
- Kaplan, A. *Sefer Yetzirah* (1990, Weiser Books)
- Blumenthal, D. *Understanding Jewish Mysticism* (1978, Ktav)
- Munk, M. *The Wisdom in the Hebrew Alphabet* (1983, ArtScroll)

---

### Pythagorean Numerology

**Source file:** `src/numerology/pythagorean.rs`
**Tradition:** Western / New Age numerology
**Method:** A cyclic 1–9 mapping based on alphabetical position. Every letter is assigned the
value of its position modulo 9 (with 0 replaced by 9). Totals are reduced to a single digit via
digital root; 11, 22, and 33 (and higher doubles) are conventionally retained as *master numbers*
before reduction.

**Letter values:**

```
A=1  B=2  C=3  D=4  E=5  F=6  G=7  H=8  I=9
J=1  K=2  L=3  M=4  N=5  O=6  P=7  Q=8  R=9
S=1  T=2  U=3  V=4  W=5  X=6  Y=7  Z=8
```

**Key source texts:**
- Nicomachus of Gerasa, *Introduction to Arithmetic* (c. 100 CE)
- Taylor, T. *The Theoretic Arithmetic of the Pythagoreans* (1816)
- Burkert, W. *Lore and Science in Ancient Pythagoreanism* (1972, Harvard)

---

### Chaldean Numerology

**Source file:** `src/numerology/chaldean.rs`
**Tradition:** Mesopotamian / Babylonian (modern codification)
**Method:** An irregular mapping where no letter is assigned the value 9 (considered sacred or
divine). Letters F, G, O, and U differ materially from the Pythagorean assignments. Totals above
9 are read as "compound numbers" before optional reduction.

**Letter values:**

```
A=1  B=2  C=3  D=4  E=5  F=8  G=3  H=5  I=1
J=1  K=2  L=3  M=4  N=5  O=7  P=8  Q=1  R=2
S=3  T=4  U=6  V=6  W=6  X=5  Y=1  Z=7
```

**Key source texts:**
- Cheiro (Count Louis Hamon), *Cheiro's Book of Numbers* (1926)
- Schimmel, A. *The Mystery of Numbers* (1993, OUP) — context and critique

---

### Greek Isopsephy

**Source file:** `src/numerology/greek.rs`
**Tradition:** Neoplatonic / Hellenistic
**Method:** The classical Greek numerical alphabet (Convention B — Latin phonetic equivalents
used here for ASCII input). Values follow the Milesian / Ionic system: alpha=1 through theta=9,
iota=10 through koppa=90, rho=100 through sampi=900. The function `isopsephy_meaning(root)` maps
digital roots to Neoplatonic philosophical interpretations rather than the Western angel-number
glossary.

**Selected values (Latin phonetic equivalents):**

| Phoneme | Greek | Value | Phoneme | Greek | Value |
|---------|-------|-------|---------|-------|-------|
| A | Α (Alpha) | 1 | N | Ν (Nu) | 50 |
| B | Β (Beta) | 2 | O | Ο (Omicron) | 70 |
| G | Γ (Gamma) | 3 | P | Π (Pi) | 80 |
| D | Δ (Delta) | 4 | R | Ρ (Rho) | 100 |
| E | Ε (Epsilon) | 5 | S | Σ (Sigma) | 200 |
| Z | Ζ (Zeta) | 7 | T | Τ (Tau) | 300 |
| H | Η (Eta) | 8 | U/Y | Υ (Upsilon) | 400 |
| TH | Θ (Theta) | 9 | PH/F | Φ (Phi) | 500 |
| I/J | Ι (Iota) | 10 | CH/X | Χ (Chi) | 600 |
| K | Κ (Kappa) | 20 | PS | Ψ (Psi) | 700 |
| L | Λ (Lambda) | 30 | O (long) | Ω (Omega) | 800 |
| M | Μ (Mu) | 40 | | | |

**Key source texts:**
- Dornseiff, F. *Das Alphabet in Mystik und Magie* (1925, Teubner)
- Iamblichus, *Theology of Arithmetic* (c. 300 CE; trans. Waterfield, 1988, Phanes)
- Nicomachus of Gerasa, *Introduction to Arithmetic*

---

### Agrippan Numerology

**Source file:** `src/numerology/agrippan.rs`
**Tradition:** Renaissance Hermeticism
**Method:** An extension of the Hebrew Gematria structure to the full Latin/English alphabet,
as codified by Heinrich Cornelius Agrippa in *De Occulta Philosophia* (1531) and systematized by
Francis Barrett in *The Magus* (1801). Letters A–I follow the Hebrew units (1–9); the extension
continues the pattern into tens and hundreds to cover the remaining English letters.

**Key source texts:**
- Agrippa, H.C. *De Occulta Philosophia Libri Tres* (1531; trans. Tyson, 1993, Llewellyn)
- Barrett, F. *The Magus, or Celestial Intelligencer* (1801; facsimile, Weiser, 1967)

---

### Simple Ordinal

**Source file:** `src/numerology/ordinal.rs`
**Tradition:** Modern English
**Method:** Each letter is assigned its ordinal position in the English alphabet: A=1, B=2, …,
Z=26. No reduction is applied to the mapping itself; digital root is applied to the total.

**Key source texts:**
- Modern English Gematria — no classical source; popularized in contemporary numerology communities.

---

### Reverse Ordinal

**Source file:** `src/numerology/ordinal.rs`
**Tradition:** Modern English
**Method:** The mirror of Simple Ordinal: A=26, B=25, …, Z=1. For any single letter, Simple
Ordinal + Reverse Ordinal = 27 (verified in the test suite).

**Key source texts:**
- Modern English Gematria — no classical source; popularized in contemporary numerology communities.

---

### Abjad

**Source file:** `src/numerology/abjad.rs`
**Tradition:** Arabic / Islamic
**Method:** The Abjad system assigns numerical values to Arabic letters following the ancient
Semitic abjad order (not the modern alphabetic order). The values follow the same unit-ten-hundred
progression as Hebrew Gematria — reflecting the shared Semitic numerical heritage — and continue
into thousands. The function `abjad_meaning(root)` provides root-number interpretations drawn from
Islamic numerological tradition. The mapping implemented here is a **phonetic approximation** to
Latin script; canonical Abjad operates on Arabic Unicode.

**Key source texts:**
- Ibn Khaldūn, *Muqaddimah* (1377; trans. Rosenthal, 1958, Princeton)
- Schimmel, A. *The Mystery of Numbers* (1993, OUP)
- Ibn ʿArabī, *Futūḥāt al-Makkiyya* (13th c.) — ʿilm al-ḥurūf tradition

---

## Enochian Angelology

The `enochian/` module covers the Angelic system received by John Dee (1527–1608) and his scryer
Edward Kelley during sessions between 1582 and 1587. Three sub-modules are exposed via
`enochian/mod.rs`:

- `alphabet` — ENOCHIAN_LETTERS static table, `enochian_lookup`, `enochian_substitute`,
  `show_enochian_table`
- `aethyrs` — AETHYRS static table, `aethyr_lookup`, `show_aethyr_table`, `show_aethyr_info`
- `messages` — `enochian_meaning`, `enochian_angelic_message`

### Alphabet

**Source file:** `src/enochian/alphabet.rs`

Dee's 21-letter alphabet was received in the angelic sessions of 1582–1583 via Kelley's
scrying, and appears in the *Book of Loagaeth* (Sloane MS 3189) and the *Holy Table* diagrams.
Each letter has a name, an English phonetic equivalent, an ordinal value (1–21), and a Golden
Dawn value.

**Enochian letter table:**

| # | Name | English | Ordinal | G.D. value |
|---|------|---------|---------|-----------|
| 1 | Un | A | 1 | 1 |
| 2 | Pa | B | 2 | 2 |
| 3 | Veh | C, K | 3 | 20 |
| 4 | Gal | D | 4 | 4 |
| 5 | Graph | E | 5 | 5 |
| 6 | Or | F | 6 | 80 |
| 7 | Ged | G | 7 | 3 |
| 8 | Na | H | 8 | 8 |
| 9 | Gon | I, J, Y | 9 | 10 |
| 10 | Ur | L | 10 | 30 |
| 11 | Tal | M | 11 | 40 |
| 12 | Drux | N | 12 | 50 |
| 13 | Med | O | 13 | 70 |
| 14 | Mals | P | 14 | 80 |
| 15 | Ger | Q | 15 | 100 |
| 16 | Don | R | 16 | 200 |
| 17 | Fam | S | 17 | 60 |
| 18 | Gisg | T | 18 | 400 |
| 19 | Van | U, V, W | 19 | 6 |
| 20 | Pal | X | 20 | 60 |
| 21 | Ceph | Z | 21 | 7 |

**Substitution rules (Elizabethan convention per Laycock):** J → I (Gon), K → C (Veh),
W → V (Van), Y → I (Gon).

**Key scholarly debates:**
1. **Pronunciation:** Reconstructions vary; Laycock's phonetic proposals remain the academic
   standard.
2. **Letter order:** The sequence used here follows Dee's own tables and Laycock's critical
   edition.
3. **Gematria:** Dee himself never specifies gematria values; both numerical systems in this
   application are post-Dee additions.

### Aethyrs

**Source file:** `src/enochian/aethyrs.rs`

The 30 Aethyrs (also spelled "Æthyrs") are celestial regions described in Dee's angelic
communications, particularly in the *Liber Scientiæ Auxilii et Victoriæ Terrestris*. Each Aethyr
has a three-letter name, numbered 1 (TEX, outermost) through 30 (LIL, innermost), with associated
Governors and angelic intelligences.

Aethyr lookup accepts either a number (1–30) or a name (e.g. `ZAX`, `LIL`). The CLI flag
`--aethyr` provides direct non-interactive access:

```bash
cargo run -- --aethyr ZAX      # look up by name
cargo run -- --aethyr 10       # look up by number
cargo run -- --aethyr          # show full table
```

When a numerology session computes an Enochian total, the application maps that total to an
Aethyr via modulo arithmetic and displays the Aethyr name and description inline.

### Angelic Calls (Keys)

**Source file:** `src/enochian/messages.rs` (root-number meanings);
call texts are embedded in `src/enochian/session.rs` under `browse_enochian_keys()`

The 19 Angelic Keys (Calls) are ritual invocations in the Enochian language received by Dee and
Kelley between April and July 1584 in Kraków and Prague. The 19th Call is the generic Aethyr
call, used to access each of the 30 Aethyrs by substituting the Aethyr name.

The texts displayed follow:
- John Dee, Cotton MS Appendix XLVI — primary manuscript source
- Crowley's *Liber Chanokh* (1912) — modernized spelling; minor textual variants
- Geoffrey James's critical edition (1984) — cross-references multiple manuscript copies

**Key source texts:**
- Dee, J. *A True and Faithful Relation…* (1659; ed. Meric Casaubon; facsimile, Magickal Childe, 1992)
- Dee, J. Sloane MS 3189 (*Liber Loagaeth* / *Book of Speech from God*), British Library
- Laycock, D. *The Complete Enochian Dictionary* (2001, Weiser)
- Crowley, A. *The Vision and the Voice* (Liber 418, 1909/1911)
- Regardie, I. *The Golden Dawn* (6th ed., 1989, Llewellyn) — Golden Dawn gematria values

---

## World Cosmologies

The `cosmology/` module is accessed from the main menu as option **4** and is further divided
into two sub-sessions: Chinese traditions and African traditions. The top-level dispatcher is
`run_world_systems_session()` in `cosmology/mod.rs`.

### Chinese Traditions

**Source file:** `src/cosmology/chinese.rs`

#### Nine Star Ki

A Japanese system derived from Chinese cosmology that assigns a natal "star" (1–9) to each
person based on their solar birth year, adjusted for the traditional new year around February 4
(Risshun). The nine stars cycle through a 3×3 magic square (Lo Shu), each associated with one
of the Five Elements, a compass direction, a trigram, and a set of personality and fate
attributes. The function `nine_star_ki_natal(year)` returns a `NineStarInfo` struct; the function
`nine_star_info(star)` provides the full description for a given star number.

#### Wu Xing (Five Elements)

The classical Chinese system of five dynamic phases: Wood (木), Fire (火), Earth (土), Metal (金),
Water (水). Each element generates the next (generating cycle) and controls an alternate (controlling
cycle). The function `wu_xing(n)` returns a `WuXingInfo` struct mapping a number 1–9 to its
element, season, direction, colour, organ, and generating/controlling relationships.

#### Ba Gua (Eight Trigrams)

The eight trigrams of the *I Ching*, each composed of three broken or unbroken lines, associated
with a natural phenomenon, a family member, a direction, and an element. Ba Gua associations are
used alongside Nine Star Ki natal readings.

#### Chinese Lucky and Unlucky Numbers

An overview of numbers considered auspicious (8, 6, 9) and inauspicious (4, pronounced similarly
to death in Mandarin) in Chinese cultural contexts. The function `chinese_lucky_meaning(n)` returns
a `ChineseLuckyInfo` struct.

**Key source texts:**
- Kushi, M. *Nine Star Ki* (1991, One Peaceful World Press)
- Yoshikawa, T. *The Ki* (1986, St. Martin's Press)
- Sachs, B. *Nine-Star Ki Astrology* (1992)
- Wilhelm, R. (trans.) *I Ching or Book of Changes* (1950, Princeton/Bollingen; Baynes English ed.)
- Needham, J. *Science and Civilisation in China*, Vol. 2 (1956, Cambridge)

---

### African Traditions

**Source file:** `src/cosmology/african.rs`

#### Yoruba Ifá

Ifá is the divination system of the Yoruba people of West Africa, transmitted through a corpus
of 256 Odù (sacred verses), each comprising a major Odù paired with a minor one in a 16×16
matrix. This application implements the **16 principal (Oju) Odù** — the senior corpus from
which all 256 are derived. The function `ifa_odu(index: u8)` accepts an index 1–16 and returns
an `IfaOdu` struct containing the Odù name, its associated Orisha, domain, and a description
of its character. The sequence follows Abimbola (1976) Oju Odù ordering.

#### Akan Day Names (Kra Names)

The Akan people of Ghana assign a soul name (*kra din*) to each person based on the day of
the week on which they were born. Each day is associated with a spiritual guardian (*kra*) and
carries character attributes. The function `akan_day_name(day_index)` returns an `AkanDay` struct
with the day name, spiritual guardian, and associated traits.

#### Kemetic Sacred Numbers

Ancient Egyptian numerological symbolism, including the significance of numbers in temple
architecture, cosmogony, and ritual. The function `kemetic_meaning(n)` returns a description
of the number's sacred significance within the Kemetic tradition.

**Key source texts:**
- Abimbola, W. *Ifá: An Exposition of Ifá Literary Corpus* (1976, OUP Nigeria)
- Bascom, W. *Ifa Divination: Communication Between Gods and Men in West Africa* (1969, IU Press)
- Gyekye, K. *An Essay on African Philosophical Thought: The Akan Conceptual Scheme* (1987, Cambridge)
- Morenz, S. *Egyptian Religion* (1973, Cornell; trans. Keep)
- Asante, M.K. *The Egyptian Philosophers* (2000, African American Images)

---

## Psi–RNG Experiment

**Source file:** `src/rng.rs`
**Main menu:** option **5**

The Psi–RNG module provides an interactive experiment for exploring the hypothesis that focused
human intention can measurably influence the output of a true hardware random number generator —
a question investigated experimentally since the 1970s, most extensively by the Princeton
Engineering Anomalies Research (PEAR) laboratory (1979–2007) under Robert Jahn and Brenda Dunne,
and before that by physicist Helmut Schmidt using electronic random event generators (REGs).

### Randomness source

The application attempts to use the **RDRAND CPU instruction** at session start. RDRAND is a
true hardware random number generator built into Intel processors since Ivy Bridge (2012) and
AMD processors since Ryzen (2017). It samples thermal noise from on-chip circuitry and passes
the result through a NIST SP 800-90A AES-CTR-DRBG conditioner before delivering a 32-bit value;
the raw thermal-noise sampling rate is approximately 3 Gbit/s. On CPUs that do not support
RDRAND, the application falls back silently to OS entropy (`getrandom`, backed by
`BCryptGenRandom` on Windows, `getrandom(2)` on Linux, or `/dev/urandom` on macOS).

The active source is displayed at the start of each session.

### Configuration

Before each session the user sets two parameters:

| Parameter | Options | Default |
|-----------|---------|---------|
| Number range | 1–9 · 1–10 · 1–100 · 1–1,000 · custom | 1–9 |
| Draw interval | 1–60 seconds (any integer or decimal) | 3 s |

The range is chosen first; then the delay. Both can be changed by starting a new session from
the main menu.

### Session mechanics

1. The user silently chooses a number within the configured range and holds it in mind.
2. Numbers are drawn automatically at the configured interval and printed to the terminal — the
   session does not pause between draws waiting for input.
3. A background thread reads stdin continuously; the main draw loop calls `recv_timeout` with the
   configured delay, so user responses are processed within the current draw window without
   interrupting the automatic timing.
4. **`Y` + Enter** — confirm that the displayed number matches the held intention. The session
   ends and statistics are shown.
5. **`Q` + Enter** — end the session without confirming a match. Statistics are shown.

The user does not declare their number in advance — the acknowledgment of a match is the only
signal. This mirrors the standard REG protocol used in PEAR lab studies.

### Statistics

After each session the following are reported:

| Statistic | Formula |
|-----------|---------|
| Chance expectation | N draws (geometric distribution mean, where N = range size and p = 1/N) |
| Match on draw k vs. expectation | k compared to N; difference stated in draws |
| Cumulative probability of a match by draw k | P(X ≤ k) = 1 − ((N − 1) / N)^k |
| No-match probability over the full session | P(X > total) = ((N − 1) / N)^total |

**Important:** a single trial cannot confirm or refute the psi hypothesis regardless of outcome.
The cumulative probability figures show how surprising the result would be under pure chance, but
only a series of independent trials analysed with appropriate statistics (e.g. binomial z-score
or meta-analytic effect size) constitutes meaningful evidence. The session note reminds users of
this after every run.

### Scientific context

The psi hypothesis — that conscious intention can shift the statistical output of a physical
random process — remains controversial in mainstream science. The PEAR lab reported small but
consistent anomalies in operator-REG studies over 12 years and ~2.5 million trials (Jahn et al.,
1997), with a mean effect size of approximately 1 part in 10,000. Independent replications have
produced mixed results; some meta-analyses find a small positive effect (Radin, 1997; Bösch,
Steinkamp & Boller, 2006), while others attribute the effect to methodological artefact or
publication bias (Alcock, 2003; Wiseman & Schlitz, 1997).

**Key source texts:**
- Jahn, R.G. & Dunne, B.J. *Margins of Reality: The Role of Consciousness in the Physical World*
  (1987, Harcourt Brace) — founding PEAR monograph
- Jahn, R.G. et al. "Correlations of Random Binary Sequences with Pre-Stated Operator Intention:
  A Review of a 12-Year Program," *Journal of Scientific Exploration* 11, no. 3 (1997): 345–367
- Schmidt, H. "PK Tests with a High-Speed Random Number Generator,"
  *Journal of Parapsychology* 37 (1973): 105–118 — foundational REG experiment
- Radin, D. *The Conscious Universe: The Scientific Truth of Psychic Phenomena*
  (1997, HarperCollins) — meta-analytic overview
- Bösch, H., Steinkamp, F. & Boller, E. "Examining Psychokinesis: The Interaction of Human
  Intention with Random Number Generators — A Meta-Analysis,"
  *Psychological Bulletin* 132, no. 4 (2006): 497–523
- Intel Corporation. *Intel® 64 and IA-32 Architectures Software Developer's Manual*, Vol. 1,
  §7.3.17 — RDRAND instruction specification

---

## Sacred Frequencies

**Source file:** `src/audio.rs`

The `audio` module synthesizes pure sine-wave tones and stereo binaural-beat WAV files using
the `rodio` crate. An `AudioSystem` wraps a `rodio` output stream and a shared `Sink`; frequency
changes during a numerology session swap the source without stopping the stream.

### Solfeggio Frequencies

| Hz | Traditional attribution | Digital root |
|----|------------------------|--------------|
| 285 | Healing & Regeneration | 6 |
| 396 | Liberation from Fear | 9 |
| 417 | Facilitating Change | 3 |
| 432 | Universal Harmony | 9 |
| 528 | Love & DNA Repair | 6 |
| 639 | Connecting Relationships | 9 |
| 741 | Awakening Intuition | 3 |
| 852 | Returning to Spiritual Order | 6 |
| 963 | God Consciousness / Pineal | 9 |

The ambient frequency at session start defaults to 432 Hz. As each word is analyzed, the
frequency is automatically retuned to the Solfeggio pitch corresponding to the word's digital root.

**Scholarly note:** The Solfeggio scale in its current popular form was codified by Joseph Puleo
(*Healing Codes for the Biological Apocalypse*, with Leonard Horowitz, 1999). Claims that these
frequencies repair DNA or activate the pineal gland are not supported by peer-reviewed biomedical
literature. They are used here as a meditative and aesthetic framework.

### Binaural Beats

Binaural beats are generated by playing two sine waves of slightly different frequencies — one in
each stereo channel. The perceptual beat at the difference frequency (default: 6 Hz, theta range)
is processed by the brain rather than the ear. The auditory mechanism is well-documented (Oster,
1973); whether the resulting entrainment produces the meditative states claimed by practitioners
remains an active area of research.

The interactive export menu (option **4**) exposes:
- Per-frequency WAV export (individual Solfeggio tones)
- Export all nine Solfeggio frequencies at once
- Custom binaural beat creation (user-specified base frequency and beat frequency)

**WAV export details:** Files are written to `exports/<name>.wav` at 44 100 Hz, 16-bit PCM,
mono (pure tone) or stereo (binaural). The CLI flag `--export-all` runs a non-interactive
batch export of all nine frequencies.

**References:**
- Oster, G. "Auditory Beats in the Brain," *Scientific American* 229, no. 4 (1973): 94–102
- Puleo, J. & Horowitz, L. *Healing Codes for the Biological Apocalypse* (Tetrahedron, 1999)
- Nolan, J. "Concert Pitch A=432 Hz: A Musicological Perspective" (2014)

---

## Getting Started

**Prerequisites:** Rust 1.70+ (stable toolchain)

```bash
git clone <repo-url>
cd cosmic-knowledge
cargo build --release
cargo run --release
```

On Linux you may need ALSA/PulseAudio development headers:

```bash
# Ubuntu / Debian
sudo apt install libasound2-dev

# Fedora / RHEL
sudo dnf install alsa-lib-devel
```

On macOS, CoreAudio is used automatically — no additional packages are required.

On Windows, the default audio backend (WASAPI) is used; no additional setup is needed.

To run the test suite:

```bash
cargo test
cargo clippy    # linting
cargo fmt       # formatting
```

---

## CLI Flags

| Flag | Short | Effect |
|------|-------|--------|
| `--fast` | `-f` | Skip the loading animation and go directly to the main menu |
| `--silent` | `-s` | Disable audio entirely; frequency export is also unavailable |
| `--export-all` | — | Non-interactively export all nine Solfeggio frequencies as WAV, then exit |
| `--aethyr <query>` | — | Look up an Aethyr by name or number and print info, then exit |
| `--aethyr` | — | Print the full Aethyr table, then exit |
| `--help` | `-h` | Print help text and exit |

All flags may be combined where compatible. For example:

```bash
cargo run --release -- --fast --silent
cargo run -- --aethyr ZAX
cargo run -- --export-all
```

---

## Exports

All exported files are written to the `exports/` directory, which is created automatically on
first save. The directory is not tracked by version control (add it to `.gitignore` if desired).

| File pattern | Format | Content | Trigger |
|---|---|---|---|
| `exports/numerology_<word>.txt` | Text | Plain-text multi-system report with per-letter breakdown | Save prompt after numerology analysis |
| `exports/numerology_<word>.html` | HTML | Styled multi-system report with cultural theming | Save prompt after numerology analysis |
| `exports/numerology_<word>.pdf` | PDF | Printpdf-generated report (Courier, paginated) | Save prompt after numerology analysis |
| `exports/enochian_translation_<word>.txt` | Text | Letter-by-letter Enochian rendering + gematria | Save prompt after translation |
| `exports/enochian_translation_<word>.html` | HTML | Styled Enochian translation report | Save prompt after translation |
| `exports/enochian_gematria_<word>.txt` | Text | Enochian-only gematria values | Save prompt after Enochian gematria |
| `exports/enochian_gematria_<word>.html` | HTML | Styled Enochian gematria report | Save prompt after Enochian gematria |
| `exports/enochian_key_<num>.txt` | Text | Full Angelic Key text and translation | Save prompt in Keys browser |
| `exports/<freq>Hz_<name>_pure_5min.wav` | WAV | 5-minute mono pure-tone Solfeggio | Frequency export → option 1 |
| `exports/<freq>Hz_<name>_binaural_10min.wav` | WAV | 10-minute stereo binaural beat | Frequency export → option 2, or `--export-all` |
| `exports/<freq>Hz_<name>_extended_30min.wav` | WAV | 30-minute stereo binaural beat | Frequency export → option 3 |
| `exports/custom_<base>Hz_<beat>beat_<min>min.wav` | WAV | Custom stereo binaural beat | Custom binaural beat export |

File stems are sanitized to alphanumerics, hyphens, and underscores to prevent path traversal.
The user may accept the suggested stem or provide a custom name at the save prompt.

---

## Source Texts and Scholarly References

### Manuscripts

| Shelfmark | Contents | Used for |
|-----------|----------|----------|
| British Library, Cotton MS Appendix XLVI | Dee's angelic diaries (1582–1587) | 19 Keys, Aethyr names and governors |
| British Library, Sloane MS 3189 | *Liber Loagaeth* (*Book of Speech from God*) | Enochian letter forms and alphabet order |
| British Library, Sloane MS 3188 | Dee's private diaries | Cross-reference for angelic sessions |

### Numerology

- Kaplan, A. *Sefer Yetzirah* (1990, Weiser Books) — Hebrew Gematria
- Blumenthal, D. *Understanding Jewish Mysticism* (1978, Ktav) — Hebrew Gematria
- Munk, M. *The Wisdom in the Hebrew Alphabet* (1983, ArtScroll) — Hebrew Gematria
- Nicomachus of Gerasa, *Introduction to Arithmetic* (c. 100 CE) — Pythagorean; Greek Isopsephy
- Taylor, T. *The Theoretic Arithmetic of the Pythagoreans* (1816) — Pythagorean
- Burkert, W. *Lore and Science in Ancient Pythagoreanism* (1972, Harvard) — Pythagorean
- Cheiro (Count Louis Hamon), *Cheiro's Book of Numbers* (1926) — Chaldean
- Schimmel, A. *The Mystery of Numbers* (1993, OUP) — Chaldean; Abjad; comparative context
- Dornseiff, F. *Das Alphabet in Mystik und Magie* (1925, Teubner) — Greek Isopsephy
- Iamblichus, *Theology of Arithmetic* (c. 300 CE; trans. Waterfield, 1988, Phanes) — Greek Isopsephy
- Agrippa, H.C. *De Occulta Philosophia Libri Tres* (1531; trans. Tyson, 1993, Llewellyn) — Agrippan
- Barrett, F. *The Magus, or Celestial Intelligencer* (1801; facsimile, Weiser, 1967) — Agrippan
- Ibn Khaldūn, *Muqaddimah* (1377; trans. Rosenthal, 1958, Princeton) — Abjad
- Ibn ʿArabī, *Futūḥāt al-Makkiyya* (13th c.) — Abjad / ʿilm al-ḥurūf tradition

### Enochian

- Dee, J. *A True and Faithful Relation…* (1659; ed. Casaubon; facsimile, Magickal Childe, 1992)
- Laycock, D. *The Complete Enochian Dictionary* (2001, Weiser) — **primary scholarly reference**
- Crowley, A. *The Vision and the Voice* (Liber 418, 1909/1911)
- Regardie, I. *The Golden Dawn* (6th ed., 1989, Llewellyn) — Golden Dawn gematria values
- James, G. *The Enochian Evocation of Dr. John Dee* (1984, Heptangle) — critical edition

### Chinese Cosmology

- Kushi, M. *Nine Star Ki* (1991, One Peaceful World Press)
- Yoshikawa, T. *The Ki* (1986, St. Martin's Press)
- Sachs, B. *Nine-Star Ki Astrology* (1992)
- Wilhelm, R. (trans.) *I Ching or Book of Changes* (1950, Princeton/Bollingen; Baynes English ed.)
- Needham, J. *Science and Civilisation in China*, Vol. 2 (1956, Cambridge)

### African Traditions

- Abimbola, W. *Ifá: An Exposition of Ifá Literary Corpus* (1976, OUP Nigeria)
- Bascom, W. *Ifa Divination: Communication Between Gods and Men in West Africa* (1969, IU Press)
- Gyekye, K. *An Essay on African Philosophical Thought: The Akan Conceptual Scheme* (1987, Cambridge)
- Morenz, S. *Egyptian Religion* (1973, Cornell; trans. Keep)
- Asante, M.K. *The Egyptian Philosophers* (2000, African American Images)

### Psi Research

- Jahn, R.G. & Dunne, B.J. *Margins of Reality: The Role of Consciousness in the Physical World* (1987, Harcourt Brace)
- Jahn, R.G. et al. "Correlations of Random Binary Sequences with Pre-Stated Operator Intention: A Review of a 12-Year Program," *Journal of Scientific Exploration* 11, no. 3 (1997): 345–367
- Schmidt, H. "PK Tests with a High-Speed Random Number Generator," *Journal of Parapsychology* 37 (1973): 105–118
- Radin, D. *The Conscious Universe: The Scientific Truth of Psychic Phenomena* (1997, HarperCollins)
- Bösch, H., Steinkamp, F. & Boller, E. "Examining Psychokinesis: The Interaction of Human Intention with Random Number Generators — A Meta-Analysis," *Psychological Bulletin* 132, no. 4 (2006): 497–523
- Intel Corporation. *Intel® 64 and IA-32 Architectures Software Developer's Manual*, Vol. 1, §7.3.17 — RDRAND instruction

### Sacred Frequencies

- Oster, G. "Auditory Beats in the Brain," *Scientific American* 229, no. 4 (1973): 94–102
- Puleo, J. & Horowitz, L. *Healing Codes for the Biological Apocalypse* (Tetrahedron, 1999)
- Nolan, J. "Concert Pitch A=432 Hz: A Musicological Perspective" (2014)

---

## Accuracy Notes

The following notes document where the implementation diverges from classical practice, uses
approximations, or reflects modern rather than ancient constructions.

1. **Abjad (Latin approximation).** The Abjad mapping is a phonetic approximation to the Latin
   script. Canonical Abjad numerology operates on Arabic Unicode characters. Results obtained
   with Latin input should be treated as approximations useful for comparative study only.

2. **Greek Isopsephy (Convention B).** The Greek Isopsephy system uses Convention B — Latin
   phonetic equivalents — for ASCII input. Native Greek Unicode input gives exact classical
   results and can be entered directly at the word prompt.

3. **Enochian Ordinal vs. Golden Dawn.** Enochian Ordinal (positional 1–21) is the most
   historically defensible system; the positional sequence is directly attested in Dee's
   manuscripts. The Golden Dawn values are a 19th-century retrofit attributed to S. L. MacGregor
   Mathers and systematized by Regardie — they are not found in Dee's original diaries or in
   the *Liber Loagaeth*.

4. **Simple and Reverse Ordinal.** These are modern English constructs with no classical
   antecedent in any ancient tradition. They are included for completeness and cross-system
   comparison.

5. **Hebrew Gematria (Latin mapping).** Classical Hebrew gematria operates on Hebrew script.
   The Latin-letter mapping used here is a transliteration convention from Western occultism
   and involves assumptions that scholars disagree on. Results should not be compared directly
   to classical Hebrew gematria without accounting for the transliteration layer.

6. **Chaldean provenance.** The historical connection of the Chaldean system to ancient
   Babylonian divination is not documented in Assyriological literature. The system as
   implemented follows Cheiro's 1926 codification, which itself is not clearly traceable to
   cuneiform sources. It should be understood as a 19th-century Western esoteric construction
   with claimed ancient provenance.

7. **Nine Star Ki — Risshun adjustment.** Nine Star Ki uses the Risshun (立春) adjustment: the
   traditional solar year begins around February 4. Births before approximately February 4 of
   a given calendar year may belong to the prior solar year's star. The application applies this
   adjustment automatically.

8. **Ifá Odù sequence.** The Ifá Odù sequence follows Abimbola (1976) Oju Odù ordering. Other
   Yoruba and diaspora traditions use different sequences; comparisons between traditions should
   note which ordering is in use.

9. **Pythagorean cycle.** Pythagorean values follow the mod-9 alphabetic cycle: A=1 … I=9,
   J=1 … R=9, S=1 … Z=8. Master numbers (11, 22, 33, etc.) are recognized as totals but are
   still fully reduced by the `digital_root` function; detection and display of master numbers
   is handled separately by `master_numbers_message`.

10. **Solfeggio frequency claims.** The Solfeggio frequencies and their attributed healing or
    spiritual properties derive from modern esoteric literature (Puleo/Horowitz, 1999), not from
    classical solfège (ut-re-mi), which uses a different set of pitches. No peer-reviewed
    biomedical evidence supports DNA repair, pineal activation, or measurable spiritual effects
    at these specific frequencies.

11. **Psi–RNG experiment.** The experiment is provided as an interactive exploration tool, not
    as a validated measurement instrument. Single-trial outcomes — whether early matches or long
    runs without a match — cannot be interpreted as evidence for or against psychokinetic
    influence on the RNG. The reported cumulative probability figures describe how surprising a
    result would be under pure chance; they are not p-values from a controlled study. Meaningful
    evidence requires many independent trials, pre-registration of the hypothesis, and analysis
    with appropriate statistics. The psi hypothesis itself remains scientifically contested (see
    the Psi Research references above).

---

## Contributing

Bug reports, translation corrections, and scholarly source additions are welcome. When submitting
corrections to letter values or historical attributions, please cite a primary source or a
peer-reviewed secondary source.

```bash
cargo test      # run all unit tests
cargo clippy    # linting
cargo fmt       # formatting
```
