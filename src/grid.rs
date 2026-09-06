mod grid {
    pub struct Rule<T> {
        kernel: Vec<T>,
        result: T,
    }

    pub struct World<T> {
        pub squares: Vec<T>,
        rules: Vec<Rule<T>>,
        world_id: usize,
    }

    //Linear World with Symmetric Rules
    pub trait LinearSymmetric {
        fn new(row_sz: usize, kernel_sz: usize) -> Self;

        //Does square index match a rule?
        fn compare(&self, ind:usize) -> Option<usize>;

        fn step(&self) {
            //TODO: Needs to borrow the self so it can edit it
        }
    }

    //Wolfram Style
    impl LinearSymmetric for World<bool> {
        //Rules look 'kernel_sz' to each side of the pixel
        fn new(row_sz: usize, kernel_sz: usize) -> World<bool> {
            assert!(row_sz != 0);
            assert!(kernel_sz != 0);

            let mut instance = World::<bool>{
                squares: vec![false; row_sz],
                rules: vec![],
                world_id: 0,
            };

            let mut i: usize = 0;
            while i < (kernel_sz*2).pow(2) {
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
                    instance.world_id += 1 << i;
                }

                instance.rules.push(law);
                i += 1;
            }

            return instance;
        }

        fn compare(&self, ind: usize) -> Option<usize> {
            return None; //TODO
        }
    }
}
