// -----------------------------------------------------------------------------
// Atelier Rust
// 2022-11-19 - 16h30h
// Animateur: Axel (darnuria) && Aurelia
//
// 13-borrow-me-if-you-can.rs
//
// Objectifs pédagogiques:
//
// - Borrow immutables et mutables
// - Régles de borrowing et contraintes
// - Usage des moves et copies.
//
// Exercice: Borrow mutables,
// Tenter de compiler ce code, observer, se creuser la tête.
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Etapes:
//
// 1. Observer l'erreur de compilation
// 2. Corriger autant que neccessaire le code pour faire compiler
// le programme.

// Questions:
// 0.0 Pourquoi ce code ne peut pas compiler?
// 1.0 Comment réussir a avoir un code qui permet d'afficher l'element 1 après le remove?
// 2.0 Comment faire si le contenu du vector implemente pas le trait `Copy`?
fn main() {
    // Indice: Les i32 implementent `Copy` et `Clone`.
    let mut v = vec![0, 1, 2];
    let a = &v[1];
    v.remove(1); // signature of Vec::remove(mut self, index: usize) -> T
    println!("{}", a);

    /* // Décommente moi pour la suite de l'exercice!
    {
        // Indice les &str n'implementent pas `Copy`, ni `Clone`.
        // Indice2: Si vous êtes pas Copy vous êtes Move! ;)
        let mut s = vec!["🧗", "👩‍💻", "🕺"];
        let a = &s[1];
        s.remove(1);
        println!("{}", a);
    }
    */
}











// Indice:
// Qui est le héros du borrowing? «Borrow-mir»
// Et oui des fois pas d'indice juste une blague.
