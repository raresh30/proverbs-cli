const PROVERBS: [&str; 313] = [
    "A aduce apă după ce s-a stins focul.",
    "A aduna nuiele pentru spinarea sa.",
    "A ajunge cuțitul la os.",
    "A auzit clopotul, dar nu știe de la ce bisericǎ.",
    "A avea ac de cojocul cuiva.",
    "A avea mai mult noroc decât minte.",
    "A bate apa-n piuă",
    "A bate calul care trage.",
    "A bate fierul cât e cald.",
    "A bate la ochi.",
    "A bate toba în tot satul.",
    "A călca pe bec.",
    "A cânta cuiva în strună.",
    "A căra apă cu ciurul.",
    "Aceași Mărie cu altă pălărie.",
    "Acela care Îl iubește pe Dumnezeu, îi iubește și pe oameni.",
    "A da bir cu fugiții.",
    "A da peste noroc.",
    "A da perle la porci.",
    "A despica firul în patru.",
    "A duce cu preșul.",
    "Adevărul este întotdeauna la mijloc.",
    "Adevărul se spune la despărțire.",
    "Adevărul stă scris printre rânduri.",
    "A dispărut de parcă l-a înghițit pământul.",
    "A face bortă în apă.",
    "A face cruce în tavan.",
    "A face cuiva hatârul.",
    "A face cum îl taie capul.",
    "A face din cal măgar și din țânțar armăsar.",
    "A face ochi dulci cuiva.",
    "A face pe cineva cu ou și cu oțet.",
    "A face pe cineva de două parale.",
    "A face treaba în doi peri.",
    "A face umbră pământului degeaba.",
    "A face un bine înseamnă a-și bate cuie în talpă.",
    "A fi cu ochii în patru.",
    "A fi pe drojdie (cu banii).",
    "A fi prins cu cioara vopsită.",
    "Ai ales pân-ai cules.",
    "Ai carte, ai parte.",
    "Ai, dai, n-ai. Ia nu da, să vezi cum ai.",
    "A dat-o cotită.",
    "A-i fi frică și de umbra sa.",
    "A împăca capra cu varza.",
    "A împușca doi iepuri dintr-un foc.",
    "A pune carul înaintea boilor.",
    "A turna gaz pe foc.",
    "Ajută-te singur și Domnul te va ajuta.",
    "Apa trece, pietrele rămân.",
    "Are un car cu boi și o sută de nevoi.",
    "Ascultă tot, dar nu crede tot.",
    "Așchia nu sare departe de trunchi.",
    "Avocatul nepoftit este bun de pălmuit.",
    "Baba/Surdul n-aude, dar le potrivește.",
    "Banii n-au miros.",
    "Banu-i ochiul dracului.",
    "Bate fierul cât e cald.",
    "Bătaia e ruptă din rai.[1][2][3][4][5]",
    "Bătrânețe - haine grele.",
    "Bine faci, bine găsești.",
    "Boii bătrâni fac brazda dreaptă[6]",
    "Bunul gospodar își face vara sanie și iarna car.",
    "Buturuga mică răstoarnă carul mare.",
    "Ca musca la arat.",
    "Calul de dar nu se caută la dinți.",
    "Capra sare masa, iada sare casa.",
    "Capul ce se pleacă, sabia nu-l taie.",
    "Cască ochii la tocmeală, iar nu după ce te-nșeală.",
    "Cămașa e mai aproape de piele decât sumanul/haina.",
    "Caută o femeie care-ți place ție, nu la alții.",
    "Călătorului îi șade bine cu drumul.",
    "Câinele care latră nu mușcă.",
    "Câinele moare de drum lung și prostul de grija altora.",
    "Câinii latră, ursul merge.",
    "Când de multe te apuci mai pe toate le încurci.",
    "Când doi îți spun că ești beat, du-te de te culcă.",
    "Când doi se ceartă al treilea câștigă.",
    "Când pisica nu-i acasă, șoarecii joacă pe masă.",
    "Când omul cade, îi piere și umbra.",
    "Când râde prostul, înțeleptul suspină.",
    "Câte bordeie, atâtea obiceie.",
    "Ce-am avut și ce-am pierdut!",
    "Ce mi-e baba Rada, ce mi-e Rada baba!",
    "Ce naște din pisică, șoareci mănâncă.",
    "Ce ție nu-ți place, altuia nu-i face.",
    "Ce-i în gușă și-n căpușă.",
    "Ce-i în mână nu-i minciună.",
    "Ce-și face omul singur nici dracul nu poate desface.",
    "Cel flămând se visează mâncând.",
    "Cele rele să se spele, cele bune să se-adune.",
    "Cine aleargă după doi iepuri nu prinde niciunul.",
    "Cine adună la tinerețe are la bătrânețe.",
    "Cine are carte, are parte.",
    "Cine are/face carte are patru ochi.",
    "Cine are prieten nărod ajunge din pod în glod.",
    "Cine dă nu uită, uită cel care ia.",
    "Cine fură azi un ou, mâine va fura un bou.",
    "Cine împarte, parte își face.",
    "Cine întreabă nu dă bucuros.",
    "Cine poate oase roade, cine nu, nici carne moale.",
    "Cine râde la urmă râde mai bine.",
    "Cine nu are un bătrân, să își cumpere.",
    "Cine nu-ncearcă, nici nu câștigă.",
    "Cine sapă groapa altuia, cade singur în ea.",
    "Cine se frige cu ciorba, suflă și în iaurt.",
    "Cine seamănă vânt, culege furtună.",
    "Cine se aseamănă, se adună.",
    "Cine se scoală de dimineață, departe ajunge.",
    "Cine se scuză, se acuză.",
    "Cine stă în casă de sticlă, nu aruncă cu pietre.",
    "Cine știe carte, are patru ochi.",
    "Copilul și bețivul spun adevărul.",
    "Corb la corb nu scoate ochii.",
    "Corb, corbului ochi nu scoate.",
    "Cu fundul în două luntre.",
    "Cu o floare nu se face primăvară.",
    "Cu tacerea ii frigi pielea.",
    "Cui pe cui scoate.",
    "Cui îi dai pe datorie nu-ți mai intră-n prăvălie / nu-l mai vezi în prăvălie.",
    "Cu un ochi râde și cu altul plânge.",
    "Cum îți așterni așa dormi.",
    "Cum se scoală - cum i-e foame, cum se culcă - cum adoarme.",
    "Cum tu mie, așa eu ție.",
    "Cum e stăpânul așa-i și sluga.",
    "Cum e turcul și pistolul.",
    "Cu răbdarea treci și marea.",
    "Dacă copilul nu plânge, mama nu-i dă țâță.",
    "Dacă doi îți spun că ești beat, du-te și te culcă.",
    "Dacă vrei să faci o meserie, îți trebuie scule, dacă vrei să ai familie, îți trebuie oameni.",
    "Dacă n-ajunge, mai rămâne.",
    "Dacă tăceai, filosof rămâneai.",
    "Dar din dar se face rai.",
    "Dați cezarului, ce-i al cezarului.",
    "De când lumea, râsul te râde și batjocura te batjocorește.",
    "De ce e mai râioasă capra, de aia stă cu coada mai sus.",
    "Dă-i nas lui Ivan, că se suie pe divan.",
    "De la un datornic rău și un sac de paie e bun.",
    "Din talpa casei cerc de bute nu se poate face.",
    "Din talpa casei nu poți face doage, nici din coadă de câine sită de mătase.",
    "Din talpa casei nu se face obadă de roată.",
    "După mine, potopul!",
    "După război mulți viteji se arată.",
    "E bună povața de bătrân și părinte.",
    "E bun numai când doarme.",
    "E învățat (cu greutățile) ca țiganul cu ciocanul (scânteia).",
    "Excepția întărește/confirmă regula.",
    "Fă bine și așteaptă răul.",
    "Ferește-mă Doamne, de prieteni, că de dușmani mă feresc singur.",
    "Ferește-te de proști, pentru că au mintea odihnită.",
    "Fă-te frate cu dracul până treci puntea.",
    "Ferește-te de câinele mut și de omul tăcut.",
    "Fiecare greșește, dar numai prostul repetă greșeala.",
    "Foamea e cel mai bun bucătar.",
    "Frate, frate, dar brânza e pe (costă) bani.",
    "Frâul de aur nu face calul mai bun.",
    "Fuga e rușinoasă, dar sănătoasă.",
    "Găina bătrână face ciorba bună.",
    "Găina vecinului pare curcă.",
    "Găină în făină.",
    "Gând la gând cu bucurie.",
    "Gerul nu vine niciodată singur.",
    "Graba strică treaba.",
    "Haina nu îl face pe om.",
    "Hoțul neprins e negustor cinstit.",
    "Iarba rea nu piere.",
    "În boul care trage, în ăla dă cu biciul",
    "Încet, încet, departe ajungi",
    "Încetul cu încetul se face oțetul",
    "Învață din nevoia altuia",
    "În țara orbilor, chiorul este împărat",
    "Jarul potolit te arde",
    "Jur acum pana nu iese soarele",
    "La plăcinte înainte, la război înapoi.",
    "La pomul lăudat să nu te duci cu sacul.",
    "Lacomului nu-i ajunge, Oltu-n gură de i-ar curge.",
    "La/în tot răul este și un bine.",
    "La omul sărac, nici boii nu trag.",
    "Lauda de sine nu miroase a bine.",
    "Laudă-mă gură, că ți-oi da friptură.",
    "Lenea e cucoană mare, care cere de mâncare",
    "Leneșul mai mult aleargă și zgârcitul (scumpul) mai mult păgubește.",
    "Lăcomia pierde omenia.",
    "Lăcusta nu are milă de bucate.",
    "Lemnul strâmb te necăjește, / omul prost te-mbătrânește.",
    "Lucrul, odată început, e pe jumătate făcut.",
    "Lupul cu slugi nu face gâtul gros.",
    "Lupul lup rămâne.",
    "Lupu-și schimbă părul, dar năravul ba.",
    "Lumea muncește și sapă și eu duc câinii la apă.",
    "Mai bine mai târziu decât niciodată.",
    "Maimuța în aur și purpură, tot maimuță rămâne.",
    "Mamă soacră, poamă acră.",
    "Măgarul duce vinul și bea apă.",
    "Mărul putred strică o grămadă de mere bune.",
    "Mătura nouă mătură bine.",
    "Mâța blândă zgârie rău.",
    "Meseria e brățară de aur.",
    "Mi-e milă de tine, dar de mine mi se rupe inima.",
    "Minciuna are picioare scurte.",
    "Mortul de la groapă nu se (mai) întoarce.",
    "Mărul nu cade departe de copac .",
    "Năravul din fire nu are lecuire.",
    "Năravul din născare leac nu are.",
    "Nemulțumitului i se ia darul.",
    "Nevoia îl învață pe om.",
    "Nevoia te învață.",
    "Nu aduce anul/ziua ce aduce ceasul/ora.",
    "N-ai carte, n-ai parte.",
    "Nu da vrabia din mână pentru cioara de pe gard.",
    "Nu e dracul (chiar) așa de negru.",
    "Nu este pădure fără uscături.",
    "Nu-i \"mult\" să nu se termine și \"puțin\" să nu ajungă.",
    "Nu face ce face popa, fă ce zice popa.",
    "Nu haina îl face pe om.",
    "Nu iese fum fără foc.",
    "Nu lăsa pe mâine ce poți face azi.",
    "Nu mor caii când vor câinii.",
    "Nu lăuda ziua înainte de asfințit.",
    "Nu poți fi și cu varza unsă și cu slănina în pod.",
    "Nu se sperie curva bătrână de vorba groasă.",
    "Nu știe stânga ce face dreapta. (Matei 6:3)",
    "Nu te încrede în cânele care dă din coadă.",
    "Nu toate muștele fac miere.",
    "Nu tot ce zboară se mănâncă.",
    "Nu-ți băga nasul unde nu-ți fierbe oala.",
    "Nu-ți dori, că ți se va întâmpla.",
    "Nu vede pădurea de copaci / din pricina copacului.",
    "Nu zice hop până n-ai sărit groapa/pârleazul.",
    "Numa-ncet, cu furca se încarcă carul.",
    "Numai în pomul care nu face roade nu dă nimeni cu pietre.",
    "Numai prostul râde de ce își aduce aminte.",
    "Obrazul de scoarță / totdeauna îi gata de harță.",
    "Ochii care nu se văd se uită.",
    "Ochii sunt oglinda/fereastra sufletului.",
    "Ochii verzi niciodată să nu-i crezi.",
    "Omul sfințește locul, nu locul pe om.",
    "Omul face haina și nu haina pe om.",
    "Omul harnic, silitor, de pâine nu duce dor.",
    "Paza bună trece primejdia rea",
    "Până ce nu dai cu capul de pragul de sus, nu-l vezi pe cel de jos.",
    "Pofta vine mâncând",
    "Prietenul bun e ca o pungă de bani.",
    "Prietenul la nevoie se cunoaște",
    "Prostia și domnia se plătește",
    "Prostia din naștere n-are leac (nu se vindecă)",
    "Prostul dacă nu-i fudul, parcă nu e prost destul",
    "Prostul moare de grija altuia",
    "Pe cine nu lași să moară, nu te lasă să trăiești.",
    "Pentru o babă surdă, nu se trag clopotele de două ori.",
    "Peștele de la cap se-mpute",
    "Peștele mare mănâncă peștele mic (sau Peștele cel mare mănâncă pe cel mic)",
    "Pică pară mălăiață în gura lui Nătăfleață",
    "Prea puțin ca să trăiești, prea mult ca să mori",
    "Prieteni noi să îți faci, dar de cei bătrâni să nu te lași![6]",
    "Printre grâu, mălura crește.",
    "Prost să fii, noroc să ai.",
    "Puneți frâu la gură și lacăt la inimă",
    "Prostul întâi o croiește apoi o gândește",
    "Prostul nu se lasă până când nu spune tot ce știe",
    "Punctualitatea-i politețea regilor",
    "Prostul nu e prost destul, dacă nu e și fudul!",
    "Rabdă inimă și taci, că n-ai alta ce să faci.",
    "Rabdă suflete cât poți, nu-ți da taina către toți.",
    "Răzbunarea e arma prostului.",
    "Râde hârb/ciob de oală spartă",
    "Râde dracul de porumbe negre și pe sine nu se vede.",
    "Râde prostul de neghiob.",
    "Rușinosul roade osul.",
    "S-a băgat ca musca-n lapte (ori, ca musca-n fundul calului).",
    "S-a dus bou și a venit vacă.",
    "S-au văzut mai mulți miei tăiați de Paște, decât oi bătrâne.[7]",
    "Să fii domn e o întâmplare, să fii om e lucru mare.",
    "Să stăm strâmb și să judecăm drept.",
    "Sătulul nu crede flămândului.",
    "Sângele apă nu se face.",
    "Sârguința e mama norocului.",
    "Scopul scuză mijloacele.",
    "Scump la tărâțe, ieftin la făină.",
    "Socoteală curată, prietenie lungă.",
    "Spune-mi cu cine te însoțești, ca să-ți spun cine ești.",
    "Stăpânul zgârcit învață sluga hoață.",
    "Și tăcerea e un răspuns.",
    "Știe mocanul ce-i șofranul? Când îl vede pe tarabă, gândește că e otravă.",
    "Tăcerea e de aur, vorba de argint.",
    "Tânăr lângă tânără, ca paiele lângă foc.",
    "Tinerii înaintea bătrânilor, să aibă urechi, nu gură![6]",
    "Toamna se numără bobocii.",
    "Toate drumurile / căile duc la Roma.",
    "Tot țiganul își laudă ciocanul.",
    "Trei lucruri nu știe omul: când se naște, când îl ia somnul și când moare.",
    "Țara arde și/iar baba se piaptănă.",
    "Țara arde de tătari, babei îi arde de lăutari.",
    "Ulciorul nu merge de multe ori la apă.",
    "Unde dai și unde crapă.",
    "Unde nu intră soarele pe fereastră, intră doctorul pe ușă.",
    "Unde se cioplește, sar și așchii.",
    "Unde-s mulți, puterea crește.",
    "Urma alege.",
    "Urma scapă turma.",
    "Un măr bolnav strică o grămada mare de mere sănătoase.",
    "Un nebun întreabă mai mult decât pot spune zece înțelepți.",
    "Unde nu-i cap, vai de picioare.",
    "Vinde pielea ursului din pădure.",
    "Vorba dulce mult aduce.",
    "Vorba e de argint, tăcerea e de aur.",
    "Vorba lungă, sărăcia omului.",
    "Vorbești de lup și lupul la ușă.",
    "Vrabia mălai visează, iar calicul praznicul.",
    "Vrei, nu vrei, bea, Grigore, aghiasmă!",
    "Vrei să faci o meserie, îți trebuie scule. Vrei să faci o familie, îți trebuie oameni.",
    "Zace pe moarte și ar mânca de toate.",
    "Ziua bună se cunoaște de dimineață.",
];

pub fn load_proverbs() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(PROVERBS.to_vec().iter().map(|x| x.to_string()).collect())
}
