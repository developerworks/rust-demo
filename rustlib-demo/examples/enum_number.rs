use num_derive::ToPrimitive;

use num_traits::ToPrimitive;

#[derive(Debug, ToPrimitive)]
pub enum GlobalMessageTypes {
    SUCCESS = 0,
    FAILED = 1,
    END = 7777,
}

use num_enum::IntoPrimitive;

#[rustfmt::skip]
#[derive(Debug, IntoPrimitive)]
#[repr(u64)]
pub enum GlobalMessage {
    Success = 1,
    Failed = 2,
    End = 3,
}

fn num_derive() {
    println!("GlobalMessageTypes: {:?}", GlobalMessageTypes::SUCCESS.to_u64().unwrap());
    println!("GlobalMessageTypes: {:?}", GlobalMessageTypes::FAILED.to_u64().unwrap());
    println!("GlobalMessageTypes: {:?}", GlobalMessageTypes::END.to_u64().unwrap());
}

fn num_enum() {
    let success: u64 = GlobalMessage::Success.into();
    println!("GlobalMessage: {:?}", success);
    let failed: u64 = GlobalMessage::Failed.into();
    println!("GlobalMessage: {:?}", failed);
    let end: u64 = GlobalMessage::End.into();
    println!("GlobalMessage: {:?}", end);
}

fn main() {
    num_derive();
    num_enum();
}