pub mod abc {

    // pqr is private cant be accessed
    mod pqr {

        fn hi_pqr() {
            println!("Hi From PQR");
        }
    }

    pub fn hi_abc() {
        println!("Hi From abc")
    }
}

