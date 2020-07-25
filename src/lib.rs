pub mod conversion;
pub mod theory_primitive;
pub mod theory_primitives_generated;

use wasm_bindgen::prelude::*;
use flatbuffers::FlatBufferBuilder;
use crate::theory_primitives_generated::theory_primitives::{ScaleArgs, Scale, Interval};
use crate::conversion::scale::get;
use js_sys::Uint8Array;

#[wasm_bindgen]
pub fn js_get_scale() -> js_sys::Uint8Array {

    let major_scale = get("major");

    let mut flat_buffer_builder = FlatBufferBuilder::new();

    let alias = flat_buffer_builder.create_string("foo");

    let args = ScaleArgs{
        name: Some(flat_buffer_builder.create_string(major_scale.name.as_str())),
        aliases: Some(flat_buffer_builder.create_vector(&[alias])),
        intervals: Some(flat_buffer_builder.create_vector(&[Interval::Major7])),
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let scale_offset = Scale::create(&mut flat_buffer_builder, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `scale_offset` created by `User::create`.
    flat_buffer_builder.finish( scale_offset, None);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = flat_buffer_builder.finished_data();

    let output = js_sys::Uint8Array::from(finished_data);

    output
}
