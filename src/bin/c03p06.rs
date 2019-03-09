#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum AnimalSpecies {
    Dog,
    Cat,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Animal {
    species: AnimalSpecies,
    name: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct AnimalShelter {
    arr: Vec<Animal>,
}

impl AnimalShelter {
    fn new() -> Self {
        AnimalShelter {
            arr: Vec::<Animal>::new(),
        }
    }

    fn enqueue(&mut self, animal: Animal) {
        self.arr.push(animal);
    }

    fn dequeue_any(&mut self) -> Option<Animal> {
        if self.arr.is_empty() {
            None
        } else {
            Some(self.arr.remove(0))
        }
    }

    fn dequeue_species(&mut self, species: AnimalSpecies) -> Option<Animal> {
        for (i, val) in self.arr.iter().enumerate() {
            if val.species == species {
                return Some(self.arr.remove(i));
            }
        }
        None
    }

    fn dequeue_cat(&mut self) -> Option<Animal> {
        self.dequeue_species(AnimalSpecies::Cat)
    }

    fn dequeue_dog(&mut self) -> Option<Animal> {
        self.dequeue_species(AnimalSpecies::Dog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animal_shelter() {
        let mut animal_shelter = AnimalShelter::new();
        animal_shelter.enqueue(Animal {
            name: "Peter".to_string(),
            species: AnimalSpecies::Cat,
        });
        animal_shelter.enqueue(Animal {
            name: "Jamal".to_string(),
            species: AnimalSpecies::Dog,
        });
        animal_shelter.enqueue(Animal {
            name: "Kaitlin".to_string(),
            species: AnimalSpecies::Cat,
        });
        animal_shelter.enqueue(Animal {
            name: "Helen".to_string(),
            species: AnimalSpecies::Cat,
        });
        animal_shelter.enqueue(Animal {
            name: "Julia".to_string(),
            species: AnimalSpecies::Dog,
        });

        let first = animal_shelter.dequeue_any().unwrap();
        assert_eq!(first.name, "Peter");

        let cat = animal_shelter.dequeue_cat().unwrap();
        assert_eq!(cat.name, "Kaitlin");

        let dog = animal_shelter.dequeue_dog().unwrap();
        assert_eq!(dog.name, "Jamal");

        let next = animal_shelter.dequeue_any().unwrap();
        assert_eq!(next.name, "Helen");
    }
}

fn main() {
    let mut animal_shelter = AnimalShelter::new();
    animal_shelter.enqueue(Animal {
        name: "Harry Potter".to_string(),
        species: AnimalSpecies::Dog,
    });
    animal_shelter.enqueue(Animal {
        name: "Hermione Granger".to_string(),
        species: AnimalSpecies::Cat,
    });
    animal_shelter.enqueue(Animal {
        name: "Ronald Weasly".to_string(),
        species: AnimalSpecies::Cat,
    });
    animal_shelter.dequeue_any();
    animal_shelter.dequeue_dog();
    animal_shelter.dequeue_cat();
}
