
extern crate hal_v2;
extern crate hal_v3;

use hal_v2::digital::OutputPin as OutputPinV2;
use hal_v3::digital::OutputPin as OutputPinV3;

struct OldImpl { 
    state: bool
}

impl OutputPinV2 for OldImpl {
    fn set_low(&mut self) {
        self.state = false;
    }
    fn set_high(&mut self) {
        self.state = true;
    }
}

struct NewImpl { 
    state: bool
}

impl OutputPinV3 for NewImpl {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = false;
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error>{
        self.state = true;
        Ok(())
    }
}

struct OldConsumer<T> {
    pin: T,
}

impl <T>OldConsumer<T> 
where T: OutputPinV2 {
    pub fn new(pin: T) -> OldConsumer<T> {
        OldConsumer{ pin }
    }
}

struct NewConsumer<T> {
    pin: T,
}

impl <T>NewConsumer<T> 
where T: OutputPinV3 {
    pub fn new(pin: T) -> NewConsumer<T> {
        NewConsumer{ pin }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_new() {
        let mut i = NewImpl{state: false};
        let mut c = NewConsumer{pin: i};
    }

    #[test]
    fn old_old() {
        let mut i = OldImpl{state: false};
        let mut c = OldConsumer{pin: i};
    }

    #[test]
    fn old_new() {
        let mut i = OldImpl{state: false};
        let mut c = NewConsumer{pin: i};
    }

    #[test]
    fn new_old() {
        let mut i = NewImpl{state: false};
        let mut c = OldConsumer{pin: i};
    }

}