use std::ffi::c_void;

// Main entry point
// Adjust the start function to include the proper imports and call the buffer data functions correctly
#[no_mangle]
pub extern "C" fn start() {
    unsafe {
        imported::setup_canvas();

        // Define the dimensions of the net
        let rows = 10;
        let cols = 10;
        let spacing = 1.0 / (cols - 1) as f32;

        // Generate the points for the net
        let mut net_vertices = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                let x = -0.5 + j as f32 * spacing;
                let y = 0.5 - i as f32 * spacing;
                let z = 0.0;
                net_vertices.push(x);
                net_vertices.push(y);
                net_vertices.push(z);
            }
        }

        // Define indices for the net
        let mut net_indices = Vec::new();
        for i in 0..(rows - 1) {
            for j in 0..cols {
                let index = i * cols + j;
                let next_index = index + cols;
                net_indices.push(index as u16);
                net_indices.push(next_index as u16);
            }
        }
        for j in 0..cols {
            for i in 0..(rows - 1) {
                let index = j * cols + i;
                let next_index = index + 1;
                net_indices.push(index as u16);
                net_indices.push(next_index as u16);
            }
        }

        // Create the vertex buffer
        let vertex_buffer = imported::create_buffer();
        imported::bind_buffer(GLEnum::ArrayBuffer, vertex_buffer);
        buffer_data_f32(GLEnum::ArrayBuffer, &net_vertices, GLEnum::StaticDraw);

        // Create the index buffer
        let index_buffer = imported::create_buffer();
        imported::bind_buffer(GLEnum::ElementArrayBuffer, index_buffer);
        buffer_data_u16(GLEnum::ElementArrayBuffer, &net_indices, GLEnum::StaticDraw);

        // Set up the shaders
        let vertex_shader = imported::create_shader(GLEnum::VertexShader);
        shader_source(
            vertex_shader,
            r#"
            attribute vec3 vertex_position;

            void main(void) {
                gl_Position = vec4(vertex_position, 1.0);
            }
        "#,
        );
        imported::compile_shader(vertex_shader);

        let fragment_shader = imported::create_shader(GLEnum::FragmentShader);
        shader_source(
            fragment_shader,
            r#"void main() {
                gl_FragColor = vec4(1.0, 0.5, 0.313, 1.0);
              }
          "#,
        );
        imported::compile_shader(fragment_shader);

        // Create the shader program
        let shader_program = imported::create_program();
        imported::attach_shader(shader_program, vertex_shader);
        imported::attach_shader(shader_program, fragment_shader);
        imported::link_program(shader_program);
        imported::use_program(shader_program);

        // Get the location of the vertex_position attribute.
        let attrib_location = get_attrib_location(shader_program, "vertex_position").unwrap();
        imported::enable_vertex_attrib_array(attrib_location);

        // Bind the vertex buffer
        imported::bind_buffer(GLEnum::ArrayBuffer, vertex_buffer);
        // Specify the layout of the vertex buffer
        imported::vertex_attrib_pointer(attrib_location as u32, 3, GLEnum::Float, false, 0, 0);

        // Bind the index buffer
        imported::bind_buffer(GLEnum::ElementArrayBuffer, index_buffer);

        // Set clear color
        imported::clear_color(0.0, 0.0, 0.0, 1.0);

        // Clear the color buffer
        imported::clear(GLEnum::ColorBufferBit);

        // Draw the net using lines
        imported::draw_elements(
            GLEnum::Lines,
            net_indices.len() as GLsizei,
            GLEnum::UnsignedShort,
            0,
        );
        //imported::update_vertex_positions(0.1);
    }
}

// A few of the external functions we'll wrap so that we can use them in a more Rusty way.

// Define a function to update vertex positions

pub fn update_vertex_positions(time: f32) {
    // Define the dimensions of the net
    let rows = 10;
    let cols = 10;
    let spacing = 1.0 / (cols - 1) as f32;

    // Generate the points for the net
    let mut net_vertices = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let x = -0.5 + j as f32 * spacing;
            let y = 0.5 - i as f32 * spacing;
            let z = 0.0;

            // Update y-coordinate based on sine wave
            let offset = (time + x) * 2.0; // Adjust speed and magnitude of the wave here
            let sine_wave = (offset).sin() * 0.1; // Adjust amplitude of the wave here
            let animated_y = y + sine_wave;

            net_vertices.push(x);
            net_vertices.push(animated_y);
            net_vertices.push(z);
        }
    }

    // Update the vertex buffer with the new positions
    buffer_data_f32(GLEnum::ArrayBuffer, &net_vertices, GLEnum::StaticDraw);
}

pub fn shader_source(shader: JSObject, source: &str) {
    unsafe { imported::shader_source(shader, source.as_ptr() as *const c_void, source.len()) }
}

pub fn get_attrib_location(program: JSObject, name: &str) -> Option<GLUint> {
    unsafe {
        let result =
            imported::get_attrib_location(program, name.as_ptr() as *const c_void, name.len());
        if result == -1 {
            None
        } else {
            Some(result as u32)
        }
    }
}

pub fn buffer_data_f32(target: GLEnum, data: &[f32], usage: GLEnum) {
    unsafe {
        imported::buffer_data_f32(
            target,
            data.as_ptr() as *const c_void,
            data.len() * std::mem::size_of::<f32>(),
            usage,
        )
    }
}

pub fn buffer_data_u16(target: GLEnum, data: &[u16], usage: GLEnum) {
    unsafe {
        imported::buffer_data_u16(
            target,
            data.as_ptr() as *const c_void,
            data.len() * std::mem::size_of::<u16>(),
            usage,
        )
    }
}

// The raw external bindings to Javascript
mod imported {
    use super::*;

    extern "C" {

        pub fn setup_canvas();
        pub fn create_buffer() -> JSObject;
        pub fn bind_buffer(target: GLEnum, gl_object: JSObject);
        pub fn buffer_data_f32(
            target: GLEnum,
            data: *const c_void,
            data_length: usize,
            usage: GLEnum,
        );
        pub fn buffer_data_u16(
            target: GLEnum,
            data: *const c_void,
            data_length: usize,
            usage: GLEnum,
        );
        pub fn create_shader(shader_type: GLEnum) -> JSObject;
        pub fn shader_source(shader: JSObject, source: *const c_void, source_length: usize);
        pub fn compile_shader(shader: JSObject);
        pub fn create_program() -> JSObject;
        pub fn attach_shader(program: JSObject, shader: JSObject);
        pub fn link_program(program: JSObject);
        pub fn use_program(program: JSObject);
        pub fn get_attrib_location(
            program: JSObject,
            name: *const c_void,
            name_length: usize,
        ) -> GLint;
        pub fn vertex_attrib_pointer(
            index: GLUint,
            size: GLint,
            _type: GLEnum,
            normalized: bool,
            stride: GLsizei,
            pointer: GLintptr,
        );
        pub fn enable_vertex_attrib_array(index: GLUint);
        pub fn clear_color(r: f32, g: f32, b: f32, a: f32);
        pub fn clear(mask: GLEnum);
        //pub fn update_vertex_positions(time: f32);
        pub fn draw_elements(mode: GLEnum, count: GLsizei, _type: GLEnum, offset: GLintptr);
    }
}

// What follows are types defined to help communicate with Javascript code.

#[derive(Clone, Copy)]
#[repr(C)]
pub struct JSObject(u32);

impl JSObject {
    pub const fn null() -> Self {
        JSObject(0)
    }
}

#[repr(u32)]
pub enum GLEnum {
    UnsignedShort = 0x1403,
    Triangles = 0x0004,
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
    VertexShader = 0x8B31,
    FragmentShader = 0x8B30,
    Float = 0x1406,
    StaticDraw = 0x88E4,
    DynamicDraw = 0x88E8,
    ColorBufferBit = 0x00004000,
    Lines = 0x0001,
}

pub type GLUint = u32;
pub type GLint = i32;
pub type GLsizei = i32;
pub type GLintptr = i32;
