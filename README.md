# inflection(Just for learning)

Inflection pluralizes and singularizes English nouns implemented by rust.

Copy from [go version](https://github.com/jinzhu/inflection).

```rust
use inflection::{singular, plural};


assert_eq!(plural::<_, String>("person"), "people".to_string());
assert_eq!(plural::<_, String>("Person"), "People".to_string());
assert_eq!(plural::<_, String>("PERSON"), "PEOPLE".to_string());
assert_eq!(plural::<_, String>("bus"), "buses".to_string());
assert_eq!(plural::<_, String>("BUS"), "BUSES".to_string());
assert_eq!(plural::<_, String>("Bus"), "Buses".to_string());
assert_eq!(plural::<_, String>("FancyPerson"), "FancyPeople".to_string()); 
                                                                             
assert_eq!(singular::<_, String>("people"), "person".to_string());           
assert_eq!(singular::<_, String>("PEOPLE"), "PERSON".to_string());           
assert_eq!(singular::<_, String>("buses"), "bus".to_string());               
assert_eq!(singular::<_, String>("People"), "Person".to_string());           
assert_eq!(singular::<_, String>("BUSES"), "BUS".to_string());               
assert_eq!(singular::<_, String>("Buses"), "Bus".to_string());               
assert_eq!(singular::<_, String>("FancyPeople"), "FancyPerson".to_string()); 
```