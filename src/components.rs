pub mod components {

    pub struct word {
        val: [u8; 3],
    }
    
    impl word {
        pub fn new() -> Self {
            word { val: [0; 3] }
        }
    }
    
} /* components */
