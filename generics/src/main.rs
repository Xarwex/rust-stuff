trait Animal {}

struct Dog;
impl Animal for Dog {}

#[derive(Debug)]
struct Cat;
impl Animal for Cat {}

trait Food {}

#[derive(Debug)]
struct DogFood;
#[derive(Debug)]
struct CatFood;

impl Food for DogFood {}
impl Food for CatFood {}

trait Barf {
    type Output: Food;

    fn barf() -> Self::Output;
}

impl Barf for Dog {
    type Output = DogFood;

    fn barf() -> Self::Output {
        DogFood {}
    }
}

impl Barf for Cat {
    type Output = CatFood;

    fn barf() -> Self::Output {
        CatFood {}
    }
}

trait ServeDinner<F: Food> {
    fn eat(food: F);
}

impl ServeDinner<DogFood> for Cat {
    fn eat(food: DogFood) {
        panic!(
            "jesus no pls, why did you give me {:?} you know I hate it ffs",
            food
        );
    }
}

impl ServeDinner<CatFood> for Cat {
    fn eat(food: CatFood) {
        println!("yum {:?}", food)
    }
}

struct Factory {}

trait FoodFactory<A: Animal> {
    type FoodOutput;

    fn produce(animal: A) -> Self::FoodOutput;
}

impl FoodFactory<Cat> for Factory {
    type FoodOutput = CatFood;

    fn produce(animal: Cat) -> Self::FoodOutput {
        println!("Putting {:?} into a blender", animal);
        CatFood {}
    }
}

trait Distribute<F: Food> {
    fn distribute(food: F);
}

impl Distribute<<Factory as FoodFactory<Cat>>::FoodOutput> for Factory {
    fn distribute(_food: <Factory as FoodFactory<Cat>>::FoodOutput) {}
}

fn main() {}
