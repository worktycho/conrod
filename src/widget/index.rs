
use graph::NodeIndex;
use rustc_serialize::{Encodable, Decodable, Encoder, Decoder};
use widget::WidgetId;


/// An index either given in the form of a publicly instantiated `Widget`'s `WidgetId`, or an
/// internally instantiated `Widget`'s `NodeIndex`,
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Index {
    Public(WidgetId),
    Internal(NodeIndex),
}


impl From<WidgetId> for Index {
    fn from(id: WidgetId) -> Index {
        Index::Public(id)
    }
}

impl From<NodeIndex> for Index {
    fn from(idx: NodeIndex) -> Index {
        Index::Internal(idx)
    }
}

impl Encodable for Index {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.emit_enum("Index", |encoder| {
            match *self {
                Index::Public(id) =>
                    encoder.emit_enum_variant("Public", 0, 2, |encoder| {
                        encoder.emit_enum_variant_arg(0, |encoder| encoder.emit_usize(id))
                    }),
                Index::Internal(idx) =>
                    encoder.emit_enum_variant("Internal", 1, 2, |encoder| {
                        encoder.emit_enum_variant_arg(0, |encoder| encoder.emit_usize(idx.index()))
                    }),
            }
        })
    }
}

impl Decodable for Index {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.read_enum("Index", |decoder| {
            decoder.read_enum_variant(&["Public", "Internal"], |decoder, i| {
                Ok(match i {
                    0 => Index::Public(try!(decoder.read_enum_variant_arg(0, |decoder| {
                        decoder.read_usize()
                    }))),
                    1 => Index::Internal(try!(decoder.read_enum_variant_arg(0, |decoder| {
                        Ok(NodeIndex::new(try!(decoder.read_usize())))
                    }))),
                    _ => unreachable!(),
                })
            })
        })
    }
}

