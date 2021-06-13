use std::mem;
use std::os::raw::c_void;

use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};

pub struct Vertex {
    vao: u32,
    _vbo: u32,
    vertex_num: i32,
}

impl Vertex {
    pub fn new(
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
        attribute_type_vec: std::vec::Vec<GLenum>,
        attribute_size_vec: std::vec::Vec<GLint>,
        // 頂点が何個おきに並んでいるのか
        stride: GLsizei,
        vertex_num: i32,
    ) -> Vertex {
        let mut vao = 0;
        let mut vbo = 0;

        // rustではC言語側の検証が出来ないので明示的にunsafeで囲む
        unsafe {
            // create vertex array object and vertex buffer object
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // bind buffer
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // VBOに初めてデータを転送するときは、gl::BufferData()で
            // 2回目以降はgl::BufferSubData()
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);

            let mut offset = 0;
            // 今回は頂点属性が座標だけなのでループは１回しか回さない
            // 法線ベクトルやテクスチャ座標などの頂点属性が増えていくと、
            // その数だけVertexAttribPointerを実行していく感じになる。
            for i in 0..attribute_type_vec.len() {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    // 頂点属性の順番
                    i as u32,
                    // 頂点属性あたりの要素数
                    attribute_size_vec[i],
                    // データ型
                    attribute_type_vec[i],
                    // 正規化の有無
                    gl::FALSE,
                    // 各頂点データの始まりが何個おきか
                    stride,
                    // 頂点データの開始地点のオフセット
                    (offset * mem::size_of::<GLfloat>()) as *const c_void,
                );
                offset += attribute_size_vec[i] as usize;
            }

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Vertex {
            vao: vao,
            _vbo: vbo,
            vertex_num: vertex_num,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            // 頂点データの開始インデックスと描画する頂点の数を指定
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num);
            gl::BindVertexArray(0);
        }
    }
}