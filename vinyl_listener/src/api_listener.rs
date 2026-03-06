use rusty_chromaprint::{Configuration, Fingerprinter};

pub struct ApiListener{
    printer: Fingerprinter,
}

impl ApiListener{
    pub fn new() -> ApiListener {
        let printer_init = Fingerprinter::new(&Configuration::preset_test2());
        ApiListener{
            printer: printer_init,
        }
    }

    pub fn listener(&mut self){
        let sample_rate: u32 = 44100;
        let num_channels: u32 = 2;
        self.printer.start(sample_rate, num_channels).unwrap();
        
        let samples: Vec<i16> = vec![0; 44100 * 2];
        self.printer.consume(&samples);
        
        self.printer.finish();

        let fingerprint = self.printer.fingerprint();

        println!("Fingerprint: {:08x?}", fingerprint);
    }
}

