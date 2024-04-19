use std::{cmp::Ordering, io};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::notes::NOTES;

mod notes;

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for &pitch in data {
                    if pitch as f64 > 16.0 {
                        get_note(pitch as f64);
                    }
                }
            },
            |err| eprintln!("an error occurred on stream: {}", err),
            Option::None,
        )
        .unwrap();

    stream.play().unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn start() {
    println!("Digite o número em hz: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada");

    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        println!("Nenhum número foi fornecido. Tente novamente");
        return;
    }

    let number: f64 = match trimmed_input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, digite um número válido!");
            return;
        }
    };

    get_note(number)
}

fn get_note(pitch: f64) {
    let mut lower = 0;
    let mut upper = NOTES.len();

    while lower < upper {
        let mid = (upper + lower) / 2;
        match NOTES[mid].in_range(pitch) {
            true => {
                let current_note = &NOTES[mid];

                if pitch.eq(&NOTES[mid].pitch) {
                    println!(
                        "=========\nAFINADO - \n{}hz, está afinado corretamente na nota {} \n=========\n\n",
                        pitch, current_note.name
                    );
                    return;
                }

                println!(
                    "=========\nDESAFINADO - {}hz afine {:.2} para a nota {} \n=========\n\n\n",
                    pitch,
                    current_note.pitch - pitch,
                    current_note.name
                );
                return;
            }
            false => match pitch.partial_cmp(&NOTES[mid].upper_bound) {
                Some(Ordering::Less) => upper = mid,
                _ => lower = mid + 1,
            },
        }
    }

    println!("{} Hz está fora do intervalo conhecido", pitch);
}
