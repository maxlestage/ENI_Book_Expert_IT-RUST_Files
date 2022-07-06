fn main() {
    module_2::creation_cercle();
 }
 
pub mod module_1 {

     pub struct Cercle{
      pub coord_centre_x : i64,
        pub coord_centre_y : i64,
        pub rayon_cercle : i64,
        pub nom_cercle : String,
     }
 }

mod module_2 {

   use crate::module_1;

   pub fn creation_cercle(){
      let cercle_1 = module_1::Cercle{
         coord_centre_x : 5,
         coord_centre_y : 5,
         rayon_cercle : 2,
         nom_cercle : String::from("cercle_1"),
      };
   }
}