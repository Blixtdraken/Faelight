use std::{ffi::{CStr, CString}, ptr::{null, null_mut}};

use gl::types::{GLchar, GLenum, GLint, GLuint};

pub struct Shader{
    id: GLuint
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum)->Result<Self, String>{
        let id = unsafe { gl::CreateShader(kind) };
        
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe {gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);}

        match success {
            0 => Err(Self::get_shader_error(id)),
            _ => Ok(Shader { id })
        }

        
    }

    fn get_shader_error(id: GLuint)->String{
        let mut len:GLint = 0;
        unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) }

        let buffer = create_string_buffer(len as usize);

        unsafe{ gl::GetShaderInfoLog(id, len, null_mut(), buffer.as_ptr() as *mut GLchar ) }

        return buffer.to_string_lossy().into_owned();
    }

    pub fn id(&self)->GLuint { self.id }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

fn create_string_buffer(len: usize)->CString{
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe {CString::from_vec_unchecked(buffer)}
}

pub struct Program{
    id: GLuint
}

impl Program{
    fn from_shaders(shaders: &[Shader])->Result<Self, String>{
        let id = unsafe{ gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id) }
        }

        Ok(Program { id })
    }
}