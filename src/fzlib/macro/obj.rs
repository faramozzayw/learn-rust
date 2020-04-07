macro_rules! obj {
    (name = $name:ident $($field: ident : $t: ty),*) => {
        #[derive(Debug, Default)]
        struct $name {
            $( $field: $t, )*
        }
    };
    
    (name = $name:ident $($field:ident : $t: ty = $dfv: expr),*) => {
        #[derive(Debug)]
        struct $name {
            $( $field: $t, )*
        }
        
        impl Default for $name {
            fn default() -> Self {
                $name {
                    $( $field: $dfv, )*
                }
            }
        }
    };
 }

 obj! {
    name = Test
 
    prop: i32 = 95,
    b: f64 = 6.8,
    v: Vec<i8> = vec![1,2,3]
}

obj!{
    name = Qww

    f: String
}
 
 fn main() {
    println!("test");
    let t = Test { prop: 5, b: 5.6, v: vec![1,2,3] };
    println!("{:#?}", t);
    
    let f = Test::default();
    println!("{:#?}", f);
    
    let strg = Qww { f: String::from("some str") };
    println!("{:#?}", strg);
 }