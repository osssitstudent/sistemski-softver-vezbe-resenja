# Uticaj predviđanja grananja

Svi savremeni procesori koriste mehanizme za predviđanje grananja
(_branch prediction_), čime se postiže bolje iskorišćenje procesora
u realnom kodu. Ovaj zadatak je praktična demonstracija tih
mehanizama na preuveličanom primeru.

Program koji ćete prevesti i pokrenuti više puta prolazi kroz niz
slučajno izabranih 8-bitnih neoznačenih vrednosti i računa zbir onih
koje su veće od 128. U osnovnoj konfiguraciji, vrednosti su nesortirane.
Vaš zadatak je da:

1. Prevedete program za računanje zbira.

2. Pokrenete program na nesortiranim ulaznim vrednostima i izračunate
   srednju vrednost vremena tri uzastopna izvršavanja programa.
   
3. Sortirate ulazne vrednosti.

4. Pokrenete program na sortiranim ulaznim vrednostima i izračunate
   srednju vrednost vremena tri uzastopna izvršavanja programa.

Pripremni koraci i očekivan format rešenja opisani su u nastavku
teksta.

## Kloniranje repozitorijuma sa zadatkom

U direktorijumu `~/zadaci` pokrenite `git clone` komandu koja će
kopirati zadatak na lokalno okruženje. Pređite u direktorijum gde
se zadatak nalazi. Njegovo ime je `2-grananje-<nalog>`, gde je
`<nalog>` Vaše korisničko ime na GitHub-u.

## Prevođenje programa

Program za računanje zbira prevodi se komandom `cargo build`. Po
završetku prevođenja, program se pokreće sa `cargo run --quiet <fajl>`,
gde je `<fajl>` ime fajla u kome su ulazni podaci. Po završetku rada,
program ispisuje izračunati zbir.

## Ulazni podaci

Ulazni podaci su u fajlu `~/data/2.input`. Kopirajte taj fajl u tekući
direktorijum komandom `cp`. Dodajte ga u repozitorijum sa `git add`.
Fajl se sastoji od 8192 linije, a svaka od njih sadrži dvocifrenu
heksadecimalnu vrednost između 0 i 255.

## Merenje vremena

Vreme izvršavanja nekog programa meri se tako što se pre imena programa
navede komanda `time`: recimo, `time cargo run --quiet`. Po završetku, ispisuju
se tri vrednosti, od kojih treba uzeti prvo, naslovljeno __real__.
Zabeležite rezultate tri izvršavanja, izračunajte srednju vrednost,
i zapišite je u fajl pod imenom `time.input`, koji ćete isto dodati sa
`git add`.

Fajl mora imati jedan red u kome se nalazi vrednost. U tom
redu ne sme biti suvišnih razmaka. Vrednost mora biti zapisana kao
decimalni broj sa dve decimale, a separator celobrojnog i decimalnog
dela mora biti tačka (`.`).

__Ovo je prvi deo rešenja: 3 poena.__

## Sortiranje vrednosti

Upotrebite način koji god želite. Vrednosti treba da budu sortirane u
rastućem redosledu. Rezultat sortiranja treba da bude fajl
pod imenom `2.sorted`, koji takođe treba dodati sa `git add`. __Ovo je drugi
deo rešenja: 5 poena.__

## Ponovno merenje vremena

Izmerite srednje vreme tri izvršavanja sa sortiranim podacima i upišite
vrednost u fajl `time.sorted`, koji ćete dodati sa `git add`. Za format
fajla, pogledajte opis u odeljku _Merenje vremena_. __Ovo je treći deo
rešenja: 2 poena.__

## Objavljivanje promena

Da bi se promene koje ste napravili priznale, morate da uradite `git commit`
i objavite ih sa `git push`.
