#![allow(dead_code)]
use std::ptr;
use std::ffi::*;

use std::error;
use std::fmt;
use std::mem;
use std::sync::Arc;
use std::os::raw::*;
use std::cell::Cell;
use std::path::Path;
use vk_loader2 as vk;
// use feature;

type VkResult<T> = Result<T, vk::Result>;
pub struct Device<'r> {
    handle: vk::Device,
    device_fn: vk::DeviceFn,
    _lifetime: ::std::marker::PhantomData<&'r ()>,
}
impl<'r> Device<'r> {
    pub unsafe fn from_raw(handle: vk::Device, device_fn: vk::DeviceFn) -> Self {
        Device {
            handle: handle,
            device_fn: device_fn,
            _lifetime: ::std::marker::PhantomData,
        }
    }
    pub fn destroy_device(&self) {
        unsafe {
            self.device_fn.destroy_device(self.handle, ptr::null());
        }
    }

    pub fn free_memory(&self, memory: vk::DeviceMemory) {
        unsafe {
            self.device_fn.free_memory(self.handle, memory, ptr::null());
        }
    }

    pub fn destroy_fence(&self, fence: vk::Fence) {
        unsafe {
            self.device_fn.destroy_fence(self.handle, fence, ptr::null());
        }
    }

    pub fn destroy_image(&self, image: vk::Image) {
        unsafe {
            self.device_fn.destroy_image(self.handle, image, ptr::null());
        }
    }

    pub fn destroy_command_pool(&self, pool: vk::CommandPool) {
        unsafe {
            self.device_fn.destroy_command_pool(self.handle, pool, ptr::null());
        }
    }

    pub fn destroy_swapchain_khr(&self, swapchain: vk::SwapchainKHR) {
        unsafe {
            self.device_fn.destroy_swapchain_khr(self.handle, swapchain, ptr::null());
        }
    }

    pub fn destroy_image_view(&self, image_view: vk::ImageView) {
        unsafe {
            self.device_fn.destroy_image_view(self.handle, image_view, ptr::null());
        }
    }

    pub fn destroy_render_pass(&self, renderpass: vk::RenderPass) {
        unsafe {
            self.device_fn.destroy_render_pass(self.handle, renderpass, ptr::null());
        }
    }

    pub fn destroy_framebuffer(&self, framebuffer: vk::Framebuffer) {
        unsafe {
            self.device_fn.destroy_framebuffer(self.handle, framebuffer, ptr::null());
        }
    }

    pub fn destroy_pipeline_layout(&self, pipeline_layout: vk::PipelineLayout) {
        unsafe {
            self.device_fn.destroy_pipeline_layout(self.handle, pipeline_layout, ptr::null());
        }
    }

    pub fn destroy_buffer(&self, buffer: vk::Buffer) {
        unsafe {
            self.device_fn.destroy_buffer(self.handle, buffer, ptr::null());
        }
    }

    pub fn destroy_shader_module(&self, shader: vk::ShaderModule) {
        unsafe {
            self.device_fn.destroy_shader_module(self.handle, shader, ptr::null());
        }
    }

    pub fn destroy_pipeline(&self, pipeline: vk::Pipeline) {
        unsafe {
            self.device_fn.destroy_pipeline(self.handle, pipeline, ptr::null());
        }
    }

    pub fn destroy_semaphore(&self, semaphore: vk::Semaphore) {
        unsafe {
            self.device_fn.destroy_semaphore(self.handle, semaphore, ptr::null());
        }
    }
    pub fn device_wait_idle(&self) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn.device_wait_idle(self.handle);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn reset_command_buffer(&self,
                                command_buffer: vk::CommandBuffer,
                                flags: vk::CommandBufferResetFlags)
                                -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .reset_command_buffer(command_buffer, flags);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }
    pub fn reset_fences(&self, fences: &[vk::Fence]) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .reset_fences(self.handle, fences.len() as u32, fences.as_ptr());
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn cmd_begin_render_pass(&self,
                                 command_buffer: vk::CommandBuffer,
                                 create_info: &vk::RenderPassBeginInfo,
                                 contents: vk::SubpassContents) {
        unsafe {
            self.device_fn.cmd_begin_render_pass(command_buffer, create_info, contents);
        }
    }

    pub fn cmd_bind_pipeline(&self,
                             command_buffer: vk::CommandBuffer,
                             pipeline_bind_point: vk::PipelineBindPoint,
                             pipeline: vk::Pipeline) {
        unsafe {
            self.device_fn.cmd_bind_pipeline(command_buffer, pipeline_bind_point, pipeline);
        }
    }

    pub fn cmd_set_scissor(&self, command_buffer: vk::CommandBuffer, scissors: &[vk::Rect2D]) {
        unsafe {
            self.device_fn
                .cmd_set_scissor(command_buffer, 0, scissors.len() as u32, scissors.as_ptr());
        }
    }

    pub fn cmd_bind_vertex_buffers(&self,
                                   command_buffer: vk::CommandBuffer,
                                   buffers: &[vk::Buffer],
                                   offsets: &vk::DeviceSize) {
        unsafe {
            self.device_fn.cmd_bind_vertex_buffers(command_buffer,
                                                   0,
                                                   buffers.len() as u32,
                                                   buffers.as_ptr(),
                                                   offsets);
        }
    }

    pub fn cmd_end_render_pass(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.device_fn.cmd_end_render_pass(command_buffer);
        }
    }
    pub fn cmd_draw(&self,
                    command_buffer: vk::CommandBuffer,
                    vertex_count: u32,
                    instance_count: u32,
                    first_vertex: u32,
                    first_instance: u32) {
        unsafe {
            self.device_fn.cmd_draw(command_buffer,
                                    vertex_count,
                                    instance_count,
                                    first_vertex,
                                    first_instance);
        }
    }

    pub fn cmd_set_viewport(&self, command_buffer: vk::CommandBuffer, viewports: &[vk::Viewport]) {
        unsafe {
            self.device_fn.cmd_set_viewport(command_buffer,
                                            0,
                                            viewports.len() as u32,
                                            viewports.as_ptr());
        }
    }
    pub fn acquire_next_image_khr(&self,
                                  swapchain: vk::SwapchainKHR,
                                  timeout: u64,
                                  semaphore: vk::Semaphore,
                                  fence: vk::Fence)
                                  -> VkResult<u32> {
        unsafe {
            let mut index = mem::uninitialized();
            let err_code = self.device_fn
                .acquire_next_image_khr(self.handle,
                                        swapchain,
                                        timeout,
                                        semaphore,
                                        fence,
                                        &mut index);
            match err_code {
                vk::Result::Success => Ok(index),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_semaphore(&self,
                            create_info: &vk::SemaphoreCreateInfo)
                            -> VkResult<vk::Semaphore> {
        unsafe {
            let mut semaphore = mem::uninitialized();
            let err_code = self.device_fn
                .create_semaphore(self.handle, create_info, ptr::null(), &mut semaphore);
            match err_code {
                vk::Result::Success => Ok(semaphore),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_graphics_pipelines(&self,
                                     pipeline_cache: vk::PipelineCache,
                                     create_infos: &[vk::GraphicsPipelineCreateInfo])
                                     -> VkResult<Vec<vk::Pipeline>> {
        unsafe {
            let mut pipelines = Vec::with_capacity(create_infos.len());
            let err_code = self.device_fn
                .create_graphics_pipelines(self.handle,
                                           pipeline_cache,
                                           create_infos.len() as u32,
                                           create_infos.as_ptr(),
                                           ptr::null(),
                                           pipelines.as_mut_ptr());
            pipelines.set_len(create_infos.len());
            match err_code {
                vk::Result::Success => Ok(pipelines),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_buffer(&self, create_info: &vk::BufferCreateInfo) -> VkResult<vk::Buffer> {
        unsafe {
            let mut buffer = mem::uninitialized();
            let err_code = self.device_fn
                .create_buffer(self.handle, create_info, ptr::null(), &mut buffer);
            match err_code {
                vk::Result::Success => Ok(buffer),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_pipeline_layout(&self,
                                  create_info: &vk::PipelineLayoutCreateInfo)
                                  -> VkResult<vk::PipelineLayout> {
        unsafe {
            let mut pipeline_layout = mem::uninitialized();
            let err_code = self.device_fn
                .create_pipeline_layout(self.handle,
                                        create_info,
                                        ptr::null(),
                                        &mut pipeline_layout);
            match err_code {
                vk::Result::Success => Ok(pipeline_layout),
                _ => Err(err_code),
            }
        }
    }
    pub fn map_memory<T>(&self,
                         memory: vk::DeviceMemory,
                         offset: vk::DeviceSize,
                         size: vk::DeviceSize,
                         flags: vk::MemoryMapFlags)
                         -> VkResult<&mut [T]> {

        unsafe {
            let mut data: *mut () = mem::uninitialized();
            let err_code = self.device_fn
                .map_memory(self.handle, memory, offset, size, flags, &mut data);
            let x: *mut T = data as *mut T;
            match err_code {
                vk::Result::Success => {
                    Ok(::std::slice::from_raw_parts_mut(x, size as usize / mem::size_of::<T>()))
                }
                _ => Err(err_code),
            }
        }
    }

    pub fn unmap_memory(&self, memory: vk::DeviceMemory) {
        unsafe {
            self.device_fn.unmap_memory(self.handle, memory);
        }
    }

    pub fn create_framebuffer(&self,
                              create_info: &vk::FramebufferCreateInfo)
                              -> VkResult<vk::Framebuffer> {
        unsafe {
            let mut framebuffer = mem::uninitialized();
            let err_code = self.device_fn
                .create_framebuffer(self.handle, create_info, ptr::null(), &mut framebuffer);
            match err_code {
                vk::Result::Success => Ok(framebuffer),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> vk::Queue {
        unsafe {
            let mut queue = mem::uninitialized();
            self.device_fn
                .get_device_queue(self.handle, queue_family_index, queue_index, &mut queue);
            queue
        }
    }

    pub fn cmd_pipeline_barrier(&self,
                                command_buffer: vk::CommandBuffer,
                                src_stage_mask: vk::PipelineStageFlags,
                                dst_stage_mask: vk::PipelineStageFlags,
                                dependency_flags: vk::DependencyFlags,
                                memory_barriers: &[vk::MemoryBarrier],
                                buffer_memory_barriers: &[vk::BufferMemoryBarrier],
                                image_memory_barriers: &[vk::ImageMemoryBarrier]) {
        unsafe {
            self.device_fn.cmd_pipeline_barrier(command_buffer,
                                                src_stage_mask,
                                                dst_stage_mask,
                                                dependency_flags,
                                                memory_barriers.len() as u32,
                                                memory_barriers.as_ptr(),
                                                buffer_memory_barriers.len() as u32,
                                                buffer_memory_barriers.as_ptr(),
                                                image_memory_barriers.len() as u32,
                                                image_memory_barriers.as_ptr());
        }
    }

    pub fn create_render_pass(&self,
                              create_info: &vk::RenderPassCreateInfo)
                              -> VkResult<vk::RenderPass> {
        unsafe {
            let mut renderpass = mem::uninitialized();
            let err_code = self.device_fn
                .create_render_pass(self.handle, create_info, ptr::null(), &mut renderpass);
            match err_code {
                vk::Result::Success => Ok(renderpass),
                _ => Err(err_code),
            }
        }
    }

    pub fn begin_command_buffer(&self,
                                command_buffer: vk::CommandBuffer,
                                create_info: &vk::CommandBufferBeginInfo)
                                -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .begin_command_buffer(command_buffer, create_info);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .end_command_buffer(command_buffer);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn wait_for_fences(&self,
                           fences: &[vk::Fence],
                           wait_all: bool,
                           timeout: u64)
                           -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .wait_for_fences(self.handle,
                                 fences.len() as u32,
                                 fences.as_ptr(),
                                 wait_all as u32,
                                 timeout);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn queue_wait_idle(&self, queue: vk::Queue) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn.queue_wait_idle(queue);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn queue_present_khr(&self,
                             queue: vk::Queue,
                             create_info: &vk::PresentInfoKHR)
                             -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .queue_present_khr(queue, create_info);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }
    pub fn queue_submit(&self,
                        queue: vk::Queue,
                        submits: &[vk::SubmitInfo],
                        fence: vk::Fence)
                        -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .queue_submit(queue, submits.len() as u32, submits.as_ptr(), fence);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_image_view(&self,
                             create_info: &vk::ImageViewCreateInfo)
                             -> VkResult<vk::ImageView> {
        unsafe {
            let mut image_view = mem::uninitialized();
            let err_code = self.device_fn
                .create_image_view(self.handle, create_info, ptr::null(), &mut image_view);
            match err_code {
                vk::Result::Success => Ok(image_view),
                _ => Err(err_code),
            }
        }
    }
    pub fn get_swapchain_images_khr(&self,
                                    swapchain: vk::SwapchainKHR)
                                    -> VkResult<Vec<vk::Image>> {
        unsafe {
            let mut count = 0;
            self.device_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, ptr::null_mut());

            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.device_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn allocate_command_buffers<I: Into<vk::CommandBufferAllocateInfo>>
        (&self,
         i: I)
         -> VkResult<Vec<vk::CommandBuffer>> {
        let create_info = i.into();
        unsafe {
            let mut buffers = Vec::with_capacity(create_info.command_buffer_count as usize);
            let err_code = self.device_fn
                .allocate_command_buffers(self.handle, &create_info, buffers.as_mut_ptr());
            buffers.set_len(create_info.command_buffer_count as usize);
            match err_code {
                vk::Result::Success => Ok(buffers),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_command_pool<I: Into<vk::CommandPoolCreateInfo>>(&self,
                                                                   i: I)
                                                                   -> VkResult<vk::CommandPool> {
        let create_info = i.into();
        unsafe {
            let mut pool = mem::uninitialized();
            let err_code = self.device_fn
                .create_command_pool(self.handle, &create_info, ptr::null(), &mut pool);
            match err_code {
                vk::Result::Success => Ok(pool),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_swapchain_khr<I: Into<vk::SwapchainCreateInfoKHR>>
        (&self,
         i: I)
         -> VkResult<vk::SwapchainKHR> {
        let create_info = i.into();
        unsafe {
            let mut swapchain = mem::uninitialized();
            let err_code = self.device_fn
                .create_swapchain_khr(self.handle, &create_info, ptr::null(), &mut swapchain);
            match err_code {
                vk::Result::Success => Ok(swapchain),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_image(&self, create_info: &vk::ImageCreateInfo) -> VkResult<vk::Image> {
        unsafe {
            let mut image = mem::uninitialized();
            let err_code = self.device_fn
                .create_image(self.handle, create_info, ptr::null(), &mut image);
            match err_code {
                vk::Result::Success => Ok(image),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_image_memory_requirements(&self, image: vk::Image) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.device_fn
                .get_image_memory_requirements(self.handle, image, &mut mem_req);
            mem_req
        }
    }

    pub fn get_buffer_memory_requirements(&self, buffer: vk::Buffer) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.device_fn
                .get_buffer_memory_requirements(self.handle, buffer, &mut mem_req);
            mem_req
        }
    }

    pub fn allocate_memory(&self,
                           create_info: &vk::MemoryAllocateInfo)
                           -> VkResult<vk::DeviceMemory> {
        unsafe {
            let mut memory = mem::uninitialized();
            let err_code = self.device_fn
                .allocate_memory(self.handle, create_info, ptr::null(), &mut memory);
            match err_code {
                vk::Result::Success => Ok(memory),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_shader_module(&self,
                                create_info: &vk::ShaderModuleCreateInfo)
                                -> VkResult<vk::ShaderModule> {
        unsafe {
            let mut shader = mem::uninitialized();
            let err_code = self.device_fn
                .create_shader_module(self.handle, create_info, ptr::null(), &mut shader);
            match err_code {
                vk::Result::Success => Ok(shader),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_fence(&self, create_info: &vk::FenceCreateInfo) -> VkResult<vk::Fence> {
        unsafe {
            let mut fence = mem::uninitialized();
            let err_code = self.device_fn
                .create_fence(self.handle, create_info, ptr::null(), &mut fence);
            match err_code {
                vk::Result::Success => Ok(fence),
                _ => Err(err_code),
            }
        }
    }

    pub fn bind_buffer_memory(&self,
                              buffer: vk::Buffer,
                              device_memory: vk::DeviceMemory,
                              offset: vk::DeviceSize)
                              -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .bind_buffer_memory(self.handle, buffer, device_memory, offset);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }
    pub fn bind_image_memory(&self,
                             image: vk::Image,
                             device_memory: vk::DeviceMemory,
                             offset: vk::DeviceSize)
                             -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .bind_image_memory(self.handle, image, device_memory, offset);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }
}
