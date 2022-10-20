// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use hex;
use anyhow::anyhow;
use prost::Message;
use aptos_protos::ferum::v1::{
    OrderCreateEvent as FerumOrderCreateEvent,
    OrderFinalizeEvent as FerumOrderFinalizeEvent,
    OrderExecutionEvent as FerumOrderExecutionEvent,
    PriceUpdateEvent as FerumPriceUpdateEvent,
    TypeInfo,
};

#[derive(Debug)]
pub enum FerumEvent {
    OrderCreate(FerumOrderCreateEvent),
    OrderFinalize(FerumOrderFinalizeEvent),
    OrderExecution(FerumOrderExecutionEvent),
    PriceUpdate(FerumPriceUpdateEvent),
}
impl FerumEvent {
    pub fn encode(&self, buf: &mut Vec<u8>) -> Result<(), prost::EncodeError>
    {
        match self {
            Self::OrderCreate(e) => e.encode(buf),
            Self::OrderFinalize(e) => e.encode(buf),
            Self::OrderExecution(e) => e.encode(buf),
            Self::PriceUpdate(e) => e.encode(buf),
        }
    }
}

pub fn convert_ferum_event(evt: &FerumEvent) -> anyhow::Result<FerumEvent> {
    let inner = (|| match evt {
        FerumEvent::OrderCreate(evt) => {
            let metadata = evt.order_metadata.as_ref()?;
            let instrument_type = metadata.instrument_type.as_ref()?;
            let quote_type = metadata.quote_type.as_ref()?;

            let cpy = &mut evt.clone();
            cpy.order_metadata.as_mut()?.instrument_type = convert_type_info(instrument_type).ok();
            cpy.order_metadata.as_mut()?.quote_type = convert_type_info(quote_type).ok();

            Some(FerumEvent::OrderCreate(cpy.clone()))
        }
        FerumEvent::OrderExecution(evt) => {
            let metadata = evt.order_metadata.as_ref()?;
            let instrument_type = metadata.instrument_type.as_ref()?;
            let quote_type = metadata.quote_type.as_ref()?;

            let cpy = &mut evt.clone();
            cpy.order_metadata.as_mut()?.instrument_type = convert_type_info(instrument_type).ok();
            cpy.order_metadata.as_mut()?.quote_type = convert_type_info(quote_type).ok();

            Some(FerumEvent::OrderExecution(cpy.clone()))
        }
        FerumEvent::OrderFinalize(evt) => {
            let metadata = evt.order_metadata.as_ref()?;
            let instrument_type = metadata.instrument_type.as_ref()?;
            let quote_type = metadata.quote_type.as_ref()?;

            let cpy = &mut evt.clone();
            cpy.order_metadata.as_mut()?.instrument_type = convert_type_info(instrument_type).ok();
            cpy.order_metadata.as_mut()?.quote_type = convert_type_info(quote_type).ok();

            Some(FerumEvent::OrderFinalize(cpy.clone()))
        }
        FerumEvent::PriceUpdate(evt) => {
            let quote = evt.data.as_ref()?;
            let instrument_type = quote.instrument_type.as_ref()?;
            let quote_type = quote.quote_type.as_ref()?;

            let cpy = &mut evt.clone();
            cpy.data.as_mut()?.instrument_type = convert_type_info(instrument_type).ok();
            cpy.data.as_mut()?.quote_type = convert_type_info(quote_type).ok();

            Some(FerumEvent::PriceUpdate(cpy.clone()))
        }
    })();

    inner.ok_or(anyhow!("error"))
}

/// Decodes the module_name and name of the type info object.
pub fn convert_type_info(type_info: &TypeInfo) -> anyhow::Result<TypeInfo> {
    let mut out = type_info.clone();
    out.module_name = convert_hex_string(out.module_name.to_string())?;
    out.struct_name = convert_hex_string(out.struct_name.to_string())?;
    Ok(out)
}

pub fn convert_hex_string(mut s: String) -> anyhow::Result<String> {
    if s.len() >= 2 && s[..2].eq("0x") {
        s = s[2..].to_string();
    }

    let decoded = hex::decode(&s)?;
    let inner = String::from_utf8(decoded)?;
    Ok(inner)
}
