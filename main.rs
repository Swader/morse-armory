use std::fs::OpenOptions;
use std::io::{self, Write};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

struct MorseCode {
    led_file: std::fs::File,
    morse_map: HashMap<char, &'static str>,
    unit_delay: Duration,
}

impl MorseCode {
    fn new(led_path: &str) -> io::Result<Self> {
        let mut morse_map = HashMap::new();
        // Basic Morse code patterns
        morse_map.insert('A', ".-");
        morse_map.insert('B', "-...");
        morse_map.insert('C', "-.-.");
        morse_map.insert('D', "-..");
        morse_map.insert('E', ".");
        morse_map.insert('F', "..-.");
        morse_map.insert('G', "--.");
        morse_map.insert('H', "....");
        morse_map.insert('I', "..");
        morse_map.insert('J', ".---");
        morse_map.insert('K', "-.-");
        morse_map.insert('L', ".-..");
        morse_map.insert('M', "--");
        morse_map.insert('N', "-.");
        morse_map.insert('O', "---");
        morse_map.insert('P', ".--.");
        morse_map.insert('Q', "--.-");
        morse_map.insert('R', ".-.");
        morse_map.insert('S', "...");
        morse_map.insert('T', "-");
        morse_map.insert('U', "..-");
        morse_map.insert('V', "...-");
        morse_map.insert('W', ".--");
        morse_map.insert('X', "-..-");
        morse_map.insert('Y', "-.--");
        morse_map.insert('Z', "--..");
        morse_map.insert('1', ".----");
        morse_map.insert('2', "..---");
        morse_map.insert('3', "...--");
        morse_map.insert('4', "....-");
        morse_map.insert('5', ".....");
        morse_map.insert('6', "-....");
        morse_map.insert('7', "--...");
        morse_map.insert('8', "---..");
        morse_map.insert('9', "----.");
        morse_map.insert('0', "-----");
        morse_map.insert(' ', " ");

        let led_file = OpenOptions::new().write(true).open(led_path)?;
        
        Ok(MorseCode {
            led_file,
            morse_map,
            unit_delay: Duration::from_millis(200), // Adjustable speed
        })
    }

    fn led_on(&mut self) -> io::Result<()> {
        self.led_file.write_all(b"1")?;
        self.led_file.flush()
    }

    fn led_off(&mut self) -> io::Result<()> {
        self.led_file.write_all(b"0")?;
        self.led_file.flush()
    }

    fn dot(&mut self) -> io::Result<()> {
        self.led_on()?;
        sleep(self.unit_delay);
        self.led_off()?;
        sleep(self.unit_delay);
        Ok(())
    }

    fn dash(&mut self) -> io::Result<()> {
        self.led_on()?;
        sleep(self.unit_delay * 3);
        self.led_off()?;
        sleep(self.unit_delay);
        Ok(())
    }

    fn transmit_char(&mut self, c: char) -> io::Result<()> {
        if let Some(pattern) = self.morse_map.get(&c.to_ascii_uppercase()) {
            for symbol in pattern.chars() {
                match symbol {
                    '.' => self.dot()?,
                    '-' => self.dash()?,
                    ' ' => sleep(self.unit_delay * 3),
                    _ => {}
                }
            }
            sleep(self.unit_delay * 2); // Space between letters
        }
        Ok(())
    }

    fn transmit_text(&mut self, text: &str) -> io::Result<()> {
        for c in text.chars() {
            self.transmit_char(c)?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut morse = MorseCode::new("/sys/class/leds/LED_BLUE/brightness")?;
    
    println!("USB Armory Morse Code Transmitter");
    println!("----------------------------------");
    println!("1. Press ENTER to transmit a message");
    println!("2. Type 'q' to quit");
    
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let text = input.trim();
        if text.eq_ignore_ascii_case("q") {
            break;
        }
        
        println!("Transmitting: {}", text);
        morse.transmit_text(text)?;
        println!("Done! Enter another message or 'q' to quit");
    }
    
    Ok(())
}
