use std::fs;

pub struct Letter{
    pub shift:usize,
    pub image:String,
    pub letter:char,
    pub uint8:u8,
    pub bin:Vec<String>,
    pub bin_graphic:Vec<String>,
}

impl Letter{
    pub fn new(letter:&char)-> Letter{
        let out:Letter = Letter {
            shift: 0,
            image:Self::genImage(letter),
            letter:Self::genLetter(letter),
            uint8:Self::genUint8(letter),
            bin:Self::genBin(letter),
            bin_graphic:Self::genBinGra(letter),
        };
        out
    }

    fn genImage(letter:&char)-> String{
        let readFrom = (*letter as u32 - 65) * 8 + 0;
        let contents = fs::read_to_string("text.txt")
            .expect("can't find the text");

        contents.lines()
            .enumerate()
            .filter_map(|(index, line)|{
                if index >= (readFrom as usize) && index < (readFrom as usize + 8){
                    Some(line.to_string())
                }else{
                    None
                }
            })
        .collect::<Vec<String>>()
            .join("\n")
    }

    fn genLetter(letter:&char) -> char {
        let out:char = *letter;
        out
    }

    fn genUint8(letter:&char)-> u8{
        let out:u8 = *letter as u8;
        out
    }

    fn genBin(letter:&char)-> Vec<String>{
        let mut str_bin = String::from("0");
        let bin:String = format!("{:b}",*letter as u8);
        str_bin = str_bin + &bin;
        let mut out:Vec<String> = Vec::new();

        for i in str_bin.chars() {
            out.push(i.to_string());
        }
        out
    }

    fn genBinGra(letter:&char)-> Vec<String>{
        let bin_vec = Self::genBin(letter);
        let mut out = Vec::new();
        for i in bin_vec {
            if i == "0" {
                out.push(" ".to_string());
            } else {
                out.push("▇".to_string());
            }
        }
        out
    }

    pub fn indexedLetter(oldOne:Letter, index:usize) -> Letter{
        let output:Letter = Letter {
            shift: index,
            image:Self::indexedImage(oldOne.image, index),
            letter:oldOne.letter,
            uint8:oldOne.uint8,
            bin:Self::indexedBin(&oldOne.bin, index),
            bin_graphic:Self::indexedBinGra(&oldOne.bin, index),
        };
        output
    }

    fn indexedImage(old:String, indexed:usize) -> String{
        let output = old.lines()
            .enumerate()
            .filter_map(|(index, line)|{
                if index == indexed % 8{
                    Some(line.to_string())
                }else{
                    None
                }
            })
        .collect::<Vec<String>>()
            .join("\n");
        output
    }

    fn indexedBin(old:&Vec<String>, indexed:usize)-> Vec<String>{
        // 准备进行右移
        let mut out: Vec<String> = Vec::new();
        let len = old.len();
        // 计算每个元素在移位后的新位置
        for (index, item) in old.iter().enumerate() {
            let new_index = (index + indexed) % len; // 使用取模操作实现循环
                                                     // 确保out有足够的长度
            if out.len() <= new_index {
                out.resize(new_index + 1, String::new());
            }
            out[new_index] = item.clone();
        }
        // 确保out的长度与outStd相同，填充任何缺失的元素
        out.resize(len, String::from("0"));
        out
    }

    fn indexedBinGra(old:&Vec<String>, indexed:usize)-> Vec<String>{
        // 准备进行右移
        let len = old.len();
        let mut out: Vec<String> = vec![String::new(); len]; // 初始化足够长度的Vec，避免resize
        // 计算每个元素在移位后的新位置
        for (index, item) in old.iter().enumerate() {
            let new_index = (index + indexed) % len; // 使用取模操作实现循环
                                                     // 根据 item 的值转换为新的字符串
            let new_item = if item == "0" { " ".to_string() } else { "▇".to_string() };
            out[new_index] = new_item;
        }

        out
    }

    pub fn recover(old:Letter) -> Letter {

        let out:Letter = Letter {
            shift: 0,
            image:Self::genImage(&old.letter),
            letter:Self::genLetter(&old.letter),
            uint8:Self::genUint8(&old.letter),
            bin:Self::genBin(&old.letter),
            bin_graphic:Self::genBinGra(&old.letter),
        };
        out
    }

    pub fn derefer(old:&Letter)-> Letter{
        let out:Letter = Letter {
            shift: 0,
            image:Self::genImage(&old.letter),
            letter:Self::genLetter(&old.letter),
            uint8:Self::genUint8(&old.letter),
            bin:Self::genBin(&old.letter),
            bin_graphic:Self::genBinGra(&old.letter),
        };
        out

    }

}

pub fn outputString(vector: &Vec<String>) -> String{
    let mut output:String = String::new();
    for i in vector {
        output = output + &i;
    }
    output
}

