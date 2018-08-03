#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

type Color = &'static str;

// https://en.wikipedia.org/wiki/ANSI_escape_code#3/4_bit
pub const PREFIX: Color = "\x1b[";

pub const BLACK: Color = "0;30m";
pub const RED: Color = "0;31m";
pub const GREEN: Color = "0;32m";
pub const YELLOW: Color = "0;33m";
pub const BLUE: Color = "0;34m";
pub const MAGENTA: Color = "0;35m";
pub const CYAN: Color = "0;36m";
pub const WHITE: Color = "0;37m";

pub const BRIGHT_BLACK: Color = "1;30m";
pub const BRIGHT_RED: Color = "1;31m";
pub const BRIGHT_GREEN: Color = "1;32m";
pub const BRIGHT_YELLOW: Color = "1;33m";
pub const BRIGHT_BLUE: Color = "1;34m";
pub const BRIGHT_MAGENTA: Color = "1;35m";
pub const BRIGHT_CYAN: Color = "1;36m";
pub const BRIGHT_WHITE: Color = "1;37m";

pub const NONE: Color = "0m";


pub fn color(text: String, color: Color) -> String {
    let mut result = PREFIX.to_string();
    result.push_str(color);
    result += &text;
    result.push_str(PREFIX);
    result.push_str(NONE);
    return result;
}
pub trait Dye{
    fn red(&self)->String;
    fn green(&self)->String;
    fn black(&self)->String;
    fn white(&self)->String;
    fn magenta(&self)->String;
    fn blue(&self)->String;
    fn yellow(&self)->String;
    fn cyan(&self)->String;

    fn bright_red(&self)->String;
    fn bright_green(&self)->String;
    fn bright_black(&self)->String;
    fn bright_white(&self)->String;
    fn bright_magenta(&self)->String;
    fn bright_blue(&self)->String;
    fn bright_yellow(&self)->String;
    fn bright_cyan(&self)->String;

    fn color(&self,c:Color)->String;
}
impl <'a>Dye for &'a str {
    fn red(&self) -> String {
        return self.color(RED);
    }

     fn green(& self) -> String {
        return self.color(GREEN);
    }
    fn black(&self) -> String {
        return self.color(BLACK);
    }

    fn white(&self) -> String {
        return self.color(WHITE);
    }

    fn magenta(&self) -> String {
        return self.color(MAGENTA);
    }

    fn blue(&self) -> String {
        return self.color(BLUE);
    }

    fn yellow(&self) -> String {
        return self.color(YELLOW);
    }

    fn cyan(&self) -> String {
        return self.color(CYAN);
    }

    fn bright_red(&self) -> String {
        return self.color(BRIGHT_RED);
    }

    fn bright_green(&self) -> String {
        return self.color(BRIGHT_GREEN);
    }

    fn bright_black(&self) -> String {
        return self.color(BRIGHT_BLACK);
    }

    fn bright_white(&self) -> String {
        return self.color(BRIGHT_WHITE);
    }

    fn bright_magenta(&self) -> String {
        return self.color(BRIGHT_MAGENTA);
    }

    fn bright_blue(&self) -> String {
        return self.color(BRIGHT_BLUE);
    }

    fn bright_yellow(&self) -> String {
        return self.color(BRIGHT_YELLOW);
    }

    fn bright_cyan(&self) -> String {
        return self.color(BRIGHT_CYAN);
    }


     fn color(&self, color: Color) -> String {
        let mut result = PREFIX.to_string();
        result.push_str(color);
        result += &self;
        result.push_str(PREFIX);
        result.push_str(NONE);
        return result;
    }


}
