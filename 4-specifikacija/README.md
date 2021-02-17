# Tumačenje specifikacije RISC-V skupa instrukcija

Važna sposobnost programera je razumevanje specifikacija različitih vrsta.
Za sistemsko programiranje to može biti specifikacija uređaja ili protokola.

## Zadatak

Preuzmite [specifikaciju](https://content.riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf)
korisničkog skupa instrukcija za RISC-V procesorsku arhitekturu. Pažljivo proučite sve do
odeljka 2.5 (_Control Transfer Instructions_), ali preskočite odeljak 2.3
(_Immediate Encoding Variants_).

Ispod se nalazi spisak od deset pitanja.

* Ako se Vaš broj indeksa završava cifrom 0 ili 5, odgovorite na pitanja 1 i 6.

* Ako se Vaš broj indeksa završava cifrom 1 ili 6, odgovorite na pitanja 2 i 7.

* Ako se Vaš broj indeksa završava cifrom 2 ili 7, odgovorite na pitanja 3 i 8.

* Ako se Vaš broj indeksa završava cifrom 3 ili 8, odgovorite na pitanja 4 i 9.

* Ako se Vaš broj indeksa završava cifrom 4 ili 9, odgovorite na pitanja 5 i 10.

Odgovore zapišite u fajl _odgovori.txt_, svaki odgovor u posebnom redu, sa
rednim brojem pitanja na početku reda, jednim razmakom iza tačke u rednom
broju, i bez razmaka iza odgovora. Ovaj fajl napravite lokalno i onda ga
objavite u repozitorijumu na GitHub-u.

Većina odgovora je numerička; u fajlu ti odgovori treba da budu zapisani kao
decimalni broj. Izvestan broj odgovora zahteva da se navede jedna instrukcija;
zapišite tu instrukciju velikim slovima.

Svako pitanje na koje odgovorite nosi __5 bodova__. (Ako ispravno odgovorite
na oba pitanja, osvojićete 10 bodova.)

Za dodatnih __5 bodova__, treba usmeno da objasnite načine za detektovanje
prekoračenja kod ovog procesora.

## Pitanja

1. Koja je širina osnovnih instrukcija za skup RV64I, u bitima

2. Koja je širina kompaktnih instrukcija za skup RV32I, u bitima

3. Koliko ima registara opšte namene u skupu RV32I

4. Koja je vrednost registra x0

5. Koje su vrednosti najniža dva bita za 32-bitne instrukcije u osnovnom skupu

6. Koliko operanada ima registarska (R) instrukcija

7. Koliko je uzastopnih instrukcija potrebno da bi se u registar učitala 32-bitna
   vrednost

8. Kako bi glasila instrukcija koja postavlja x1 na 1 ako je x2 jednako 0

9. Kako bi glasila instrukcija koja postavlja x1 na 0 ako je x2 jednako 0

10. Kako se kodira instrukcija bez efekta
