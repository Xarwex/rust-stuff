use rand::Rng;

trait Crab {
    fn new() -> Self
    where
        Self: Sized;

    fn snip(&self);
    fn length(&self) -> u8;
}

struct Croge {
    length: u8,
}

impl Crab for Croge {
    fn new() -> Self
    where
        Self: Sized,
    {
        Croge {
            length: rand::thread_rng().gen(),
        }
    }

    fn snip(&self) {
        println!("snip snip");
    }

    fn length(&self) -> u8 {
        self.length
    }
}

trait Zoo {
    fn open(&mut self);
    fn close(&mut self);
    fn is_open(&self) -> bool;
}

struct Zoge {
    open: bool,
}

impl Zoo for Zoge {
    fn open(&mut self) {
        assert!(!self.open);
        self.open = true;
    }

    fn close(&mut self) {
        assert!(self.open);
        self.open = false;
    }

    fn is_open(&self) -> bool {
        self.open
    }
}

impl Zoge {
    fn new() -> Self {
        Zoge { open: false }
    }
}

struct CrabAndZoo {
    crabs: Vec<Box<dyn Crab>>,
    zoo: Box<dyn Zoo>,
}

impl CrabAndZoo {
    fn day_in_a_crab_zoo(&mut self) {
        self.zoo.open();
        println!("Zoo is open");
        for crab in &self.crabs {
            crab.snip();
            println!("What a big crab it is {} long", crab.length());
        }
        self.zoo.close();
        println!("Zoo is closed");
    }
}

fn main() {
    let crabs: Vec<Box<dyn Crab>> = vec![Box::new(Croge::new()), Box::new(Croge::new())];
    let zoo = Box::new(Zoge::new());

    CrabAndZoo { crabs, zoo }.day_in_a_crab_zoo();
}
