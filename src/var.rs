use std::any::{Any, TypeId};

#[derive(Debug)]
pub enum Var<'a> {
    Bool(&'a mut bool),

    F32(&'a mut f32),
    F64(&'a mut f64),

    I8(&'a mut i8),
    I16(&'a mut i16),
    I32(&'a mut i32),
    I64(&'a mut i64),
    I128(&'a mut i128),
    Isize(&'a mut isize),

    U8(&'a mut u8),
    U16(&'a mut u16),
    U32(&'a mut u32),
    U64(&'a mut u64),
    U128(&'a mut u128),
    Usize(&'a mut usize),

    String(&'a mut String),
}

impl<'a> Var<'a> {
    pub fn new(var: &'a mut dyn Any) -> Self {
        if (*var).type_id() == TypeId::of::<bool>() {
            Self::Bool(var.downcast_mut::<bool>().unwrap())
        } else if (*var).type_id() == TypeId::of::<i8>() {
            Self::I8(var.downcast_mut::<i8>().unwrap())
        } else if (*var).type_id() == TypeId::of::<i16>() {
            Self::I16(var.downcast_mut::<i16>().unwrap())
        } else if (*var).type_id() == TypeId::of::<i32>() {
            Self::I32(var.downcast_mut::<i32>().unwrap())
        } else if (*var).type_id() == TypeId::of::<i64>() {
            Self::I64(var.downcast_mut::<i64>().unwrap())
        } else if (*var).type_id() == TypeId::of::<i128>() {
            Self::I128(var.downcast_mut::<i128>().unwrap())
        } else if (*var).type_id() == TypeId::of::<isize>() {
            Self::Isize(var.downcast_mut::<isize>().unwrap())
        } else if (*var).type_id() == TypeId::of::<u8>() {
            Self::U8(var.downcast_mut::<u8>().unwrap())
        } else if (*var).type_id() == TypeId::of::<u16>() {
            Self::U16(var.downcast_mut::<u16>().unwrap())
        } else if (*var).type_id() == TypeId::of::<u32>() {
            Self::U32(var.downcast_mut::<u32>().unwrap())
        } else if (*var).type_id() == TypeId::of::<u64>() {
            Self::U64(var.downcast_mut::<u64>().unwrap())
        } else if (*var).type_id() == TypeId::of::<u128>() {
            Self::U128(var.downcast_mut::<u128>().unwrap())
        } else if (*var).type_id() == TypeId::of::<usize>() {
            Self::Usize(var.downcast_mut::<usize>().unwrap())
        } else if (*var).type_id() == TypeId::of::<f32>() {
            Self::F32(var.downcast_mut::<f32>().unwrap())
        } else if (*var).type_id() == TypeId::of::<f64>() {
            Self::F64(var.downcast_mut::<f64>().unwrap())
        } else if (*var).type_id() == TypeId::of::<String>() {
            Self::String(var.downcast_mut::<String>().unwrap())
        } else {
            panic!("Unsupported flag variable type.")
        }
    }

    pub fn unquotename(&self) -> &'static str {
        match self {
            Self::Bool(_) => "",
            Self::F32(_) => "f32",
            Self::F64(_) => "f64",
            Self::I8(_) => "i8",
            Self::I16(_) => "i16",
            Self::I32(_) => "i32",
            Self::I64(_) => "i64",
            Self::I128(_) => "i128",
            Self::Isize(_) => "isize",
            Self::U8(_) => "u8",
            Self::U16(_) => "u16",
            Self::U32(_) => "u32",
            Self::U64(_) => "u64",
            Self::U128(_) => "u128",
            Self::Usize(_) => "usize",
            Self::String(_) => "string",
        }
    }

    pub fn parse(&mut self, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::F32(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::F64(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::I8(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::I16(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::I32(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::I64(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::I128(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::Isize(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::U8(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::U16(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::U32(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::U64(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::U128(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::Usize(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::Bool(var) => {
                **var = match value.parse() {
                    Ok(var) => var,
                    Err(err) => return Err(Box::new(err)),
                };
            }
            Self::String(var) => **var = value.to_owned(),
        }
        Ok(())
    }
}
