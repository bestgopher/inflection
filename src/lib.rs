//! Inflection pluralizes and singularizes English nouns

extern crate lazy_static;
extern crate regex;
extern crate titlecase;

use lazy_static::lazy_static;
use regex::Regex;
use titlecase::titlecase;

use std::sync::RwLock;

lazy_static! {
    static ref PLURAL_INFLECTIONS: RwLock<Vec<Regular>> = RwLock::new(
        vec![
            ("([a-z])$", "${1}s"),
            ("s$", "s"),
            ("^(ax|test)is$", "${1}es"),
            ("(octop|vir)us$", "${1}i"),
            ("(octop|vir)i$", "${1}i"),
            ("(alias|status|campus)$", "${1}es"),
            ("(bu)s$", "${1}ses"),
            ("(buffal|tomat)o$", "${1}oes"),
            ("([ti])um$", "${1}a"),
            ("([ti])a$", "${1}a"),
            ("sis$", "ses"),
            ("(?:([^f])fe|([lr])f)$", "${1}${2}ves"),
            ("(hive)$", "${1}s"),
            ("([^aeiouy]|qu)y$", "${1}ies"),
            ("(x|ch|ss|sh)$", "${1}es"),
            ("(matr|vert|ind)(?:ix|ex)$", "${1}ices"),
            ("^(m|l)ouse$", "${1}ice"),
            ("^(m|l)ice$", "${1}ice"),
            ("^(ox)$", "${1}en"),
            ("^(oxen)$", "${1}"),
            ("(quiz)$", "${1}zes"),
            ("(drive)$", "${1}s")
        ]
        .into_iter()
        .rev()
        .map(|(x, y)|Regular{find: x.to_string(), replace: y.to_string()})
        .collect()
    );

    static ref SINGULAR_INFLECTIONS: RwLock<Vec<Regular>> = RwLock::new(
        vec![
            ("s$", ""),
            ("(ss)$", "${1}"),
            ("(n)ews$", "${1}ews"),
            ("([ti])a$", "${1}um"),
            ("((a)naly|(b)a|(d)iagno|(p)arenthe|(p)rogno|(s)ynop|(t)he)(sis|ses)$", "${1}sis"),
            ("(^analy)(sis|ses)$", "${1}sis"),
            ("([^f])ves$", "${1}fe"),
            ("(hive)s$", "${1}"),
            ("(tive)s$", "${1}"),
            ("([lr])ves$", "${1}f"),
            ("([^aeiouy]|qu)ies$", "${1}y"),
            ("(s)eries$", "${1}eries"),
            ("(m)ovies$", "${1}ovie"),
            ("(c)ookies$", "${1}ookie"),
            ("(x|ch|ss|sh)es$", "${1}"),
            ("^(m|l)ice$", "${1}ouse"),
            ("(bus|campus)(es)?$", "${1}"),
            ("(o)es$", "${1}"),
            ("(shoe)s$", "${1}"),
            ("(cris|test)(is|es)$", "${1}is"),
            ("^(a)x[ie]s$", "${1}xis"),
            ("(octop|vir)(us|i)$", "${1}us"),
            ("(alias|status)(es)?$", "${1}"),
            ("^(ox)en", "${1}"),
            ("(vert|ind)ices$", "${1}ex"),
            ("(matr)ices$", "${1}ix"),
            ("(quiz)zes$", "${1}"),
            ("(database)s$", "${1}"),
            ("(drive)s$", "${1}")
        ]
        .into_iter()
        .rev()
        .map(|(x, y)|Regular{find: x.to_string(), replace: y.to_string()})
        .collect()
    );

    static ref IRREGULAR_INFLECTIONS: RwLock<Vec<Irregular>> = RwLock::new(
        vec![
            ("person", "people"),
            ("man", "men"),
            ("child", "children"),
            ("sex", "sexes"),
            ("move", "moves"),
            ("ombie", "ombies"),
            ("goose", "geese"),
            ("foot", "feet"),
            ("moose", "moose"),
            ("tooth", "teeth"),
        ]
        .into_iter()
        .map(|(x, y)|Irregular{singular: x.to_string(), plural: y.to_string()})
        .collect()
    );

    static ref  UNCOUNTABLE_INFLECTIONS: RwLock<Vec<String>> = RwLock::new(
        vec![
            "equipment", "information", "rice", "money", "species", "series", "fish",
            "sheep", "jeans", "police", "milk", "salt", "time", "water", "paper", "food",
            "art", "cash", "music", "help", "luck", "oil", "progress", "rain",
            "research", "shopping", "software", "traffic"
        ]
        .into_iter()
        .map(|x|x.to_string())
        .collect()
    );

    static ref COMPILED_PLURAL_MAPS: RwLock<Vec<Inflection>> = {
        let mut v = vec![];

        UNCOUNTABLE_INFLECTIONS.read().unwrap().iter().for_each(|x| {
            compile_uncountable(&mut v, x);
        });

        IRREGULAR_INFLECTIONS.read().unwrap().iter().for_each(|Irregular{singular, plural}| {
            compile_irregular_inflections(&mut v, singular, plural);
        });

        PLURAL_INFLECTIONS.read().unwrap().iter().for_each(|Regular{find, replace}| {
            compile_inflections(&mut v, find, replace);
        });

        RwLock::new(v)
    };


     static ref COMPILED_SINGULAR_MAPS: RwLock<Vec<Inflection>> = {
        let mut v = vec![];

        UNCOUNTABLE_INFLECTIONS.read().unwrap().iter().for_each(|x| {
            compile_uncountable(&mut v, x);
        });

        IRREGULAR_INFLECTIONS.read().unwrap().iter().for_each(|Irregular{singular, plural}| {
            compile_irregular_inflections(&mut v, plural, singular);
        });

        SINGULAR_INFLECTIONS.read().unwrap().iter().for_each(|Regular{find, replace}| {
            compile_inflections(&mut v, find, replace);
        });

        RwLock::new(v)
    };
}

#[derive(Clone, Debug)]
pub struct Regular {
    pub find: String,
    pub replace: String,
}

#[derive(Clone, Debug)]
pub struct Irregular {
    pub singular: String,
    pub plural: String,
}

#[derive(Clone, Debug)]
pub struct Inflection {
    pub regex: regex::Regex,
    pub replace: String,
}

impl Inflection {
    fn new(rex: String, replace: String) -> Result<Inflection, Box<dyn std::error::Error>> {
        Ok(Inflection { regex: Regex::new(&rex)?, replace })
    }
}

fn compile_inflections(p: &mut Vec<Inflection>, a: &str, b: &str) {
    p.push(Inflection::new(a.to_uppercase(), b.to_uppercase()).unwrap());
    p.push(Inflection::new(a.to_string(), b.to_string()).unwrap());
    p.push(Inflection::new(format!("(?i){}", a), b.to_string()).unwrap());
}

fn compile_irregular_inflections(p: &mut Vec<Inflection>, a: &str, b: &str) {
    p.push(Inflection::new(format!("{}$", a.to_uppercase()), b.to_uppercase()).unwrap());
    p.push(Inflection::new(format!("{}$", titlecase(a)), titlecase(b)).unwrap());
    p.push(Inflection::new(format!("{}$", a), b.to_string()).unwrap());
}

fn compile_uncountable(p: &mut Vec<Inflection>, x: &str) {
    p.push(Inflection::new(format!("^(?i)({})$", x), "${1}".to_string()).unwrap());
}


fn re_compile() {
    re_plural_compile();
    re_singular_compile()
}

fn re_plural_compile() {
    let mut p = COMPILED_PLURAL_MAPS.write().unwrap();
    p.clear();

    UNCOUNTABLE_INFLECTIONS.read().unwrap().iter().for_each(|x| {
        compile_uncountable(&mut p, x);
    });

    IRREGULAR_INFLECTIONS.read().unwrap().iter().for_each(|Irregular { singular, plural }| {
        compile_irregular_inflections(&mut p, singular, plural);
    });

    PLURAL_INFLECTIONS.read().unwrap().iter().for_each(|Regular { find, replace }| {
        compile_inflections(&mut p, find, replace);
    });
}

fn re_singular_compile() {
    let mut s = COMPILED_SINGULAR_MAPS.write().unwrap();
    s.clear();

    UNCOUNTABLE_INFLECTIONS.read().unwrap().iter().for_each(|x| {
        compile_uncountable(&mut s, x);
    });

    IRREGULAR_INFLECTIONS.read().unwrap().iter().for_each(|Irregular { singular, plural }| {
        compile_irregular_inflections(&mut s, plural, singular);
    });

    SINGULAR_INFLECTIONS.read().unwrap().iter().for_each(|Regular { find, replace }| {
        compile_inflections(&mut s, find, replace);
    });
}

pub fn add_plural<T: Into<String>>(find: T, replace: T) {
    let r = Regular { find: find.into(), replace: replace.into() };

    if let Ok(mut s) = PLURAL_INFLECTIONS.write() {
        s.push(r);
    }

    re_plural_compile();
}

pub fn add_singular<T: Into<String>>(find: T, replace: T) {
    let r = Regular { find: find.into(), replace: replace.into() };

    if let Ok(mut s) = SINGULAR_INFLECTIONS.write() {
        s.push(r);
    }

    re_singular_compile();
}

pub fn add_irregular<T: Into<String>>(singular: T, plural: T) {
    let r = Irregular { singular: singular.into(), plural: plural.into() };

    if let Ok(mut s) = IRREGULAR_INFLECTIONS.write() {
        s.push(r);
    }

    re_compile();
}

pub fn add_uncountable<T: Into<String>, I: Iterator<Item=T>>(values: I) {
    if let Ok(mut s) = UNCOUNTABLE_INFLECTIONS.write() {
        s.extend(values.into_iter().map(|x| x.into()));
    }

    re_compile();
}

pub fn get_plural() -> Vec<Regular> {
    if let Ok(s) = PLURAL_INFLECTIONS.read() {
        s.iter().cloned().collect()
    } else {
        vec![]
    }
}

pub fn get_singular() -> Vec<Regular> {
    if let Ok(s) = SINGULAR_INFLECTIONS.read() {
        s.iter().cloned().collect()
    } else {
        vec![]
    }
}

pub fn get_irregular() -> Vec<Irregular> {
    if let Ok(s) = IRREGULAR_INFLECTIONS.read() {
        s.iter().cloned().collect()
    } else {
        vec![]
    }
}

pub fn get_uncountable() -> Vec<String> {
    if let Ok(s) = UNCOUNTABLE_INFLECTIONS.read() {
        s.iter().cloned().collect()
    } else {
        vec![]
    }
}

/// plural converts a word to its plural form.
/// # example
/// ```
/// use inflection::plural;
///
/// assert_eq!(plural::<_, String>("person"), "people".to_string());
/// assert_eq!(plural::<_, String>("Person"), "People".to_string());
/// assert_eq!(plural::<_, String>("PERSON"), "PEOPLE".to_string());
/// assert_eq!(plural::<_, String>("bus"), "buses".to_string());
/// assert_eq!(plural::<_, String>("BUS"), "BUSES".to_string());
/// assert_eq!(plural::<_, String>("Bus"), "Buses".to_string());
/// assert_eq!(plural::<_, String>("FancyPerson"), "FancyPeople".to_string());
/// ```
pub fn plural<T, F>(input: T) -> F
    where T: Into<String>,
          F: From<String>
{
    let m = input.into();
    if let Ok(s) = COMPILED_PLURAL_MAPS.read() {
        for x in s.iter() {
            if x.regex.is_match(&m) {
                return From::from(x.regex.replace(&m, x.replace.as_str()).to_string());
            }
        }
    }

    From::from(m)
}


/// singular converts a word to its singular form
/// # example
/// ```
/// use inflection::singular;
///
/// assert_eq!(singular::<_, String>("people"), "person".to_string());
/// assert_eq!(singular::<_, String>("PEOPLE"), "PERSON".to_string());
/// assert_eq!(singular::<_, String>("buses"), "bus".to_string());
/// assert_eq!(singular::<_, String>("People"), "Person".to_string());
/// assert_eq!(singular::<_, String>("BUSES"), "BUS".to_string());
/// assert_eq!(singular::<_, String>("Buses"), "Bus".to_string());
/// assert_eq!(singular::<_, String>("FancyPeople"), "FancyPerson".to_string());
/// ```
pub fn singular<T, F>(input: T) -> F
    where T: Into<String>,
          F: From<String>
{
    let m = input.into();
    if let Ok(s) = COMPILED_SINGULAR_MAPS.read() {
        for x in s.iter() {
            if x.regex.is_match(&m) {
                return From::from(x.regex.replace(&m, x.replace.as_str()).to_string());
            }
        }
    }

    From::from(m)
}

pub fn set_plural<I: Iterator<Item=Regular>>(data: I) {
    if let Ok(mut s) = PLURAL_INFLECTIONS.write() {
        s.clear();
        s.extend(data)
    }

    re_plural_compile()
}

pub fn set_singular<I: Iterator<Item=Regular>>(data: I) {
    if let Ok(mut s) = SINGULAR_INFLECTIONS.write() {
        s.clear();
        s.extend(data)
    }

    re_singular_compile()
}

pub fn set_irregular<I: Iterator<Item=Irregular>>(data: I) {
    if let Ok(mut s) = IRREGULAR_INFLECTIONS.write() {
        s.clear();
        s.extend(data)
    }

    re_compile()
}

pub fn set_uncountable<I: Iterator<Item=String>>(data: I) {
    if let Ok(mut s) = UNCOUNTABLE_INFLECTIONS.write() {
        s.clear();
        s.extend(data)
    }

    re_compile()
}

#[cfg(test)]
mod tests {
    macro_rules! hash {
        ($($x:expr=>$y:expr), *) => {
            {
                let mut temp_vec = std::collections::HashMap::new();
                $(
                  temp_vec.insert($x, $y);
                )*

                temp_vec
             }
       }
    }

    use lazy_static::lazy_static;
    use titlecase::titlecase;

    use std::collections::HashMap;

    use super::*;

    lazy_static! {
        static ref INFLECTIONS: HashMap<&'static str, &'static str> = {
            add_irregular("criterion", "criteria");

            hash![
                "star"=>"stars",
                "STAR"=>"STARS",
                "Star"=>"Stars",
                "bus"=>"buses",
                "fish"=>"fish",
                "mouse"=>"mice",
                "query"=>"queries",
                "ability"=>"abilities",
                "agency"=>"agencies",
                "movie"=>"movies",
                "archive"=>"archives",
                "index"=>"indices",
                "wife"=>"wives",
                "safe"=>"saves",
                "half"=>"halves",
                "move"=>"moves",
                "salesperson"=>"salespeople",
                "person"=>"people",
                "spokesman"=>"spokesmen",
                "man"=>"men",
                "woman"=>"women",
                "basis"=>"bases",
                "diagnosis"=>"diagnoses",
                "diagnosis_a"=>"diagnosis_as",
                "datum"=>"data",
                "medium"=>"media",
                "stadium"=>"stadia",
                "analysis"=>"analyses",
                "node_child"=>"node_children",
                "child"=>"children",
                "experience"=>"experiences",
                "day"=>"days",
                "comment"=>"comments",
                "foobar"=>"foobars",
                "newsletter"=>"newsletters",
                "old_news"=>"old_news",
                "news"=>"news",
                "series"=>"series",
                "species"=>"species",
                "quiz"=>"quizzes",
                "perspective"=>"perspectives",
                "ox"=>"oxen",
                "photo"=>"photos",
                "buffalo"=>"buffaloes",
                "tomato"=>"tomatoes",
                "dwarf"=>"dwarves",
                "elf"=>"elves",
                "information"=>"information",
                "equipment"=>"equipment",
                "criterion"=>"criteria",
                "foot"=>"feet",
                "goose"=>"geese",
                "moose"=>"moose",
                "tooth"=>"teeth",
                "milk"=>"milk",
                "salt"=>"salt",
                "time"=>"time",
                "water"=>"water",
                "paper"=>"paper",
                "music"=>"music",
                "help"=>"help",
                "luck"=>"luck",
                "oil"=>"oil",
                "progress"=>"progress",
                "rain"=>"rain",
                "research"=>"research",
                "shopping"=>"shopping",
                "software"=>"software",
                "traffic"=>"traffic",
                "zombie"=>"zombies",
                "campus"=>"campuses",
                "harddrive"=>"harddrives",
                "drive"=>"drives"
            ]
        };
    }

    #[test]
    fn it_plural() {
        for (i, v) in INFLECTIONS.iter() {
            assert_eq!(plural::<_, String>(i.to_uppercase()), v.to_uppercase());
            assert_eq!(plural::<_, String>(titlecase(i)), titlecase(v));
            assert_eq!(plural::<_, String>(i.to_string()), v.to_string());
        }
    }

    #[test]
    fn it_singular() {
        for (i, v) in INFLECTIONS.iter() {
            assert_eq!(i.to_uppercase(), singular::<_, String>(v.to_uppercase()));
            assert_eq!(titlecase(i), singular::<_, String>(titlecase(v)));
            assert_eq!(i.to_string(), singular::<_, String>(v.to_string()));
        }
    }

    #[test]
    fn test_add_plural() {
        let len = PLURAL_INFLECTIONS.read().unwrap().len();
        add_plural("aaaaa", "aaaaa");
        assert_eq!(len + 1, PLURAL_INFLECTIONS.read().unwrap().len());
    }

    #[test]
    fn test_add_singular() {
        let len = SINGULAR_INFLECTIONS.read().unwrap().len();
        add_singular("aaaaa", "aaaaa");
        assert_eq!(len + 1, SINGULAR_INFLECTIONS.read().unwrap().len());
    }

    #[test]
    fn test_add_uncountable() {
        let len = UNCOUNTABLE_INFLECTIONS.read().unwrap().len();
        add_uncountable(vec!["aaaaa"].into_iter());
        assert_eq!(len + 1, UNCOUNTABLE_INFLECTIONS.read().unwrap().len());
    }

    #[test]
    fn test_add_irregular() {
        let len = IRREGULAR_INFLECTIONS.read().unwrap().len();
        add_irregular("aaaaa", "aaaaa");
        assert_eq!(len + 1, IRREGULAR_INFLECTIONS.read().unwrap().len());
    }

    #[test]
    fn test_get() {
        assert_eq!(get_irregular().len(), IRREGULAR_INFLECTIONS.read().unwrap().len());
        assert_eq!(get_plural().len(), PLURAL_INFLECTIONS.read().unwrap().len());
        assert_eq!(get_uncountable().len(), UNCOUNTABLE_INFLECTIONS.read().unwrap().len());
        assert_eq!(get_singular().len(), SINGULAR_INFLECTIONS.read().unwrap().len());
    }
}
