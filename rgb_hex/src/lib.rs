use std::fmt;

macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
      if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
    };
  }
  
  struct Hex (i32);

  trait CheckingNumber {
      fn is_negative_or_exceed(&mut self) -> String;
  }

  impl Hex {
      fn new(color: i32) -> Hex {
          Hex(color)
      }
  }

  impl fmt::UpperHex for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0;

        fmt::UpperHex::fmt(&value, f)
    }
  }

  impl CheckingNumber for Hex {
      fn is_negative_or_exceed(&mut self) -> String {
        if self.0 <= 0 {
            "00".to_string()
        } else if self.0 > 255 {
            "FF".to_string()
        } else if self.0 > 0 && self.0 < 15 {
            let color_hex = Hex(self.0);
            let n = format!("0{:X}", color_hex);
            n
        } else {
            let color_hex = Hex(self.0);
            let n = format!("{:X}", color_hex);
            n
        }
        
      }
  }

  fn rgb(r: i32, g: i32, b: i32) -> String {
    let mut red = Hex::new(r);
    let mut green = Hex::new(g);
    let mut blue = Hex::new(b);

    let hex = format!("{}{}{}", 
        Hex::is_negative_or_exceed(&mut red), 
        Hex::is_negative_or_exceed(&mut green), 
        Hex::is_negative_or_exceed(&mut blue)
    );
    hex
}
  

  #[cfg(test)]
  mod sample_tests {
      use self::super::*;
  
      #[test]
      fn tests() {
          compare!(rgb(0, 0, 0), "000000");
          compare!(rgb(1, 2, 3), "010203");
          compare!(rgb(255, 255, 255), "FFFFFF");
          compare!(rgb(254, 253, 252), "FEFDFC");
          compare!(rgb(-20, 275, 125), "00FF7D");
      }
  }