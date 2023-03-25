use std::io;

fn main() {
    // Schritte für das Waffelrezept
    let recipe = [
        "Eier, Zucker und Butter miteinander verrührt",
        "Mehl und Backpulver hinzugefügt",
        "den Teig in die Waffeleisen gegeben",
        "die Waffeln gebacken",
        "die Waffeln genossen",
        "den Abwasch gemacht"
    ];

    // Schritte durchlaufen und Benutzereingaben abfragen
    for step in recipe.iter() {
        println!("Hast du {} ? (y/n)", step);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fehler beim Lesen der Benutzereingabe");
        let cleaned_input = input.trim().to_lowercase();

        // Wenn der Benutzer "n" eingibt, brechen wir die Schleife ab
        if cleaned_input == "n" {
            println!("Okay, wir hören auf.");
            break;
        }
    }

    println!("Fertig!");
}
