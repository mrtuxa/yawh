use std::io;

fn main() {
    // Schritte für das Waffelrezept
    let recipe = [
        "Mehl, Backpulver, Zucker und Salz vermischt",
        "Wasser, Öl und Vanilleextrakt hinzugefügt und gut durchgerührt",
        "das Waffeleisen vorgeheitzt und leicht eingefettet?",
        "Teig in das Waffeleisen geben und geschaut das er gleichmäßig verteilt ist und nicht anbrennt",
        "die Waffeln mit Toppings serviert",
        "den Tisch abgeräumt und die Küche aufgeräumt"
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
