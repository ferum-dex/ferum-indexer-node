// @generated
impl serde::Serialize for CancelAgent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "CancelAgentNone",
            Self::Ioc => "CancelAgentIOC",
            Self::Fok => "CancelAgentFOK",
            Self::User => "CancelAgentUser",
            Self::Post => "CancelAgentPost",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CancelAgent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CancelAgentNone",
            "CancelAgentIOC",
            "CancelAgentFOK",
            "CancelAgentUser",
            "CancelAgentPost",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelAgent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(CancelAgent::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(CancelAgent::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CancelAgentNone" => Ok(CancelAgent::None),
                    "CancelAgentIOC" => Ok(CancelAgent::Ioc),
                    "CancelAgentFOK" => Ok(CancelAgent::Fok),
                    "CancelAgentUser" => Ok(CancelAgent::User),
                    "CancelAgentPost" => Ok(CancelAgent::Post),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FixedPoint64 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.val != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.FixedPoint64", len)?;
        if self.val != 0 {
            struct_ser.serialize_field("val", ToString::to_string(&self.val).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FixedPoint64 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "val",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Val,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "val" => Ok(GeneratedField::Val),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FixedPoint64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.FixedPoint64")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FixedPoint64, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut val__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Val => {
                            if val__.is_some() {
                                return Err(serde::de::Error::duplicate_field("val"));
                            }
                            val__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(FixedPoint64 {
                    val: val__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.FixedPoint64", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderCreateEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order_id.is_some() {
            len += 1;
        }
        if self.order_metadata.is_some() {
            len += 1;
        }
        if self.timestamp_micro_seconds != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.OrderCreateEvent", len)?;
        if let Some(v) = self.order_id.as_ref() {
            struct_ser.serialize_field("orderID", v)?;
        }
        if let Some(v) = self.order_metadata.as_ref() {
            struct_ser.serialize_field("orderMetadata", v)?;
        }
        if self.timestamp_micro_seconds != 0 {
            struct_ser.serialize_field("timestampMicroSeconds", ToString::to_string(&self.timestamp_micro_seconds).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orderID",
            "orderMetadata",
            "timestampMicroSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            OrderMetadata,
            TimestampMicroSeconds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "orderID" => Ok(GeneratedField::OrderId),
                            "orderMetadata" => Ok(GeneratedField::OrderMetadata),
                            "timestampMicroSeconds" => Ok(GeneratedField::TimestampMicroSeconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderCreateEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.OrderCreateEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OrderCreateEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut order_metadata__ = None;
                let mut timestamp_micro_seconds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderID"));
                            }
                            order_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OrderMetadata => {
                            if order_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderMetadata"));
                            }
                            order_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimestampMicroSeconds => {
                            if timestamp_micro_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampMicroSeconds"));
                            }
                            timestamp_micro_seconds__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(OrderCreateEvent {
                    order_id: order_id__,
                    order_metadata: order_metadata__,
                    timestamp_micro_seconds: timestamp_micro_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.OrderCreateEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderExecutionEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order_id.is_some() {
            len += 1;
        }
        if self.order_metadata.is_some() {
            len += 1;
        }
        if self.opposite_order_id.is_some() {
            len += 1;
        }
        if self.opposite_order_metadata.is_some() {
            len += 1;
        }
        if self.price.is_some() {
            len += 1;
        }
        if self.qty.is_some() {
            len += 1;
        }
        if self.timestamp_micro_seconds != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.OrderExecutionEvent", len)?;
        if let Some(v) = self.order_id.as_ref() {
            struct_ser.serialize_field("orderID", v)?;
        }
        if let Some(v) = self.order_metadata.as_ref() {
            struct_ser.serialize_field("orderMetadata", v)?;
        }
        if let Some(v) = self.opposite_order_id.as_ref() {
            struct_ser.serialize_field("oppositeOrderID", v)?;
        }
        if let Some(v) = self.opposite_order_metadata.as_ref() {
            struct_ser.serialize_field("oppositeOrderMetadata", v)?;
        }
        if let Some(v) = self.price.as_ref() {
            struct_ser.serialize_field("price", v)?;
        }
        if let Some(v) = self.qty.as_ref() {
            struct_ser.serialize_field("qty", v)?;
        }
        if self.timestamp_micro_seconds != 0 {
            struct_ser.serialize_field("timestampMicroSeconds", ToString::to_string(&self.timestamp_micro_seconds).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderExecutionEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orderID",
            "orderMetadata",
            "oppositeOrderID",
            "oppositeOrderMetadata",
            "price",
            "qty",
            "timestampMicroSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            OrderMetadata,
            OppositeOrderId,
            OppositeOrderMetadata,
            Price,
            Qty,
            TimestampMicroSeconds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "orderID" => Ok(GeneratedField::OrderId),
                            "orderMetadata" => Ok(GeneratedField::OrderMetadata),
                            "oppositeOrderID" => Ok(GeneratedField::OppositeOrderId),
                            "oppositeOrderMetadata" => Ok(GeneratedField::OppositeOrderMetadata),
                            "price" => Ok(GeneratedField::Price),
                            "qty" => Ok(GeneratedField::Qty),
                            "timestampMicroSeconds" => Ok(GeneratedField::TimestampMicroSeconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderExecutionEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.OrderExecutionEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OrderExecutionEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut order_metadata__ = None;
                let mut opposite_order_id__ = None;
                let mut opposite_order_metadata__ = None;
                let mut price__ = None;
                let mut qty__ = None;
                let mut timestamp_micro_seconds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderID"));
                            }
                            order_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OrderMetadata => {
                            if order_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderMetadata"));
                            }
                            order_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::OppositeOrderId => {
                            if opposite_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oppositeOrderID"));
                            }
                            opposite_order_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OppositeOrderMetadata => {
                            if opposite_order_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oppositeOrderMetadata"));
                            }
                            opposite_order_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map.next_value()?);
                        }
                        GeneratedField::Qty => {
                            if qty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("qty"));
                            }
                            qty__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimestampMicroSeconds => {
                            if timestamp_micro_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampMicroSeconds"));
                            }
                            timestamp_micro_seconds__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(OrderExecutionEvent {
                    order_id: order_id__,
                    order_metadata: order_metadata__,
                    opposite_order_id: opposite_order_id__,
                    opposite_order_metadata: opposite_order_metadata__,
                    price: price__,
                    qty: qty__,
                    timestamp_micro_seconds: timestamp_micro_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.OrderExecutionEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderFinalizeEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order_id.is_some() {
            len += 1;
        }
        if self.order_metadata.is_some() {
            len += 1;
        }
        if self.cancel_agent != 0 {
            len += 1;
        }
        if self.timestamp_micro_seconds != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.OrderFinalizeEvent", len)?;
        if let Some(v) = self.order_id.as_ref() {
            struct_ser.serialize_field("orderID", v)?;
        }
        if let Some(v) = self.order_metadata.as_ref() {
            struct_ser.serialize_field("orderMetadata", v)?;
        }
        if self.cancel_agent != 0 {
            let v = CancelAgent::from_i32(self.cancel_agent)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.cancel_agent)))?;
            struct_ser.serialize_field("cancelAgent", &v)?;
        }
        if self.timestamp_micro_seconds != 0 {
            struct_ser.serialize_field("timestampMicroSeconds", ToString::to_string(&self.timestamp_micro_seconds).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderFinalizeEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orderID",
            "orderMetadata",
            "cancelAgent",
            "timestampMicroSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            OrderMetadata,
            CancelAgent,
            TimestampMicroSeconds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "orderID" => Ok(GeneratedField::OrderId),
                            "orderMetadata" => Ok(GeneratedField::OrderMetadata),
                            "cancelAgent" => Ok(GeneratedField::CancelAgent),
                            "timestampMicroSeconds" => Ok(GeneratedField::TimestampMicroSeconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderFinalizeEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.OrderFinalizeEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OrderFinalizeEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut order_metadata__ = None;
                let mut cancel_agent__ = None;
                let mut timestamp_micro_seconds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderID"));
                            }
                            order_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OrderMetadata => {
                            if order_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderMetadata"));
                            }
                            order_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::CancelAgent => {
                            if cancel_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelAgent"));
                            }
                            cancel_agent__ = Some(map.next_value::<CancelAgent>()? as i32);
                        }
                        GeneratedField::TimestampMicroSeconds => {
                            if timestamp_micro_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampMicroSeconds"));
                            }
                            timestamp_micro_seconds__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(OrderFinalizeEvent {
                    order_id: order_id__,
                    order_metadata: order_metadata__,
                    cancel_agent: cancel_agent__.unwrap_or_default(),
                    timestamp_micro_seconds: timestamp_micro_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.OrderFinalizeEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.counter != 0 {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.OrderID", len)?;
        if self.counter != 0 {
            struct_ser.serialize_field("counter", ToString::to_string(&self.counter).as_str())?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "counter",
            "owner",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Counter,
            Owner,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "counter" => Ok(GeneratedField::Counter),
                            "owner" => Ok(GeneratedField::Owner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.OrderID")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OrderId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut counter__ = None;
                let mut owner__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Counter => {
                            if counter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counter"));
                            }
                            counter__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OrderId {
                    counter: counter__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.OrderID", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instrument_type.is_some() {
            len += 1;
        }
        if self.quote_type.is_some() {
            len += 1;
        }
        if self.side != 0 {
            len += 1;
        }
        if self.remaining_qty.is_some() {
            len += 1;
        }
        if self.original_qty.is_some() {
            len += 1;
        }
        if self.price.is_some() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.client_order_id.is_empty() {
            len += 1;
        }
        if self.execution_counter != 0 {
            len += 1;
        }
        if self.update_counter != 0 {
            len += 1;
        }
        if self.user_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.OrderMetadata", len)?;
        if let Some(v) = self.instrument_type.as_ref() {
            struct_ser.serialize_field("instrumentType", v)?;
        }
        if let Some(v) = self.quote_type.as_ref() {
            struct_ser.serialize_field("quoteType", v)?;
        }
        if self.side != 0 {
            let v = OrderSide::from_i32(self.side)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.side)))?;
            struct_ser.serialize_field("side", &v)?;
        }
        if let Some(v) = self.remaining_qty.as_ref() {
            struct_ser.serialize_field("remainingQty", v)?;
        }
        if let Some(v) = self.original_qty.as_ref() {
            struct_ser.serialize_field("originalQty", v)?;
        }
        if let Some(v) = self.price.as_ref() {
            struct_ser.serialize_field("price", v)?;
        }
        if self.r#type != 0 {
            let v = OrderType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.status != 0 {
            let v = OrderStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.client_order_id.is_empty() {
            struct_ser.serialize_field("clientOrderID", &self.client_order_id)?;
        }
        if self.execution_counter != 0 {
            struct_ser.serialize_field("executionCounter", ToString::to_string(&self.execution_counter).as_str())?;
        }
        if self.update_counter != 0 {
            struct_ser.serialize_field("updateCounter", ToString::to_string(&self.update_counter).as_str())?;
        }
        if let Some(v) = self.user_identifier.as_ref() {
            struct_ser.serialize_field("userIdentifier", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instrumentType",
            "quoteType",
            "side",
            "remainingQty",
            "originalQty",
            "price",
            "type",
            "status",
            "clientOrderID",
            "executionCounter",
            "updateCounter",
            "userIdentifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstrumentType,
            QuoteType,
            Side,
            RemainingQty,
            OriginalQty,
            Price,
            Type,
            Status,
            ClientOrderId,
            ExecutionCounter,
            UpdateCounter,
            UserIdentifier,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instrumentType" => Ok(GeneratedField::InstrumentType),
                            "quoteType" => Ok(GeneratedField::QuoteType),
                            "side" => Ok(GeneratedField::Side),
                            "remainingQty" => Ok(GeneratedField::RemainingQty),
                            "originalQty" => Ok(GeneratedField::OriginalQty),
                            "price" => Ok(GeneratedField::Price),
                            "type" => Ok(GeneratedField::Type),
                            "status" => Ok(GeneratedField::Status),
                            "clientOrderID" => Ok(GeneratedField::ClientOrderId),
                            "executionCounter" => Ok(GeneratedField::ExecutionCounter),
                            "updateCounter" => Ok(GeneratedField::UpdateCounter),
                            "userIdentifier" => Ok(GeneratedField::UserIdentifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.OrderMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OrderMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instrument_type__ = None;
                let mut quote_type__ = None;
                let mut side__ = None;
                let mut remaining_qty__ = None;
                let mut original_qty__ = None;
                let mut price__ = None;
                let mut r#type__ = None;
                let mut status__ = None;
                let mut client_order_id__ = None;
                let mut execution_counter__ = None;
                let mut update_counter__ = None;
                let mut user_identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstrumentType => {
                            if instrument_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentType"));
                            }
                            instrument_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::QuoteType => {
                            if quote_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteType"));
                            }
                            quote_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Side => {
                            if side__.is_some() {
                                return Err(serde::de::Error::duplicate_field("side"));
                            }
                            side__ = Some(map.next_value::<OrderSide>()? as i32);
                        }
                        GeneratedField::RemainingQty => {
                            if remaining_qty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remainingQty"));
                            }
                            remaining_qty__ = Some(map.next_value()?);
                        }
                        GeneratedField::OriginalQty => {
                            if original_qty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalQty"));
                            }
                            original_qty__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<OrderType>()? as i32);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<OrderStatus>()? as i32);
                        }
                        GeneratedField::ClientOrderId => {
                            if client_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientOrderID"));
                            }
                            client_order_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExecutionCounter => {
                            if execution_counter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionCounter"));
                            }
                            execution_counter__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::UpdateCounter => {
                            if update_counter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateCounter"));
                            }
                            update_counter__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::UserIdentifier => {
                            if user_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIdentifier"));
                            }
                            user_identifier__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OrderMetadata {
                    instrument_type: instrument_type__,
                    quote_type: quote_type__,
                    side: side__.unwrap_or_default(),
                    remaining_qty: remaining_qty__,
                    original_qty: original_qty__,
                    price: price__,
                    r#type: r#type__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    client_order_id: client_order_id__.unwrap_or_default(),
                    execution_counter: execution_counter__.unwrap_or_default(),
                    update_counter: update_counter__.unwrap_or_default(),
                    user_identifier: user_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.OrderMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderSide {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "OrderSideNone",
            Self::Sell => "OrderSideSell",
            Self::Buy => "OrderSideBuy",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OrderSide {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OrderSideNone",
            "OrderSideSell",
            "OrderSideBuy",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderSide;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(OrderSide::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(OrderSide::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OrderSideNone" => Ok(OrderSide::None),
                    "OrderSideSell" => Ok(OrderSide::Sell),
                    "OrderSideBuy" => Ok(OrderSide::Buy),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OrderStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "OrderStatusNone",
            Self::Pending => "OrderStatusPending",
            Self::Cancelled => "OrderStatusCancelled",
            Self::Filled => "OrderStatusFilled",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OrderStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OrderStatusNone",
            "OrderStatusPending",
            "OrderStatusCancelled",
            "OrderStatusFilled",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(OrderStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(OrderStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OrderStatusNone" => Ok(OrderStatus::None),
                    "OrderStatusPending" => Ok(OrderStatus::Pending),
                    "OrderStatusCancelled" => Ok(OrderStatus::Cancelled),
                    "OrderStatusFilled" => Ok(OrderStatus::Filled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OrderType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "OrderTypeNone",
            Self::Resting => "OrderTypeResting",
            Self::Post => "OrderTypePost",
            Self::Ioc => "OrderTypeIOC",
            Self::Fok => "OrderTypeFOK",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OrderType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OrderTypeNone",
            "OrderTypeResting",
            "OrderTypePost",
            "OrderTypeIOC",
            "OrderTypeFOK",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(OrderType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(OrderType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OrderTypeNone" => Ok(OrderType::None),
                    "OrderTypeResting" => Ok(OrderType::Resting),
                    "OrderTypePost" => Ok(OrderType::Post),
                    "OrderTypeIOC" => Ok(OrderType::Ioc),
                    "OrderTypeFOK" => Ok(OrderType::Fok),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PriceUpdateEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.PriceUpdateEvent", len)?;
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PriceUpdateEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceUpdateEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.PriceUpdateEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PriceUpdateEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PriceUpdateEvent {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.PriceUpdateEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Quote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instrument_type.is_some() {
            len += 1;
        }
        if self.quote_type.is_some() {
            len += 1;
        }
        if self.max_bid.is_some() {
            len += 1;
        }
        if self.bid_size.is_some() {
            len += 1;
        }
        if self.min_ask.is_some() {
            len += 1;
        }
        if self.ask_size.is_some() {
            len += 1;
        }
        if self.timestamp_micro_seconds != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.Quote", len)?;
        if let Some(v) = self.instrument_type.as_ref() {
            struct_ser.serialize_field("instrumentType", v)?;
        }
        if let Some(v) = self.quote_type.as_ref() {
            struct_ser.serialize_field("quoteType", v)?;
        }
        if let Some(v) = self.max_bid.as_ref() {
            struct_ser.serialize_field("maxBid", v)?;
        }
        if let Some(v) = self.bid_size.as_ref() {
            struct_ser.serialize_field("bidSize", v)?;
        }
        if let Some(v) = self.min_ask.as_ref() {
            struct_ser.serialize_field("minAsk", v)?;
        }
        if let Some(v) = self.ask_size.as_ref() {
            struct_ser.serialize_field("askSize", v)?;
        }
        if self.timestamp_micro_seconds != 0 {
            struct_ser.serialize_field("timestampMicroSeconds", ToString::to_string(&self.timestamp_micro_seconds).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instrumentType",
            "quoteType",
            "maxBid",
            "bidSize",
            "minAsk",
            "askSize",
            "timestampMicroSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstrumentType,
            QuoteType,
            MaxBid,
            BidSize,
            MinAsk,
            AskSize,
            TimestampMicroSeconds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instrumentType" => Ok(GeneratedField::InstrumentType),
                            "quoteType" => Ok(GeneratedField::QuoteType),
                            "maxBid" => Ok(GeneratedField::MaxBid),
                            "bidSize" => Ok(GeneratedField::BidSize),
                            "minAsk" => Ok(GeneratedField::MinAsk),
                            "askSize" => Ok(GeneratedField::AskSize),
                            "timestampMicroSeconds" => Ok(GeneratedField::TimestampMicroSeconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.Quote")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Quote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instrument_type__ = None;
                let mut quote_type__ = None;
                let mut max_bid__ = None;
                let mut bid_size__ = None;
                let mut min_ask__ = None;
                let mut ask_size__ = None;
                let mut timestamp_micro_seconds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstrumentType => {
                            if instrument_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentType"));
                            }
                            instrument_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::QuoteType => {
                            if quote_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteType"));
                            }
                            quote_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxBid => {
                            if max_bid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBid"));
                            }
                            max_bid__ = Some(map.next_value()?);
                        }
                        GeneratedField::BidSize => {
                            if bid_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bidSize"));
                            }
                            bid_size__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinAsk => {
                            if min_ask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minAsk"));
                            }
                            min_ask__ = Some(map.next_value()?);
                        }
                        GeneratedField::AskSize => {
                            if ask_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("askSize"));
                            }
                            ask_size__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimestampMicroSeconds => {
                            if timestamp_micro_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampMicroSeconds"));
                            }
                            timestamp_micro_seconds__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(Quote {
                    instrument_type: instrument_type__,
                    quote_type: quote_type__,
                    max_bid: max_bid__,
                    bid_size: bid_size__,
                    min_ask: min_ask__,
                    ask_size: ask_size__,
                    timestamp_micro_seconds: timestamp_micro_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.Quote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypeInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account_address.is_empty() {
            len += 1;
        }
        if !self.module_name.is_empty() {
            len += 1;
        }
        if !self.struct_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.TypeInfo", len)?;
        if !self.account_address.is_empty() {
            struct_ser.serialize_field("account_address", &self.account_address)?;
        }
        if !self.module_name.is_empty() {
            struct_ser.serialize_field("module_name", &self.module_name)?;
        }
        if !self.struct_name.is_empty() {
            struct_ser.serialize_field("struct_name", &self.struct_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypeInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_address",
            "module_name",
            "struct_name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountAddress,
            ModuleName,
            StructName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "account_address" => Ok(GeneratedField::AccountAddress),
                            "module_name" => Ok(GeneratedField::ModuleName),
                            "struct_name" => Ok(GeneratedField::StructName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.TypeInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TypeInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_address__ = None;
                let mut module_name__ = None;
                let mut struct_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AccountAddress => {
                            if account_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account_address"));
                            }
                            account_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModuleName => {
                            if module_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("module_name"));
                            }
                            module_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::StructName => {
                            if struct_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("struct_name"));
                            }
                            struct_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TypeInfo {
                    account_address: account_address__.unwrap_or_default(),
                    module_name: module_name__.unwrap_or_default(),
                    struct_name: struct_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.TypeInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.protocol_address.is_empty() {
            len += 1;
        }
        if !self.user_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.ferum.v1.UserIdentifier", len)?;
        if !self.protocol_address.is_empty() {
            struct_ser.serialize_field("protocolAddress", &self.protocol_address)?;
        }
        if !self.user_address.is_empty() {
            struct_ser.serialize_field("userAddress", &self.user_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocolAddress",
            "userAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtocolAddress,
            UserAddress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "protocolAddress" => Ok(GeneratedField::ProtocolAddress),
                            "userAddress" => Ok(GeneratedField::UserAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.ferum.v1.UserIdentifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol_address__ = None;
                let mut user_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProtocolAddress => {
                            if protocol_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolAddress"));
                            }
                            protocol_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserAddress => {
                            if user_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAddress"));
                            }
                            user_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UserIdentifier {
                    protocol_address: protocol_address__.unwrap_or_default(),
                    user_address: user_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.ferum.v1.UserIdentifier", FIELDS, GeneratedVisitor)
    }
}
