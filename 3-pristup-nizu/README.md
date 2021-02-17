# Uticaj keširanja na pristup memoriji

Za očekivane performanse pristupa podacima u memoriji neophodno je
da što veći broj pristupa bude posredstvom keša za podatke. Procesori
koriste razne mehanizme za predviđanje načina pristupa i u stanju su
da obezbede značajno iskorišćenje keša. Moguće je konstruisati
programe koji namerno pristupaju memoriji na način koji će izazvati
promašaje u kešu, radi ilustracije.

Program koji ćete pokretati alocira niz od 8 MB, i pristupa, na preskok,
pojedinačnim elementima. Korak za porast indeksa je promenljiv, i treba
da se menja eksponencijalno: prvo sukcesivni bajtovi, pa svaki drugi,
svaki četvrti... svaki prolaz kroz niz se ponavlja određen broj puta.

Vaš zadatak je da:

1. Prepravite program tako da povećava korak po eksponencijalnoj
   progresiji i ispisuje rezultate u traženom formatu.

2. Prevedete program i pokrenete ga.

3. Zabeležite i protumačite rezultate.

Pripremni koraci i očekivan format rešenja opisani su u nastavku
teksta.

## Kloniranje repozitorijuma sa zadatkom

U direktorijumu `~/zadaci` pokrenite `git clone` komandu koja će
kopirati zadatak na lokalno okruženje. Pređite u direktorijum gde
se zadatak nalazi.

## Prevođenje programa

Program za računanje zbira prevodi se komandom `cargo build --release`.
Po izvršavanju, u direktorijumu `target/release` treba da se nalazi fajl
`awalk`.

## Izvršavanje programa

Program se pokreće sa `cargo run --quiet --release`. Po izvršavanju, za
svaku iteraciju ispisuju se:

* Redni broj iteracije

* Proteklo vreme za iteraciju, u nanosekundama

## Očekivani format izlaznih podataka

Umesto broja nanosekundi, u ispisu za svaku iteraciju treba da se nalazi
mera nanosekundi po operaciji, što znači da treba na odgovarajući način
prepraviti izraz koji se ispisuje u okviru komande `println!`. Sve
potrebne vrednosti za izračunavanje već su na raspolaganju u trenutku
izvršavanja `println!`. Obratite pažnju na to da tip vrednosti `nanos`
za ispravno izračunavanje mora da se pretvori u `f64` -- pogledajte
kako se transformacija radi u drugom delu programa.

Treba da napravite fajl, `timing.csv`, koji će sadržati rezultat
izvršavanja programa. Fajl treba da dodate u repozitorijum.
__Ovo je prvi deo rešenja: 5 poena.__

Treba editujete ovaj fajl (README.md) i označite broj iteracije gde
vreme izvršavanja dostiže drugi plato. Označavanje se radi tako što
upišete `X` između uglastih zagrada pored broja iteracije u nastavku
teksta.

- [ ] 0
- [ ] 1
- [ ] 2
- [ ] 3
- [ ] 4
- [ ] 5
- [ ] 6
- [ ] 7
- [ ] 8
- [ ] 9
- [ ] 10
- [ ] 11
- [ ] 12
- [ ] 13
- [ ] 14
- [ ] 15

Takođe, upišite prosečno vreme izvršavanja u tom trenutku: NN.NN ns/op.
Promenu pošaljite na GH. __Ovo je drugi deo rešenja: 5 poena.__
