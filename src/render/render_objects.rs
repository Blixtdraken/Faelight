use std::{ffi::{CStr, CString}, ptr::{null, null_mut}};

use gl::{UseProgram, types::{GLchar, GLenum, GLint, GLuint}};

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

        unsafe { gl::LinkProgram(id); }

        let mut success: GLint = 1;
        unsafe {gl::GetProgramiv(id, gl::COMPILE_STATUS, &mut success);}

        match success {
            0 => Err(Self::get_program_error(id)),
            _ => Ok(Program { id })
        }
    }

    fn get_program_error(id: GLuint)->String{
        let mut len:GLint = 0;
        unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) }

        let buffer = create_string_buffer(len as usize);

        unsafe{ gl::GetProgramInfoLog(id, len, null_mut(), buffer.as_ptr() as *mut GLchar ) }

        return buffer.to_string_lossy().into_owned();
    }

    pub fn set(&self){
        unsafe{ UseProgram(self.id); }
    }
}

impl Drop for Program{
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

pub fn create_program() -> Result<Program, String>{
    let vert_shader = Shader::from_source(&CString::new(include_str!("../../shaders/.vert")).unwrap(), gl::VERTEX_SHADER).unwrap();
    let frag_shader = Shader::from_source(&CString::new(include_str!("../../shaders/.frag")).unwrap(), gl::FRAGMENT_SHADER).unwrap();

    let shaders: &[Shader] = &[vert_shader, frag_shader];

    let program = Program::from_shaders(shaders)?;

    return Ok(program);
}


pub struct Vbo {
    id: GLuint,
}

impl Drop for Vbo{
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Vbo {

    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        Vbo{ id }
    }

    pub fn set(&self, data: &Vec<f32>) {
        self.bind();
        self.data(data);
    }

    pub fn data(&self, verts: &Vec<f32>){
        unsafe{ 
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
                verts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }

    pub fn delete(&self){
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }

}


pub struct Ibo {
    id: GLuint,
}

impl Drop for Ibo{
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Ibo {

    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        Ibo{ id }
    }

    pub fn set(&self, data: &Vec<u32>) {
        self.bind();
        self.data(data);
    }

    pub fn data(&self, indices: &Vec<u32>){
        unsafe{ 
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, 
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
    }

    pub fn delete(&self){
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }

}


pub struct Vao {
    id: GLuint,
}

impl Drop for Vao{
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Vao {

    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        Vao{ id }
    }

    pub fn set(&self) {
        self.bind();
        self.setup();
    }

   pub fn setup(&self) {
    unsafe{ 
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, 
            2, 
            gl::FLOAT, 
            gl::FALSE, 
            (2 * std::mem::size_of::<f32>()) as GLint, 
            null()
        );
    }

   }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray( self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray( 0); }
    }

    pub fn delete(&self){
        unsafe { gl::DeleteVertexArrays(1, &self.id); }
    }

}