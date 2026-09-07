pub mod grid {
    pub struct Rule<T> {
        pub kernel: Vec<T>, //TODO: Figure out how to do assymetric rules
        pub result: T,
    }

    pub struct World<T> {
        pub squares: Vec<T>,
        pub rules: Vec<Rule<T>>,
        pub world_id: u128,
    }

    //Linear (1-Dimensional) World with Symmetrical Kernel Checking
    pub trait Dim1SymKernel {
        //Rules look 'kernel_sz' to each side of the pixel, but not pixel itself
        fn new(row_sz: usize, kernel_sz: usize) -> Self;

        //Does square index match a rule?
        fn compare(&self, ind:usize) -> Option<usize>;

        fn step(&mut self); //TODO: Feels like there should be a way of generalizing this, I just don't
                            //      know enough Rust to do so yet
    }

    //Wolfram Style
    impl Dim1SymKernel for World<bool> {
        fn new(row_sz: usize, kernel_sz: usize) -> World<bool> {
            assert!(row_sz != 0);
            assert!(kernel_sz != 0);
            assert!(kernel_sz < row_sz);
            assert!(kernel_sz < 3); //hardcoded so it cannot be more than the world_id max

            let mut instance = World::<bool>{
                squares: vec![false; row_sz],
                rules: vec![],
                world_id: 0,
            };

            let rule_cnt = (2_u32).pow((kernel_sz * 2) as u32);
            //println!("{0} -> {1}", kernel_sz, rule_cnt);
            for i in 0..rule_cnt {
                let mut law = Rule::<bool>{
                    kernel: vec![false; kernel_sz*2],
                    result: rand::random(),
                };

                let mut j = 0;
                while j < kernel_sz*2 {
                    law.kernel[j] = (i & (1 << j)) != 0; //binary counting all possibilities
                    j += 1;
                }

                if law.result {
                    //println!("{0} + {1}", instance.world_id, 1 << i);
                    instance.world_id += 1 << i;
                }

                instance.rules.push(law);
            }

            return instance;
        }

        fn compare(&self, ind: usize) -> Option<usize> {
            let kernel_sz = self.rules[0].kernel.len() / 2; //all len's should be the same

            for i in 0..self.rules.len() { //check all rules
                let mut valid_cnt = 0;
                for j in 0..self.rules[i].kernel.len() { //check kernel
                    let mut ind_chk: i32;

                    if j < kernel_sz {
                        ind_chk = (-1 * ((kernel_sz - j) as i32)) + (ind as i32);
                    } else {
                        ind_chk = ((j - kernel_sz + 1) as i32) + (ind as i32); //don't check itself
                    }


                    if ind_chk < 0 { ind_chk += self.squares.len() as i32; } //overflow
                    else if ind_chk >= self.squares.len().try_into().unwrap() { ind_chk -= self.squares.len() as i32 }

                    if self.squares[ind_chk as usize] == self.rules[i].kernel[j] { //this kernel slot matches
                        valid_cnt += 1;
                    }
                }

                if valid_cnt == self.rules[i].kernel.len() {
                    return Some(i);
                }
            }

            return None;
        }

        fn step(&mut self) {
            let mut nstate: Vec<bool> = vec![];

            for i in 0..self.squares.len() {
                match self.compare(i) {
                    Some(x) => nstate.push(self.rules[x].result),
                    None => assert!(false), // should be impossible, there should be a rule for all
                                            // possibilities
                };
            }

            assert!(nstate.len() == self.squares.len());
            self.squares = nstate;
        }
    }
}
