use headers::serialization_utils::{unquote_string, push_quoted_string, quoted_string, WriterUtil};

#[deriving(Clone)]
pub struct EntityTag {
    weak: bool,
    opaque_tag: ~str,
}

impl ToStr for EntityTag {
    fn to_str(&self) -> ~str {
        if self.weak {
            push_quoted_string(~"W/", self.opaque_tag)
        } else {
            quoted_string(self.opaque_tag)
        }
    }
}

impl super::HeaderConvertible for EntityTag {
    fn from_stream<T: Reader>(reader: &mut super::HeaderValueByteIterator<T>) -> Option<EntityTag> {
        let s = reader.collect_to_str();
        let weak;
        let opaque_tag;
        match reader.next() {
            Some(b) if b == 'W' as u8 => match reader.next() {
                Some(b) if b == '/' as u8 => match reader.next() {
                    weak = true;
                    match reader.read_quoted_string(false) {
                        Some(tag) => opaque_tag = tag,
                        None => return None,
                    }
                },
                _ => return None,
            },
            Some(b) if b == '"' as u8 => {
                weak = false;
                match reader.read_quoted_string(true) {
                    Some(tag) => opaque_tag = tag,
                    None => return None,
                }
            },
            _ => {
                return None,
            }
        };
        Some(EntityTag {
            weak: weak,
            opaque_tag: opaque_tag,
        })
    }

    fn to_stream<W: Writer>(&self, writer: &mut W) {
        if self.weak {
            writer.write(bytes!("W/"));
        }
        writer.write_quoted_string(self.opaque_tag);
    }

    fn http_value(&self) -> ~str {
        self.to_str()
    }
}