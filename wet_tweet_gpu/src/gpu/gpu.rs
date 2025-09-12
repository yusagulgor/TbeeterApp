
type Color = (u8,u8,u8);

#[derive(Copy, Clone, Debug)]
pub enum ColorName {
    Red,
    Green,
    Blue,
    Black,
    White,
}

impl From<ColorName> for (u8, u8, u8) {
    fn from(c: ColorName) -> Self {
        match c {
            ColorName::Red   => (255, 0, 0),
            ColorName::Green => (0, 255, 0),
            ColorName::Blue  => (0, 0, 255),
            ColorName::Black => (255, 255, 255), 
            ColorName::White => (0, 0, 0),
        }
    }
}

fn color_to_ansi(c: Color) -> String {
    match c {
        (0,0,0) => "\x1b[97m█\x1b[0m".to_string(),
        (255,255,255) => "\x1b[30m█\x1b[0m".to_string(),
        (255,0,0) => "\x1b[31m█\x1b[0m".to_string(),
        (0,255,0) => "\x1b[32m█\x1b[0m".to_string(),
        (0,0,255) => "\x1b[34m█\x1b[0m".to_string(),
        _ => "\x1b[37m█\x1b[0m".to_string(),
    }
}

fn color_to_ansi_char(fg: Color, ch: char) -> String {
    match fg {
        (255,0,0) => format!("\x1b[31m{}\x1b[0m", ch),
        (0,255,0) => format!("\x1b[32m{}\x1b[0m", ch),
        (0,0,255) => format!("\x1b[34m{}\x1b[0m", ch),
        (255,255,255) => format!("\x1b[30m{}\x1b[0m", ch),
        (0,0,0) => format!("\x1b[97m{}\x1b[0m", ch),
        _ => format!("\x1b[37m{}\x1b[0m", ch),
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GpuTransistor {
    val: Color
}

impl GpuTransistor {
    pub fn new() -> Self {
        Self { val: (0,0,0) }
    }

    pub fn set(&mut self, val: Color) {
        self.val = val;
    }

    pub fn get(&self) -> Color {
        self.val
    }
}

pub trait WebShowGPU{
    fn web_show(&self,row_size:usize,text:Vec<Text>);
}

#[derive(Clone, Debug)]
pub struct Gpu {
    transistors: Vec<GpuTransistor>
}

// struct Text{
//     text: String,
//     start_idx: usize,
// }

// impl Text {
//     pub fn new(&self,text:String,start_idx:usize)->Self{
//         Self{text,start_idx}
//     }
// }

pub struct Text {
    text: String,
    start_idx: usize,
}

impl Text {
    pub fn new<T: Into<String>>(text: T, start_idx: usize) -> Self {
        Self { 
            text: text.into(), 
            start_idx 
        }
    }
}

impl Gpu {
    pub fn new(total_transistors: usize) -> Self {
        let transistors = vec![GpuTransistor::new(); total_transistors];
        Self { transistors}
    }

    pub fn set_tsis_len(&mut self,tsis_len:usize){
        self.transistors=vec![GpuTransistor::new(); tsis_len];
    }

    pub fn set_x_transistor(&mut self,x:usize,color: Color){
        if let Some(tsis) = self.transistors.get_mut(x) {
            tsis.set(color);
        } else {
            eprintln!("Transistor index out of bounds: {}", x);
        }
    }

    pub fn set_all(&mut self, color: Color) {
        for t in &mut self.transistors {
            t.set(color);
        }
    }

    pub fn set_range(&mut self, start_index: usize, end_index: usize, color: Color) {
        for i in start_index..end_index {
            if let Some(t) = self.transistors.get_mut(i) {
                t.set(color);
            }
        }
    }

    pub fn show(&self, row_size: usize) {
        for (i,tsi) in self.transistors.iter().enumerate(){
            print!("{}", color_to_ansi(tsi.get()));
            if (i+1) % row_size == 0 {println!();} 
        }
    }

    
}

impl WebShowGPU for Gpu {
    fn web_show(&self, row_size: usize, texts: Vec<Text>) {
        for (i, tsi) in self.transistors.iter().enumerate() {
            let mut printed = false;
            
            for txt in &texts {
                let rel_idx = i as isize - txt.start_idx as isize;
                
                if rel_idx >= 0 {
                    let rel_idx_usize = rel_idx as usize;
                    
                    if rel_idx_usize < txt.text.len() {
                        if let Some(ch) = txt.text.chars().nth(rel_idx_usize) {
                            print!("{}", color_to_ansi_char(tsi.get(), ch));
                            printed = true;
                            break;
                        }
                    }
                }
            }
            
            if !printed {
                print!("{}", color_to_ansi(tsi.get()));
            }
            
            if (i + 1) % row_size == 0 {
                println!();
            }
        }
    }
}