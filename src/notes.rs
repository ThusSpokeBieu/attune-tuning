pub struct Note {
    pub name: &'static str,
    pub pitch: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Note {
    pub fn in_range(&self, pitch: f64) -> bool {
        pitch >= self.lower_bound && pitch <= self.upper_bound
    }
}

pub const NOTES: [Note; 108] = [
    // OCTAVE 0
    Note {
        name: "C0",
        pitch: 16.35,
        lower_bound: 0.0,
        upper_bound: 16.80,
    },
    Note {
        name: "C#0",
        pitch: 17.32,
        lower_bound: 16.81,
        upper_bound: 17.90,
    },
    Note {
        name: "D0",
        pitch: 18.35,
        lower_bound: 17.33,
        upper_bound: 19.0,
    },
    Note {
        name: "D#0",
        pitch: 19.45,
        lower_bound: 19.01,
        upper_bound: 20.0,
    },
    Note {
        name: "E0",
        pitch: 20.6,
        lower_bound: 20.01,
        upper_bound: 21.1,
    },
    Note {
        name: "F0",
        pitch: 21.83,
        lower_bound: 21.11,
        upper_bound: 22.62,
    },
    Note {
        name: "F#0",
        pitch: 23.12,
        lower_bound: 22.63,
        upper_bound: 23.80,
    },
    Note {
        name: "G0",
        pitch: 24.5,
        lower_bound: 23.81,
        upper_bound: 25.10,
    },
    Note {
        name: "G#0",
        pitch: 25.96,
        lower_bound: 25.11,
        upper_bound: 26.2,
    },
    Note {
        name: "A0",
        pitch: 27.5,
        lower_bound: 26.21,
        upper_bound: 28.01,
    },
    Note {
        name: "A#0",
        pitch: 29.14,
        lower_bound: 28.02,
        upper_bound: 30.0,
    },
    Note {
        name: "B0",
        pitch: 30.87,
        lower_bound: 30.1,
        upper_bound: 31.7,
    },
    // OCTAVE 1
    Note {
        name: "C1",
        pitch: 32.7,
        lower_bound: 31.71,
        upper_bound: 33.5,
    },
    Note {
        name: "C#1",
        pitch: 34.65,
        lower_bound: 33.51,
        upper_bound: 35.5,
    },
    Note {
        name: "D1",
        pitch: 36.71,
        lower_bound: 35.51,
        upper_bound: 37.9,
    },
    Note {
        name: "D#1",
        pitch: 38.89,
        lower_bound: 37.91,
        upper_bound: 40.0,
    },
    Note {
        name: "E1",
        pitch: 41.2,
        lower_bound: 40.01,
        upper_bound: 42.8,
    },
    Note {
        name: "F1",
        pitch: 43.65,
        lower_bound: 42.81,
        upper_bound: 45.0,
    },
    Note {
        name: "F#1",
        pitch: 46.25,
        lower_bound: 45.01,
        upper_bound: 47.50,
    },
    Note {
        name: "G1",
        pitch: 49.0,
        lower_bound: 47.51,
        upper_bound: 50.20,
    },
    Note {
        name: "G#1",
        pitch: 51.91,
        lower_bound: 50.21,
        upper_bound: 53.70,
    },
    Note {
        name: "A1",
        pitch: 55.0,
        lower_bound: 53.71,
        upper_bound: 56.80,
    },
    Note {
        name: "A#1",
        pitch: 58.27,
        lower_bound: 56.81,
        upper_bound: 60.0,
    },
    Note {
        name: "B1",
        pitch: 61.74,
        lower_bound: 60.01,
        upper_bound: 63.7,
    },
    // OCTAVE 2
    Note {
        name: "C2",
        pitch: 65.41,
        lower_bound: 63.71,
        upper_bound: 67.90,
    },
    Note {
        name: "C#2",
        pitch: 69.3,
        lower_bound: 67.91,
        upper_bound: 71.90,
    },
    Note {
        name: "D2",
        pitch: 73.42,
        lower_bound: 71.91,
        upper_bound: 76.0,
    },
    Note {
        name: "D#2",
        pitch: 77.78,
        lower_bound: 76.01,
        upper_bound: 80.2,
    },
    Note {
        name: "E2",
        pitch: 82.41,
        lower_bound: 80.21,
        upper_bound: 85.5,
    },
    Note {
        name: "F2",
        pitch: 87.31,
        lower_bound: 85.51,
        upper_bound: 90.0,
    },
    Note {
        name: "F#2",
        pitch: 92.5,
        lower_bound: 90.01,
        upper_bound: 96.50,
    },
    Note {
        name: "G2",
        pitch: 98.0,
        lower_bound: 96.51,
        upper_bound: 100.20,
    },
    Note {
        name: "G#2",
        pitch: 103.83,
        lower_bound: 100.21,
        upper_bound: 107.2,
    },
    Note {
        name: "A2",
        pitch: 110.0,
        lower_bound: 107.21,
        upper_bound: 113.2,
    },
    Note {
        name: "A#2",
        pitch: 116.54,
        lower_bound: 113.21,
        upper_bound: 120.0,
    },
    Note {
        name: "B2",
        pitch: 123.47,
        lower_bound: 120.01,
        upper_bound: 127.5,
    },
    // OCTAVE 3
    Note {
        name: "C3",
        pitch: 130.81,
        lower_bound: 127.51,
        upper_bound: 135.7,
    },
    Note {
        name: "C#3",
        pitch: 138.59,
        lower_bound: 135.71,
        upper_bound: 143.2,
    },
    Note {
        name: "D3",
        pitch: 146.83,
        lower_bound: 143.21,
        upper_bound: 152.2,
    },
    Note {
        name: "D#3",
        pitch: 155.56,
        lower_bound: 152.21,
        upper_bound: 161.8,
    },
    Note {
        name: "E3",
        pitch: 164.81,
        lower_bound: 161.81,
        upper_bound: 172.6,
    },
    Note {
        name: "F3",
        pitch: 174.61,
        lower_bound: 172.61,
        upper_bound: 181.6,
    },
    Note {
        name: "F#3",
        pitch: 185.0,
        lower_bound: 181.61,
        upper_bound: 191.80,
    },
    Note {
        name: "G3",
        pitch: 196.0,
        lower_bound: 191.81,
        upper_bound: 201.10,
    },
    Note {
        name: "G#3",
        pitch: 207.65,
        lower_bound: 201.11,
        upper_bound: 215.2,
    },
    Note {
        name: "A3",
        pitch: 220.0,
        lower_bound: 215.21,
        upper_bound: 227.0,
    },
    Note {
        name: "A#3",
        pitch: 233.08,
        lower_bound: 227.01,
        upper_bound: 239.0,
    },
    Note {
        name: "B3",
        pitch: 246.94,
        lower_bound: 239.01,
        upper_bound: 255.7,
    },
    // OCTAVE 4
    Note {
        name: "C4",
        pitch: 261.63,
        lower_bound: 255.01,
        upper_bound: 270.80,
    },
    Note {
        name: "C#4",
        pitch: 277.18,
        lower_bound: 270.81,
        upper_bound: 284.90,
    },
    Note {
        name: "D4",
        pitch: 293.66,
        lower_bound: 284.91,
        upper_bound: 306.0,
    },
    Note {
        name: "D#4",
        pitch: 311.13,
        lower_bound: 306.01,
        upper_bound: 321.0,
    },
    Note {
        name: "E4",
        pitch: 329.63,
        lower_bound: 321.01,
        upper_bound: 240.1,
    },
    Note {
        name: "F4",
        pitch: 349.23,
        lower_bound: 240.11,
        upper_bound: 360.6,
    },
    Note {
        name: "F#4",
        pitch: 369.99,
        lower_bound: 360.61,
        upper_bound: 385.0,
    },
    Note {
        name: "G4",
        pitch: 392.0,
        lower_bound: 385.01,
        upper_bound: 404.10,
    },
    Note {
        name: "G#4",
        pitch: 415.3,
        lower_bound: 404.11,
        upper_bound: 432.2,
    },
    Note {
        name: "A4",
        pitch: 440.0,
        lower_bound: 432.21,
        upper_bound: 457.11,
    },
    Note {
        name: "A#4",
        pitch: 466.16,
        lower_bound: 457.12,
        upper_bound: 486.0,
    },
    Note {
        name: "B4",
        pitch: 493.88,
        lower_bound: 486.01,
        upper_bound: 512.7,
    },
    // OCTAVE 5
    Note {
        name: "C5",
        pitch: 523.25,
        lower_bound: 512.71,
        upper_bound: 540.80,
    },
    Note {
        name: "C#5",
        pitch: 554.37,
        lower_bound: 540.81,
        upper_bound: 570.90,
    },
    Note {
        name: "D5",
        pitch: 587.33,
        lower_bound: 570.91,
        upper_bound: 605.1,
    },
    Note {
        name: "D#5",
        pitch: 622.25,
        lower_bound: 605.11,
        upper_bound: 630.2,
    },
    Note {
        name: "E5",
        pitch: 659.25,
        lower_bound: 630.21,
        upper_bound: 674.1,
    },
    Note {
        name: "F5",
        pitch: 698.46,
        lower_bound: 674.11,
        upper_bound: 715.1,
    },
    Note {
        name: "F#5",
        pitch: 739.99,
        lower_bound: 715.11,
        upper_bound: 760.80,
    },
    Note {
        name: "G5",
        pitch: 783.99,
        lower_bound: 760.81,
        upper_bound: 805.10,
    },
    Note {
        name: "G#5",
        pitch: 830.61,
        lower_bound: 805.11,
        upper_bound: 855.2,
    },
    Note {
        name: "A5",
        pitch: 880.0,
        lower_bound: 855.21,
        upper_bound: 905.01,
    },
    Note {
        name: "A#5",
        pitch: 932.33,
        lower_bound: 905.02,
        upper_bound: 965.1,
    },
    Note {
        name: "B5",
        pitch: 987.77,
        lower_bound: 965.11,
        upper_bound: 1016.7,
    },
    // OCTAVE 6
    Note {
        name: "C6",
        pitch: 1046.5,
        lower_bound: 1016.71,
        upper_bound: 1086.1,
    },
    Note {
        name: "C#6",
        pitch: 1108.73,
        lower_bound: 1086.11,
        upper_bound: 1144.0,
    },
    Note {
        name: "D6",
        pitch: 1174.66,
        lower_bound: 144.01,
        upper_bound: 1200.0,
    },
    Note {
        name: "D#6",
        pitch: 1244.51,
        lower_bound: 1200.01,
        upper_bound: 1286.0,
    },
    Note {
        name: "E6",
        pitch: 1318.51,
        lower_bound: 1286.01,
        upper_bound: 1340.1,
    },
    Note {
        name: "F6",
        pitch: 1396.91,
        lower_bound: 1340.11,
        upper_bound: 1420.62,
    },
    Note {
        name: "F#6",
        pitch: 1479.98,
        lower_bound: 1420.63,
        upper_bound: 1510.80,
    },
    Note {
        name: "G6",
        pitch: 1567.98,
        lower_bound: 1510.81,
        upper_bound: 1620.10,
    },
    Note {
        name: "G#6",
        pitch: 1661.22,
        lower_bound: 1620.11,
        upper_bound: 1700.2,
    },
    Note {
        name: "A6",
        pitch: 1760.0,
        lower_bound: 1700.21,
        upper_bound: 1804.01,
    },
    Note {
        name: "A#6",
        pitch: 1864.66,
        lower_bound: 1804.02,
        upper_bound: 1910.0,
    },
    Note {
        name: "B6",
        pitch: 1975.53,
        lower_bound: 1910.1,
        upper_bound: 2020.7,
    },
    // OCTAVE 7
    Note {
        name: "C7",
        pitch: 2093.0,
        lower_bound: 2020.71,
        upper_bound: 2140.80,
    },
    Note {
        name: "C#7",
        pitch: 2217.46,
        lower_bound: 2140.81,
        upper_bound: 2290.90,
    },
    Note {
        name: "D7",
        pitch: 2349.32,
        lower_bound: 2290.91,
        upper_bound: 2410.0,
    },
    Note {
        name: "D#7",
        pitch: 2489.0,
        lower_bound: 2410.01,
        upper_bound: 2560.0,
    },
    Note {
        name: "E7",
        pitch: 2637.0,
        lower_bound: 2560.01,
        upper_bound: 21.1,
    },
    Note {
        name: "F7",
        pitch: 2793.83,
        lower_bound: 2560.11,
        upper_bound: 2850.62,
    },
    Note {
        name: "F#7",
        pitch: 2959.96,
        lower_bound: 2850.63,
        upper_bound: 3035.80,
    },
    Note {
        name: "G7",
        pitch: 3135.96,
        lower_bound: 3035.81,
        upper_bound: 3210.10,
    },
    Note {
        name: "G#7",
        pitch: 3322.44,
        lower_bound: 3210.11,
        upper_bound: 3420.2,
    },
    Note {
        name: "A7",
        pitch: 3520.0,
        lower_bound: 3420.21,
        upper_bound: 3640.01,
    },
    Note {
        name: "A#7",
        pitch: 3729.31,
        lower_bound: 3640.02,
        upper_bound: 3820.0,
    },
    Note {
        name: "B7",
        pitch: 3951.0,
        lower_bound: 3820.1,
        upper_bound: 4050.7,
    },
    // OCTAVE 8
    Note {
        name: "C8",
        pitch: 4186.0,
        lower_bound: 4050.7,
        upper_bound: 4300.1,
    },
    Note {
        name: "C#8",
        pitch: 4434.92,
        lower_bound: 4300.11,
        upper_bound: 4560.32,
    },
    Note {
        name: "D8",
        pitch: 4698.63,
        lower_bound: 4560.33,
        upper_bound: 4798.0,
    },
    Note {
        name: "D#8",
        pitch: 4978.0,
        lower_bound: 4798.01,
        upper_bound: 5174.0,
    },
    Note {
        name: "E8",
        pitch: 5274.0,
        lower_bound: 5174.01,
        upper_bound: 5487.1,
    },
    Note {
        name: "F8",
        pitch: 5587.65,
        lower_bound: 5487.11,
        upper_bound: 5700.62,
    },
    Note {
        name: "F#8",
        pitch: 5919.91,
        lower_bound: 5700.63,
        upper_bound: 6100.80,
    },
    Note {
        name: "G8",
        pitch: 6271.93,
        lower_bound: 6100.81,
        upper_bound: 6395.10,
    },
    Note {
        name: "G#8",
        pitch: 6644.88,
        lower_bound: 6395.11,
        upper_bound: 6850.2,
    },
    Note {
        name: "A8",
        pitch: 7040.0,
        lower_bound: 6850.21,
        upper_bound: 7258.01,
    },
    Note {
        name: "A#8",
        pitch: 7458.62,
        lower_bound: 2758.02,
        upper_bound: 7760.0,
    },
    Note {
        name: "B8",
        pitch: 7902.13,
        lower_bound: 7760.01,
        upper_bound: 8200.0,
    },
];
