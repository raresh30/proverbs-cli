use phf::phf_map;

const DIACRITICS: phf::Map<char, char> = phf_map! {
    'ă' => 'a',
    'ǎ' => 'a',
    'â' => 'a',
    'î' => 'i',
    'ș' => 's',
    'ş' => 's',
    'ț' => 't',
    'ţ' => 't',
    'Ă' => 'A',
    'Â' => 'A',
    'Î' => 'I',
    'Ș' => 'S',
    'Ş' => 'S',
    'Ț' => 'T',
    'Ţ' => 'T',
};

fn canonical_form(proverb: &str) -> String {
    proverb.chars().map(|x| {
        let new = DIACRITICS.get(&x).cloned();
        match new {
            Some(new_char) => new_char,
            None => x,
        }
    }).collect::<String>().to_lowercase()
}

pub fn search(proverbs: &Vec<String>, pattern: String) {
    let results = proverbs.iter()
        .filter(|x| canonical_form(&x).contains(&pattern.to_lowercase()));
    for proverb in results {
        println!("{}", proverb);
    }
}

pub fn list(proverbs: &Vec<String>) {
    for proverb in proverbs {
        println!("{}", proverb);
    }
}
