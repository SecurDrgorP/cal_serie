
//My Functions:

 pub fn ver_into(vect: &Vec<char>) -> Vec<String> {
    let mut nvect = Vec::new();
    let mut str = String::new();

    for chr in vect.iter() {
        if chr.is_digit(10)  {
            str.push(*chr);
        } else if *chr == ')' || *chr == '(' || *chr == '+' || *chr == '-' || *chr == '*' || *chr == '/' {
            if !str.is_empty() {
                nvect.push(str.clone());
                str.clear();
            }
            nvect.push(chr.to_string());
        }
    }

    if !str.is_empty() {
        nvect.push(str);
    }

    nvect
 }

 pub fn vector_of(serie: &str) -> Vec<char> {

    let mut new_vect = Vec::new();
    for str in serie.chars() {
        if str != '\n' {
           new_vect.push(str);
        }
    }
    new_vect

//serie.chars().collect()
 }

 pub fn slice(vec: &Vec<String>, start: usize, chars: &str) -> Vec<String> {
    let mut nvect = Vec::new();
    for i in start..vec.len() { //here-----------------------------
        if vec.equal_bool(i, &chars) {
            break;
        } else {
            nvect.push(vec[i].clone());
        }
    }
    nvect
 }

 pub fn find_index(vec: &Vec<String>, start: usize, target: &str) -> usize {
    let mut i = start; //here---------------------------------------
    while i < vec.len() {
        if vec.equal_bool(i, &target) {
            return i;
        }
	i += 1;
    }
    vec.len() - 1
 }

/*--------------------------- */

/* that is for the operations. */

 pub fn choose_sign(character: &str, colect: &mut f32, add: &String) {
   let add: f32 = match add.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("It's not a number.choose_sign: {:?}", add);
            return;
        }
    };

   match character {
   "+" => *colect += add,
   "-" => *colect -= add,
   "*" => *colect *= add,
   "/" => {
      if add != 0.0 {
         *colect /= add;
      }
   }
   _ => println!("realy!!"),
   }
 }



//My Methods

//define the custom methode here for an external type.
pub trait RunCalExt {
    fn run_cal(&self) -> String;
    fn parentheses_cal(&self) -> Vec<String>;
    fn equal_bool(&self, i: usize, eq: &str) -> bool;
}

impl RunCalExt for Vec<String> {

    fn equal_bool(&self, i: usize, eq: &str) -> bool {
      if i >= self.len() {
         false
      }else{
         for str in eq.chars() {
            if self[i] == str.to_string() {
               return true;
            }
         }
         false
      }
   }

   fn run_cal(&self) -> String {
    let mut result: f32 = match self[0].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("It's not a number.run_cal: {:?}", self[0]);
            return String::new();
        }
    };
    //you can use here also parentheses_cal().
    let mut i = 0;
    while i < self.len() - 1 {
       if self.equal_bool(i+3, &("*/".to_string())) {
          let s = slice(&self, i+2, &("+-".to_string()));
          //println!("{:?} and the index is {}", s, i); //test----------
          choose_sign(&self[i+1], &mut result, &(s.run_cal()));
          //println!("the result: {}", result);//test--------------
          i = find_index(&self, i+3, &("+-".to_string()));
          //println!("the index after the increase: {}", i);//test---------
       } else if i + 2 >= self.len() {
        break;
       }else{
          choose_sign(&self[i+1], &mut result, &self[i+2]);
       }
       i+=1;
    }
    result.to_string()
   }

   //that for calculate the operations between the Parentheses. 
   fn parentheses_cal(&self) -> Vec<String> {
   let mut nvect = Vec::new();
   let mut i = 0;
   while i < self.len() {
      if self[i]=="(" {
         let s = slice(&self, i+1, &(")".to_string()));
         println!("{:?}", s);
         nvect.push(s.run_cal());
         i = find_index(&self, i, &(")".to_string()));
      }else{
         nvect.push(self[i].clone());
      }
      i+=1;
   }
   nvect
   }

}

/* --------------------------------- */
