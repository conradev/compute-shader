// Copyright 2017 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::metal::buffer::BUFFER_FUNCTIONS;
use api::metal::ffi::{MTLDevice, MTLCompileOptions, MTLLanguageVersion, MTLLibrary, MTLFunction, MTLFunctionType, MTLResourceCPUCacheModeDefaultCache};
use api::metal::program::PROGRAM_FUNCTIONS;
use api::metal::queue::QUEUE_FUNCTIONS;
use buffer::{Buffer, BufferData, Protection};
use device::{Device, DeviceFunctions};
use error::Error;
use euclid::Size2D;
use image::{Format, Image};
use program::Program;
use queue::Queue;
use std::ptr;
use std::slice;
use std::str;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSString, NSFastEnumeration};

pub static DEVICE_FUNCTIONS: DeviceFunctions = DeviceFunctions {
    destroy: destroy,
    create_queue: create_queue,
    create_program: create_program,
    create_buffer: create_buffer,
    create_image: create_image,
};

unsafe fn destroy(this: &Device) {
    let _: () = msg_send![this.data() as id, release];
}

fn create_queue(this: &Device) -> Result<Queue, Error> {
    unsafe {
        let device = this.data() as id;
        let queue = device.newCommandQueue();
        if !queue.is_null() {
            Ok(Queue::from_raw_data(queue as usize, &QUEUE_FUNCTIONS))
        } else {
            Err(Error::Failed)
        }
    }
}

fn create_program(this: &Device, source: &str) -> Result<Program, Error> {
    unsafe {
        let device = this.data() as id;
        
        let string = NSString::alloc(nil).init_str(source);
        
        let options = MTLCompileOptions::new(nil);

        let mut error = ptr::null_mut();
        let library = device.newLibraryWithSource(string, options, &mut error);
        let _: () = msg_send![options, release];
        let _: () = msg_send![string, release];
        if library.is_null() {
            if error.is_null() {
                return Err(Error::Failed)
            } else {
                let desc: id = msg_send![error, localizedDescription];
                let bytes = desc.UTF8String() as *const u8;
                let desc_str = str::from_utf8(slice::from_raw_parts(bytes, desc.len())).unwrap().to_string();
                return Err(Error::CompileFailed(desc_str))
            }
        }

        let names: Vec<_> = library.functionNames().iter().collect();
        let name = match names.len() {
            0 => return Err(Error::Failed),
            1 => names[0],
            _ => return Err(Error::Failed),
        };
        
        let function = library.newFunctionWithName(name);

        if function.functionType() != MTLFunctionType::Kernel {
            return Err(Error::Failed)
        }

        let state = device.newComputePipelineStateWithFunction(function, &mut error);
        if state.is_null() {
            if error.is_null() {
                return Err(Error::Failed)
            } else {
                let desc: id = msg_send![error, localizedDescription];
                let bytes = desc.UTF8String() as *const u8;
                let desc_str = str::from_utf8(slice::from_raw_parts(bytes, desc.len())).unwrap().to_string();
                return Err(Error::CompileFailed(desc_str))
            }
        }

        Ok(Program::from_raw_data(state as usize, &PROGRAM_FUNCTIONS))
    }
}

fn create_buffer(this: &Device, protection: Protection, mut data: BufferData)
                 -> Result<Buffer, Error> {
    unsafe {
        let device = this.data() as id;
        let buffer = match data {
            BufferData::HostAllocated(ref mut buffer) => {
                device.newBufferWithBytes(buffer.as_ptr(), buffer.size(), MTLResourceCPUCacheModeDefaultCache)
            }
            BufferData::Uninitialized(in_size) => {
                device.newBufferWithLength(in_size, MTLResourceCPUCacheModeDefaultCache)
            }
        };
        if !buffer.is_null() {
            Ok(Buffer::from_raw_data(buffer as usize, &BUFFER_FUNCTIONS))
        } else {
            Err(Error::Failed)
        }
    }
}

fn create_image(this: &Device, format: Format, protection: Protection, size: &Size2D<u32>)
                -> Result<Image, Error> {
    unsafe {
        unimplemented!()
    }
}
