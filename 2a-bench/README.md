# Merenje performansi alatima u okviru jezičkog okruženja

U prethodnom zadatku je za merenje performansi programa sa različitim
ulaznim podacima bila korišćena Unix komanda `time`. Ukoliko je softver
koji se razvija predviđen za rad na više platformi, ovakav način merenja
ne može u opštem slučaju svugde da se primeni. Zato neki programski
jezici imaju izvesne tipove alata za merenje performansi koji su
dostupni iz osnovnog jezičkog okruženja i nezavisni od platforme. Za
programski jezik Rust, korišćen u prethodnom zadatku, to je `bench`
podsistem.

Za izradu ovog zadatka, trebaće Vam fajlovi korišćeni u prethodnom,
`2.input` i `2.sorted`. Vaš zadatak je da:

1. Prevedete i pokrenete program za merenje performansi.

2. Modifikujete program tako da omogućite merenje performansi za
   još jedan ulazni fajl.

3. Izračunate odnose vremenâ izvršavanja iteracija za nesortirane
   i sortirane ulazne podatke.

Koraci su navedeni u nastavku teksta.

## Kloniranje repozitorijuma sa zadatkom

U direktorijumu `~/zadaci` pokrenite `git clone` komandu koja će
kopirati zadatak na lokalno okruženje. Pređite u direktorijum gde
se zadatak nalazi. Njegovo ime je `2a-bench-<nalog>`, gde je
`<nalog>` Vaše korisničko ime na GitHub-u.

## Prevođenje programa

Program se prevodi i pokreće sa `cargo +nightly bench`. Rezultat
merenja je prikazan u jedinicama ns/iter (nanosekunde po iteraciji).
Da bi početna verzija mogla da se prevede i izvrši, na pravo mesto
u repozitorijumu sa zadatkom mora se kopirati fajl `2.input`.
Odredite to mesto.

## Izmena programa

Program se nalazi u fajlu `src/lib.rs`, relativno u odnosu na osnovni
direktorijum repozitorijuma. U njemu se nalazi funkcija `unsorted` koja
meri brzinu rada sa nesortiranim nizom vrednosti. Modifikujte fajl tako
što ćete napraviti analognu funkciju, `sorted`, koja će kao ulaz
koristiti fajl `2.sorted`, dobijen u prethodnom zadatku. Funkcija mora
biti označena atributom `#[bench]`.

Promenu fajla `src/lib.rs` zabeležite u repozitorijumu (`git add`,
`git commit`).

## Ponovno pokretanje

Kad se ispravno modifikovan program ponovo pokrene, treba da prikaže
rezultate merenja za obe funkcije, `sorted` i `unsorted`. Pokrenite
program nekoliko puta, dok ne dobijete merenja kod kojih je varijacija
za obe funkcije ispod 1.000.000 ns. Izračunajte odnos vremenâ za sortirane
i nesortirane podatke. Zapišite odnos kao decimalni broj sa najviše dve
decimale, bez jedinica, u fajl `iter.ratio` u osnovnom direktorijumu
repozitorijuma.

## Objavljivanje

Fajlove `2.input`, `2.sorted` i `iter.ratio` dodajte u repozitorijum.
Sve promene objavite na GitHub-u sa `git push`.

## Bodovanje

Zadatak nosi __5 poena__.
