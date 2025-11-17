#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#define STB_TRUETYPE_IMPLEMENTATION  // force following include to generate implementation
#include "stb_truetype.h"

GLFWwindow* window;

unsigned char *load_file(const char *path, size_t *out_size) {
    FILE *f = fopen(path, "rb");
    if (!f) return NULL;

    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);

    unsigned char *data = malloc(size);
    fread(data, 1, size, f);
    fclose(f);

    if (out_size) *out_size = (size_t)size;
    return data;
}

struct FontData{
    uint8_t* atlas_bitmap;
    stbtt_bakedchar* baked_chars;
};

struct FontData create_font_data(const char *ttf_path, float pixel_height, int atlas_width, int atlas_height) {
    int num_chars = 96;
    size_t ttf_size;
    unsigned char *ttf_buffer = load_file(ttf_path, &ttf_size);
    struct FontData fontdata = {0};

    if (!ttf_buffer) {
        printf("Failed to load font: %s\n", ttf_path);
        return fontdata;
    }

    uint8_t* atlas_bitmap = malloc(atlas_width * atlas_height);
    stbtt_bakedchar* baked_chars = malloc(num_chars * sizeof(stbtt_bakedchar));

    int res = stbtt_BakeFontBitmap(
        ttf_buffer,            // font data
        0,                     // font index (0 = first)
        pixel_height,          // pixel height
        atlas_bitmap,          // output bitmap
        atlas_width,
        atlas_height,
        32,                    // first char (space)
        num_chars,                    // num chars
        baked_chars            // output char info
    );

    if (res <= 0) {
        printf("stbtt_BakeFontBitmap failed\n");
    }

    free(ttf_buffer);
    fontdata.atlas_bitmap = atlas_bitmap;
    fontdata.baked_chars = baked_chars;
    return fontdata;
}

unsigned int create_texture(){
    unsigned int texture;
    glGenTextures(1, &texture);
    glBindTexture(GL_TEXTURE_2D, texture);
    // set the texture wrapping/filtering options (on the currently bound texture object)
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    // load and generate the texture
    return texture;
}

void tex_image_2d(uint8_t* ptr, int width, int height, int channels){
    if(channels == 1){
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RED, GL_UNSIGNED_BYTE, ptr);
    }
    else if(channels == 3){
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, ptr);
    }
    else if(channels == 4){
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, ptr);
    }
    else{
        printf("channels should be 1, 3 or 4");
    }
}

void generate_mipmap_2d(){
    glGenerateMipmap(GL_TEXTURE_2D);
}

void viewport(int x, int y, int w, int h){
    glViewport(x,y,w,h);
}

void clear_color_buffer_bit(float r, float g, float b, float a){
    glClearColor(0.2f, 0.3f, 0.3f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);
}

void draw_triangle_arrays(unsigned int num_vertices){
    glDrawArrays(GL_TRIANGLES, 0, num_vertices);
}

void swap_buffers(){
    glfwSwapBuffers(window);
}

void poll_events(){
    glfwPollEvents();
}

void initialize(int screenWidth, int screenHeight)
{
    // glfw: initialize and configure
    // ------------------------------
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

#ifdef __APPLE__
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
#endif

    // glfw window creation
    // --------------------
    window = glfwCreateWindow(screenWidth, screenHeight, "LearnOpenGL", NULL, NULL);
    if (window == NULL)
    {
        printf("Failed to create GLFW window");
        glfwTerminate();
        return;
    }
    glfwMakeContextCurrent(window);

    // glad: load all OpenGL function pointers
    // ---------------------------------------
    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
    {
        printf("Failed to initialize GLAD");
        return;
    }
}

unsigned int createShader(const char* source, int shaderType)
{
    unsigned int shader;
    shader = glCreateShader(shaderType);
    glShaderSource(shader, 1, &source, NULL);
    glCompileShader(shader);

    int  success;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
    if(!success)
    {
        char infoLog[512];
        glGetShaderInfoLog(shader, 512, NULL, infoLog);
        printf("%s\n", source);
        printf("--- shader failed ---\n");
        printf("%s\n", infoLog);
    }
    return shader;
}

unsigned int initialize_program(char* vertexSource, char* fragmentSource){
    unsigned int vertexShader = createShader(vertexSource, GL_VERTEX_SHADER);
    unsigned int fragmentShader = createShader(fragmentSource, GL_FRAGMENT_SHADER);
    unsigned int program = glCreateProgram();
    glAttachShader(program, vertexShader);
    glAttachShader(program, fragmentShader);
    glLinkProgram(program);

    int  success;
    glGetProgramiv(program, GL_LINK_STATUS, &success);
    if(!success) {
        char infoLog[512];
        glGetProgramInfoLog(program, 512, NULL, infoLog);
        printf("--- compilation failed ---\n");
        printf("%s\n", infoLog);
    }
    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);
    return program;
}

void set_matrix4(unsigned int program, char* name, float* ptr){
    unsigned int loc = glGetUniformLocation(program, name);
    glUniformMatrix4fv(loc, 1, GL_TRUE, ptr);
}

void set_vector3(unsigned int program, char* name, float x, float y, float z){
    unsigned int loc = glGetUniformLocation(program, name);
    glUniform3f(loc, x, y, z);
}

void bind_program(unsigned int program){
    glUseProgram(program);
}

void bind_vao(unsigned int vao){
    glBindVertexArray(vao);
}

void bind_vbo(unsigned int vbo){
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
}

void bind_texture(unsigned int texture){
    glBindTexture(GL_TEXTURE_2D, texture);
}

void update_vertices_static(uint8_t* ptr, unsigned int size){
    glBufferData(GL_ARRAY_BUFFER, size, ptr, GL_STATIC_DRAW);
}

void update_vertices_dynamic(uint8_t* ptr, unsigned int size){
    glBufferData(GL_ARRAY_BUFFER, size, ptr, GL_DYNAMIC_DRAW);
}

void vertex_attrib_pointer_float(unsigned int id, int count, uint stride, size_t ptr){
    glVertexAttribPointer(id, count, GL_FLOAT, GL_FALSE, stride, (void*)ptr);
    glEnableVertexAttribArray(id);
}

unsigned int create_vao(){
    unsigned int VAO;
    glGenVertexArrays(1, &VAO);
    return VAO;
}

unsigned int create_vbo(){
    unsigned int VBO;
    glGenBuffers(1, &VBO);
    return VBO;
}

int window_should_close(){
    return glfwWindowShouldClose(window);
}

void enable_transparency(){
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
}

void terminate(){
    glfwTerminate();
}

struct Vec2i{
    int x;
    int y;
};

struct Vec2i get_window_size(){
    struct Vec2i window_size;
    glfwGetFramebufferSize(window, &window_size.x, &window_size.y);
    return window_size;
}
