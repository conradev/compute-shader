// Copyright 2017 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::metal::ffi::{MTLDevice, MTLCommandQueue, MTLCommandBuffer, MTLComputeCommandEncoder, MTLResourceCPUCacheModeDefaultCache};
use buffer::Buffer;
use error::Error;
use image::{Color, Image};
use profile_event::ProfileEvent;
use program::Program;
use queue::{Queue, QueueFunctions, Uniform};
use sync_event::SyncEvent;

use byteorder::{ByteOrder, NativeEndian};
use cocoa::base::id;

pub static QUEUE_FUNCTIONS: QueueFunctions = QueueFunctions {
    destroy: destroy,
    flush: flush,
    finish: finish,
    submit_compute: submit_compute,
    submit_clear: submit_clear,
    submit_read_buffer: submit_read_buffer,
    submit_sync_event: submit_sync_event,
};

unsafe fn destroy(this: &Queue) {
    let _: () = msg_send![this.data() as id, release];
}

fn flush(this: &Queue) -> Result<(), Error> {
    unsafe {
        unimplemented!()
    }
}

fn finish(this: &Queue) -> Result<(), Error> {
    unsafe {
        unimplemented!()
    }
}

fn submit_compute(this: &Queue,
                  program: &Program,
                  num_groups: &[u32],
                  uniforms: &[(u32, Uniform)],
                  events: &[SyncEvent])
                  -> Result<ProfileEvent, Error> {
    unsafe {
        let queue = this.data() as id;
        let cmd_buf = queue.commandBuffer();
        let cmd_enc = cmd_buf.computeCommandEncoder();

        cmd_enc.setComputePipelineState(program.data() as id);

        for &(uniform_index, ref uniform) in uniforms {
            let err = match *uniform {
                Uniform::Buffer(buffer) => {
                    cmd_enc.setBuffer(buffer.data() as id, 0, uniform_index as usize)
                }
                Uniform::Image(image) => {
                    unimplemented!()
                }
                Uniform::U32(ref value) => {
                    let mut buf = [0; 4];
                    NativeEndian::write_u32(&mut buf, *value);
                    let buffer = queue.device().newBufferWithBytes(buf.as_ptr(), buf.len(), MTLResourceCPUCacheModeDefaultCache);
                    cmd_enc.setBuffer(buffer, 0, uniform_index as usize)
                }
                Uniform::UVec4(ref value) => {
                    unimplemented!()
                }
            };
        }

        cmd_enc.endEncoding();
        cmd_buf.commit();

        unreachable!()
    }
}

fn submit_clear(this: &Queue, image: &Image, color: &Color, events: &[SyncEvent])
                -> Result<ProfileEvent, Error> {
    unsafe {
        unimplemented!()
    }
}

fn submit_read_buffer(this: &Queue,
                      dest: &mut [u8],
                      buffer: &Buffer,
                      start: usize,
                      events: &[SyncEvent])
                      -> Result<ProfileEvent, Error> {
    unsafe {
        unimplemented!()
    }
}

fn submit_sync_event(this: &Queue) -> Result<SyncEvent, Error> {
    unsafe {
        unimplemented!()
    }
}
