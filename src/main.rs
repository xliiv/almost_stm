#[derive(Debug)]
pub enum Error {
    Overflow,
    Other(String),
}

pub struct Stm {
    inner: i8,
}

impl Stm {
    pub fn new(data: i8) -> Stm {
        Stm { inner: data }
    }

    pub fn get(&self) -> i8 {
        self.inner
    }

    fn change(&mut self, value: i8) -> Result<(), Error> {
        self.inner += value;
        Ok(())
        //TODO:: up overflow
        //TODO:: down overflow
        //TODO:: minus value for unsigned overflow
    }

    pub fn add(&mut self, value: i8) -> Result<(), Error> {
        self.change(value)
    }

    pub fn sub(&mut self, value: i8) -> Result<(), Error> {
        self.change(-value)
    }

    //pub fn destroy() -> Result<(), Error> {
    //    self.change(-value)
    //}

}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn stm_get_works() {
        let expected = 0;

        let stm = Stm::new(expected);

        assert_eq!(stm.get(), expected);
    }

    #[warn(overflowing_literals)]
    #[test]
    fn stm_get_fails_on_overflow() {
        let overflow = 256;
        let initial = 0;
        let mut stm = Stm::new(initial);

        let res = stm.add(overflow);

        println!("{:?}", stm.get());
        println!("{:?}", res);
        //assert_eq!(res, Error::Overflow);
        assert_eq!(stm.get(), initial);
    }


    #[test]
    fn stm_get_works_in_thread() {
        let expected = 0;
        let stm = Stm::new(expected);

        let child = thread::spawn(move || {
            assert_eq!(stm.get(), expected);
        });

        child.join().unwrap();
    }

    #[test]
    fn stm_add_works() {
        let expected = 5;
        let mut stm = Stm::new(0);

        stm.add(expected).unwrap();

        assert_eq!(stm.get(), expected);
    }

    #[test]
    fn stm_add_works_in_thread() {
        let expected = 0;
        let stm = Stm::new(expected);


        let child = thread::spawn(move || {
            assert_eq!(stm.get(), expected);
        });
        child.join().unwrap();
    }

}

