// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct HandshakeFromHost {
    // message fields
    pub urand: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HandshakeFromHost {}

impl HandshakeFromHost {
    pub fn new() -> HandshakeFromHost {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HandshakeFromHost {
        static mut instance: ::protobuf::lazy::Lazy<HandshakeFromHost> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HandshakeFromHost,
        };
        unsafe {
            instance.get(HandshakeFromHost::new)
        }
    }

    // bytes urand = 1;

    pub fn clear_urand(&mut self) {
        self.urand.clear();
    }

    // Param is passed by value, moved
    pub fn set_urand(&mut self, v: ::std::vec::Vec<u8>) {
        self.urand = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_urand(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.urand
    }

    // Take field
    pub fn take_urand(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.urand, ::std::vec::Vec::new())
    }

    pub fn get_urand(&self) -> &[u8] {
        &self.urand
    }

    fn get_urand_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.urand
    }

    fn mut_urand_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.urand
    }
}

impl ::protobuf::Message for HandshakeFromHost {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.urand)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.urand.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.urand);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.urand.is_empty() {
            os.write_bytes(1, &self.urand)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HandshakeFromHost {
    fn new() -> HandshakeFromHost {
        HandshakeFromHost::new()
    }

    fn descriptor_static(_: ::std::option::Option<HandshakeFromHost>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "urand",
                    HandshakeFromHost::get_urand_for_reflect,
                    HandshakeFromHost::mut_urand_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HandshakeFromHost>(
                    "HandshakeFromHost",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HandshakeFromHost {
    fn clear(&mut self) {
        self.clear_urand();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HandshakeFromHost {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HandshakeFromHost {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HandshakeFromClient {
    // message fields
    pub signature: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HandshakeFromClient {}

impl HandshakeFromClient {
    pub fn new() -> HandshakeFromClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HandshakeFromClient {
        static mut instance: ::protobuf::lazy::Lazy<HandshakeFromClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HandshakeFromClient,
        };
        unsafe {
            instance.get(HandshakeFromClient::new)
        }
    }

    // bytes signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }
}

impl ::protobuf::Message for HandshakeFromClient {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.signature);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.signature.is_empty() {
            os.write_bytes(1, &self.signature)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HandshakeFromClient {
    fn new() -> HandshakeFromClient {
        HandshakeFromClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<HandshakeFromClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    HandshakeFromClient::get_signature_for_reflect,
                    HandshakeFromClient::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HandshakeFromClient>(
                    "HandshakeFromClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HandshakeFromClient {
    fn clear(&mut self) {
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HandshakeFromClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HandshakeFromClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Command {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Command {}

impl Command {
    pub fn new() -> Command {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Command {
        static mut instance: ::protobuf::lazy::Lazy<Command> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Command,
        };
        unsafe {
            instance.get(Command::new)
        }
    }
}

impl ::protobuf::Message for Command {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Command {
    fn new() -> Command {
        Command::new()
    }

    fn descriptor_static(_: ::std::option::Option<Command>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Command>(
                    "Command",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Command {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Command {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Command {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Sequence {
    // message fields
    pub name: ::std::string::String,
    tasks: ::protobuf::RepeatedField<Task>,
    triggers: ::protobuf::RepeatedField<Trigger>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Sequence {}

impl Sequence {
    pub fn new() -> Sequence {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Sequence {
        static mut instance: ::protobuf::lazy::Lazy<Sequence> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Sequence,
        };
        unsafe {
            instance.get(Sequence::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .Task tasks = 2;

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_tasks(&mut self, v: ::protobuf::RepeatedField<Task>) {
        self.tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tasks(&mut self) -> &mut ::protobuf::RepeatedField<Task> {
        &mut self.tasks
    }

    // Take field
    pub fn take_tasks(&mut self) -> ::protobuf::RepeatedField<Task> {
        ::std::mem::replace(&mut self.tasks, ::protobuf::RepeatedField::new())
    }

    pub fn get_tasks(&self) -> &[Task] {
        &self.tasks
    }

    fn get_tasks_for_reflect(&self) -> &::protobuf::RepeatedField<Task> {
        &self.tasks
    }

    fn mut_tasks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Task> {
        &mut self.tasks
    }

    // repeated .Trigger triggers = 3;

    pub fn clear_triggers(&mut self) {
        self.triggers.clear();
    }

    // Param is passed by value, moved
    pub fn set_triggers(&mut self, v: ::protobuf::RepeatedField<Trigger>) {
        self.triggers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_triggers(&mut self) -> &mut ::protobuf::RepeatedField<Trigger> {
        &mut self.triggers
    }

    // Take field
    pub fn take_triggers(&mut self) -> ::protobuf::RepeatedField<Trigger> {
        ::std::mem::replace(&mut self.triggers, ::protobuf::RepeatedField::new())
    }

    pub fn get_triggers(&self) -> &[Trigger] {
        &self.triggers
    }

    fn get_triggers_for_reflect(&self) -> &::protobuf::RepeatedField<Trigger> {
        &self.triggers
    }

    fn mut_triggers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Trigger> {
        &mut self.triggers
    }
}

impl ::protobuf::Message for Sequence {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tasks)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.triggers)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        };
        for value in &self.tasks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.triggers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        };
        for v in &self.tasks {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.triggers {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Sequence {
    fn new() -> Sequence {
        Sequence::new()
    }

    fn descriptor_static(_: ::std::option::Option<Sequence>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Sequence::get_name_for_reflect,
                    Sequence::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Task>>(
                    "tasks",
                    Sequence::get_tasks_for_reflect,
                    Sequence::mut_tasks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Trigger>>(
                    "triggers",
                    Sequence::get_triggers_for_reflect,
                    Sequence::mut_triggers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Sequence>(
                    "Sequence",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Sequence {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_tasks();
        self.clear_triggers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Sequence {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Sequence {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Trigger {
    // message fields
    pub id: u64,
    pub delay: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Trigger {}

impl Trigger {
    pub fn new() -> Trigger {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Trigger {
        static mut instance: ::protobuf::lazy::Lazy<Trigger> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Trigger,
        };
        unsafe {
            instance.get(Trigger::new)
        }
    }

    // uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.id
    }

    // uint64 delay = 2;

    pub fn clear_delay(&mut self) {
        self.delay = 0;
    }

    // Param is passed by value, moved
    pub fn set_delay(&mut self, v: u64) {
        self.delay = v;
    }

    pub fn get_delay(&self) -> u64 {
        self.delay
    }

    fn get_delay_for_reflect(&self) -> &u64 {
        &self.delay
    }

    fn mut_delay_for_reflect(&mut self) -> &mut u64 {
        &mut self.delay
    }
}

impl ::protobuf::Message for Trigger {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.delay = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.delay != 0 {
            my_size += ::protobuf::rt::value_size(2, self.delay, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        };
        if self.delay != 0 {
            os.write_uint64(2, self.delay)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Trigger {
    fn new() -> Trigger {
        Trigger::new()
    }

    fn descriptor_static(_: ::std::option::Option<Trigger>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Trigger::get_id_for_reflect,
                    Trigger::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "delay",
                    Trigger::get_delay_for_reflect,
                    Trigger::mut_delay_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Trigger>(
                    "Trigger",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Trigger {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_delay();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Trigger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Trigger {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Task {
    // message fields
    pub id: u32,
    pub trigger: u64,
    // message oneof groups
    task: ::std::option::Option<Task_oneof_task>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Task {}

#[derive(Clone,PartialEq)]
pub enum Task_oneof_task {
    exec(TaskExec),
    match_window(TaskMatchWindow),
    place_content(TaskPlaceContent),
    show_text(TaskShowText),
}

impl Task {
    pub fn new() -> Task {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Task {
        static mut instance: ::protobuf::lazy::Lazy<Task> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Task,
        };
        unsafe {
            instance.get(Task::new)
        }
    }

    // uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // uint64 trigger = 2;

    pub fn clear_trigger(&mut self) {
        self.trigger = 0;
    }

    // Param is passed by value, moved
    pub fn set_trigger(&mut self, v: u64) {
        self.trigger = v;
    }

    pub fn get_trigger(&self) -> u64 {
        self.trigger
    }

    fn get_trigger_for_reflect(&self) -> &u64 {
        &self.trigger
    }

    fn mut_trigger_for_reflect(&mut self) -> &mut u64 {
        &mut self.trigger
    }

    // .TaskExec exec = 10;

    pub fn clear_exec(&mut self) {
        self.task = ::std::option::Option::None;
    }

    pub fn has_exec(&self) -> bool {
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::exec(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_exec(&mut self, v: TaskExec) {
        self.task = ::std::option::Option::Some(Task_oneof_task::exec(v))
    }

    // Mutable pointer to the field.
    pub fn mut_exec(&mut self) -> &mut TaskExec {
        if let ::std::option::Option::Some(Task_oneof_task::exec(_)) = self.task {
        } else {
            self.task = ::std::option::Option::Some(Task_oneof_task::exec(TaskExec::new()));
        }
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::exec(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_exec(&mut self) -> TaskExec {
        if self.has_exec() {
            match self.task.take() {
                ::std::option::Option::Some(Task_oneof_task::exec(v)) => v,
                _ => panic!(),
            }
        } else {
            TaskExec::new()
        }
    }

    pub fn get_exec(&self) -> &TaskExec {
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::exec(ref v)) => v,
            _ => TaskExec::default_instance(),
        }
    }

    // .TaskMatchWindow match_window = 11;

    pub fn clear_match_window(&mut self) {
        self.task = ::std::option::Option::None;
    }

    pub fn has_match_window(&self) -> bool {
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::match_window(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_match_window(&mut self, v: TaskMatchWindow) {
        self.task = ::std::option::Option::Some(Task_oneof_task::match_window(v))
    }

    // Mutable pointer to the field.
    pub fn mut_match_window(&mut self) -> &mut TaskMatchWindow {
        if let ::std::option::Option::Some(Task_oneof_task::match_window(_)) = self.task {
        } else {
            self.task = ::std::option::Option::Some(Task_oneof_task::match_window(TaskMatchWindow::new()));
        }
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::match_window(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_match_window(&mut self) -> TaskMatchWindow {
        if self.has_match_window() {
            match self.task.take() {
                ::std::option::Option::Some(Task_oneof_task::match_window(v)) => v,
                _ => panic!(),
            }
        } else {
            TaskMatchWindow::new()
        }
    }

    pub fn get_match_window(&self) -> &TaskMatchWindow {
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::match_window(ref v)) => v,
            _ => TaskMatchWindow::default_instance(),
        }
    }

    // .TaskPlaceContent place_content = 12;

    pub fn clear_place_content(&mut self) {
        self.task = ::std::option::Option::None;
    }

    pub fn has_place_content(&self) -> bool {
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::place_content(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_place_content(&mut self, v: TaskPlaceContent) {
        self.task = ::std::option::Option::Some(Task_oneof_task::place_content(v))
    }

    // Mutable pointer to the field.
    pub fn mut_place_content(&mut self) -> &mut TaskPlaceContent {
        if let ::std::option::Option::Some(Task_oneof_task::place_content(_)) = self.task {
        } else {
            self.task = ::std::option::Option::Some(Task_oneof_task::place_content(TaskPlaceContent::new()));
        }
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::place_content(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_place_content(&mut self) -> TaskPlaceContent {
        if self.has_place_content() {
            match self.task.take() {
                ::std::option::Option::Some(Task_oneof_task::place_content(v)) => v,
                _ => panic!(),
            }
        } else {
            TaskPlaceContent::new()
        }
    }

    pub fn get_place_content(&self) -> &TaskPlaceContent {
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::place_content(ref v)) => v,
            _ => TaskPlaceContent::default_instance(),
        }
    }

    // .TaskShowText show_text = 13;

    pub fn clear_show_text(&mut self) {
        self.task = ::std::option::Option::None;
    }

    pub fn has_show_text(&self) -> bool {
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::show_text(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_show_text(&mut self, v: TaskShowText) {
        self.task = ::std::option::Option::Some(Task_oneof_task::show_text(v))
    }

    // Mutable pointer to the field.
    pub fn mut_show_text(&mut self) -> &mut TaskShowText {
        if let ::std::option::Option::Some(Task_oneof_task::show_text(_)) = self.task {
        } else {
            self.task = ::std::option::Option::Some(Task_oneof_task::show_text(TaskShowText::new()));
        }
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::show_text(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_show_text(&mut self) -> TaskShowText {
        if self.has_show_text() {
            match self.task.take() {
                ::std::option::Option::Some(Task_oneof_task::show_text(v)) => v,
                _ => panic!(),
            }
        } else {
            TaskShowText::new()
        }
    }

    pub fn get_show_text(&self) -> &TaskShowText {
        match self.task {
            ::std::option::Option::Some(Task_oneof_task::show_text(ref v)) => v,
            _ => TaskShowText::default_instance(),
        }
    }
}

impl ::protobuf::Message for Task {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.trigger = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.task = ::std::option::Option::Some(Task_oneof_task::exec(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.task = ::std::option::Option::Some(Task_oneof_task::match_window(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.task = ::std::option::Option::Some(Task_oneof_task::place_content(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.task = ::std::option::Option::Some(Task_oneof_task::show_text(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.trigger != 0 {
            my_size += ::protobuf::rt::value_size(2, self.trigger, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.task {
            match v {
                &Task_oneof_task::exec(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Task_oneof_task::match_window(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Task_oneof_task::place_content(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Task_oneof_task::show_text(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        };
        if self.trigger != 0 {
            os.write_uint64(2, self.trigger)?;
        };
        if let ::std::option::Option::Some(ref v) = self.task {
            match v {
                &Task_oneof_task::exec(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Task_oneof_task::match_window(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Task_oneof_task::place_content(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Task_oneof_task::show_text(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Task {
    fn new() -> Task {
        Task::new()
    }

    fn descriptor_static(_: ::std::option::Option<Task>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    Task::get_id_for_reflect,
                    Task::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "trigger",
                    Task::get_trigger_for_reflect,
                    Task::mut_trigger_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TaskExec>(
                    "exec",
                    Task::has_exec,
                    Task::get_exec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TaskMatchWindow>(
                    "match_window",
                    Task::has_match_window,
                    Task::get_match_window,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TaskPlaceContent>(
                    "place_content",
                    Task::has_place_content,
                    Task::get_place_content,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TaskShowText>(
                    "show_text",
                    Task::has_show_text,
                    Task::get_show_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Task>(
                    "Task",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Task {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_trigger();
        self.clear_exec();
        self.clear_match_window();
        self.clear_place_content();
        self.clear_show_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Task {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Task {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TaskExec {
    // message fields
    pub program: ::std::string::String,
    args: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TaskExec {}

impl TaskExec {
    pub fn new() -> TaskExec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TaskExec {
        static mut instance: ::protobuf::lazy::Lazy<TaskExec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TaskExec,
        };
        unsafe {
            instance.get(TaskExec::new)
        }
    }

    // string program = 1;

    pub fn clear_program(&mut self) {
        self.program.clear();
    }

    // Param is passed by value, moved
    pub fn set_program(&mut self, v: ::std::string::String) {
        self.program = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_program(&mut self) -> &mut ::std::string::String {
        &mut self.program
    }

    // Take field
    pub fn take_program(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.program, ::std::string::String::new())
    }

    pub fn get_program(&self) -> &str {
        &self.program
    }

    fn get_program_for_reflect(&self) -> &::std::string::String {
        &self.program
    }

    fn mut_program_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.program
    }

    // repeated string args = 2;

    pub fn clear_args(&mut self) {
        self.args.clear();
    }

    // Param is passed by value, moved
    pub fn set_args(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_args(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.args
    }

    // Take field
    pub fn take_args(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.args, ::protobuf::RepeatedField::new())
    }

    pub fn get_args(&self) -> &[::std::string::String] {
        &self.args
    }

    fn get_args_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.args
    }

    fn mut_args_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.args
    }
}

impl ::protobuf::Message for TaskExec {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.program)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.args)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.program.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.program);
        };
        for value in &self.args {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.program.is_empty() {
            os.write_string(1, &self.program)?;
        };
        for v in &self.args {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TaskExec {
    fn new() -> TaskExec {
        TaskExec::new()
    }

    fn descriptor_static(_: ::std::option::Option<TaskExec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "program",
                    TaskExec::get_program_for_reflect,
                    TaskExec::mut_program_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "args",
                    TaskExec::get_args_for_reflect,
                    TaskExec::mut_args_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TaskExec>(
                    "TaskExec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TaskExec {
    fn clear(&mut self) {
        self.clear_program();
        self.clear_args();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TaskExec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TaskExec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TaskShowText {
    // message fields
    pub content_id: u32,
    pub text: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TaskShowText {}

impl TaskShowText {
    pub fn new() -> TaskShowText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TaskShowText {
        static mut instance: ::protobuf::lazy::Lazy<TaskShowText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TaskShowText,
        };
        unsafe {
            instance.get(TaskShowText::new)
        }
    }

    // uint32 content_id = 1;

    pub fn clear_content_id(&mut self) {
        self.content_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_content_id(&mut self, v: u32) {
        self.content_id = v;
    }

    pub fn get_content_id(&self) -> u32 {
        self.content_id
    }

    fn get_content_id_for_reflect(&self) -> &u32 {
        &self.content_id
    }

    fn mut_content_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.content_id
    }

    // string text = 2;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        &mut self.text
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.text, ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    fn get_text_for_reflect(&self) -> &::std::string::String {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.text
    }
}

impl ::protobuf::Message for TaskShowText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.content_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.text)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.content_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.content_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.text.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.text);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.content_id != 0 {
            os.write_uint32(1, self.content_id)?;
        };
        if !self.text.is_empty() {
            os.write_string(2, &self.text)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TaskShowText {
    fn new() -> TaskShowText {
        TaskShowText::new()
    }

    fn descriptor_static(_: ::std::option::Option<TaskShowText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "content_id",
                    TaskShowText::get_content_id_for_reflect,
                    TaskShowText::mut_content_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    TaskShowText::get_text_for_reflect,
                    TaskShowText::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TaskShowText>(
                    "TaskShowText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TaskShowText {
    fn clear(&mut self) {
        self.clear_content_id();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TaskShowText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TaskShowText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TaskMatchWindow {
    // message fields
    pub content_id: u32,
    pub match_history: bool,
    pub title: ::std::string::String,
    pub class: ::std::string::String,
    pub role: ::std::string::String,
    pub pid: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TaskMatchWindow {}

impl TaskMatchWindow {
    pub fn new() -> TaskMatchWindow {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TaskMatchWindow {
        static mut instance: ::protobuf::lazy::Lazy<TaskMatchWindow> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TaskMatchWindow,
        };
        unsafe {
            instance.get(TaskMatchWindow::new)
        }
    }

    // uint32 content_id = 1;

    pub fn clear_content_id(&mut self) {
        self.content_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_content_id(&mut self, v: u32) {
        self.content_id = v;
    }

    pub fn get_content_id(&self) -> u32 {
        self.content_id
    }

    fn get_content_id_for_reflect(&self) -> &u32 {
        &self.content_id
    }

    fn mut_content_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.content_id
    }

    // bool match_history = 2;

    pub fn clear_match_history(&mut self) {
        self.match_history = false;
    }

    // Param is passed by value, moved
    pub fn set_match_history(&mut self, v: bool) {
        self.match_history = v;
    }

    pub fn get_match_history(&self) -> bool {
        self.match_history
    }

    fn get_match_history_for_reflect(&self) -> &bool {
        &self.match_history
    }

    fn mut_match_history_for_reflect(&mut self) -> &mut bool {
        &mut self.match_history
    }

    // string title = 3;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string class = 4;

    pub fn clear_class(&mut self) {
        self.class.clear();
    }

    // Param is passed by value, moved
    pub fn set_class(&mut self, v: ::std::string::String) {
        self.class = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_class(&mut self) -> &mut ::std::string::String {
        &mut self.class
    }

    // Take field
    pub fn take_class(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.class, ::std::string::String::new())
    }

    pub fn get_class(&self) -> &str {
        &self.class
    }

    fn get_class_for_reflect(&self) -> &::std::string::String {
        &self.class
    }

    fn mut_class_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.class
    }

    // string role = 5;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // Take field
    pub fn take_role(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.role, ::std::string::String::new())
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    fn get_role_for_reflect(&self) -> &::std::string::String {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // uint32 pid = 6;

    pub fn clear_pid(&mut self) {
        self.pid = 0;
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: u32) {
        self.pid = v;
    }

    pub fn get_pid(&self) -> u32 {
        self.pid
    }

    fn get_pid_for_reflect(&self) -> &u32 {
        &self.pid
    }

    fn mut_pid_for_reflect(&mut self) -> &mut u32 {
        &mut self.pid
    }
}

impl ::protobuf::Message for TaskMatchWindow {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.content_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.match_history = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.class)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.role)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.pid = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.content_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.content_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.match_history != false {
            my_size += 2;
        };
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.title);
        };
        if !self.class.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.class);
        };
        if !self.role.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.role);
        };
        if self.pid != 0 {
            my_size += ::protobuf::rt::value_size(6, self.pid, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.content_id != 0 {
            os.write_uint32(1, self.content_id)?;
        };
        if self.match_history != false {
            os.write_bool(2, self.match_history)?;
        };
        if !self.title.is_empty() {
            os.write_string(3, &self.title)?;
        };
        if !self.class.is_empty() {
            os.write_string(4, &self.class)?;
        };
        if !self.role.is_empty() {
            os.write_string(5, &self.role)?;
        };
        if self.pid != 0 {
            os.write_uint32(6, self.pid)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TaskMatchWindow {
    fn new() -> TaskMatchWindow {
        TaskMatchWindow::new()
    }

    fn descriptor_static(_: ::std::option::Option<TaskMatchWindow>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "content_id",
                    TaskMatchWindow::get_content_id_for_reflect,
                    TaskMatchWindow::mut_content_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "match_history",
                    TaskMatchWindow::get_match_history_for_reflect,
                    TaskMatchWindow::mut_match_history_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    TaskMatchWindow::get_title_for_reflect,
                    TaskMatchWindow::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "class",
                    TaskMatchWindow::get_class_for_reflect,
                    TaskMatchWindow::mut_class_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "role",
                    TaskMatchWindow::get_role_for_reflect,
                    TaskMatchWindow::mut_role_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pid",
                    TaskMatchWindow::get_pid_for_reflect,
                    TaskMatchWindow::mut_pid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TaskMatchWindow>(
                    "TaskMatchWindow",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TaskMatchWindow {
    fn clear(&mut self) {
        self.clear_content_id();
        self.clear_match_history();
        self.clear_title();
        self.clear_class();
        self.clear_role();
        self.clear_pid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TaskMatchWindow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TaskMatchWindow {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TaskPlaceContent {
    // message fields
    pub content_id: u32,
    // message oneof groups
    location: ::std::option::Option<TaskPlaceContent_oneof_location>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TaskPlaceContent {}

#[derive(Clone,PartialEq)]
pub enum TaskPlaceContent_oneof_location {
    split(LocationSplit),
}

impl TaskPlaceContent {
    pub fn new() -> TaskPlaceContent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TaskPlaceContent {
        static mut instance: ::protobuf::lazy::Lazy<TaskPlaceContent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TaskPlaceContent,
        };
        unsafe {
            instance.get(TaskPlaceContent::new)
        }
    }

    // uint32 content_id = 1;

    pub fn clear_content_id(&mut self) {
        self.content_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_content_id(&mut self, v: u32) {
        self.content_id = v;
    }

    pub fn get_content_id(&self) -> u32 {
        self.content_id
    }

    fn get_content_id_for_reflect(&self) -> &u32 {
        &self.content_id
    }

    fn mut_content_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.content_id
    }

    // .LocationSplit split = 2;

    pub fn clear_split(&mut self) {
        self.location = ::std::option::Option::None;
    }

    pub fn has_split(&self) -> bool {
        match self.location {
            ::std::option::Option::Some(TaskPlaceContent_oneof_location::split(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_split(&mut self, v: LocationSplit) {
        self.location = ::std::option::Option::Some(TaskPlaceContent_oneof_location::split(v))
    }

    // Mutable pointer to the field.
    pub fn mut_split(&mut self) -> &mut LocationSplit {
        if let ::std::option::Option::Some(TaskPlaceContent_oneof_location::split(_)) = self.location {
        } else {
            self.location = ::std::option::Option::Some(TaskPlaceContent_oneof_location::split(LocationSplit::new()));
        }
        match self.location {
            ::std::option::Option::Some(TaskPlaceContent_oneof_location::split(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_split(&mut self) -> LocationSplit {
        if self.has_split() {
            match self.location.take() {
                ::std::option::Option::Some(TaskPlaceContent_oneof_location::split(v)) => v,
                _ => panic!(),
            }
        } else {
            LocationSplit::new()
        }
    }

    pub fn get_split(&self) -> &LocationSplit {
        match self.location {
            ::std::option::Option::Some(TaskPlaceContent_oneof_location::split(ref v)) => v,
            _ => LocationSplit::default_instance(),
        }
    }
}

impl ::protobuf::Message for TaskPlaceContent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.content_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.location = ::std::option::Option::Some(TaskPlaceContent_oneof_location::split(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.content_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.content_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.location {
            match v {
                &TaskPlaceContent_oneof_location::split(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.content_id != 0 {
            os.write_uint32(1, self.content_id)?;
        };
        if let ::std::option::Option::Some(ref v) = self.location {
            match v {
                &TaskPlaceContent_oneof_location::split(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TaskPlaceContent {
    fn new() -> TaskPlaceContent {
        TaskPlaceContent::new()
    }

    fn descriptor_static(_: ::std::option::Option<TaskPlaceContent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "content_id",
                    TaskPlaceContent::get_content_id_for_reflect,
                    TaskPlaceContent::mut_content_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, LocationSplit>(
                    "split",
                    TaskPlaceContent::has_split,
                    TaskPlaceContent::get_split,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TaskPlaceContent>(
                    "TaskPlaceContent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TaskPlaceContent {
    fn clear(&mut self) {
        self.clear_content_id();
        self.clear_split();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TaskPlaceContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TaskPlaceContent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LocationSplit {
    // message fields
    pub side: Side,
    pub ratio: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocationSplit {}

impl LocationSplit {
    pub fn new() -> LocationSplit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocationSplit {
        static mut instance: ::protobuf::lazy::Lazy<LocationSplit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocationSplit,
        };
        unsafe {
            instance.get(LocationSplit::new)
        }
    }

    // .Side side = 1;

    pub fn clear_side(&mut self) {
        self.side = Side::TOP;
    }

    // Param is passed by value, moved
    pub fn set_side(&mut self, v: Side) {
        self.side = v;
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    fn get_side_for_reflect(&self) -> &Side {
        &self.side
    }

    fn mut_side_for_reflect(&mut self) -> &mut Side {
        &mut self.side
    }

    // float ratio = 2;

    pub fn clear_ratio(&mut self) {
        self.ratio = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ratio(&mut self, v: f32) {
        self.ratio = v;
    }

    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }

    fn get_ratio_for_reflect(&self) -> &f32 {
        &self.ratio
    }

    fn mut_ratio_for_reflect(&mut self) -> &mut f32 {
        &mut self.ratio
    }
}

impl ::protobuf::Message for LocationSplit {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.side = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.ratio = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.side != Side::TOP {
            my_size += ::protobuf::rt::enum_size(1, self.side);
        };
        if self.ratio != 0. {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.side != Side::TOP {
            os.write_enum(1, self.side.value())?;
        };
        if self.ratio != 0. {
            os.write_float(2, self.ratio)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LocationSplit {
    fn new() -> LocationSplit {
        LocationSplit::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocationSplit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Side>>(
                    "side",
                    LocationSplit::get_side_for_reflect,
                    LocationSplit::mut_side_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "ratio",
                    LocationSplit::get_ratio_for_reflect,
                    LocationSplit::mut_ratio_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocationSplit>(
                    "LocationSplit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocationSplit {
    fn clear(&mut self) {
        self.clear_side();
        self.clear_ratio();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocationSplit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocationSplit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Side {
    TOP = 0,
    BOTTOM = 1,
    LEFT = 2,
    RIGHT = 3,
}

impl ::protobuf::ProtobufEnum for Side {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Side> {
        match value {
            0 => ::std::option::Option::Some(Side::TOP),
            1 => ::std::option::Option::Some(Side::BOTTOM),
            2 => ::std::option::Option::Some(Side::LEFT),
            3 => ::std::option::Option::Some(Side::RIGHT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Side] = &[
            Side::TOP,
            Side::BOTTOM,
            Side::LEFT,
            Side::RIGHT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Side>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Side", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Side {
}

impl ::std::default::Default for Side {
    fn default() -> Self {
        Side::TOP
    }
}

impl ::protobuf::reflect::ProtobufValue for Side {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x29, 0x0a, 0x11, 0x48, 0x61, 0x6e, 0x64, 0x73, 0x68, 0x61, 0x6b, 0x65, 0x46, 0x72, 0x6f,
    0x6d, 0x48, 0x6f, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x75, 0x72, 0x61, 0x6e, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x75, 0x72, 0x61, 0x6e, 0x64, 0x22, 0x33, 0x0a, 0x13, 0x48,
    0x61, 0x6e, 0x64, 0x73, 0x68, 0x61, 0x6b, 0x65, 0x46, 0x72, 0x6f, 0x6d, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65,
    0x22, 0x09, 0x0a, 0x07, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x22, 0x0a, 0x0a, 0x08, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x61, 0x0a, 0x08, 0x53, 0x65, 0x71, 0x75, 0x65,
    0x6e, 0x63, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x05, 0x74, 0x61, 0x73, 0x6b, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x52, 0x05, 0x74,
    0x61, 0x73, 0x6b, 0x73, 0x12, 0x24, 0x0a, 0x08, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x73,
    0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72,
    0x52, 0x08, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x73, 0x22, 0x2f, 0x0a, 0x07, 0x54, 0x72,
    0x69, 0x67, 0x67, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x22, 0xf8, 0x01, 0x0a, 0x04,
    0x54, 0x61, 0x73, 0x6b, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x02, 0x69, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x12, 0x1f,
    0x0a, 0x04, 0x65, 0x78, 0x65, 0x63, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x54,
    0x61, 0x73, 0x6b, 0x45, 0x78, 0x65, 0x63, 0x48, 0x00, 0x52, 0x04, 0x65, 0x78, 0x65, 0x63, 0x12,
    0x35, 0x0a, 0x0c, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x4d, 0x61, 0x74, 0x63,
    0x68, 0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x48, 0x00, 0x52, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x12, 0x38, 0x0a, 0x0d, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x5f,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x54, 0x61, 0x73, 0x6b, 0x50, 0x6c, 0x61, 0x63, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x48, 0x00, 0x52, 0x0c, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x12, 0x2c, 0x0a, 0x09, 0x73, 0x68, 0x6f, 0x77, 0x5f, 0x74, 0x65, 0x78, 0x74, 0x18, 0x0d, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x53, 0x68, 0x6f, 0x77, 0x54, 0x65,
    0x78, 0x74, 0x48, 0x00, 0x52, 0x08, 0x73, 0x68, 0x6f, 0x77, 0x54, 0x65, 0x78, 0x74, 0x42, 0x06,
    0x0a, 0x04, 0x74, 0x61, 0x73, 0x6b, 0x22, 0x38, 0x0a, 0x08, 0x54, 0x61, 0x73, 0x6b, 0x45, 0x78,
    0x65, 0x63, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x12, 0x12, 0x0a, 0x04,
    0x61, 0x72, 0x67, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x04, 0x61, 0x72, 0x67, 0x73,
    0x22, 0x41, 0x0a, 0x0c, 0x54, 0x61, 0x73, 0x6b, 0x53, 0x68, 0x6f, 0x77, 0x54, 0x65, 0x78, 0x74,
    0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12,
    0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74,
    0x65, 0x78, 0x74, 0x22, 0xa7, 0x01, 0x0a, 0x0f, 0x54, 0x61, 0x73, 0x6b, 0x4d, 0x61, 0x74, 0x63,
    0x68, 0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f,
    0x68, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x74,
    0x69, 0x74, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x69, 0x74, 0x6c,
    0x65, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x05, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x70,
    0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x03, 0x70, 0x69, 0x64, 0x22, 0x65, 0x0a,
    0x10, 0x54, 0x61, 0x73, 0x6b, 0x50, 0x6c, 0x61, 0x63, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x49, 0x64,
    0x12, 0x26, 0x0a, 0x05, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0e, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x48,
    0x00, 0x52, 0x05, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x42, 0x0a, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x22, 0x40, 0x0a, 0x0d, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x53, 0x70, 0x6c, 0x69, 0x74, 0x12, 0x19, 0x0a, 0x04, 0x73, 0x69, 0x64, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x05, 0x2e, 0x53, 0x69, 0x64, 0x65, 0x52, 0x04, 0x73, 0x69, 0x64, 0x65,
    0x12, 0x14, 0x0a, 0x05, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x05, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x2a, 0x30, 0x0a, 0x04, 0x53, 0x69, 0x64, 0x65, 0x12, 0x07,
    0x0a, 0x03, 0x54, 0x4f, 0x50, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x42, 0x4f, 0x54, 0x54, 0x4f,
    0x4d, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x4c, 0x45, 0x46, 0x54, 0x10, 0x02, 0x12, 0x09, 0x0a,
    0x05, 0x52, 0x49, 0x47, 0x48, 0x54, 0x10, 0x03, 0x4a, 0x90, 0x1a, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x62, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x02, 0x08, 0x19, 0x0a, 0x64, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04,
    0x08, 0x18, 0x1a, 0x57, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74,
    0x75, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x48, 0x61, 0x6e, 0x64, 0x73, 0x68, 0x61, 0x6b, 0x65,
    0x46, 0x72, 0x6f, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x04, 0x08, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x04, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x04, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07, 0x08, 0x1b, 0x0a, 0x1f, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x08, 0x1c, 0x1a, 0x12, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x09, 0x08, 0x07, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x0e, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0c, 0x00,
    0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0f, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x10, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x10, 0x08, 0x10, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x14, 0x00,
    0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x38,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x16, 0x08, 0x18, 0x1a, 0x2b, 0x20, 0x61, 0x20,
    0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x20, 0x69,
    0x74, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x16, 0x08, 0x14, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x16, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x16, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x16,
    0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x17, 0x08, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x17, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x17, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03,
    0x18, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x18, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x18, 0x11, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x19, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x18, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x05, 0x12, 0x04, 0x1b, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x0f, 0x0a, 0x4c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x08, 0x16,
    0x1a, 0x3f, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2d, 0x61, 0x73, 0x73, 0x69,
    0x67, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x69, 0x64, 0x20,
    0x28, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x29,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1d, 0x08, 0x1b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x0f, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x14, 0x15, 0x0a, 0x8e, 0x01, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x20, 0x08, 0x19, 0x1a, 0x80, 0x01, 0x20, 0x61, 0x20, 0x64, 0x65,
    0x6c, 0x61, 0x79, 0x20, 0x28, 0x69, 0x6e, 0x20, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x65, 0x63,
    0x6f, 0x6e, 0x64, 0x73, 0x29, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x66, 0x69,
    0x72, 0x65, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x73, 0x73,
    0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x61,
    0x20, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x7a, 0x65, 0x72, 0x6f, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x72, 0x75, 0x6c, 0x79, 0x20, 0x69, 0x6e, 0x73,
    0x74, 0x61, 0x6e, 0x74, 0x61, 0x6e, 0x65, 0x6f, 0x75, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0x20, 0x08, 0x1d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x20, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x20, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x23, 0x00, 0x2f, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x23, 0x08, 0x0c, 0x0a, 0x49, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x25, 0x08, 0x16, 0x1a, 0x3c, 0x20, 0x61, 0x20, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x2d, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x61,
    0x73, 0x6b, 0x20, 0x69, 0x64, 0x20, 0x28, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x65, 0x71,
    0x75, 0x65, 0x6e, 0x63, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x25, 0x08, 0x23, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x25, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25,
    0x0f, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x14, 0x15,
    0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x27, 0x08, 0x1b, 0x1a, 0x2f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72,
    0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x73, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x27, 0x08, 0x25, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x27, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x08, 0x00, 0x12,
    0x04, 0x29, 0x08, 0x2e, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x08, 0x00, 0x01, 0x12, 0x03,
    0x29, 0x0e, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x2a, 0x10, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2a, 0x10, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x19, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x03, 0x12, 0x03, 0x2b, 0x10, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x2b, 0x10, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x2b, 0x20, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2b, 0x2f,
    0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x2c, 0x10, 0x34, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x06, 0x12, 0x03, 0x2c, 0x10, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2c, 0x21, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x2c, 0x31, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05,
    0x12, 0x03, 0x2d, 0x10, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x06, 0x12, 0x03,
    0x2d, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2d, 0x1d,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2d, 0x29, 0x2b, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x31, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x07, 0x01, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12,
    0x03, 0x32, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x32,
    0x08, 0x31, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x0f, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x33, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x33, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x33, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x33, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33,
    0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x36, 0x00, 0x3b, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x36, 0x08, 0x14, 0x0a, 0x4c, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x00, 0x12, 0x03, 0x38, 0x08, 0x1e, 0x1a, 0x3f, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x2d, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x20, 0x69, 0x64, 0x20, 0x28, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x65,
    0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x38, 0x08, 0x36, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x38, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x38, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x1c,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x3a, 0x08, 0x38, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x3a, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x3d,
    0x00, 0x4d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x17, 0x0a,
    0x7c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x40, 0x08, 0x1e, 0x1a, 0x6f, 0x20, 0x61,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2d, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x64, 0x20, 0x28, 0x75, 0x6e, 0x69,
    0x71, 0x75, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x74, 0x20, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x29, 0x0a, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x64, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x40, 0x08, 0x3d, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x40, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x40, 0x1c, 0x1d, 0x0a, 0x8f, 0x01, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12,
    0x03, 0x44, 0x08, 0x1f, 0x1a, 0x81, 0x01, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x61,
    0x67, 0x61, 0x69, 0x6e, 0x73, 0x74, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x73, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x6f, 0x70, 0x65, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72,
    0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x77, 0x61, 0x73, 0x20,
    0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x65, 0x64, 0x3f, 0x0a, 0x20, 0x6f, 0x72, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x69, 0x74, 0x20, 0x77, 0x61, 0x69, 0x74, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x20, 0x74,
    0x6f, 0x20, 0x6f, 0x70, 0x65, 0x6e, 0x3f, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x44, 0x08, 0x40, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x44, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x44, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x1d,
    0x1e, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x47, 0x08, 0x19, 0x1a, 0x33,
    0x20, 0x72, 0x65, 0x67, 0x65, 0x78, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x61, 0x67, 0x61, 0x69, 0x6e, 0x73, 0x74,
    0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x69,
    0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0x47, 0x08,
    0x44, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x47, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x47, 0x0f, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x47, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x03, 0x12, 0x03, 0x48, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x48, 0x08, 0x47, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x48, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x48, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03, 0x48,
    0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04, 0x12, 0x03, 0x49, 0x08, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x04, 0x49, 0x08, 0x48, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x05, 0x12, 0x03, 0x49, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x03, 0x49, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x49, 0x16, 0x17, 0x0a, 0x84, 0x01, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x05, 0x12, 0x03, 0x4c, 0x08, 0x17, 0x1a, 0x48, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x77, 0x69,
    0x6e, 0x64, 0x6f, 0x77, 0x73, 0x20, 0x64, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x61, 0x20, 0x70,
    0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x20, 0x69, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72,
    0x74, 0x79, 0x20, 0x28, 0x62, 0x75, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x67, 0x75, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x64, 0x29, 0x0a,
    0x22, 0x2d, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x61, 0x6c, 0x6c, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x04, 0x12, 0x04, 0x4c, 0x08, 0x49, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4c, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x4c, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04,
    0x4f, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x18,
    0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x51, 0x08, 0x1e, 0x1a, 0x20, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x51, 0x08, 0x4f, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x51, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x51, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x08, 0x00,
    0x12, 0x04, 0x53, 0x08, 0x55, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x08, 0x00, 0x01, 0x12,
    0x03, 0x53, 0x0e, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x54, 0x10,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x03, 0x54, 0x10, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x1e, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x05,
    0x00, 0x12, 0x04, 0x58, 0x00, 0x5d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03,
    0x58, 0x05, 0x09, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x59, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x08, 0x0b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x59, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x5a, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5b,
    0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5b, 0x0f, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x5c, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x5c, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x5f,
    0x00, 0x62, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x60, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x60, 0x08, 0x5f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x60, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x60, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x60, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x61,
    0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x61, 0x08, 0x60,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x61, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x61, 0x0e, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x61, 0x16, 0x17, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
