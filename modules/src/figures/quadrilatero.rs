pub mod solid {
    #[derive(Debug)]
    pub struct Parallelepipedo {
        pub base: f32,
        pub altezza: f32,
        pub name: String,
    }

    impl Parallelepipedo {
        pub fn area(&self) -> f32 {
            self.base * self.altezza
        }
    }

    #[derive(Debug)]
    pub struct Triangle {
        base: f32,
        altezza: f32,
        name: String,
    }

    // In this case is acceptable to make dimensions not public as a function can acces them,
    // it is like an opaque in C
    impl Triangle {
        pub fn init(base: f32, altezza: f32, name: String) -> Triangle {
            Triangle {
                base,
                altezza,
                name,
            }
        }
    }
}
