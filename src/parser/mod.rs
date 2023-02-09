use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process;

use std::str;


// fn parse_config(args: &[String]) -> &str {
//     let file_path = &args[1];

//     file_path
// }

//
pub fn run() -> Result<(), Box<dyn Error>> {
    // let args: Vec<String> = env::args().collect();

    // let file_path = parse_config(&args);

    let file_path = dat2csv::get_dat_filename()?;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut file_name: String;
    let mut line;
    let mut export_count: u8 = 0; // Number of Record in the dat file
    let mut slash_count: u32 = 0; // Count for slashes within the Record Block. After slash #2 is Data
    let mut data_count: u32 = 0;

    let mut current_record = String::from("None");
    let mut record_fields = String::new();
    let mut data_line = String::new();
    let mut csv_data: String; 
    let mut csv_header: String; 

    let re_export = Regex::new(r"^EXPORT ").unwrap();
    let re_slash = Regex::new(r"^/").unwrap();
    let re_double_slash = Regex::new(r"^//").unwrap();

    let mut double_slash_match: bool;
    let mut slash_match: bool;
    // char defined with '' String or slice with ""
    // also how to assign backslash to a char '\' or '\\'
    let ps_codes: Vec<&str> = vec![
        "\"", r"\(", r"\)", "FL", "\\", "FN", "~", "OCICKM", "MCLP", "MCLP", "MCLP", "MCLP",
        "MCLP", "MCLP", "MCLP", "MCLP", "MFKA", "MCLP", "MFJC", "MFLN", "`", "MCLP", "MCLP",
        "MCLP", "MCLP", "MCLP", "MCLP", "MCLP", "MFKB", "MCLP", "MFJD", "MFLO", "MFLI", "MCKB",
        "MCKC", "MCKD", "MCLP", "MCKF", "MCLP", "MCKH", "MCLP", "MCKJ", "MCKK", "MCKL", "MCKM",
        "MCKN", "MCKO", "MCKP", "MCLA", "MCLB", "MCLC", "MCLD", "MCLP", "MCLF", "MCLG", "MCLH",
        "MCLP", "MCLJ", "MCLK", "MCLL", "MCLP", "MCLP", "MCLP", "MCLP", "MDIA", "MDIB", "MDIC",
        "MDID", "MDIE", "MDIF", "MDIG", "MDIH", "MDII", "MDIJ", "MDIK", "MDIL", "MDIM", "MDIN",
        "MDIO", "MDIP", "MDJA", "MDJB", "MDJC", "MDJD", "MDJE", "MDJF", "MDJG", "MDJH", "MDJI",
        "MDJJ", "MDJK", "MDJL", "MDJM", "MDJN", "MDJO", "MDJP", "MDKA", "MDKB", "MDKC", "MDKD",
        "MDKE", "MDKF", "MDKG", "MDKH", "MDKI", "MDKJ", "MDKK", "MDKL", "MDKM", "MDKN", "MDKO",
        "MDKP", "MDLA", "MDLB", "MDLC", "MDLD", "MDLE", "MDLF", "MDLG", "MDLH", "MDLI", "MDLJ",
        "MDLK", "MDLL", "MDLM", "MDLN", "MDLO", "MDLP",
    ];
    let iso_list: Vec<char> = vec![
        '"', '(', ')', '[', '\\', ']', '~', '€', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹',
        'Œ', 'Ž', '‘', '’', '“', '”', '•', '–', '—', '™', 'š', '›', 'œ', 'ž', 'Ÿ', '¡', '¢', '£',
        '¤', '¥', '¦', '§', '¨', '©', 'ª', '«', '¬', '­', '®', '¯', '°', '±', '²', '³', '´', 'µ',
        '¶', '·', '¸', '¹', 'º', '»', '¼', '½', '¾', '¿', 'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç',
        'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', 'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù',
        'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß', 'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë',
        'ì', 'í', 'î', 'ï', 'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý',
        'þ', 'ÿ',
    ];

    let mut code_hash = HashMap::new();
    let mut i = 0;
    for s in ps_codes {
        code_hash.insert(s, iso_list[i]);
        i += 1;
    }

    let mut whole_data_line = String::new();
    // TODO: refactor with std::iter::peekable() and String::char_indices()
    for l in reader.lines() {
        line = l.unwrap();

        slash_match = false;
        double_slash_match = false;

        // Get Record Name when EXPORT is found
        if re_export.is_match(&line) {
            export_count += 1;
            if export_count > 1 {
                current_record = extract_recname(&line).unwrap();
            }
            continue;
        }
        // Get Record Field Structure (slash 2) and Data (slash > 2) (may span through consecutive lines: block_line)
        // EXPORT Record, 1 /: Garbage, 2 /: Fields, 3 /: Data [double //], 4 /: end.
        if current_record != "None" {
            if re_double_slash.is_match(&line) {
                double_slash_match = true;
            } else if re_slash.is_match(&line) {
                slash_count += 1;
                slash_match = true;
            }

            // Get Field Structure
            if slash_count == 2 && !slash_match {
                record_fields.push_str(extract_fields(&line).unwrap().as_str());
                continue;
            }
            if slash_count == 3 && slash_match && data_count == 0 {
                // Once we are over the Fields block (slash_count==2), 
                // remove the last comma and add a newline to the record list 
                record_fields.pop(); 
                record_fields.push('\n');
            }
            if slash_count == 3 && !slash_match {
                // First data row starts with single slash
                if !double_slash_match {
                    // reference lines without \n, until double slash is found, then extract data
                    whole_data_line.push_str(&line);
                } else {
                    // when a double slash is found, extract data and reference move to data_line.
                    data_line
                        .push_str(extract_data(&whole_data_line, &code_hash).unwrap().as_str());
                    whole_data_line = String::new();
                
                    // when we find a // we start a new csvline
                    data_line.pop(); // delete last comma
                    data_line.push_str("\n"); // add newline at the end of each data row
                    data_count += 1;
                }
            }
            // Clean up Record data
            if slash_count == 4 && slash_match {
                file_name = current_record.clone();
                file_name.push_str(".csv");

                csv_data = data_line.clone();
                csv_header = record_fields.clone();
                // join header and data
                csv_header.push_str(&csv_data);
                println!("Parsing {}", file_name);
                if let Err(err) = write_csv(&csv_header, &file_name) {
                    println!("error running example: {}", err);
                    process::exit(1);
                }

                // data_line.pop();
                slash_count = 0;
                data_count = 0;
                record_fields = String::from("");
                data_line = String::from("");
            }
        }
    }
    // Create a json file
    Ok(())
}

// Writes a text file from a String of parsed values.
fn write_csv(dat: &str, f_name: &str) -> Result<(), Box<dyn Error>> {
    let mut output = File::create(f_name)?;
    output.write(dat.as_bytes()).unwrap();
    Ok(())
}

// Might want to try Clone-on-Write type Cow: https://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html/
fn extract_recname(s: &str) -> Result<String, Box<dyn Error>> {
    let mut rec = String::with_capacity(s.len());
    for (i, c) in s.chars().enumerate() {
        if c == '.' {
            break;
        }
        // EXPORT has 5 characters
        if i > 6 && c != ' ' {
            rec.push(c);
        }
    }
    match rec.len() {
        0 => Err(From::from("Record Name not found")),
        _ => Ok(rec),
    }
}

// Obtain a list of the fields contained in the record.
fn extract_fields(s: &str) -> Result<String, Box<dyn Error>> {
    let mut recfields = String::new();
    let mut eofield = 0;
    let mut tilde_count = 0;
    for (i, c) in s.chars().enumerate() {
        if c == ':' {
            // End of field
            eofield = i;
        }
        if c == '~' {
            // After 3 tildes ~ new fieldRec
            tilde_count += 1;
        }
        if tilde_count == 3 {
            tilde_count = 0;
            eofield = 0;
            recfields.push(',');
            continue;
        }
        if eofield > 0 {
            continue;
        }
        recfields.push(c);
    }
    match recfields.len() {
        0 => Err(From::from("Record Field Definition couldn't be found")),
        _ => Ok(recfields),
    }
}

// Parses data. It gets complete data row of codes A() and B(). Comma separated.
// Problem is some B Code come in separate blocks when newline
// PS escapes <"> <\> <(> <)> with <\> so, first <\> indicates a escape
// TODO: Use std::iter::Peekable and str::char_indices(&self)
fn extract_data(s: &str, code_hash: &HashMap<&str, char>) -> Result<String, Box<dyn Error>> {
    let mut data_list = String::new();
    let fld_separator: char = ',';
    let mut idx_a = 10;
    let mut idx_b = 10;
    let mut idx_p = 10;

    // flags for next read
    let mut read_data = false; // read data until closing )
    let mut read_code = false; // read code until closing )
    let mut part_code = false; // signal possible partial code read B(XX).

    let mut escape = 0;
    let mut code = String::new();
    let mut bcode_tracker = String::new();
    let mut closure = false;

    // Parses the s String char by char and copy it to data_list String
    for (i, c) in s.chars().enumerate() {
        // A closure is a closing block parenthesis. Make sure a escape doesn't precede it.
        if c == ')' && escape + 1 != i {
            closure = true;
        }
        // read_data flag true when the sequence A( is identified
        if read_data && !closure {
            data_list.push(c);
        }
        // keep track for a escape char \. We need to escape the char itself.
        if c == '\\' {
            escape = i;
        }
        // Keep track of B Code
        if c == 'B' {
            idx_b = i;
            if idx_p == 0 {
                idx_p = i;
            }
        }
        // read_code flag true when a sequence of B( is identified
        if !closure && read_code {
            code.push(c);
        }
        // deal with parted code B(MD)B(IJ) due to newline in the source
        if !closure && (read_code || part_code) && escape + 1 != i && c != '(' && idx_b != i {
            bcode_tracker.push(c);
        }
        // May start an A block of values
        if c == 'A' {
            idx_a = i;
        }
        // if ( follows A/B we're reading data/code. Set read_data read_code for next iterations
        if c == '(' {
            //
            if idx_a + 1 == i {
                read_data = true;
            }
            if idx_b + 1 == i {
                read_code = true;
            }
        }
        // Do not read more data after ). Ensure there's no Preceding Escape
        if closure && read_data {
            read_data = false;
        }
        // Do not read more code after ). Ensure there's no Preceding Escape.
        // Translate Code
        if closure && read_code {
            // println!("Read Code: {:?}", code);
            match code_hash.get(&code.as_str()) {
                Some(t) => {
                    data_list.push(*t);
                    bcode_tracker = String::new();
                    part_code = false;
                    // println!("Converted to {}", t);
                }
                None => {
                    // println!("Couldn't convert it. Set part_code to true just in case");
                    part_code = true; // if we couldn't convert it, maybe it's a parted code.
                }
            }
        }

        // part_code was set when second B was found. So when the next closure comes, either has a value in hash or we reset it.
        if closure && part_code {
            // println!("trying to match bcode {}", bcode_tracker);
            match code_hash.get(&bcode_tracker[..]) {
                Some(t) => {
                    data_list.push(*t);
                    // println!("Converted parted code to {}", t);
                    bcode_tracker = String::new();
                    part_code = false;
                }
                None => (),
            }
        }
        // field separator, inherited from source
        if c == ',' && escape + 1 != i {
            data_list.push(fld_separator);
        }
        // use a closure to reset flags except part_code
        if closure {
            closure = false;
            read_code = false;
            read_data = false;
            code = String::new();
        }
    }
    match data_list.len() {
        0 => Err(From::from("Data not found")),
        _ => Ok(data_list),
    }
}