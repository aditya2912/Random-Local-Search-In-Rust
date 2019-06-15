use std::f64::consts::E;

mod neuronMod {

#[derive(Debug, Copy, Clone)]
pub struct Unit {
    pub value: f64,
    pub gradient: f64,
}

impl Unit {
    pub fn empty() -> Unit {
        return Unit {value: 0f64, gradient: 0.0};
    }

    pub fn new(value: f64, gradient: f64) -> Unit {
        return Unit { value: value, gradient: gradient};
    }
 }

pub struct MultiplyGate {
    pub u0: Unit,
    pub u1: Unit,
    pub utop: Unit
}

impl MultiplyGate {
    pub fn empty() -> MultiplyGate {
        return MultiplyGate {
             u0: Unit::empty(),
             u1: Unit::empty(),
             utop: Unit::empty(),
        }
    }

    pub fn forward(mut self, u0: Unit, u1: Unit) -> MultiplyGate {
            self.u0 = u1;
            self.u1 = u0;
            self.utop = Unit::new(self.u0.value * self.u1.value, 0.0);
            self
    }

    pub fn backwards(mut self) -> MultiplyGate {
        self.u0.gradient += self.u1.value * self.utop.gradient;
        self.u1.gradient += self.u1.value * self.utop.gradient;
        self
    }
}

pub struct AddGate {
    pub u0: Unit,
    pub u1: Unit,
    pub utop: Unit,
}

impl AddGate {
    pub fn empty() -> AddGate {
            return AddGate {
                u0: Unit::empty(),
                u1: Unit::empty(),
                utop: Unit::empty(),
            }
    }

    pub fn forward(mut self, u0: Unit, u1: Unit) -> AddGate {
            self.u0 = u0;
            self.u1 = u1;
            self.utop = Unit::new(self.u0.value + self.u1.value, 0.0);
            self
    }

    pub fn backward(mut self) -> AddGate {
        self.u0.gradient += 1.0 * self.utop.gradient;
        self.u1.gradient += 1.0 * self.utop.gradient;
        self
    }
}

pub struct SigmoidGate {
    pub u0: Unit,
    pub grad: f64,
    pub utop: Unit,
}

pub fn sigmoid(x: f64 ) -> f64 {
    let mut sigmoid: f64 = 0f64;
       if x <= 0.0 {
           sigmoid = 1.0 / (1.0 + (1.0 /  std::f64::consts::E.powi(x as i32)));
       } else {
           sigmoid = 1.0 / (1.0 +  std::f64::consts::E.powi(x as i32) );
       }
    return sigmoid;
}

impl SigmoidGate {
    pub fn empty() -> SigmoidGate {
            return SigmoidGate {
                u0: Unit::empty(),
                grad: 0f64,
                utop: Unit::empty()
            }
    }

    pub fn forward(mut self, u0 : Unit) -> SigmoidGate {
        self.u0 = u0;
        let mut sig: f64 = sigmoid(self.u0.value);
        println!("{:?} SIG", sig);
        self.utop = Unit::new(sig, 0.0);
        println!("{:?} SELF.UTOP.VALUE",self.utop.value);
        self
    }

    pub fn backward(mut self) -> SigmoidGate {
        let mut sig: f64 = sigmoid(self.u0.value);
        self.u0.gradient += (sig * (1.0 - sig)) * self.utop.gradient;
        self
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
     fn test_case() {
         let mut a: neuronMod::Unit = neuronMod::Unit::new(1.0, 2.0);
         let mut b: neuronMod::Unit = neuronMod::Unit::new(2.0, 0.0);
         let mut c: neuronMod::Unit = neuronMod::Unit::new(-3.0, 0.0);
         let mut x: neuronMod::Unit = neuronMod::Unit::new(-1.0, 0.0);
         let mut y: neuronMod::Unit = neuronMod::Unit::new(3.0, 0.0);

         let mut mulg0: neuronMod::MultiplyGate = neuronMod::MultiplyGate::empty();
         let mut mulg1: neuronMod::MultiplyGate = neuronMod::MultiplyGate::empty();
         let mut addg0: neuronMod::AddGate = neuronMod::AddGate::empty();
         let mut addg1: neuronMod::AddGate = neuronMod::AddGate::empty();
         let mut sg0: neuronMod::SigmoidGate = neuronMod::SigmoidGate::empty();

         let mut ax: neuronMod::MultiplyGate = mulg0.forward(a, x);
         let mut by: neuronMod::MultiplyGate = mulg1.forward(b, y);
         let mut axpby: neuronMod::AddGate = addg0.forward(ax.utop, by.utop);
         let mut axpbypc: neuronMod::AddGate = addg1.forward(axpby.utop, c);
         let mut s: neuronMod::SigmoidGate = sg0.forward(axpbypc.utop);


         println!("{:?} {:?} {:?} {:?} {:?}",ax.utop.value, by.utop.value, axpby.utop.value, axpbypc.utop.value, s.utop.value);


         // s.grad = 1.0;
         // sg0 = neuronMod::SigmoidGate::backward(sg0);
         // addg1.backward();
         // addg0.backward();

     }

}
