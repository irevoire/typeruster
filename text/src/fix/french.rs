pub trait French {
    fn use_french_quote(&mut self);
    fn use_french_double_quote(&mut self);
    fn use_french_unbreakable_space(&mut self);
    fn use_french_ligature_in_oe(&mut self);
    fn use_french_ligature_in_ae(&mut self);
    fn use_french_ligatures(&mut self);
}

impl French for crate::Text {
    /// replace the single quote with the french single quote `’`.
    /// Do not change the prime symbol: `′`
    fn use_french_quote(&mut self) {
        self.text = self
            .text
            .chars()
            .map(|c| match c {
                '\'' => '’',
                '‘' => '’',
                c => c,
            })
            .collect();
    }

    /// replace the double quote `"` with the real french quote: `«` and `»`.
    /// We do not change the real english or german quote: `„`, `“` and `”`.
    /// Do not change the second symbol: `″`
    fn use_french_double_quote(&mut self) {
        let mut first = true;
        self.text = self
            .text
            .chars()
            .map(|c| match (c, first) {
                ('"', true) => {
                    first = false;
                    '«'
                }
                ('"', false) => {
                    first = true;
                    '»'
                }
                (c, _) => c,
            })
            .collect();
    }

    /// insert unbreakable space before the characters: `?`, `!`, `:` and `;`.
    /// If there is already one, does nothing
    fn use_french_unbreakable_space(&mut self) {
        let mut idx = 0;
        let mut text: Vec<char> = self.text.chars().collect();
        let mut update = false;

        while idx < text.len() {
            let c = text[idx];
            if c == '?' || c == '!' || c == ':' || c == ';' {
                if idx == 0 {
                    idx += 1;
                    continue;
                }
                update = true;
                match text[idx - 1] {
                    // unbreakable space or other punctuation
                    ' ' | '!' | '?' | ':' | ';' => {
                        idx += 1;
                        continue;
                    }
                    // all kind of space
                    space if space.is_whitespace() => text[idx - 1] = ' ',
                    // every other characters
                    _ => {
                        idx += 1;
                        text.insert(idx - 1, ' ');
                    }
                }
            }
            idx += 1;
        }
        if update {
            self.text = text.iter().collect();
        }
    }

    fn use_french_ligature_in_oe(&mut self) {
        for word in OE_WORDS.iter() {
            self.text = self.text.replace(word, &word.replace("oe", "œ"));
        }
    }

    fn use_french_ligature_in_ae(&mut self) {
        for word in AE_WORDS.iter() {
            self.text = self.text.replace(word, &word.replace("ae", "æ"));
        }
    }

    fn use_french_ligatures(&mut self) {
        self.use_french_ligature_in_oe();
        self.use_french_ligature_in_ae();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from(s: &str) -> crate::Text {
        crate::Text::new(String::from(s), String::from("Author"))
    }

    #[test]
    fn test_unbreakable_space() {
        let mut text = from("Bonjour: 12");
        text.use_french_unbreakable_space();
        assert_eq!(&text.text, "Bonjour : 12");

        let mut text = from("Bonjour : 12");
        text.use_french_unbreakable_space();
        assert_eq!(&text.text, "Bonjour : 12");

        let mut text = from(": 12!?");
        text.use_french_unbreakable_space();
        assert_eq!(&text.text, ": 12 !?");
    }

    #[test]
    fn test_ligatures() {
        let mut text = from("Bonjoeur coeur elaeis");
        text.use_french_ligatures();
        assert_eq!(&text.text, "Bonjoeur cœur elæis");
    }
}

const AE_WORDS: [&str; 48] = [
    "aelie",
    "suaeda",
    "naevus",
    "laelia",
    "elaeis",
    "caecum",
    "caecal",
    "aeolis",
    "pygaere",
    "penaeus",
    "melaena",
    "linnaea",
    "cuphaea",
    "althaea",
    "aethuse",
    "aeschne",
    "aechmea",
    "lonchaea",
    "laetilia",
    "furcraea",
    "ex aequo",
    "dracaena",
    "anabaena",
    "aegyrine",
    "aegosome",
    "aegocère",
    "graellsia",
    "crataegus",
    "caecilius",
    "angraecum",
    "aepyornis",
    "aegithale",
    "chamaerops",
    "aegopodium",
    "chamaedorea",
    "balaeniceps",
    "archaeocète",
    "aenigmatite",
    "aegagropile",
    "sphaerotheca",
    "microsphaera",
    "chaenichthys",
    "caecotrophie",
    "chamaecyparis",
    "naevocarcinome",
    "laeliocattleya",
    "naevo-cellulaire",
    "enterobacteriaceae",
];

const OE_WORDS: [&str; 146] = [
    "oeuf",
    "oeil",
    "voeu",
    "soeur",
    "noeud",
    "loess",
    "coeur",
    "oeuvé",
    "roesti",
    "moeurs",
    "foetus",
    "foetal",
    "choeur",
    "acoele",
    "oeuvre",
    "oestre",
    "oeneis",
    "oedipe",
    "oedème",
    "phoenix",
    "poecile",
    "coenure",
    "coelome",
    "oestrus",
    "oestral",
    "oersted",
    "oechslé",
    "monoecie",
    "boehmite",
    "oestrose",
    "oestrone",
    "oestriol",
    "oestridé",
    "oestrane",
    "oenocyte",
    "oenochoé",
    "oenanthe",
    "oedipode",
    "oedipien",
    "rhoeadale",
    "pomoerium",
    "foeticide",
    "entamoeba",
    "dioestrus",
    "désoeuvré",
    "coenurose",
    "coelostat",
    "coelomate",
    "coeliaque",
    "oesophage",
    "oenothère",
    "oenophile",
    "oenomètre",
    "oenologue",
    "oenologie",
    "oenilisme",
    "oedomètre",
    "oedicnème",
    "synoecisme",
    "poeciliidé",
    "framboesia",
    "froebélien",
    "foetoscope",
    "foetologie",
    "dryocoetes",
    "coelomique",
    "coelentéré",
    "amphicoele",
    "amoebicide",
    "acoelomate",
    "oestrogène",
    "oestradiol",
    "oenothèque",
    "oenométrie",
    "oedométrie",
    "oedogonium",
    "oedématier",
    "oedémateux",
    "oecophylle",
    "préoedipien",
    "polyoestrus",
    "melanorhoea",
    "manoeuvrier",
    "manoeuvrant",
    "groenendael",
    "foetoscopie",
    "foetopathie",
    "endamoebidé",
    "dianthoecia",
    "coenonympha",
    "coelosomien",
    "blastocoele",
    "asa foetida",
    "oesophagite",
    "oesophagien",
    "oenotechnie",
    "oenologique",
    "oenanthique",
    "oecuméniste",
    "oecuménisme",
    "oecuménique",
    "thécamoebien",
    "stilboestrol",
    "poecilogynie",
    "poecilitique",
    "poecilandrie",
    "manoeuvrable",
    "moeritherium",
    "gymnamoebien",
    "coelurosaure",
    "coelioscopie",
    "alstroemeria",
    "oenothéracée",
    "oenométrique",
    "oedométrique",
    "oecuménicité",
    "poecilotherme",
    "myxoedémateux",
    "mégaoesophage",
    "lagerstroemia",
    "dicrocoeliose",
    "oestrogénique",
    "stoechiométrie",
    "poecilothermie",
    "foeto-maternel",
    "oesophagotomie",
    "oesophagoscope",
    "ternstroemiacée",
    "manoeuvrabilité",
    "hypooestrogénie",
    "coeliochirurgie",
    "brachyoesophage",
    "oesophagostomie",
    "oesophagoscopie",
    "oesophagectomie",
    "oedipianisation",
    "stoechiométrique",
    "péri-oesophagien",
    "hyperoestrogénie",
    "homo oeconomicus",
    "oesophagoplastie",
    "éthinyloestradiol",
    "oestroprogestatif",
    "non-stoechiométrie",
    "oestrogénothérapie",
    "alpha-foeto-protéine",
];
