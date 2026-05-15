use std::io;

struct Sensor {
    vibration: f32,
}

impl Sensor {

    fn display(&self) {

        println!("\nNilai Getaran: {:.2} mm/s", self.vibration);

        if self.vibration >= 4.0 {

            println!("Status Mesin: BAHAYA");
            println!("ALARM !!!");

        } else if self.vibration >= 2.5 {

            println!("Status Mesin: WARNING");

        } else {

            println!("Status Mesin: NORMAL");
        }
    }
}

fn calculate_average(data: &Vec<f32>) -> f32 {

    let mut total = 0.0;

    for value in data {

        total += value;
    }

    total / data.len() as f32
}

fn main() {

    let mut history: Vec<f32> = Vec::new();

    loop {

        println!("\n=== VIBRATION MONITORING SYSTEM ===");

        let mut input = String::new();

        println!("Masukkan nilai getaran:");

        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        let vibration: f32 = match input.trim().parse() {

            Ok(num) => num,

            Err(_) => {

                println!("Input harus angka!");
                continue;
            }
        };

        let sensor = Sensor {
            vibration,
        };

        history.push(vibration);

        sensor.display();

        let average = calculate_average(&history);

        println!("\nMoving Average: {:.2} mm/s", average);

        println!("\n=== HISTORY DATA ===");

        for (i, data) in history.iter().enumerate() {

            println!("Data {} = {:.2} mm/s", i + 1, data);
        }

        println!("\nInput lagi? (y/n)");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Gagal membaca pilihan");

        if choice.trim() == "n" {

            break;
        }
    }

    println!("\nProgram selesai.");
}
