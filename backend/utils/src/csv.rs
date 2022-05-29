use serde::de::DeserializeOwned;
use encoding_rs_io::DecodeReaderBytes;
use std::error::Error;

pub fn load_csv<T: DeserializeOwned>(buf: &[u8]) -> Result<Vec<T>, Box<dyn Error>> {
    let mut items: Vec<T> = Vec::new();

    // decode UTF-16LE to UTF-8
    let buf_as_utf8 = DecodeReaderBytes::new(buf);

    let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b'\t')
                    .has_headers(false)
                    .double_quote(true)
                    .flexible(true)
                    .escape(Some(b'\\'))
                    .from_reader(buf_as_utf8);

    for record in rdr.records().skip(2) { // skip headers
        let record_typed = record?.deserialize(None);
        let p: T = match record_typed {
            Ok(result) => {
                result
            },
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };
        items.push(p);
    }
    Ok(items)
}

