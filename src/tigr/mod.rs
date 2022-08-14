use ::libc;
extern "C" {
    pub type __GLXcontextRec;
    pub type _XIC;
    pub type _XDisplay;
    pub type _XGC;
    pub type _XrmHashBucketRec;
    pub type _XPrivate;
    pub type __GLXFBConfigRec;
    pub type _XIM;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int) -> Atom;
    fn XCreateColormap(
        _: *mut Display,
        _: Window,
        _: *mut Visual,
        _: libc::c_int,
    ) -> Colormap;
    fn XCreatePixmapCursor(
        _: *mut Display,
        _: Pixmap,
        _: Pixmap,
        _: *mut XColor,
        _: *mut XColor,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Cursor;
    fn XCreateBitmapFromData(
        _: *mut Display,
        _: Drawable,
        _: *const libc::c_char,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Pixmap;
    fn XCreateWindow(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_uint,
        _: *mut Visual,
        _: libc::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> Window;
    fn XSetWMProtocols(
        _: *mut Display,
        _: Window,
        _: *mut Atom,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_uchar,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XCheckTypedWindowEvent(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: *mut XEvent,
    ) -> libc::c_int;
    fn XDefineCursor(_: *mut Display, _: Window, _: Cursor) -> libc::c_int;
    fn XDestroyWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XFlush(_: *mut Display) -> libc::c_int;
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    fn XFreeCursor(_: *mut Display, _: Cursor) -> libc::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> libc::c_int;
    fn XGetInputFocus(
        _: *mut Display,
        _: *mut Window,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> libc::c_int;
    fn XGrabKeyboard(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: Time,
    ) -> libc::c_int;
    fn XGrabPointer(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_int,
        _: Window,
        _: Cursor,
        _: Time,
    ) -> libc::c_int;
    fn XMapRaised(_: *mut Display, _: Window) -> libc::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XMoveResizeWindow(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> libc::c_int;
    fn XQueryKeymap(_: *mut Display, _: *mut libc::c_char) -> libc::c_int;
    fn XQueryPointer(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uint,
    ) -> libc::c_int;
    fn XResizeWindow(
        _: *mut Display,
        _: Window,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn XOpenIM(
        _: *mut Display,
        _: *mut _XrmHashBucketRec,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
    ) -> XIM;
    fn Xutf8LookupString(
        _: XIC,
        _: *mut XKeyPressedEvent,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut KeySym,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XCreateIC(_: XIM, _: ...) -> XIC;
    fn XSetICFocus(_: XIC);
    fn glBindBuffer(target: GLenum, buffer: GLuint);
    fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
    fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const libc::c_void,
        usage: GLenum,
    );
    fn glAttachShader(program: GLuint, shader: GLuint);
    fn glCompileShader(shader: GLuint);
    fn glCreateProgram() -> GLuint;
    fn glCreateShader(type_0: GLenum) -> GLuint;
    fn glDeleteProgram(program: GLuint);
    fn glDeleteShader(shader: GLuint);
    fn glEnableVertexAttribArray(index: GLuint);
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
    fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn glLinkProgram(program: GLuint);
    fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
    fn glUseProgram(program: GLuint);
    fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
    fn glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_0: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const libc::c_void,
    );
    fn glBindVertexArray(array: GLuint);
    fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn glBindTexture(target: GLenum, texture: GLuint);
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    fn glEnable(cap: GLenum);
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glClear(mask: GLbitfield);
    fn glActiveTexture(texture: GLenum);
    fn glMatrixMode(mode: GLenum);
    fn glLoadIdentity();
    fn glOrtho(
        left: GLdouble,
        right: GLdouble,
        bottom: GLdouble,
        top: GLdouble,
        near_val: GLdouble,
        far_val: GLdouble,
    );
    fn glDisable(cap: GLenum);
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        border_0: GLint,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    fn glBegin(mode: GLenum);
    fn glTexCoord2f(s: GLfloat, t: GLfloat);
    fn glVertex2i(x: GLint, y: GLint);
    fn glEnd();
    fn glGetError() -> GLenum;
    fn glPixelStorei(pname: GLenum, param: GLint);
    fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    fn XSetTextProperty(_: *mut Display, _: Window, _: *mut XTextProperty, _: Atom);
    fn glXSwapBuffers(dpy_0: *mut Display, drawable: GLXDrawable);
    fn glXMakeCurrent(
        dpy_0: *mut Display,
        drawable: GLXDrawable,
        ctx: GLXContext,
    ) -> libc::c_int;
    fn glXChooseFBConfig(
        dpy_0: *mut Display,
        screen: libc::c_int,
        attribList: *const libc::c_int,
        nitems: *mut libc::c_int,
    ) -> *mut GLXFBConfig;
    fn glXGetVisualFromFBConfig(
        dpy_0: *mut Display,
        config: GLXFBConfig,
    ) -> *mut XVisualInfo;
    fn glXDestroyContext(dpy_0: *mut Display, ctx: GLXContext);
    fn glXQueryExtensionsString(
        dpy_0: *mut Display,
        screen: libc::c_int,
    ) -> *const libc::c_char;
    fn Xutf8TextListToTextProperty(
        display: *mut Display,
        list: *mut *mut libc::c_char,
        count: libc::c_int,
        style: XICCEncodingStyle,
        text_prop_return: *mut XTextProperty,
    ) -> libc::c_int;
    fn glXCreateContext(
        dpy_0: *mut Display,
        vis: *mut XVisualInfo,
        shareList: GLXContext,
        direct: libc::c_int,
    ) -> GLXContext;
    fn glXGetProcAddressARB(_: *const GLubyte) -> __GLXextFuncPtr;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn XkbKeycodeToKeysym(
        _: *mut Display,
        _: KeyCode,
        _: libc::c_int,
        _: libc::c_int,
    ) -> KeySym;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPixel {
    pub r: libc::c_uchar,
    pub g: libc::c_uchar,
    pub b: libc::c_uchar,
    pub a: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tigr {
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub cx: libc::c_int,
    pub cy: libc::c_int,
    pub cw: libc::c_int,
    pub ch: libc::c_int,
    pub pix: *mut TPixel,
    pub handle: *mut libc::c_void,
    pub blitMode: libc::c_int,
}
pub type GLXContext = *mut __GLXcontextRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TigrInternal {
    pub shown: libc::c_int,
    pub closed: libc::c_int,
    pub gl: GLStuff,
    pub dpy: *mut Display,
    pub win: Window,
    pub glc: GLXContext,
    pub ic: XIC,
    pub widgets: *mut Tigr,
    pub widgetsWanted: libc::c_int,
    pub widgetAlpha: libc::c_uchar,
    pub widgetsScale: libc::c_float,
    pub p1: libc::c_float,
    pub p2: libc::c_float,
    pub p3: libc::c_float,
    pub p4: libc::c_float,
    pub flags: libc::c_int,
    pub scale: libc::c_int,
    pub pos: [libc::c_int; 4],
    pub lastChar: libc::c_int,
    pub keys: [libc::c_char; 256],
    pub prev: [libc::c_char; 256],
    pub mouseButtons: libc::c_int,
    pub mouseX: libc::c_int,
    pub mouseY: libc::c_int,
}
pub type XIC = *mut _XIC;
pub type Window = XID;
pub type XID = libc::c_ulong;
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLStuff {
    pub tex: [GLuint; 2],
    pub vao: GLuint,
    pub program: GLuint,
    pub uniform_projection: GLuint,
    pub uniform_model: GLuint,
    pub uniform_parameters: GLuint,
    pub gl_legacy: libc::c_int,
    pub gl_user_opengl_rendering: libc::c_int,
}
pub type GLuint = libc::c_uint;
pub type GLXDrawable = XID;
pub type GLenum = libc::c_uint;
pub type va_list = __builtin_va_list;
pub type GLint = libc::c_int;
pub type GLsizei = libc::c_int;
pub type GLchar = libc::c_char;
pub type GLfloat = libc::c_float;
pub type GLboolean = libc::c_uchar;
pub type GLsizeiptr = khronos_ssize_t;
pub type khronos_ssize_t = libc::c_long;
pub type Pixmap = XID;
pub type Cursor = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: libc::c_ulong,
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub flags: libc::c_char,
    pub pad: libc::c_char,
}
pub type Drawable = XID;
pub const TIGR_BLEND_ALPHA: TIGRBlitMode = 1;
pub type PFNGLXSWAPINTERVALSGIPROC = Option::<
    unsafe extern "C" fn(libc::c_int) -> libc::c_int,
>;
pub type __GLXextFuncPtr = Option::<unsafe extern "C" fn() -> ()>;
pub type GLubyte = libc::c_uchar;
pub type _XPrivDisplay = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
pub type Colormap = XID;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
pub type VisualID = libc::c_ulong;
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option::<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
pub type PFNGLXSWAPINTERVALMESAPROC = Option::<
    unsafe extern "C" fn(libc::c_uint) -> libc::c_int,
>;
pub type PFNGLXSWAPINTERVALEXTPROC = Option::<
    unsafe extern "C" fn(*mut Display, GLXDrawable, libc::c_int) -> (),
>;
pub type GLXFBConfig = *mut __GLXFBConfigRec;
pub type PFNGLXCREATECONTEXTATTRIBSARBPROC = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXFBConfig,
        GLXContext,
        libc::c_int,
        *const libc::c_int,
    ) -> GLXContext,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: libc::c_int,
    pub depth: libc::c_int,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub colormap_size: libc::c_int,
    pub bits_per_rgb: libc::c_int,
}
pub type Atom = libc::c_ulong;
pub type XIM = *mut _XIM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XTextProperty {
    pub value: *mut libc::c_uchar,
    pub encoding: Atom,
    pub format: libc::c_int,
    pub nitems: libc::c_ulong,
}
pub type XICCEncodingStyle = libc::c_uint;
pub const XUTF8StringStyle: XICCEncodingStyle = 4;
pub const XStdICCTextStyle: XICCEncodingStyle = 3;
pub const XTextStyle: XICCEncodingStyle = 2;
pub const XCompoundTextStyle: XICCEncodingStyle = 1;
pub const XStringStyle: XICCEncodingStyle = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub depth: libc::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: libc::c_int,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub colormap: Colormap,
    pub map_installed: libc::c_int,
    pub map_state: libc::c_int,
    pub all_event_masks: libc::c_long,
    pub your_event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub screen: *mut Screen,
}
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
pub type Time = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WindowHints {
    pub flags: libc::c_ulong,
    pub functions: libc::c_ulong,
    pub decorations: libc::c_ulong,
    pub inputMode: libc::c_long,
    pub status: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
pub type KeySym = XID;
pub type XKeyPressedEvent = XKeyEvent;
pub const TK_RALT: C2RustUnnamed_1 = 185;
pub const TK_LALT: C2RustUnnamed_1 = 184;
pub const TK_ALT: C2RustUnnamed_1 = 161;
pub const TK_RCONTROL: C2RustUnnamed_1 = 183;
pub const TK_LCONTROL: C2RustUnnamed_1 = 182;
pub const TK_CONTROL: C2RustUnnamed_1 = 160;
pub const TK_RSHIFT: C2RustUnnamed_1 = 181;
pub const TK_LSHIFT: C2RustUnnamed_1 = 180;
pub const TK_SHIFT: C2RustUnnamed_1 = 159;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type KeyCode = libc::c_uchar;
pub const TK_TICK: C2RustUnnamed_1 = 196;
pub const TK_RSQUARE: C2RustUnnamed_1 = 195;
pub const TK_BACKSLASH: C2RustUnnamed_1 = 194;
pub const TK_LSQUARE: C2RustUnnamed_1 = 193;
pub const TK_BACKTICK: C2RustUnnamed_1 = 192;
pub const TK_SLASH: C2RustUnnamed_1 = 191;
pub const TK_DOT: C2RustUnnamed_1 = 190;
pub const TK_MINUS: C2RustUnnamed_1 = 189;
pub const TK_COMMA: C2RustUnnamed_1 = 188;
pub const TK_EQUALS: C2RustUnnamed_1 = 187;
pub const TK_SEMICOLON: C2RustUnnamed_1 = 186;
pub const TK_SCROLL: C2RustUnnamed_1 = 179;
pub const TK_NUMLOCK: C2RustUnnamed_1 = 178;
pub const TK_RWIN: C2RustUnnamed_1 = 177;
pub const TK_LWIN: C2RustUnnamed_1 = 176;
pub const TK_DELETE: C2RustUnnamed_1 = 175;
pub const TK_INSERT: C2RustUnnamed_1 = 174;
pub const TK_DOWN: C2RustUnnamed_1 = 173;
pub const TK_RIGHT: C2RustUnnamed_1 = 172;
pub const TK_UP: C2RustUnnamed_1 = 171;
pub const TK_LEFT: C2RustUnnamed_1 = 170;
pub const TK_HOME: C2RustUnnamed_1 = 169;
pub const TK_END: C2RustUnnamed_1 = 168;
pub const TK_PAGEDN: C2RustUnnamed_1 = 167;
pub const TK_PAGEUP: C2RustUnnamed_1 = 166;
pub const TK_SPACE: C2RustUnnamed_1 = 165;
pub const TK_ESCAPE: C2RustUnnamed_1 = 164;
pub const TK_CAPSLOCK: C2RustUnnamed_1 = 163;
pub const TK_PAUSE: C2RustUnnamed_1 = 162;
pub const TK_RETURN: C2RustUnnamed_1 = 158;
pub const TK_TAB: C2RustUnnamed_1 = 157;
pub const TK_BACKSPACE: C2RustUnnamed_1 = 156;
pub const TK_F12: C2RustUnnamed_1 = 155;
pub const TK_F11: C2RustUnnamed_1 = 154;
pub const TK_F10: C2RustUnnamed_1 = 153;
pub const TK_F9: C2RustUnnamed_1 = 152;
pub const TK_F8: C2RustUnnamed_1 = 151;
pub const TK_F7: C2RustUnnamed_1 = 150;
pub const TK_F6: C2RustUnnamed_1 = 149;
pub const TK_F5: C2RustUnnamed_1 = 148;
pub const TK_F4: C2RustUnnamed_1 = 147;
pub const TK_F3: C2RustUnnamed_1 = 146;
pub const TK_F2: C2RustUnnamed_1 = 145;
pub const TK_F1: C2RustUnnamed_1 = 144;
pub const TK_PADENTER: C2RustUnnamed_1 = 140;
pub const TK_PADDOT: C2RustUnnamed_1 = 142;
pub const TK_PADSUB: C2RustUnnamed_1 = 141;
pub const TK_PADADD: C2RustUnnamed_1 = 139;
pub const TK_PADDIV: C2RustUnnamed_1 = 143;
pub const TK_PADMUL: C2RustUnnamed_1 = 138;
pub const TK_PAD9: C2RustUnnamed_1 = 137;
pub const TK_PAD8: C2RustUnnamed_1 = 136;
pub const TK_PAD7: C2RustUnnamed_1 = 135;
pub const TK_PAD6: C2RustUnnamed_1 = 134;
pub const TK_PAD5: C2RustUnnamed_1 = 133;
pub const TK_PAD4: C2RustUnnamed_1 = 132;
pub const TK_PAD3: C2RustUnnamed_1 = 131;
pub const TK_PAD2: C2RustUnnamed_1 = 130;
pub const TK_PAD1: C2RustUnnamed_1 = 129;
pub const TK_PAD0: C2RustUnnamed_1 = 128;
pub type GLvoid = ();
pub type GLdouble = libc::c_double;
pub type GLbitfield = libc::c_uint;
pub type GLclampf = libc::c_float;
pub type TIGRBlitMode = libc::c_uint;
pub const TIGR_KEEP_ALPHA: TIGRBlitMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TigrGlyph {
    pub code: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TigrFont {
    pub bitmap: *mut Tigr,
    pub numGlyphs: libc::c_int,
    pub glyphs: *mut TigrGlyph,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PNG {
    pub p: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct State {
    pub bits: libc::c_uint,
    pub count: libc::c_uint,
    pub in_0: *const libc::c_uchar,
    pub inend: *const libc::c_uchar,
    pub out: *mut libc::c_uchar,
    pub outend: *mut libc::c_uchar,
    pub jmp: jmp_buf,
    pub litcodes: [libc::c_uint; 288],
    pub distcodes: [libc::c_uint; 32],
    pub lencodes: [libc::c_uint; 19],
    pub tlit: libc::c_int,
    pub tdist: libc::c_int,
    pub tlen: libc::c_int,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __jmp_buf = [libc::c_long; 8];
pub type C2RustUnnamed_1 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TigrTouchPoint {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Save {
    pub crc: libc::c_uint,
    pub adler: libc::c_uint,
    pub bits: libc::c_uint,
    pub prev: libc::c_uint,
    pub runlen: libc::c_uint,
    pub out: *mut FILE,
    pub crcTable: [libc::c_uint; 256],
}
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __time_t = libc::c_long;
#[inline]
unsafe extern "C" fn tigrRGB(
    mut r: libc::c_uchar,
    mut g: libc::c_uchar,
    mut b: libc::c_uchar,
) -> TPixel {
    let mut p: TPixel = TPixel { r: 0, g: 0, b: 0, a: 0 };
    p.r = r;
    p.g = g;
    p.b = b;
    p.a = 0xff as libc::c_int as libc::c_uchar;
    return p;
}
#[inline]
unsafe extern "C" fn tigrRGBA(
    mut r: libc::c_uchar,
    mut g: libc::c_uchar,
    mut b: libc::c_uchar,
    mut a: libc::c_uchar,
) -> TPixel {
    let mut p: TPixel = TPixel { r: 0, g: 0, b: 0, a: 0 };
    p.r = r;
    p.g = g;
    p.b = b;
    p.a = a;
    return p;
}
#[no_mangle]
pub static mut tigr_upscale_gl_vs: [libc::c_char; 235] = unsafe {
    *::core::mem::transmute::<
        &[u8; 235],
        &[libc::c_char; 235],
    >(
        b"#version 330 core\nlayout (location = 0) in vec2 pos_in;layout (location = 1) in vec2 uv_in;out vec2 uv;uniform mat4 model;uniform mat4 projection;void main(){   uv = uv_in;   gl_Position = projection * model * vec4(pos_in, 0.0, 1.0);}\0",
    )
};
#[no_mangle]
pub static mut tigr_upscale_gl_vs_size: libc::c_int = ::core::mem::size_of::<[libc::c_char; 235]>() as libc::c_ulong as libc::c_int
        - 1 as libc::c_int;
#[no_mangle]
pub static mut tigr_upscale_gl_fs: [libc::c_char; 175] = unsafe {
    *::core::mem::transmute::<
        &[u8; 175],
        &[libc::c_char; 175],
    >(
        b"#version 330 core\nin vec2 uv;out vec4 color;uniform sampler2D image;uniform vec4 parameters;void fxShader(out vec4 color, in vec2 coord);void main(){   fxShader(color, uv);}\n\0",
    )
};
#[no_mangle]
pub static mut tigr_upscale_gl_fs_size: libc::c_int = ::core::mem::size_of::<[libc::c_char; 175]>() as libc::c_ulong as libc::c_int
        - 1 as libc::c_int;
#[no_mangle]
pub static mut tigr_default_fx_gl_fs: [libc::c_char; 368] = unsafe {
    *::core::mem::transmute::<
        &[u8; 368],
        &[libc::c_char; 368],
    >(
        b"void fxShader(out vec4 color, in vec2 uv) {   vec2 tex_size = vec2(textureSize(image, 0));   vec2 uv_blur = mix(floor(uv * tex_size) + 0.5, uv * tex_size, parameters.xy) / tex_size;   vec4 c = texture(image, uv_blur);   c.rgb *= mix(0.5, 1.0 - fract(uv.y * tex_size.y), parameters.z) * 2.0; //scanline\n   c = mix(vec4(0.5), c, parameters.w); //contrast\n   color = c;}\0",
    )
};
#[no_mangle]
pub static mut tigr_default_fx_gl_fs_size: libc::c_int = ::core::mem::size_of::<[libc::c_char; 368]>() as libc::c_ulong as libc::c_int
        - 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn tigrBitmap2(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut extra: libc::c_int,
) -> *mut Tigr {
    let mut tigr: *mut Tigr = calloc(
        1 as libc::c_int as libc::c_ulong,
        (::core::mem::size_of::<Tigr>() as libc::c_ulong)
            .wrapping_add(extra as libc::c_ulong),
    ) as *mut Tigr;
    (*tigr).w = w;
    (*tigr).h = h;
    (*tigr).cw = -(1 as libc::c_int);
    (*tigr).ch = -(1 as libc::c_int);
    (*tigr)
        .pix = calloc(
        (w * h) as libc::c_ulong,
        ::core::mem::size_of::<TPixel>() as libc::c_ulong,
    ) as *mut TPixel;
    (*tigr).blitMode = TIGR_BLEND_ALPHA as libc::c_int;
    return tigr;
}
#[no_mangle]
pub unsafe extern "C" fn tigrBitmap(
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> *mut Tigr {
    return tigrBitmap2(w, h, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tigrResize(
    mut bmp: *mut Tigr,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    if (*bmp).w == w && (*bmp).h == h {
        return;
    }
    let mut y: libc::c_int = 0;
    let mut cw: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut newpix: *mut TPixel = calloc(
        (w * h) as libc::c_ulong,
        ::core::mem::size_of::<TPixel>() as libc::c_ulong,
    ) as *mut TPixel;
    cw = if w < (*bmp).w { w } else { (*bmp).w };
    ch = if h < (*bmp).h { h } else { (*bmp).h };
    y = 0 as libc::c_int;
    while y < ch {
        memcpy(
            newpix.offset((y * w) as isize) as *mut libc::c_void,
            ((*bmp).pix).offset((y * (*bmp).w) as isize) as *const libc::c_void,
            (cw as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TPixel>() as libc::c_ulong),
        );
        y += 1;
    }
    free((*bmp).pix as *mut libc::c_void);
    (*bmp).pix = newpix;
    (*bmp).w = w;
    (*bmp).h = h;
}
#[no_mangle]
pub unsafe extern "C" fn tigrCalcScale(
    mut bmpW: libc::c_int,
    mut bmpH: libc::c_int,
    mut areaW: libc::c_int,
    mut areaH: libc::c_int,
) -> libc::c_int {
    let mut scale: libc::c_int = 0 as libc::c_int;
    loop {
        scale += 1;
        if !(bmpW * scale > areaW || bmpH * scale > areaH) {
            continue;
        }
        scale -= 1;
        break;
    }
    return if scale > 1 as libc::c_int { scale } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn tigrEnforceScale(
    mut scale: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    if flags & 8 as libc::c_int != 0 && scale < 4 as libc::c_int {
        scale = 4 as libc::c_int;
    }
    if flags & 4 as libc::c_int != 0 && scale < 3 as libc::c_int {
        scale = 3 as libc::c_int;
    }
    if flags & 2 as libc::c_int != 0 && scale < 2 as libc::c_int {
        scale = 2 as libc::c_int;
    }
    return scale;
}
#[no_mangle]
pub unsafe extern "C" fn tigrPosition(
    mut bmp: *mut Tigr,
    mut scale: libc::c_int,
    mut windowW: libc::c_int,
    mut windowH: libc::c_int,
    mut out: *mut libc::c_int,
) {
    *out
        .offset(
            0 as libc::c_int as isize,
        ) = (windowW - (*bmp).w * scale) / 2 as libc::c_int;
    *out
        .offset(
            1 as libc::c_int as isize,
        ) = (windowH - (*bmp).h * scale) / 2 as libc::c_int;
    *out
        .offset(
            2 as libc::c_int as isize,
        ) = *out.offset(0 as libc::c_int as isize) + (*bmp).w * scale;
    *out
        .offset(
            3 as libc::c_int as isize,
        ) = *out.offset(1 as libc::c_int as isize) + (*bmp).h * scale;
}
#[no_mangle]
pub unsafe extern "C" fn tigrClear(mut bmp: *mut Tigr, mut color: TPixel) {
    let mut count: libc::c_int = (*bmp).w * (*bmp).h;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < count {
        *((*bmp).pix).offset(n as isize) = color;
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrFill(
    mut bmp: *mut Tigr,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut color: TPixel,
) {
    let mut td: *mut TPixel = 0 as *mut TPixel;
    let mut dt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if x < 0 as libc::c_int {
        w += x;
        x = 0 as libc::c_int;
    }
    if y < 0 as libc::c_int {
        h += y;
        y = 0 as libc::c_int;
    }
    if x + w > (*bmp).w {
        w = (*bmp).w - x;
    }
    if y + h > (*bmp).h {
        h = (*bmp).h - y;
    }
    if w <= 0 as libc::c_int || h <= 0 as libc::c_int {
        return;
    }
    td = &mut *((*bmp).pix).offset((y * (*bmp).w + x) as isize) as *mut TPixel;
    dt = (*bmp).w;
    loop {
        i = 0 as libc::c_int;
        while i < w {
            *td.offset(i as isize) = color;
            i += 1;
        }
        td = td.offset(dt as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrLine(
    mut bmp: *mut Tigr,
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut color: TPixel,
) {
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut e2: libc::c_int = 0;
    dx = abs(x1 - x0);
    dy = abs(y1 - y0);
    if x0 < x1 {
        sx = 1 as libc::c_int;
    } else {
        sx = -(1 as libc::c_int);
    }
    if y0 < y1 {
        sy = 1 as libc::c_int;
    } else {
        sy = -(1 as libc::c_int);
    }
    err = dx - dy;
    loop {
        tigrPlot(bmp, x0, y0, color);
        e2 = 2 as libc::c_int * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
        if !(x0 != x1 || y0 != y1) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrFillRect(
    mut bmp: *mut Tigr,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut color: TPixel,
) {
    x += 1 as libc::c_int;
    y += 1 as libc::c_int;
    w -= 2 as libc::c_int;
    h -= 2 as libc::c_int;
    let mut cx: libc::c_int = (*bmp).cx;
    let mut cy: libc::c_int = (*bmp).cy;
    let mut cw: libc::c_int = if (*bmp).cw >= 0 as libc::c_int {
        (*bmp).cw
    } else {
        (*bmp).w
    };
    let mut ch: libc::c_int = if (*bmp).ch >= 0 as libc::c_int {
        (*bmp).ch
    } else {
        (*bmp).h
    };
    if x < cx {
        w += x - cx;
        x = cx;
    }
    if y < cy {
        h += y - cy;
        y = cy;
    }
    if x + w > cx + cw {
        w -= x + w - (cx + cw);
    }
    if y + h > cy + ch {
        h -= y + h - (cy + ch);
    }
    if w <= 0 as libc::c_int || h <= 0 as libc::c_int {
        return;
    }
    let mut td: *mut TPixel = &mut *((*bmp).pix).offset((y * (*bmp).w + x) as isize)
        as *mut TPixel;
    let mut dt: libc::c_int = (*bmp).w;
    let mut xa: libc::c_int = color.a as libc::c_int
        + (color.a as libc::c_int > 0 as libc::c_int) as libc::c_int;
    let mut a: libc::c_int = xa * xa;
    loop {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < w {
            let ref mut fresh0 = (*td.offset(i as isize)).r;
            *fresh0 = (*fresh0 as libc::c_int
                + ((color.r as libc::c_int - (*td.offset(i as isize)).r as libc::c_int)
                    * a >> 16 as libc::c_int) as libc::c_uchar as libc::c_int)
                as libc::c_uchar;
            let ref mut fresh1 = (*td.offset(i as isize)).g;
            *fresh1 = (*fresh1 as libc::c_int
                + ((color.g as libc::c_int - (*td.offset(i as isize)).g as libc::c_int)
                    * a >> 16 as libc::c_int) as libc::c_uchar as libc::c_int)
                as libc::c_uchar;
            let ref mut fresh2 = (*td.offset(i as isize)).b;
            *fresh2 = (*fresh2 as libc::c_int
                + ((color.b as libc::c_int - (*td.offset(i as isize)).b as libc::c_int)
                    * a >> 16 as libc::c_int) as libc::c_uchar as libc::c_int)
                as libc::c_uchar;
            let ref mut fresh3 = (*td.offset(i as isize)).a;
            *fresh3 = (*fresh3 as libc::c_int
                + (*bmp).blitMode
                    * ((color.a as libc::c_int
                        - (*td.offset(i as isize)).a as libc::c_int) * a
                        >> 16 as libc::c_int) as libc::c_uchar as libc::c_int)
                as libc::c_uchar;
            i += 1;
        }
        td = td.offset(dt as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrRect(
    mut bmp: *mut Tigr,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut color: TPixel,
) {
    let mut x1: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    if w <= 0 as libc::c_int || h <= 0 as libc::c_int {
        return;
    }
    x1 = x + w - 1 as libc::c_int;
    y1 = y + h - 1 as libc::c_int;
    tigrLine(bmp, x, y, x1, y, color);
    tigrLine(bmp, x1, y, x1, y1, color);
    tigrLine(bmp, x1, y1, x, y1, color);
    tigrLine(bmp, x, y1, x, y, color);
}
#[no_mangle]
pub unsafe extern "C" fn tigrFillCircle(
    mut bmp: *mut Tigr,
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut r: libc::c_int,
    mut color: TPixel,
) {
    if r <= 0 as libc::c_int {
        return;
    }
    let mut E: libc::c_int = 1 as libc::c_int - r;
    let mut dx: libc::c_int = 0 as libc::c_int;
    let mut dy: libc::c_int = -(2 as libc::c_int) * r;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = r;
    tigrLine(bmp, x0 - r + 1 as libc::c_int, y0, x0 + r, y0, color);
    while x < y - 1 as libc::c_int {
        x += 1;
        if E >= 0 as libc::c_int {
            y -= 1;
            dy += 2 as libc::c_int;
            E += dy;
            tigrLine(bmp, x0 - x + 1 as libc::c_int, y0 + y, x0 + x, y0 + y, color);
            tigrLine(bmp, x0 - x + 1 as libc::c_int, y0 - y, x0 + x, y0 - y, color);
        }
        dx += 2 as libc::c_int;
        E += dx + 1 as libc::c_int;
        if x != y {
            tigrLine(bmp, x0 - y + 1 as libc::c_int, y0 + x, x0 + y, y0 + x, color);
            tigrLine(bmp, x0 - y + 1 as libc::c_int, y0 - x, x0 + y, y0 - x, color);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrCircle(
    mut bmp: *mut Tigr,
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut r: libc::c_int,
    mut color: TPixel,
) {
    let mut E: libc::c_int = 1 as libc::c_int - r;
    let mut dx: libc::c_int = 0 as libc::c_int;
    let mut dy: libc::c_int = -(2 as libc::c_int) * r;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = r;
    tigrPlot(bmp, x0, y0 + r, color);
    tigrPlot(bmp, x0, y0 - r, color);
    tigrPlot(bmp, x0 + r, y0, color);
    tigrPlot(bmp, x0 - r, y0, color);
    while x < y - 1 as libc::c_int {
        x += 1;
        if E >= 0 as libc::c_int {
            y -= 1;
            dy += 2 as libc::c_int;
            E += dy;
        }
        dx += 2 as libc::c_int;
        E += dx + 1 as libc::c_int;
        tigrPlot(bmp, x0 + x, y0 + y, color);
        tigrPlot(bmp, x0 - x, y0 + y, color);
        tigrPlot(bmp, x0 + x, y0 - y, color);
        tigrPlot(bmp, x0 - x, y0 - y, color);
        if x != y {
            tigrPlot(bmp, x0 + y, y0 + x, color);
            tigrPlot(bmp, x0 - y, y0 + x, color);
            tigrPlot(bmp, x0 + y, y0 - x, color);
            tigrPlot(bmp, x0 - y, y0 - x, color);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrGet(
    mut bmp: *mut Tigr,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> TPixel {
    let mut empty: TPixel = {
        let mut init = TPixel {
            r: 0 as libc::c_int as libc::c_uchar,
            g: 0 as libc::c_int as libc::c_uchar,
            b: 0 as libc::c_int as libc::c_uchar,
            a: 0 as libc::c_int as libc::c_uchar,
        };
        init
    };
    if x >= 0 as libc::c_int && y >= 0 as libc::c_int && x < (*bmp).w && y < (*bmp).h {
        return *((*bmp).pix).offset((y * (*bmp).w + x) as isize);
    }
    return empty;
}
#[no_mangle]
pub unsafe extern "C" fn tigrPlot(
    mut bmp: *mut Tigr,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut pix: TPixel,
) {
    let mut xa: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut cx: libc::c_int = (*bmp).cx;
    let mut cy: libc::c_int = (*bmp).cy;
    let mut cw: libc::c_int = if (*bmp).cw >= 0 as libc::c_int {
        (*bmp).cw
    } else {
        (*bmp).w
    };
    let mut ch: libc::c_int = if (*bmp).ch >= 0 as libc::c_int {
        (*bmp).ch
    } else {
        (*bmp).h
    };
    if x >= cx && y >= cy && x < cx + cw && y < cy + ch {
        xa = pix.a as libc::c_int
            + (pix.a as libc::c_int > 0 as libc::c_int) as libc::c_int;
        a = xa * xa;
        i = y * (*bmp).w + x;
        let ref mut fresh4 = (*((*bmp).pix).offset(i as isize)).r;
        *fresh4 = (*fresh4 as libc::c_int
            + ((pix.r as libc::c_int
                - (*((*bmp).pix).offset(i as isize)).r as libc::c_int) * a
                >> 16 as libc::c_int) as libc::c_uchar as libc::c_int) as libc::c_uchar;
        let ref mut fresh5 = (*((*bmp).pix).offset(i as isize)).g;
        *fresh5 = (*fresh5 as libc::c_int
            + ((pix.g as libc::c_int
                - (*((*bmp).pix).offset(i as isize)).g as libc::c_int) * a
                >> 16 as libc::c_int) as libc::c_uchar as libc::c_int) as libc::c_uchar;
        let ref mut fresh6 = (*((*bmp).pix).offset(i as isize)).b;
        *fresh6 = (*fresh6 as libc::c_int
            + ((pix.b as libc::c_int
                - (*((*bmp).pix).offset(i as isize)).b as libc::c_int) * a
                >> 16 as libc::c_int) as libc::c_uchar as libc::c_int) as libc::c_uchar;
        let ref mut fresh7 = (*((*bmp).pix).offset(i as isize)).a;
        *fresh7 = (*fresh7 as libc::c_int
            + (*bmp).blitMode
                * ((pix.a as libc::c_int
                    - (*((*bmp).pix).offset(i as isize)).a as libc::c_int) * a
                    >> 16 as libc::c_int) as libc::c_uchar as libc::c_int)
            as libc::c_uchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrClip(
    mut bmp: *mut Tigr,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut cw: libc::c_int,
    mut ch: libc::c_int,
) {
    (*bmp).cx = cx;
    (*bmp).cy = cy;
    (*bmp).cw = cw;
    (*bmp).ch = ch;
}
#[no_mangle]
pub unsafe extern "C" fn tigrBlit(
    mut dst: *mut Tigr,
    mut src: *mut Tigr,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut sx: libc::c_int,
    mut sy: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut cw: libc::c_int = if (*dst).cw >= 0 as libc::c_int {
        (*dst).cw
    } else {
        (*dst).w
    };
    let mut ch: libc::c_int = if (*dst).ch >= 0 as libc::c_int {
        (*dst).ch
    } else {
        (*dst).h
    };
    if dx < (*dst).cx {
        let mut D: libc::c_int = (*dst).cx - dx;
        w -= D;
        sx += D;
        dx += D;
    }
    if dy < (*dst).cy {
        let mut D_0: libc::c_int = (*dst).cy - dy;
        h -= D_0;
        sy += D_0;
        dy += D_0;
    }
    if sx < 0 as libc::c_int {
        let mut D_1: libc::c_int = 0 as libc::c_int - sx;
        w -= D_1;
        dx += D_1;
        sx += D_1;
    }
    if sy < 0 as libc::c_int {
        let mut D_2: libc::c_int = 0 as libc::c_int - sy;
        h -= D_2;
        dy += D_2;
        sy += D_2;
    }
    if dx + w > (*dst).cx + cw {
        w = (*dst).cx + cw - dx;
    }
    if dy + h > (*dst).cy + ch {
        h = (*dst).cy + ch - dy;
    }
    if sx + w > (*src).w {
        w = (*src).w - sx;
    }
    if sy + h > (*src).h {
        h = (*src).h - sy;
    }
    if w <= 0 as libc::c_int || h <= 0 as libc::c_int {
        return;
    }
    let mut ts: *mut TPixel = &mut *((*src).pix).offset((sy * (*src).w + sx) as isize)
        as *mut TPixel;
    let mut td: *mut TPixel = &mut *((*dst).pix).offset((dy * (*dst).w + dx) as isize)
        as *mut TPixel;
    let mut st: libc::c_int = (*src).w;
    let mut dt: libc::c_int = (*dst).w;
    loop {
        memcpy(
            td as *mut libc::c_void,
            ts as *const libc::c_void,
            (w as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TPixel>() as libc::c_ulong),
        );
        ts = ts.offset(st as isize);
        td = td.offset(dt as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrBlitTint(
    mut dst: *mut Tigr,
    mut src: *mut Tigr,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut sx: libc::c_int,
    mut sy: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut tint: TPixel,
) {
    let mut cw: libc::c_int = if (*dst).cw >= 0 as libc::c_int {
        (*dst).cw
    } else {
        (*dst).w
    };
    let mut ch: libc::c_int = if (*dst).ch >= 0 as libc::c_int {
        (*dst).ch
    } else {
        (*dst).h
    };
    if dx < (*dst).cx {
        let mut D: libc::c_int = (*dst).cx - dx;
        w -= D;
        sx += D;
        dx += D;
    }
    if dy < (*dst).cy {
        let mut D_0: libc::c_int = (*dst).cy - dy;
        h -= D_0;
        sy += D_0;
        dy += D_0;
    }
    if sx < 0 as libc::c_int {
        let mut D_1: libc::c_int = 0 as libc::c_int - sx;
        w -= D_1;
        dx += D_1;
        sx += D_1;
    }
    if sy < 0 as libc::c_int {
        let mut D_2: libc::c_int = 0 as libc::c_int - sy;
        h -= D_2;
        dy += D_2;
        sy += D_2;
    }
    if dx + w > (*dst).cx + cw {
        w = (*dst).cx + cw - dx;
    }
    if dy + h > (*dst).cy + ch {
        h = (*dst).cy + ch - dy;
    }
    if sx + w > (*src).w {
        w = (*src).w - sx;
    }
    if sy + h > (*src).h {
        h = (*src).h - sy;
    }
    if w <= 0 as libc::c_int || h <= 0 as libc::c_int {
        return;
    }
    let mut xr: libc::c_int = tint.r as libc::c_int
        + (tint.r as libc::c_int > 0 as libc::c_int) as libc::c_int;
    let mut xg: libc::c_int = tint.g as libc::c_int
        + (tint.g as libc::c_int > 0 as libc::c_int) as libc::c_int;
    let mut xb: libc::c_int = tint.b as libc::c_int
        + (tint.b as libc::c_int > 0 as libc::c_int) as libc::c_int;
    let mut xa: libc::c_int = tint.a as libc::c_int
        + (tint.a as libc::c_int > 0 as libc::c_int) as libc::c_int;
    let mut ts: *mut TPixel = &mut *((*src).pix).offset((sy * (*src).w + sx) as isize)
        as *mut TPixel;
    let mut td: *mut TPixel = &mut *((*dst).pix).offset((dy * (*dst).w + dx) as isize)
        as *mut TPixel;
    let mut st: libc::c_int = (*src).w;
    let mut dt: libc::c_int = (*dst).w;
    loop {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            let mut r: libc::c_uint = (xr * (*ts.offset(x as isize)).r as libc::c_int
                >> 8 as libc::c_int) as libc::c_uint;
            let mut g: libc::c_uint = (xg * (*ts.offset(x as isize)).g as libc::c_int
                >> 8 as libc::c_int) as libc::c_uint;
            let mut b: libc::c_uint = (xb * (*ts.offset(x as isize)).b as libc::c_int
                >> 8 as libc::c_int) as libc::c_uint;
            let mut a: libc::c_uint = (xa
                * ((*ts.offset(x as isize)).a as libc::c_int
                    + ((*ts.offset(x as isize)).a as libc::c_int > 0 as libc::c_int)
                        as libc::c_int)) as libc::c_uint;
            let ref mut fresh8 = (*td.offset(x as isize)).r;
            *fresh8 = (*fresh8 as libc::c_int
                + (r
                    .wrapping_sub((*td.offset(x as isize)).r as libc::c_uint)
                    .wrapping_mul(a) >> 16 as libc::c_int) as libc::c_uchar
                    as libc::c_int) as libc::c_uchar;
            let ref mut fresh9 = (*td.offset(x as isize)).g;
            *fresh9 = (*fresh9 as libc::c_int
                + (g
                    .wrapping_sub((*td.offset(x as isize)).g as libc::c_uint)
                    .wrapping_mul(a) >> 16 as libc::c_int) as libc::c_uchar
                    as libc::c_int) as libc::c_uchar;
            let ref mut fresh10 = (*td.offset(x as isize)).b;
            *fresh10 = (*fresh10 as libc::c_int
                + (b
                    .wrapping_sub((*td.offset(x as isize)).b as libc::c_uint)
                    .wrapping_mul(a) >> 16 as libc::c_int) as libc::c_uchar
                    as libc::c_int) as libc::c_uchar;
            let ref mut fresh11 = (*td.offset(x as isize)).a;
            *fresh11 = (*fresh11 as libc::c_int
                + (*dst).blitMode
                    * ((((*ts.offset(x as isize)).a as libc::c_int
                        - (*td.offset(x as isize)).a as libc::c_int) as libc::c_uint)
                        .wrapping_mul(a) >> 16 as libc::c_int) as libc::c_uchar
                        as libc::c_int) as libc::c_uchar;
            x += 1;
        }
        ts = ts.offset(st as isize);
        td = td.offset(dt as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrBlitAlpha(
    mut dst: *mut Tigr,
    mut src: *mut Tigr,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut sx: libc::c_int,
    mut sy: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut alpha: libc::c_float,
) {
    alpha = if alpha < 0 as libc::c_int as libc::c_float {
        0 as libc::c_int as libc::c_float
    } else if alpha > 1 as libc::c_int as libc::c_float {
        1 as libc::c_int as libc::c_float
    } else {
        alpha
    };
    tigrBlitTint(
        dst,
        src,
        dx,
        dy,
        sx,
        sy,
        w,
        h,
        tigrRGBA(
            0xff as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            (alpha * 255 as libc::c_int as libc::c_float) as libc::c_uchar,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn tigrBlitMode(mut dst: *mut Tigr, mut mode: libc::c_int) {
    (*dst).blitMode = mode;
}
unsafe extern "C" fn get32(mut v: *const libc::c_uchar) -> libc::c_uint {
    return ((*v.offset(0 as libc::c_int as isize) as libc::c_int) << 24 as libc::c_int
        | (*v.offset(1 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
        | (*v.offset(2 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *v.offset(3 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn find(
    mut png: *mut PNG,
    mut chunk: *const libc::c_char,
    mut minlen: libc::c_uint,
) -> *const libc::c_uchar {
    let mut start: *const libc::c_uchar = 0 as *const libc::c_uchar;
    while (*png).p < (*png).end {
        let mut len: libc::c_uint = get32(((*png).p).offset(0 as libc::c_int as isize));
        start = (*png).p;
        (*png)
            .p = ((*png).p)
            .offset(len.wrapping_add(12 as libc::c_int as libc::c_uint) as isize);
        if memcmp(
            start.offset(4 as libc::c_int as isize) as *const libc::c_void,
            chunk as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int && len >= minlen && (*png).p <= (*png).end
        {
            return start.offset(8 as libc::c_int as isize);
        }
    }
    return 0 as *const libc::c_uchar;
}
unsafe extern "C" fn paeth(
    mut a: libc::c_uchar,
    mut b: libc::c_uchar,
    mut c: libc::c_uchar,
) -> libc::c_uchar {
    let mut p: libc::c_int = a as libc::c_int + b as libc::c_int - c as libc::c_int;
    let mut pa: libc::c_int = abs(p - a as libc::c_int);
    let mut pb: libc::c_int = abs(p - b as libc::c_int);
    let mut pc: libc::c_int = abs(p - c as libc::c_int);
    return (if pa <= pb && pa <= pc {
        a as libc::c_int
    } else if pb <= pc {
        b as libc::c_int
    } else {
        c as libc::c_int
    }) as libc::c_uchar;
}
unsafe extern "C" fn rowBytes(mut w: libc::c_int, mut bipp: libc::c_int) -> libc::c_int {
    let mut rowBits: libc::c_int = w * bipp;
    return rowBits / 8 as libc::c_int
        + (if rowBits % 8 as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        });
}
unsafe extern "C" fn unfilter(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bipp: libc::c_int,
    mut raw: *mut libc::c_uchar,
) -> libc::c_int {
    let mut len: libc::c_int = rowBytes(w, bipp);
    let mut bpp: libc::c_int = rowBytes(1 as libc::c_int, bipp);
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut first: *mut libc::c_uchar = malloc((len + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_uchar;
    memset(
        first as *mut libc::c_void,
        0 as libc::c_int,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    let mut prev: *mut libc::c_uchar = first;
    y = 0 as libc::c_int;
    while y < h {
        let fresh12 = raw;
        raw = raw.offset(1);
        match *fresh12 as libc::c_int {
            0 => {}
            1 => {
                x = 0 as libc::c_int;
                while x < bpp {
                    let ref mut fresh13 = *raw.offset(x as isize);
                    *fresh13 = (*fresh13 as libc::c_int + 0 as libc::c_int)
                        as libc::c_uchar;
                    x += 1;
                }
                while x < len {
                    let ref mut fresh14 = *raw.offset(x as isize);
                    *fresh14 = (*fresh14 as libc::c_int
                        + *raw.offset((x - bpp) as isize) as libc::c_int)
                        as libc::c_uchar;
                    x += 1;
                }
            }
            2 => {
                x = 0 as libc::c_int;
                while x < bpp {
                    let ref mut fresh15 = *raw.offset(x as isize);
                    *fresh15 = (*fresh15 as libc::c_int
                        + *prev.offset(x as isize) as libc::c_int) as libc::c_uchar;
                    x += 1;
                }
                while x < len {
                    let ref mut fresh16 = *raw.offset(x as isize);
                    *fresh16 = (*fresh16 as libc::c_int
                        + *prev.offset(x as isize) as libc::c_int) as libc::c_uchar;
                    x += 1;
                }
            }
            3 => {
                x = 0 as libc::c_int;
                while x < bpp {
                    let ref mut fresh17 = *raw.offset(x as isize);
                    *fresh17 = (*fresh17 as libc::c_int
                        + *prev.offset(x as isize) as libc::c_int / 2 as libc::c_int)
                        as libc::c_uchar;
                    x += 1;
                }
                while x < len {
                    let ref mut fresh18 = *raw.offset(x as isize);
                    *fresh18 = (*fresh18 as libc::c_int
                        + (*raw.offset((x - bpp) as isize) as libc::c_int
                            + *prev.offset(x as isize) as libc::c_int)
                            / 2 as libc::c_int) as libc::c_uchar;
                    x += 1;
                }
            }
            4 => {
                x = 0 as libc::c_int;
                while x < bpp {
                    let ref mut fresh19 = *raw.offset(x as isize);
                    *fresh19 = (*fresh19 as libc::c_int
                        + *prev.offset(x as isize) as libc::c_int) as libc::c_uchar;
                    x += 1;
                }
                while x < len {
                    let ref mut fresh20 = *raw.offset(x as isize);
                    *fresh20 = (*fresh20 as libc::c_int
                        + paeth(
                            *raw.offset((x - bpp) as isize),
                            *prev.offset(x as isize),
                            *prev.offset((x - bpp) as isize),
                        ) as libc::c_int) as libc::c_uchar;
                    x += 1;
                }
            }
            _ => return 0 as libc::c_int,
        }
        y += 1;
        prev = raw;
        raw = raw.offset(len as isize);
    }
    free(first as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn convert(
    mut bypp: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut src: *const libc::c_uchar,
    mut dest: *mut TPixel,
    mut trns: *const libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < h {
        src = src.offset(1);
        x = 0 as libc::c_int;
        while x < w {
            match bypp {
                1 => {
                    let mut c: libc::c_uchar = *src.offset(0 as libc::c_int as isize);
                    if !trns.is_null() && c as libc::c_int == *trns as libc::c_int {
                        let fresh21 = dest;
                        dest = dest.offset(1);
                        *fresh21 = tigrRGBA(c, c, c, 0 as libc::c_int as libc::c_uchar);
                    } else {
                        let fresh22 = dest;
                        dest = dest.offset(1);
                        *fresh22 = tigrRGB(c, c, c);
                    }
                }
                2 => {
                    let fresh23 = dest;
                    dest = dest.offset(1);
                    *fresh23 = tigrRGBA(
                        *src.offset(0 as libc::c_int as isize),
                        *src.offset(0 as libc::c_int as isize),
                        *src.offset(0 as libc::c_int as isize),
                        *src.offset(1 as libc::c_int as isize),
                    );
                }
                3 => {
                    let mut r: libc::c_uchar = *src.offset(0 as libc::c_int as isize);
                    let mut g: libc::c_uchar = *src.offset(1 as libc::c_int as isize);
                    let mut b: libc::c_uchar = *src.offset(2 as libc::c_int as isize);
                    if !trns.is_null()
                        && *trns.offset(1 as libc::c_int as isize) as libc::c_int
                            == r as libc::c_int
                        && *trns.offset(3 as libc::c_int as isize) as libc::c_int
                            == g as libc::c_int
                        && *trns.offset(5 as libc::c_int as isize) as libc::c_int
                            == b as libc::c_int
                    {
                        let fresh24 = dest;
                        dest = dest.offset(1);
                        *fresh24 = tigrRGBA(r, g, b, 0 as libc::c_int as libc::c_uchar);
                    } else {
                        let fresh25 = dest;
                        dest = dest.offset(1);
                        *fresh25 = tigrRGB(r, g, b);
                    }
                }
                4 => {
                    let fresh26 = dest;
                    dest = dest.offset(1);
                    *fresh26 = tigrRGBA(
                        *src.offset(0 as libc::c_int as isize),
                        *src.offset(1 as libc::c_int as isize),
                        *src.offset(2 as libc::c_int as isize),
                        *src.offset(3 as libc::c_int as isize),
                    );
                }
                _ => {}
            }
            x += 1;
            src = src.offset(bypp as isize);
        }
        y += 1;
    }
}
unsafe extern "C" fn depalette(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut src: *mut libc::c_uchar,
    mut dest: *mut TPixel,
    mut bipp: libc::c_int,
    mut plte: *const libc::c_uchar,
    mut trns: *const libc::c_uchar,
    mut trnsSize: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut alpha: libc::c_uchar = 0;
    let mut mask: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    match bipp {
        4 => {
            mask = 15 as libc::c_int;
            len = 1 as libc::c_int;
        }
        2 => {
            mask = 3 as libc::c_int;
            len = 3 as libc::c_int;
        }
        1 => {
            mask = 1 as libc::c_int;
            len = 7 as libc::c_int;
        }
        _ => {}
    }
    y = 0 as libc::c_int;
    while y < h {
        src = src.offset(1);
        x = 0 as libc::c_int;
        while x < w {
            if bipp == 8 as libc::c_int {
                let fresh27 = src;
                src = src.offset(1);
                c = *fresh27 as libc::c_int;
            } else {
                let mut pos: libc::c_int = x & len;
                c = *src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> (len - pos) * bipp & mask;
                if pos == len {
                    src = src.offset(1);
                }
            }
            alpha = 255 as libc::c_int as libc::c_uchar;
            if c < trnsSize {
                alpha = *trns.offset(c as isize);
            }
            let fresh28 = dest;
            dest = dest.offset(1);
            *fresh28 = tigrRGBA(
                *plte.offset((c * 3 as libc::c_int + 0 as libc::c_int) as isize),
                *plte.offset((c * 3 as libc::c_int + 1 as libc::c_int) as isize),
                *plte.offset((c * 3 as libc::c_int + 2 as libc::c_int) as isize),
                alpha,
            );
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn outsize(mut bmp: *mut Tigr, mut bipp: libc::c_int) -> libc::c_int {
    return (rowBytes((*bmp).w, bipp) + 1 as libc::c_int) * (*bmp).h;
}
unsafe extern "C" fn tigrLoadPng(mut png: *mut PNG) -> *mut Tigr {
    let mut current_block: u64;
    let mut ihdr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut idat: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut plte: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut trns: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut first: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut trnsSize: libc::c_int = 0 as libc::c_int;
    let mut depth: libc::c_int = 0;
    let mut ctype: libc::c_int = 0;
    let mut bipp: libc::c_int = 0;
    let mut datalen: libc::c_int = 0 as libc::c_int;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bmp: *mut Tigr = 0 as *mut Tigr;
    if !(memcmp(
        (*png).p as *const libc::c_void,
        b"\x89PNG\r\n\x1A\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int)
    {
        *__errno_location() = 22 as libc::c_int;
    } else {
        (*png).p = ((*png).p).offset(8 as libc::c_int as isize);
        first = (*png).p;
        ihdr = find(
            png,
            b"IHDR\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int as libc::c_uint,
        );
        if ihdr.is_null() {
            *__errno_location() = 22 as libc::c_int;
        } else {
            depth = *ihdr.offset(8 as libc::c_int as isize) as libc::c_int;
            ctype = *ihdr.offset(9 as libc::c_int as isize) as libc::c_int;
            match ctype {
                0 => {
                    bipp = depth;
                    current_block = 224731115979188411;
                }
                2 => {
                    bipp = 3 as libc::c_int * depth;
                    current_block = 224731115979188411;
                }
                3 => {
                    bipp = depth;
                    current_block = 224731115979188411;
                }
                4 => {
                    bipp = 2 as libc::c_int * depth;
                    current_block = 224731115979188411;
                }
                6 => {
                    bipp = 4 as libc::c_int * depth;
                    current_block = 224731115979188411;
                }
                _ => {
                    *__errno_location() = 22 as libc::c_int;
                    current_block = 1177326862408033208;
                }
            }
            match current_block {
                1177326862408033208 => {}
                _ => {
                    bmp = tigrBitmap(
                        (get32(ihdr.offset(0 as libc::c_int as isize)))
                            .wrapping_add(1 as libc::c_int as libc::c_uint)
                            as libc::c_int,
                        get32(ihdr.offset(4 as libc::c_int as isize)) as libc::c_int,
                    );
                    if bmp.is_null() {
                        *__errno_location() = 22 as libc::c_int;
                    } else {
                        (*bmp).w -= 1;
                        if !(depth != 16 as libc::c_int
                            && *ihdr.offset(10 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                            && *ihdr.offset(11 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                            && *ihdr.offset(12 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int)
                        {
                            *__errno_location() = 22 as libc::c_int;
                        } else {
                            idat = find(
                                png,
                                b"IDAT\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int as libc::c_uint,
                            );
                            while !idat.is_null() {
                                let mut len: libc::c_uint = get32(
                                    idat.offset(-(8 as libc::c_int as isize)),
                                );
                                data = realloc(
                                    data as *mut libc::c_void,
                                    (datalen as libc::c_uint).wrapping_add(len) as libc::c_ulong,
                                ) as *mut libc::c_uchar;
                                if data.is_null() {
                                    break;
                                }
                                memcpy(
                                    data.offset(datalen as isize) as *mut libc::c_void,
                                    idat as *const libc::c_void,
                                    len as libc::c_ulong,
                                );
                                datalen = (datalen as libc::c_uint).wrapping_add(len)
                                    as libc::c_int as libc::c_int;
                                idat = find(
                                    png,
                                    b"IDAT\0" as *const u8 as *const libc::c_char,
                                    0 as libc::c_int as libc::c_uint,
                                );
                            }
                            (*png).p = first;
                            plte = find(
                                png,
                                b"PLTE\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int as libc::c_uint,
                            );
                            (*png).p = first;
                            trns = find(
                                png,
                                b"tRNS\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int as libc::c_uint,
                            );
                            if !trns.is_null() {
                                trnsSize = get32(trns.offset(-(8 as libc::c_int as isize)))
                                    as libc::c_int;
                            }
                            if !(!data.is_null() && datalen >= 6 as libc::c_int) {
                                *__errno_location() = 22 as libc::c_int;
                            } else if !(*data.offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf as libc::c_int == 0x8 as libc::c_int
                                    && *data.offset(0 as libc::c_int as isize) as libc::c_int
                                        & 0xf0 as libc::c_int <= 0x70 as libc::c_int
                                    && *data.offset(1 as libc::c_int as isize) as libc::c_int
                                        & 0x20 as libc::c_int == 0 as libc::c_int)
                                {
                                *__errno_location() = 22 as libc::c_int;
                            } else {
                                out = ((*bmp).pix as *mut libc::c_uchar)
                                    .offset(outsize(bmp, 32 as libc::c_int) as isize)
                                    .offset(-(outsize(bmp, bipp) as isize));
                                if tigrInflate(
                                    out as *mut libc::c_void,
                                    outsize(bmp, bipp) as libc::c_uint,
                                    data.offset(2 as libc::c_int as isize)
                                        as *const libc::c_void,
                                    (datalen - 6 as libc::c_int) as libc::c_uint,
                                ) == 0
                                {
                                    *__errno_location() = 22 as libc::c_int;
                                } else if unfilter((*bmp).w, (*bmp).h, bipp, out) == 0 {
                                    *__errno_location() = 22 as libc::c_int;
                                } else {
                                    if ctype == 3 as libc::c_int {
                                        if plte.is_null() {
                                            *__errno_location() = 22 as libc::c_int;
                                            current_block = 1177326862408033208;
                                        } else {
                                            depalette(
                                                (*bmp).w,
                                                (*bmp).h,
                                                out,
                                                (*bmp).pix,
                                                bipp,
                                                plte,
                                                trns,
                                                trnsSize,
                                            );
                                            current_block = 6528285054092551010;
                                        }
                                    } else if !(bipp % 8 as libc::c_int == 0 as libc::c_int) {
                                        *__errno_location() = 22 as libc::c_int;
                                        current_block = 1177326862408033208;
                                    } else {
                                        convert(
                                            bipp / 8 as libc::c_int,
                                            (*bmp).w,
                                            (*bmp).h,
                                            out,
                                            (*bmp).pix,
                                            trns,
                                        );
                                        current_block = 6528285054092551010;
                                    }
                                    match current_block {
                                        1177326862408033208 => {}
                                        _ => {
                                            free(data as *mut libc::c_void);
                                            return bmp;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !data.is_null() {
        free(data as *mut libc::c_void);
    }
    if !bmp.is_null() {
        tigrFree(bmp);
    }
    return 0 as *mut Tigr;
}
#[no_mangle]
pub unsafe extern "C" fn tigrLoadImageMem(
    mut data: *const libc::c_void,
    mut length: libc::c_int,
) -> *mut Tigr {
    let mut png: PNG = PNG {
        p: 0 as *const libc::c_uchar,
        end: 0 as *const libc::c_uchar,
    };
    png.p = data as *mut libc::c_uchar;
    png.end = (data as *mut libc::c_uchar).offset(length as isize);
    return tigrLoadPng(&mut png);
}
#[no_mangle]
pub unsafe extern "C" fn tigrLoadImage(mut fileName: *const libc::c_char) -> *mut Tigr {
    let mut len: libc::c_int = 0;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bmp: *mut Tigr = 0 as *mut Tigr;
    data = tigrReadFile(fileName, &mut len);
    if data.is_null() {
        return 0 as *mut Tigr;
    }
    bmp = tigrLoadImageMem(data, len);
    free(data);
    return bmp;
}
static mut crctable: [libc::c_uint; 16] = [
    0 as libc::c_int as libc::c_uint,
    0x1db71064 as libc::c_int as libc::c_uint,
    0x3b6e20c8 as libc::c_int as libc::c_uint,
    0x26d930ac as libc::c_int as libc::c_uint,
    0x76dc4190 as libc::c_int as libc::c_uint,
    0x6b6b51f4 as libc::c_int as libc::c_uint,
    0x4db26158 as libc::c_int as libc::c_uint,
    0x5005713c as libc::c_int as libc::c_uint,
    0xedb88320 as libc::c_uint,
    0xf00f9344 as libc::c_uint,
    0xd6d6a3e8 as libc::c_uint,
    0xcb61b38c as libc::c_uint,
    0x9b64c2b0 as libc::c_uint,
    0x86d3d2d4 as libc::c_uint,
    0xa00ae278 as libc::c_uint,
    0xbdbdf21c as libc::c_uint,
];
unsafe extern "C" fn put(mut s: *mut Save, mut v: libc::c_uint) {
    fputc(v as libc::c_int, (*s).out);
    (*s)
        .crc = (*s).crc >> 4 as libc::c_int
        ^ crctable[((*s).crc & 15 as libc::c_int as libc::c_uint
            ^ v & 15 as libc::c_int as libc::c_uint) as usize];
    (*s)
        .crc = (*s).crc >> 4 as libc::c_int
        ^ crctable[((*s).crc & 15 as libc::c_int as libc::c_uint ^ v >> 4 as libc::c_int)
            as usize];
}
unsafe extern "C" fn updateAdler(mut s: *mut Save, mut v: libc::c_uint) {
    let mut s1: libc::c_uint = (*s).adler & 0xffff as libc::c_int as libc::c_uint;
    let mut s2: libc::c_uint = (*s).adler >> 16 as libc::c_int
        & 0xffff as libc::c_int as libc::c_uint;
    s1 = s1.wrapping_add(v).wrapping_rem(65521 as libc::c_int as libc::c_uint);
    s2 = s2.wrapping_add(s1).wrapping_rem(65521 as libc::c_int as libc::c_uint);
    (*s).adler = (s2 << 16 as libc::c_int).wrapping_add(s1);
}
unsafe extern "C" fn put32(mut s: *mut Save, mut v: libc::c_uint) {
    put(s, v >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint);
    put(s, v >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint);
    put(s, v >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint);
    put(s, v & 0xff as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn putbits(
    mut s: *mut Save,
    mut data: libc::c_uint,
    mut bitcount: libc::c_uint,
) {
    loop {
        let fresh29 = bitcount;
        bitcount = bitcount.wrapping_sub(1);
        if !(fresh29 != 0) {
            break;
        }
        let mut prev: libc::c_uint = (*s).bits;
        (*s)
            .bits = (*s).bits >> 1 as libc::c_int
            | (data & 1 as libc::c_int as libc::c_uint) << 7 as libc::c_int;
        data >>= 1 as libc::c_int;
        if prev & 1 as libc::c_int as libc::c_uint != 0 {
            put(s, (*s).bits);
            (*s).bits = 0x80 as libc::c_int as libc::c_uint;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn putbitsr(
    mut s: *mut Save,
    mut data: libc::c_uint,
    mut bitcount: libc::c_uint,
) {
    loop {
        let fresh30 = bitcount;
        bitcount = bitcount.wrapping_sub(1);
        if !(fresh30 != 0) {
            break;
        }
        putbits(s, data >> bitcount, 1 as libc::c_int as libc::c_uint);
    };
}
unsafe extern "C" fn begin(
    mut s: *mut Save,
    mut id: *const libc::c_char,
    mut len: libc::c_uint,
) {
    put32(s, len);
    (*s).crc = 0xffffffff as libc::c_uint;
    put(s, *id.offset(0 as libc::c_int as isize) as libc::c_uint);
    put(s, *id.offset(1 as libc::c_int as isize) as libc::c_uint);
    put(s, *id.offset(2 as libc::c_int as isize) as libc::c_uint);
    put(s, *id.offset(3 as libc::c_int as isize) as libc::c_uint);
}
unsafe extern "C" fn literal(mut s: *mut Save, mut v: libc::c_uint) {
    if v < 144 as libc::c_int as libc::c_uint {
        putbitsr(
            s,
            (0x30 as libc::c_int as libc::c_uint)
                .wrapping_add(v)
                .wrapping_sub(0 as libc::c_int as libc::c_uint),
            8 as libc::c_int as libc::c_uint,
        );
    } else if v < 256 as libc::c_int as libc::c_uint {
        putbitsr(
            s,
            (0x190 as libc::c_int as libc::c_uint)
                .wrapping_add(v)
                .wrapping_sub(144 as libc::c_int as libc::c_uint),
            9 as libc::c_int as libc::c_uint,
        );
    } else if v < 280 as libc::c_int as libc::c_uint {
        putbitsr(
            s,
            (0 as libc::c_int as libc::c_uint)
                .wrapping_add(v)
                .wrapping_sub(256 as libc::c_int as libc::c_uint),
            7 as libc::c_int as libc::c_uint,
        );
    } else {
        putbitsr(
            s,
            (0xc0 as libc::c_int as libc::c_uint)
                .wrapping_add(v)
                .wrapping_sub(280 as libc::c_int as libc::c_uint),
            8 as libc::c_int as libc::c_uint,
        );
    };
}
unsafe extern "C" fn encodelen(
    mut s: *mut Save,
    mut code: libc::c_uint,
    mut bits_0: libc::c_uint,
    mut len: libc::c_uint,
) {
    literal(s, code.wrapping_add(len >> bits_0));
    putbits(s, len, bits_0);
    putbits(s, 0 as libc::c_int as libc::c_uint, 5 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn endrun(mut s: *mut Save) {
    (*s).runlen = ((*s).runlen).wrapping_sub(1);
    literal(s, (*s).prev);
    if (*s).runlen >= 67 as libc::c_int as libc::c_uint {
        encodelen(
            s,
            277 as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
            ((*s).runlen).wrapping_sub(67 as libc::c_int as libc::c_uint),
        );
    } else if (*s).runlen >= 35 as libc::c_int as libc::c_uint {
        encodelen(
            s,
            273 as libc::c_int as libc::c_uint,
            3 as libc::c_int as libc::c_uint,
            ((*s).runlen).wrapping_sub(35 as libc::c_int as libc::c_uint),
        );
    } else if (*s).runlen >= 19 as libc::c_int as libc::c_uint {
        encodelen(
            s,
            269 as libc::c_int as libc::c_uint,
            2 as libc::c_int as libc::c_uint,
            ((*s).runlen).wrapping_sub(19 as libc::c_int as libc::c_uint),
        );
    } else if (*s).runlen >= 11 as libc::c_int as libc::c_uint {
        encodelen(
            s,
            265 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
            ((*s).runlen).wrapping_sub(11 as libc::c_int as libc::c_uint),
        );
    } else if (*s).runlen >= 3 as libc::c_int as libc::c_uint {
        encodelen(
            s,
            257 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
            ((*s).runlen).wrapping_sub(3 as libc::c_int as libc::c_uint),
        );
    } else {
        loop {
            let fresh31 = (*s).runlen;
            (*s).runlen = ((*s).runlen).wrapping_sub(1);
            if !(fresh31 != 0) {
                break;
            }
            literal(s, (*s).prev);
        }
    };
}
unsafe extern "C" fn encodeByte(mut s: *mut Save, mut v: libc::c_uchar) {
    updateAdler(s, v as libc::c_uint);
    if (*s).prev == v as libc::c_uint && (*s).runlen < 115 as libc::c_int as libc::c_uint
    {
        (*s).runlen = ((*s).runlen).wrapping_add(1);
    } else {
        if (*s).runlen != 0 {
            endrun(s);
        }
        (*s).prev = v as libc::c_uint;
        (*s).runlen = 1 as libc::c_int as libc::c_uint;
    };
}
unsafe extern "C" fn savePngHeader(mut s: *mut Save, mut bmp: *mut Tigr) {
    fwrite(
        b"\x89PNG\r\n\x1A\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*s).out,
    );
    begin(
        s,
        b"IHDR\0" as *const u8 as *const libc::c_char,
        13 as libc::c_int as libc::c_uint,
    );
    put32(s, (*bmp).w as libc::c_uint);
    put32(s, (*bmp).h as libc::c_uint);
    put(s, 8 as libc::c_int as libc::c_uint);
    put(s, 6 as libc::c_int as libc::c_uint);
    put(s, 0 as libc::c_int as libc::c_uint);
    put(s, 0 as libc::c_int as libc::c_uint);
    put(s, 0 as libc::c_int as libc::c_uint);
    put32(s, !(*s).crc);
}
unsafe extern "C" fn savePngData(
    mut s: *mut Save,
    mut bmp: *mut Tigr,
    mut dataPos: libc::c_long,
) -> libc::c_long {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dataSize: libc::c_long = 0;
    begin(
        s,
        b"IDAT\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_uint,
    );
    put(s, 0x8 as libc::c_int as libc::c_uint);
    put(s, 0x1d as libc::c_int as libc::c_uint);
    putbits(s, 3 as libc::c_int as libc::c_uint, 3 as libc::c_int as libc::c_uint);
    y = 0 as libc::c_int;
    while y < (*bmp).h {
        let mut row: *mut TPixel = &mut *((*bmp).pix).offset((y * (*bmp).w) as isize)
            as *mut TPixel;
        let mut prev: TPixel = tigrRGBA(
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        );
        encodeByte(s, 1 as libc::c_int as libc::c_uchar);
        x = 0 as libc::c_int;
        while x < (*bmp).w {
            encodeByte(
                s,
                ((*row.offset(x as isize)).r as libc::c_int - prev.r as libc::c_int)
                    as libc::c_uchar,
            );
            encodeByte(
                s,
                ((*row.offset(x as isize)).g as libc::c_int - prev.g as libc::c_int)
                    as libc::c_uchar,
            );
            encodeByte(
                s,
                ((*row.offset(x as isize)).b as libc::c_int - prev.b as libc::c_int)
                    as libc::c_uchar,
            );
            encodeByte(
                s,
                ((*row.offset(x as isize)).a as libc::c_int - prev.a as libc::c_int)
                    as libc::c_uchar,
            );
            prev = *row.offset(x as isize);
            x += 1;
        }
        y += 1;
    }
    endrun(s);
    literal(s, 256 as libc::c_int as libc::c_uint);
    while (*s).bits != 0x80 as libc::c_int as libc::c_uint {
        putbits(s, 0 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint);
    }
    put32(s, (*s).adler);
    dataSize = ftell((*s).out) - dataPos - 8 as libc::c_int as libc::c_long;
    put32(s, !(*s).crc);
    return dataSize;
}
#[no_mangle]
pub unsafe extern "C" fn tigrSaveImage(
    mut fileName: *const libc::c_char,
    mut bmp: *mut Tigr,
) -> libc::c_int {
    let mut s: Save = Save {
        crc: 0,
        adler: 0,
        bits: 0,
        prev: 0,
        runlen: 0,
        out: 0 as *mut FILE,
        crcTable: [0; 256],
    };
    let mut dataPos: libc::c_long = 0;
    let mut dataSize: libc::c_long = 0;
    let mut err: libc::c_long = 0;
    let mut out: *mut FILE = fopen(
        fileName,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if out.is_null() {
        return 1 as libc::c_int;
    }
    s.out = out;
    s.adler = 1 as libc::c_int as libc::c_uint;
    s.bits = 0x80 as libc::c_int as libc::c_uint;
    s.prev = 0xffff as libc::c_int as libc::c_uint;
    s.runlen = 0 as libc::c_int as libc::c_uint;
    savePngHeader(&mut s, bmp);
    dataPos = ftell(s.out);
    dataSize = savePngData(&mut s, bmp, dataPos);
    begin(
        &mut s,
        b"IEND\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_uint,
    );
    put32(&mut s, !s.crc);
    fseek(out, dataPos, 0 as libc::c_int);
    put32(&mut s, dataSize as libc::c_uint);
    err = ferror(out) as libc::c_long;
    fclose(out);
    return (err == 0) as libc::c_int;
}
static mut order: [libc::c_char; 19] = [
    16 as libc::c_int as libc::c_char,
    17 as libc::c_int as libc::c_char,
    18 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
];
static mut lenBits: [libc::c_char; 31] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
static mut lenBase: [libc::c_int; 31] = [
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    13 as libc::c_int,
    15 as libc::c_int,
    17 as libc::c_int,
    19 as libc::c_int,
    23 as libc::c_int,
    27 as libc::c_int,
    31 as libc::c_int,
    35 as libc::c_int,
    43 as libc::c_int,
    51 as libc::c_int,
    59 as libc::c_int,
    67 as libc::c_int,
    83 as libc::c_int,
    99 as libc::c_int,
    115 as libc::c_int,
    131 as libc::c_int,
    163 as libc::c_int,
    195 as libc::c_int,
    227 as libc::c_int,
    258 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
static mut distBits: [libc::c_char; 32] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
static mut distBase: [libc::c_int; 32] = [
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    7 as libc::c_int,
    9 as libc::c_int,
    13 as libc::c_int,
    17 as libc::c_int,
    25 as libc::c_int,
    33 as libc::c_int,
    49 as libc::c_int,
    65 as libc::c_int,
    97 as libc::c_int,
    129 as libc::c_int,
    193 as libc::c_int,
    257 as libc::c_int,
    385 as libc::c_int,
    513 as libc::c_int,
    769 as libc::c_int,
    1025 as libc::c_int,
    1537 as libc::c_int,
    2049 as libc::c_int,
    3073 as libc::c_int,
    4097 as libc::c_int,
    6145 as libc::c_int,
    8193 as libc::c_int,
    12289 as libc::c_int,
    16385 as libc::c_int,
    24577 as libc::c_int,
    0,
    0,
];
static mut reverseTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    (0 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (0 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    (2 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (2 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    (1 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (1 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    (3 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 8 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 4 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 128 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 64 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 192 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 32 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 16 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int) as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 128 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 64 as libc::c_int)
        as libc::c_uchar,
    (3 as libc::c_int + 12 as libc::c_int + 48 as libc::c_int + 192 as libc::c_int)
        as libc::c_uchar,
];
unsafe extern "C" fn rev16(mut n: libc::c_uint) -> libc::c_uint {
    return ((reverseTable[(n & 0xff as libc::c_int as libc::c_uint) as usize]
        as libc::c_int) << 8 as libc::c_int
        | reverseTable[(n >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as usize] as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn bits(mut s: *mut State, mut n: libc::c_int) -> libc::c_int {
    let mut v: libc::c_int = ((*s).bits
        & (((1 as libc::c_int) << n) - 1 as libc::c_int) as libc::c_uint) as libc::c_int;
    (*s).bits >>= n;
    (*s).count = ((*s).count).wrapping_sub(n as libc::c_uint);
    while (*s).count < 16 as libc::c_int as libc::c_uint {
        if !((*s).in_0 != (*s).inend) {
            longjmp(((*s).jmp).as_mut_ptr(), 1 as libc::c_int);
        }
        let fresh32 = (*s).in_0;
        (*s).in_0 = ((*s).in_0).offset(1);
        (*s).bits |= ((*fresh32 as libc::c_int) << (*s).count) as libc::c_uint;
        (*s).count = ((*s).count).wrapping_add(8 as libc::c_int as libc::c_uint);
    }
    return v;
}
unsafe extern "C" fn emit(
    mut s: *mut State,
    mut len: libc::c_int,
) -> *mut libc::c_uchar {
    (*s).out = ((*s).out).offset(len as isize);
    if !((*s).out <= (*s).outend) {
        longjmp(((*s).jmp).as_mut_ptr(), 1 as libc::c_int);
    }
    return ((*s).out).offset(-(len as isize));
}
unsafe extern "C" fn copy(
    mut s: *mut State,
    mut src: *const libc::c_uchar,
    mut len: libc::c_int,
) {
    let mut dest: *mut libc::c_uchar = emit(s, len);
    loop {
        let fresh33 = len;
        len = len - 1;
        if !(fresh33 != 0) {
            break;
        }
        let fresh34 = src;
        src = src.offset(1);
        let fresh35 = dest;
        dest = dest.offset(1);
        *fresh35 = *fresh34;
    };
}
unsafe extern "C" fn build(
    mut s: *mut State,
    mut tree: *mut libc::c_uint,
    mut lens: *mut libc::c_uchar,
    mut symcount: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut codes: [libc::c_int; 16] = [0; 16];
    let mut first: [libc::c_int; 16] = [0; 16];
    let mut counts: [libc::c_int; 16] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    n = 0 as libc::c_int;
    while n < symcount {
        counts[*lens.offset(n as isize) as usize] += 1;
        n += 1;
    }
    first[0 as libc::c_int as usize] = 0 as libc::c_int;
    codes[0 as libc::c_int as usize] = first[0 as libc::c_int as usize];
    counts[0 as libc::c_int as usize] = codes[0 as libc::c_int as usize];
    n = 1 as libc::c_int;
    while n <= 15 as libc::c_int {
        codes[n
            as usize] = codes[(n - 1 as libc::c_int) as usize]
            + counts[(n - 1 as libc::c_int) as usize] << 1 as libc::c_int;
        first[n
            as usize] = first[(n - 1 as libc::c_int) as usize]
            + counts[(n - 1 as libc::c_int) as usize];
        n += 1;
    }
    if !(first[15 as libc::c_int as usize] + counts[15 as libc::c_int as usize]
        <= symcount)
    {
        longjmp(((*s).jmp).as_mut_ptr(), 1 as libc::c_int);
    }
    n = 0 as libc::c_int;
    while n < symcount {
        let mut len: libc::c_int = *lens.offset(n as isize) as libc::c_int;
        if len != 0 as libc::c_int {
            let fresh36 = codes[len as usize];
            codes[len as usize] = codes[len as usize] + 1;
            let mut code: libc::c_int = fresh36;
            let fresh37 = first[len as usize];
            first[len as usize] = first[len as usize] + 1;
            let mut slot: libc::c_int = fresh37;
            *tree
                .offset(
                    slot as isize,
                ) = (code << 32 as libc::c_int - len | n << 4 as libc::c_int | len)
                as libc::c_uint;
        }
        n += 1;
    }
    return first[15 as libc::c_int as usize];
}
unsafe extern "C" fn decode(
    mut s: *mut State,
    mut tree: *mut libc::c_uint,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut lo: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hi: libc::c_uint = max as libc::c_uint;
    let mut key: libc::c_uint = 0;
    let mut search: libc::c_uint = rev16((*s).bits) << 16 as libc::c_int
        | 0xffff as libc::c_int as libc::c_uint;
    while lo < hi {
        let mut guess: libc::c_uint = lo
            .wrapping_add(hi)
            .wrapping_div(2 as libc::c_int as libc::c_uint);
        if search < *tree.offset(guess as isize) {
            hi = guess;
        } else {
            lo = guess.wrapping_add(1 as libc::c_int as libc::c_uint);
        }
    }
    key = *tree.offset(lo.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    if !((search ^ key)
        >> (32 as libc::c_int as libc::c_uint)
            .wrapping_sub(key & 0xf as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint)
    {
        longjmp(((*s).jmp).as_mut_ptr(), 1 as libc::c_int);
    }
    bits(s, (key & 0xf as libc::c_int as libc::c_uint) as libc::c_int);
    return (key >> 4 as libc::c_int & 0xfff as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn run(mut s: *mut State, mut sym: libc::c_int) {
    let mut length: libc::c_int = bits(s, lenBits[sym as usize] as libc::c_int)
        + lenBase[sym as usize];
    let mut dsym: libc::c_int = decode(s, ((*s).distcodes).as_mut_ptr(), (*s).tdist);
    let mut offs: libc::c_int = bits(s, distBits[dsym as usize] as libc::c_int)
        + distBase[dsym as usize];
    copy(s, ((*s).out).offset(-(offs as isize)), length);
}
unsafe extern "C" fn block(mut s: *mut State) {
    loop {
        let mut sym: libc::c_int = decode(s, ((*s).litcodes).as_mut_ptr(), (*s).tlit);
        if sym < 256 as libc::c_int {
            *emit(s, 1 as libc::c_int) = sym as libc::c_uchar;
        } else {
            if !(sym > 256 as libc::c_int) {
                break;
            }
            run(s, sym - 257 as libc::c_int);
        }
    };
}
unsafe extern "C" fn stored(mut s: *mut State) {
    let mut len: libc::c_int = 0;
    bits(s, ((*s).count & 7 as libc::c_int as libc::c_uint) as libc::c_int);
    len = bits(s, 16 as libc::c_int);
    if !((len as libc::c_uint ^ (*s).bits) & 0xffff as libc::c_int as libc::c_uint
        == 0xffff as libc::c_int as libc::c_uint)
    {
        longjmp(((*s).jmp).as_mut_ptr(), 1 as libc::c_int);
    }
    if !(((*s).in_0).offset(len as isize) <= (*s).inend) {
        longjmp(((*s).jmp).as_mut_ptr(), 1 as libc::c_int);
    }
    copy(s, (*s).in_0, len);
    (*s).in_0 = ((*s).in_0).offset(len as isize);
    bits(s, 16 as libc::c_int);
}
unsafe extern "C" fn fixed(mut s: *mut State) {
    let mut n: libc::c_int = 0;
    let mut lens: [libc::c_uchar; 320] = [0; 320];
    n = 0 as libc::c_int;
    while n <= 143 as libc::c_int {
        lens[n as usize] = 8 as libc::c_int as libc::c_uchar;
        n += 1;
    }
    n = 144 as libc::c_int;
    while n <= 255 as libc::c_int {
        lens[n as usize] = 9 as libc::c_int as libc::c_uchar;
        n += 1;
    }
    n = 256 as libc::c_int;
    while n <= 279 as libc::c_int {
        lens[n as usize] = 7 as libc::c_int as libc::c_uchar;
        n += 1;
    }
    n = 280 as libc::c_int;
    while n <= 287 as libc::c_int {
        lens[n as usize] = 8 as libc::c_int as libc::c_uchar;
        n += 1;
    }
    n = 0 as libc::c_int;
    while n < 32 as libc::c_int {
        lens[(288 as libc::c_int + n) as usize] = 5 as libc::c_int as libc::c_uchar;
        n += 1;
    }
    (*s)
        .tlit = build(
        s,
        ((*s).litcodes).as_mut_ptr(),
        lens.as_mut_ptr(),
        288 as libc::c_int,
    );
    (*s)
        .tdist = build(
        s,
        ((*s).distcodes).as_mut_ptr(),
        lens.as_mut_ptr().offset(288 as libc::c_int as isize),
        32 as libc::c_int,
    );
}
unsafe extern "C" fn dynamic(mut s: *mut State) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nlit: libc::c_int = 0;
    let mut ndist: libc::c_int = 0;
    let mut nlen: libc::c_int = 0;
    let mut lenlens: [libc::c_uchar; 19] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut lens: [libc::c_uchar; 320] = [0; 320];
    nlit = 257 as libc::c_int + bits(s, 5 as libc::c_int);
    ndist = 1 as libc::c_int + bits(s, 5 as libc::c_int);
    nlen = 4 as libc::c_int + bits(s, 4 as libc::c_int);
    n = 0 as libc::c_int;
    while n < nlen {
        lenlens[order[n as usize] as usize] = bits(s, 3 as libc::c_int) as libc::c_uchar;
        n += 1;
    }
    (*s)
        .tlen = build(
        s,
        ((*s).lencodes).as_mut_ptr(),
        lenlens.as_mut_ptr(),
        19 as libc::c_int,
    );
    n = 0 as libc::c_int;
    while n < nlit + ndist {
        let mut sym: libc::c_int = decode(s, ((*s).lencodes).as_mut_ptr(), (*s).tlen);
        match sym {
            16 => {
                i = 3 as libc::c_int + bits(s, 2 as libc::c_int);
                while i != 0 {
                    lens[n as usize] = lens[(n - 1 as libc::c_int) as usize];
                    i -= 1;
                    n += 1;
                }
            }
            17 => {
                i = 3 as libc::c_int + bits(s, 3 as libc::c_int);
                while i != 0 {
                    lens[n as usize] = 0 as libc::c_int as libc::c_uchar;
                    i -= 1;
                    n += 1;
                }
            }
            18 => {
                i = 11 as libc::c_int + bits(s, 7 as libc::c_int);
                while i != 0 {
                    lens[n as usize] = 0 as libc::c_int as libc::c_uchar;
                    i -= 1;
                    n += 1;
                }
            }
            _ => {
                let fresh38 = n;
                n = n + 1;
                lens[fresh38 as usize] = sym as libc::c_uchar;
            }
        }
    }
    (*s).tlit = build(s, ((*s).litcodes).as_mut_ptr(), lens.as_mut_ptr(), nlit);
    (*s)
        .tdist = build(
        s,
        ((*s).distcodes).as_mut_ptr(),
        lens.as_mut_ptr().offset(nlit as isize),
        ndist,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tigrInflate(
    mut out: *mut libc::c_void,
    mut outlen: libc::c_uint,
    mut in_0: *const libc::c_void,
    mut inlen: libc::c_uint,
) -> libc::c_int {
    let mut last: libc::c_int = 0;
    let mut s: *mut State = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<State>() as libc::c_ulong,
    ) as *mut State;
    (*s).in_0 = in_0 as *mut libc::c_uchar;
    (*s).inend = ((*s).in_0).offset(inlen as isize).offset(2 as libc::c_int as isize);
    (*s).out = out as *mut libc::c_uchar;
    (*s).outend = ((*s).out).offset(outlen as isize);
    (*s).bits = 0 as libc::c_int as libc::c_uint;
    (*s).count = 0 as libc::c_int as libc::c_uint;
    bits(s, 0 as libc::c_int);
    if _setjmp(((*s).jmp).as_mut_ptr()) == 1 as libc::c_int {
        free(s as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    loop {
        last = bits(s, 1 as libc::c_int);
        match bits(s, 2 as libc::c_int) {
            0 => {
                stored(s);
            }
            1 => {
                fixed(s);
                block(s);
            }
            2 => {
                dynamic(s);
                block(s);
            }
            3 => {
                longjmp(((*s).jmp).as_mut_ptr(), 1 as libc::c_int);
            }
            _ => {}
        }
        if !(last == 0) {
            break;
        }
    }
    free(s as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut tigr_font: [libc::c_uchar; 3843] = [
    0x89 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut tigr_font_size: libc::c_int = ::core::mem::size_of::<[libc::c_uchar; 3843]>() as libc::c_ulong as libc::c_int;
#[no_mangle]
pub static mut tigrStockFont: TigrFont = TigrFont {
    bitmap: 0 as *const Tigr as *mut Tigr,
    numGlyphs: 0,
    glyphs: 0 as *const TigrGlyph as *mut TigrGlyph,
};
#[no_mangle]
pub static mut tfont: *mut TigrFont = unsafe {
    &tigrStockFont as *const TigrFont as *mut TigrFont
};
static mut cp1252: [libc::c_int; 128] = [
    0x20ac as libc::c_int,
    0xfffd as libc::c_int,
    0x201a as libc::c_int,
    0x192 as libc::c_int,
    0x201e as libc::c_int,
    0x2026 as libc::c_int,
    0x2020 as libc::c_int,
    0x2021 as libc::c_int,
    0x2c6 as libc::c_int,
    0x2030 as libc::c_int,
    0x160 as libc::c_int,
    0x2039 as libc::c_int,
    0x152 as libc::c_int,
    0xfffd as libc::c_int,
    0x17d as libc::c_int,
    0xfffd as libc::c_int,
    0xfffd as libc::c_int,
    0x2018 as libc::c_int,
    0x2019 as libc::c_int,
    0x201c as libc::c_int,
    0x201d as libc::c_int,
    0x2022 as libc::c_int,
    0x2013 as libc::c_int,
    0x2014 as libc::c_int,
    0x2dc as libc::c_int,
    0x2122 as libc::c_int,
    0x161 as libc::c_int,
    0x203a as libc::c_int,
    0x153 as libc::c_int,
    0xfffd as libc::c_int,
    0x17e as libc::c_int,
    0x178 as libc::c_int,
    0xa0 as libc::c_int,
    0xa1 as libc::c_int,
    0xa2 as libc::c_int,
    0xa3 as libc::c_int,
    0xa4 as libc::c_int,
    0xa5 as libc::c_int,
    0xa6 as libc::c_int,
    0xa7 as libc::c_int,
    0xa8 as libc::c_int,
    0xa9 as libc::c_int,
    0xaa as libc::c_int,
    0xab as libc::c_int,
    0xac as libc::c_int,
    0xad as libc::c_int,
    0xae as libc::c_int,
    0xaf as libc::c_int,
    0xb0 as libc::c_int,
    0xb1 as libc::c_int,
    0xb2 as libc::c_int,
    0xb3 as libc::c_int,
    0xb4 as libc::c_int,
    0xb5 as libc::c_int,
    0xb6 as libc::c_int,
    0xb7 as libc::c_int,
    0xb8 as libc::c_int,
    0xb9 as libc::c_int,
    0xba as libc::c_int,
    0xbb as libc::c_int,
    0xbc as libc::c_int,
    0xbd as libc::c_int,
    0xbe as libc::c_int,
    0xbf as libc::c_int,
    0xc0 as libc::c_int,
    0xc1 as libc::c_int,
    0xc2 as libc::c_int,
    0xc3 as libc::c_int,
    0xc4 as libc::c_int,
    0xc5 as libc::c_int,
    0xc6 as libc::c_int,
    0xc7 as libc::c_int,
    0xc8 as libc::c_int,
    0xc9 as libc::c_int,
    0xca as libc::c_int,
    0xcb as libc::c_int,
    0xcc as libc::c_int,
    0xcd as libc::c_int,
    0xce as libc::c_int,
    0xcf as libc::c_int,
    0xd0 as libc::c_int,
    0xd1 as libc::c_int,
    0xd2 as libc::c_int,
    0xd3 as libc::c_int,
    0xd4 as libc::c_int,
    0xd5 as libc::c_int,
    0xd6 as libc::c_int,
    0xd7 as libc::c_int,
    0xd8 as libc::c_int,
    0xd9 as libc::c_int,
    0xda as libc::c_int,
    0xdb as libc::c_int,
    0xdc as libc::c_int,
    0xdd as libc::c_int,
    0xde as libc::c_int,
    0xdf as libc::c_int,
    0xe0 as libc::c_int,
    0xe1 as libc::c_int,
    0xe2 as libc::c_int,
    0xe3 as libc::c_int,
    0xe4 as libc::c_int,
    0xe5 as libc::c_int,
    0xe6 as libc::c_int,
    0xe7 as libc::c_int,
    0xe8 as libc::c_int,
    0xe9 as libc::c_int,
    0xea as libc::c_int,
    0xeb as libc::c_int,
    0xec as libc::c_int,
    0xed as libc::c_int,
    0xee as libc::c_int,
    0xef as libc::c_int,
    0xf0 as libc::c_int,
    0xf1 as libc::c_int,
    0xf2 as libc::c_int,
    0xf3 as libc::c_int,
    0xf4 as libc::c_int,
    0xf5 as libc::c_int,
    0xf6 as libc::c_int,
    0xf7 as libc::c_int,
    0xf8 as libc::c_int,
    0xf9 as libc::c_int,
    0xfa as libc::c_int,
    0xfb as libc::c_int,
    0xfc as libc::c_int,
    0xfd as libc::c_int,
    0xfe as libc::c_int,
    0xff as libc::c_int,
];
unsafe extern "C" fn border(
    mut bmp: *mut Tigr,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    let mut top: TPixel = tigrGet(bmp, 0 as libc::c_int, 0 as libc::c_int);
    let mut c: TPixel = tigrGet(bmp, x, y);
    return (c.r as libc::c_int == top.r as libc::c_int
        && c.g as libc::c_int == top.g as libc::c_int
        && c.b as libc::c_int == top.b as libc::c_int || x >= (*bmp).w || y >= (*bmp).h)
        as libc::c_int;
}
unsafe extern "C" fn scan(
    mut bmp: *mut Tigr,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut rowh: *mut libc::c_int,
) {
    while *y < (*bmp).h {
        if *x >= (*bmp).w {
            *x = 0 as libc::c_int;
            *y += *rowh;
            *rowh = 1 as libc::c_int;
        }
        if border(bmp, *x, *y) == 0 {
            return;
        }
        *x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrLoadGlyphs(
    mut font: *mut TigrFont,
    mut codepage: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut rowh: libc::c_int = 1 as libc::c_int;
    let mut g: *mut TigrGlyph = 0 as *mut TigrGlyph;
    match codepage {
        0 => {
            (*font).numGlyphs = 128 as libc::c_int - 32 as libc::c_int;
        }
        1252 => {
            (*font).numGlyphs = 256 as libc::c_int - 32 as libc::c_int;
        }
        _ => {}
    }
    (*font)
        .glyphs = calloc(
        (*font).numGlyphs as libc::c_ulong,
        ::core::mem::size_of::<TigrGlyph>() as libc::c_ulong,
    ) as *mut TigrGlyph;
    i = 32 as libc::c_int;
    while i < (*font).numGlyphs + 32 as libc::c_int {
        scan((*font).bitmap, &mut x, &mut y, &mut rowh);
        if y >= (*(*font).bitmap).h {
            *__errno_location() = 22 as libc::c_int;
            return 0 as libc::c_int;
        }
        h = 0 as libc::c_int;
        w = h;
        while border((*font).bitmap, x + w, y) == 0 {
            w += 1;
        }
        while border((*font).bitmap, x, y + h) == 0 {
            h += 1;
        }
        g = &mut *((*font).glyphs).offset((i - 32 as libc::c_int) as isize)
            as *mut TigrGlyph;
        if i < 128 as libc::c_int {
            (*g).code = i;
        } else if codepage == 1252 as libc::c_int {
            (*g).code = cp1252[(i - 128 as libc::c_int) as usize];
        } else {
            *__errno_location() = 22 as libc::c_int;
            return 0 as libc::c_int;
        }
        (*g).x = x;
        (*g).y = y;
        (*g).w = w;
        (*g).h = h;
        x += w;
        if h != (*((*font).glyphs).offset(0 as libc::c_int as isize)).h {
            *__errno_location() = 22 as libc::c_int;
            return 0 as libc::c_int;
        }
        if h > rowh {
            rowh = h;
        }
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < (*font).numGlyphs {
        let mut j: libc::c_int = i;
        let mut g_0: TigrGlyph = *((*font).glyphs).offset(i as isize);
        while j > 0 as libc::c_int
            && (*((*font).glyphs).offset((j - 1 as libc::c_int) as isize)).code
                > g_0.code
        {
            *((*font).glyphs)
                .offset(
                    j as isize,
                ) = *((*font).glyphs).offset((j - 1 as libc::c_int) as isize);
            j -= 1;
        }
        *((*font).glyphs).offset(j as isize) = g_0;
        i += 1;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tigrLoadFont(
    mut bitmap: *mut Tigr,
    mut codepage: libc::c_int,
) -> *mut TigrFont {
    let mut font: *mut TigrFont = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<TigrFont>() as libc::c_ulong,
    ) as *mut TigrFont;
    (*font).bitmap = bitmap;
    if tigrLoadGlyphs(font, codepage) == 0 {
        tigrFreeFont(font);
        return 0 as *mut TigrFont;
    }
    return font;
}
#[no_mangle]
pub unsafe extern "C" fn tigrFreeFont(mut font: *mut TigrFont) {
    tigrFree((*font).bitmap);
    free((*font).glyphs as *mut libc::c_void);
    free(font as *mut libc::c_void);
}
unsafe extern "C" fn get(
    mut font: *mut TigrFont,
    mut code: libc::c_int,
) -> *mut TigrGlyph {
    let mut lo: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hi: libc::c_uint = (*font).numGlyphs as libc::c_uint;
    while lo < hi {
        let mut guess: libc::c_uint = lo
            .wrapping_add(hi)
            .wrapping_div(2 as libc::c_int as libc::c_uint);
        if code < (*((*font).glyphs).offset(guess as isize)).code {
            hi = guess;
        } else {
            lo = guess.wrapping_add(1 as libc::c_int as libc::c_uint);
        }
    }
    if lo == 0 as libc::c_int as libc::c_uint
        || (*((*font).glyphs)
            .offset(lo.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .code != code
    {
        return &mut *((*font).glyphs).offset(('?' as i32 - 32 as libc::c_int) as isize)
            as *mut TigrGlyph
    } else {
        return &mut *((*font).glyphs)
            .offset(lo.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as *mut TigrGlyph
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrSetupFont(mut font: *mut TigrFont) {
    if font == tfont && ((*tfont).bitmap).is_null() {
        (*tfont)
            .bitmap = tigrLoadImageMem(
            tigr_font.as_ptr() as *const libc::c_void,
            tigr_font_size,
        );
        tigrLoadGlyphs(tfont, 1252 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrPrint(
    mut dest: *mut Tigr,
    mut font: *mut TigrFont,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut color: TPixel,
    mut text: *const libc::c_char,
    mut args: ...
) {
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut g: *mut TigrGlyph = 0 as *mut TigrGlyph;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut start: libc::c_int = x;
    let mut c: libc::c_int = 0;
    tigrSetupFont(font);
    args_0 = args.clone();
    vsnprintf(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        text,
        args_0.as_va_list(),
    );
    tmp[(::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    p = tmp.as_mut_ptr();
    while *p != 0 {
        p = tigrDecodeUTF8(p, &mut c);
        if c == '\r' as i32 {
            continue;
        }
        if c == '\n' as i32 {
            x = start;
            y += tigrTextHeight(font, b"\0" as *const u8 as *const libc::c_char);
        } else {
            g = get(font, c);
            tigrBlitTint(
                dest,
                (*font).bitmap,
                x,
                y,
                (*g).x,
                (*g).y,
                (*g).w,
                (*g).h,
                color,
            );
            x += (*g).w;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrTextWidth(
    mut font: *mut TigrFont,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut w: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    tigrSetupFont(font);
    while *text != 0 {
        text = tigrDecodeUTF8(text, &mut c);
        if c == '\n' as i32 || c == '\r' as i32 {
            x = 0 as libc::c_int;
        } else {
            x += (*get(font, c)).w;
            w = if x > w { x } else { w };
        }
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn tigrTextHeight(
    mut font: *mut TigrFont,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut rowh: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    tigrSetupFont(font);
    rowh = (*get(font, 0 as libc::c_int)).h;
    h = rowh;
    while *text != 0 {
        text = tigrDecodeUTF8(text, &mut c);
        if c == '\n' as i32 && *text as libc::c_int != 0 {
            h += rowh;
        }
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn tigrInternal(mut bmp: *mut Tigr) -> *mut TigrInternal {
    if !((*bmp).handle).is_null() {} else {
        __assert_fail(
            b"bmp->handle\0" as *const u8 as *const libc::c_char,
            b"src/tigr.c\0" as *const u8 as *const libc::c_char,
            1857 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"TigrInternal *tigrInternal(Tigr *)\0"))
                .as_ptr(),
        );
    }
    return bmp.offset(1 as libc::c_int as isize) as *mut TigrInternal;
}
static mut dpy: *mut Display = 0 as *const Display as *mut Display;
static mut root: Window = 0;
static mut vi: *mut XVisualInfo = 0 as *const XVisualInfo as *mut XVisualInfo;
static mut wmDeleteMessage: Atom = 0;
static mut inputMethod: XIM = 0 as *const _XIM as *mut _XIM;
static mut fbConfig: GLXFBConfig = 0 as *const __GLXFBConfigRec as *mut __GLXFBConfigRec;
#[no_mangle]
pub static mut glXCreateContextAttribsARB: PFNGLXCREATECONTEXTATTRIBSARBPROC = None;
unsafe extern "C" fn initX11Stuff() {
    static mut done: libc::c_int = 0 as libc::c_int;
    if done == 0 {
        dpy = XOpenDisplay(0 as *const libc::c_char);
        if dpy.is_null() {
            tigrError(
                0 as *mut Tigr,
                b"Cannot connect to X server\0" as *const u8 as *const libc::c_char,
            );
        }
        root = (*((*(dpy as _XPrivDisplay)).screens)
            .offset((*(dpy as _XPrivDisplay)).default_screen as isize))
            .root;
        static mut attribList: [libc::c_int; 13] = [
            0x8011 as libc::c_int,
            0x1 as libc::c_int,
            0x8010 as libc::c_int,
            0x1 as libc::c_int,
            5 as libc::c_int,
            1 as libc::c_int,
            8 as libc::c_int,
            1 as libc::c_int,
            9 as libc::c_int,
            1 as libc::c_int,
            10 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_long as libc::c_int,
        ];
        let mut fbcCount: libc::c_int = 0 as libc::c_int;
        let mut fbc: *mut GLXFBConfig = glXChooseFBConfig(
            dpy,
            (*(dpy as _XPrivDisplay)).default_screen,
            attribList.as_mut_ptr(),
            &mut fbcCount,
        );
        if fbc.is_null() {
            tigrError(
                0 as *mut Tigr,
                b"Failed to choose FB config\0" as *const u8 as *const libc::c_char,
            );
        }
        fbConfig = *fbc.offset(0 as libc::c_int as isize);
        vi = glXGetVisualFromFBConfig(dpy, fbConfig);
        if vi.is_null() {
            tigrError(
                0 as *mut Tigr,
                b"No appropriate visual found\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut tmpCtx: GLXContext = glXCreateContext(
            dpy,
            vi,
            0 as GLXContext,
            1 as libc::c_int,
        );
        glXCreateContextAttribsARB = ::core::mem::transmute::<
            __GLXextFuncPtr,
            PFNGLXCREATECONTEXTATTRIBSARBPROC,
        >(
            glXGetProcAddressARB(
                b"glXCreateContextAttribsARB\0" as *const u8 as *const libc::c_char
                    as *const GLubyte,
            ),
        );
        glXDestroyContext(dpy, tmpCtx);
        if glXCreateContextAttribsARB.is_none() {
            tigrError(
                0 as *mut Tigr,
                b"Failed to get glXCreateContextAttribsARB\0" as *const u8
                    as *const libc::c_char,
            );
        }
        inputMethod = XOpenIM(
            dpy,
            0 as *mut _XrmHashBucketRec,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
        );
        if inputMethod.is_null() {
            tigrError(
                0 as *mut Tigr,
                b"Failed to create input method\0" as *const u8 as *const libc::c_char,
            );
        }
        wmDeleteMessage = XInternAtom(
            dpy,
            b"WM_DELETE_WINDOW\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        done = 1 as libc::c_int;
    }
}
unsafe extern "C" fn hasGLXExtension(
    mut display: *mut Display,
    mut wanted: *const libc::c_char,
) -> libc::c_int {
    let mut extensions: *const libc::c_char = glXQueryExtensionsString(
        display,
        (*(display as _XPrivDisplay)).default_screen,
    );
    let mut dup: *mut libc::c_char = strdup(extensions);
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = dup;
    loop {
        found = strtok(start, b" \0" as *const u8 as *const libc::c_char);
        if found.is_null() || strcmp(found, wanted) == 0 as libc::c_int {
            break;
        }
        start = 0 as *mut libc::c_char;
    }
    free(dup as *mut libc::c_void);
    return (found != 0 as *mut libc::c_char) as libc::c_int;
}
unsafe extern "C" fn setupVSync(mut display: *mut Display, mut win: Window) {
    if hasGLXExtension(
        display,
        b"GLX_EXT_swap_control\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        let mut glXSwapIntervalEXT: PFNGLXSWAPINTERVALEXTPROC = ::core::mem::transmute::<
            __GLXextFuncPtr,
            PFNGLXSWAPINTERVALEXTPROC,
        >(
            glXGetProcAddressARB(
                b"glXSwapIntervalEXT\0" as *const u8 as *const libc::c_char
                    as *const GLubyte,
            ),
        );
        if glXSwapIntervalEXT.is_some() {
            glXSwapIntervalEXT
                .expect("non-null function pointer")(display, win, 1 as libc::c_int);
        }
    } else if hasGLXExtension(
            display,
            b"GLX_MESA_swap_control\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
        let mut glXSwapIntervalMESA: PFNGLXSWAPINTERVALMESAPROC = ::core::mem::transmute::<
            __GLXextFuncPtr,
            PFNGLXSWAPINTERVALMESAPROC,
        >(
            glXGetProcAddressARB(
                b"glXSwapIntervalMESA\0" as *const u8 as *const libc::c_char
                    as *const GLubyte,
            ),
        );
        if glXSwapIntervalMESA.is_some() {
            glXSwapIntervalMESA
                .expect("non-null function pointer")(1 as libc::c_int as libc::c_uint);
        }
    } else if hasGLXExtension(
            display,
            b"GLX_SGI_swap_control\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
        let mut glXSwapIntervalSGI: PFNGLXSWAPINTERVALSGIPROC = ::core::mem::transmute::<
            __GLXextFuncPtr,
            PFNGLXSWAPINTERVALSGIPROC,
        >(
            glXGetProcAddressARB(
                b"glXSwapIntervalSGI\0" as *const u8 as *const libc::c_char
                    as *const GLubyte,
            ),
        );
        if glXSwapIntervalSGI.is_some() {
            glXSwapIntervalSGI.expect("non-null function pointer")(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn tigrHideCursor(mut win: *mut TigrInternal) {
    let mut invisibleCursor: Cursor = 0;
    let mut bitmapNoData: Pixmap = 0;
    let mut black: XColor = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    static mut noData: [libc::c_char; 8] = [
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    black.blue = 0 as libc::c_int as libc::c_ushort;
    black.green = black.blue;
    black.red = black.green;
    bitmapNoData = XCreateBitmapFromData(
        (*win).dpy,
        (*win).win,
        noData.as_mut_ptr(),
        8 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
    );
    invisibleCursor = XCreatePixmapCursor(
        (*win).dpy,
        bitmapNoData,
        bitmapNoData,
        &mut black,
        &mut black,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    XDefineCursor((*win).dpy, (*win).win, invisibleCursor);
    XFreeCursor((*win).dpy, invisibleCursor);
    XFreePixmap((*win).dpy, bitmapNoData);
}
#[no_mangle]
pub unsafe extern "C" fn tigrWindow(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut title: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut Tigr {
    let mut bmp: *mut Tigr = 0 as *mut Tigr;
    let mut cmap: Colormap = 0;
    let mut swa: XSetWindowAttributes = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut xwin: Window = 0;
    let mut glc: GLXContext = 0 as *mut __GLXcontextRec;
    let mut ic: XIC = 0 as *mut _XIC;
    let mut scale: libc::c_int = 0;
    initX11Stuff();
    if flags & 1 as libc::c_int != 0 {
        scale = 1 as libc::c_int;
    } else {
        let mut screen: *mut Screen = &mut *((*(dpy as _XPrivDisplay)).screens)
            .offset((*(dpy as _XPrivDisplay)).default_screen as isize) as *mut Screen;
        let mut maxW: libc::c_int = (*screen).width;
        let mut maxH: libc::c_int = (*screen).height;
        scale = tigrCalcScale(w, h, maxW, maxH);
    }
    scale = tigrEnforceScale(scale, flags);
    cmap = XCreateColormap(dpy, root, (*vi).visual, 0 as libc::c_int);
    swa.colormap = cmap;
    swa.event_mask = (1 as libc::c_long) << 17 as libc::c_int;
    xwin = XCreateWindow(
        dpy,
        root,
        0 as libc::c_int,
        0 as libc::c_int,
        (w * scale) as libc::c_uint,
        (h * scale) as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        (*vi).depth,
        1 as libc::c_int as libc::c_uint,
        (*vi).visual,
        ((1 as libc::c_long) << 13 as libc::c_int
            | (1 as libc::c_long) << 11 as libc::c_int) as libc::c_ulong,
        &mut swa,
    );
    XMapWindow(dpy, xwin);
    if flags & 64 as libc::c_int != 0 {
        let mut hints: WindowHints = WindowHints {
            flags: 0,
            functions: 0,
            decorations: 0,
            inputMode: 0,
            status: 0,
        };
        let mut property: Atom = 0;
        hints.flags = 2 as libc::c_int as libc::c_ulong;
        hints.decorations = 0 as libc::c_int as libc::c_ulong;
        property = XInternAtom(
            dpy,
            b"_MOTIF_WM_HINTS\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        XChangeProperty(
            dpy,
            xwin,
            property,
            property,
            32 as libc::c_int,
            0 as libc::c_int,
            &mut hints as *mut WindowHints as *mut libc::c_uchar,
            5 as libc::c_int,
        );
        let mut screen_0: libc::c_int = (*(dpy as _XPrivDisplay)).default_screen;
        let mut dWidth: libc::c_int = (*((*(dpy as _XPrivDisplay)).screens)
            .offset(screen_0 as isize))
            .width;
        let mut dHeight: libc::c_int = (*((*(dpy as _XPrivDisplay)).screens)
            .offset(screen_0 as isize))
            .height;
        XMoveResizeWindow(
            dpy,
            xwin,
            0 as libc::c_int,
            0 as libc::c_int,
            dWidth as libc::c_uint,
            dHeight as libc::c_uint,
        );
        XMapRaised(dpy, xwin);
        XGrabPointer(
            dpy,
            xwin,
            1 as libc::c_int,
            0 as libc::c_int as libc::c_uint,
            1 as libc::c_int,
            1 as libc::c_int,
            xwin,
            0 as libc::c_long as Cursor,
            0 as libc::c_long as Time,
        );
        XGrabKeyboard(
            dpy,
            xwin,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_long as Time,
        );
    } else {
        loop {
            let mut e: XEvent = _XEvent { type_0: 0 };
            XNextEvent(dpy, &mut e);
            if e.type_0 == 19 as libc::c_int {
                break;
            }
        }
        let mut wa: XWindowAttributes = XWindowAttributes {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            border_width: 0,
            depth: 0,
            visual: 0 as *mut Visual,
            root: 0,
            class: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            colormap: 0,
            map_installed: 0,
            map_state: 0,
            all_event_masks: 0,
            your_event_mask: 0,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            screen: 0 as *mut Screen,
        };
        XGetWindowAttributes(dpy, xwin, &mut wa);
        scale = tigrCalcScale(w, h, wa.width, wa.height);
        scale = tigrEnforceScale(scale, flags);
        XResizeWindow(
            dpy,
            xwin,
            (w * scale) as libc::c_uint,
            (h * scale) as libc::c_uint,
        );
    }
    let mut prop: XTextProperty = XTextProperty {
        value: 0 as *mut libc::c_uchar,
        encoding: 0,
        format: 0,
        nitems: 0,
    };
    let mut result: libc::c_int = Xutf8TextListToTextProperty(
        dpy,
        &mut title as *mut *const libc::c_char as *mut *mut libc::c_char,
        1 as libc::c_int,
        XUTF8StringStyle,
        &mut prop,
    );
    if result == 0 as libc::c_int {
        let mut wmName: Atom = XInternAtom(
            dpy,
            b"_NET_WM_NAME\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        XSetTextProperty(dpy, xwin, &mut prop, wmName);
        XFree(prop.value as *mut libc::c_void);
    }
    ic = XCreateIC(
        inputMethod,
        b"inputStyle\0" as *const u8 as *const libc::c_char,
        0x8 as libc::c_long | 0x400 as libc::c_long,
        b"clientWindow\0" as *const u8 as *const libc::c_char,
        xwin,
        0 as *mut libc::c_void,
    );
    if ic.is_null() {
        printf(
            b"Failed to create input context\n\0" as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    XSetICFocus(ic);
    XSetWMProtocols(dpy, xwin, &mut wmDeleteMessage, 1 as libc::c_int);
    glc = glXCreateContext(dpy, vi, 0 as GLXContext, 1 as libc::c_int);
    let mut contextAttributes: [libc::c_int; 5] = [
        0x2091 as libc::c_int,
        3 as libc::c_int,
        0x2092 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_long as libc::c_int,
    ];
    glc = glXCreateContextAttribsARB
        .expect(
            "non-null function pointer",
        )(
        dpy,
        fbConfig,
        0 as GLXContext,
        1 as libc::c_int,
        contextAttributes.as_mut_ptr(),
    );
    glXMakeCurrent(dpy, xwin, glc);
    setupVSync(dpy, xwin);
    bmp = tigrBitmap2(
        w,
        h,
        ::core::mem::size_of::<TigrInternal>() as libc::c_ulong as libc::c_int,
    );
    (*bmp).handle = xwin as *mut libc::c_void;
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    (*win).win = xwin;
    (*win).dpy = dpy;
    (*win).glc = glc;
    (*win).ic = ic;
    (*win).shown = 0 as libc::c_int;
    (*win).closed = 0 as libc::c_int;
    (*win).scale = scale;
    (*win).lastChar = 0 as libc::c_int;
    (*win).flags = flags;
    (*win).p3 = 0 as libc::c_int as libc::c_float;
    (*win).p2 = (*win).p3;
    (*win).p1 = (*win).p2;
    (*win).p4 = 1 as libc::c_int as libc::c_float;
    (*win).widgetsWanted = 0 as libc::c_int;
    (*win).widgetAlpha = 0 as libc::c_int as libc::c_uchar;
    (*win).widgetsScale = 0 as libc::c_int as libc::c_float;
    (*win).widgets = 0 as *mut Tigr;
    (*win).gl.gl_legacy = 0 as libc::c_int;
    memset(
        ((*win).keys).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    memset(
        ((*win).prev).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    if flags & 32 as libc::c_int != 0 {
        tigrHideCursor(win);
    }
    tigrPosition(bmp, (*win).scale, (*bmp).w, (*bmp).h, ((*win).pos).as_mut_ptr());
    tigrGAPICreate(bmp);
    tigrGAPIBegin(bmp);
    return bmp;
}
#[no_mangle]
pub unsafe extern "C" fn tigrClosed(mut bmp: *mut Tigr) -> libc::c_int {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    return ((*win).win == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tigrGAPIBegin(mut bmp: *mut Tigr) -> libc::c_int {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    return if glXMakeCurrent((*win).dpy, (*win).win, (*win).glc) != 0 {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrGAPIEnd(mut _bmp: *mut Tigr) -> libc::c_int {
    return if glXMakeCurrent(
        0 as *mut Display,
        0 as libc::c_int as GLXDrawable,
        0 as GLXContext,
    ) != 0
    {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrKeyDown(
    mut bmp: *mut Tigr,
    mut key: libc::c_int,
) -> libc::c_int {
    let mut win: *mut TigrInternal = 0 as *mut TigrInternal;
    if key < 256 as libc::c_int {} else {
        __assert_fail(
            b"key < 256\0" as *const u8 as *const libc::c_char,
            b"src/tigr.c\0" as *const u8 as *const libc::c_char,
            4611 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"int tigrKeyDown(Tigr *, int)\0"))
                .as_ptr(),
        );
    }
    win = tigrInternal(bmp);
    return ((*win).keys[key as usize] as libc::c_int != 0
        && (*win).prev[key as usize] == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tigrKeyHeld(
    mut bmp: *mut Tigr,
    mut key: libc::c_int,
) -> libc::c_int {
    let mut win: *mut TigrInternal = 0 as *mut TigrInternal;
    if key < 256 as libc::c_int {} else {
        __assert_fail(
            b"key < 256\0" as *const u8 as *const libc::c_char,
            b"src/tigr.c\0" as *const u8 as *const libc::c_char,
            4618 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"int tigrKeyHeld(Tigr *, int)\0"))
                .as_ptr(),
        );
    }
    win = tigrInternal(bmp);
    return (*win).keys[key as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tigrReadChar(mut bmp: *mut Tigr) -> libc::c_int {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    let mut c: libc::c_int = (*win).lastChar;
    (*win).lastChar = 0 as libc::c_int;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn tigrKeyFromX11(mut sym: KeySym) -> uint8_t {
    if sym >= 'a' as i32 as libc::c_ulong && sym <= 'z' as i32 as libc::c_ulong {
        return (sym as uint8_t as libc::c_int - ('a' as i32 - 'A' as i32)) as uint8_t;
    }
    if sym >= '0' as i32 as libc::c_ulong && sym <= '9' as i32 as libc::c_ulong {
        return sym as uint8_t;
    }
    match sym {
        65456 => return TK_PAD0 as libc::c_int as uint8_t,
        65457 => return TK_PAD1 as libc::c_int as uint8_t,
        65458 => return TK_PAD2 as libc::c_int as uint8_t,
        65459 => return TK_PAD3 as libc::c_int as uint8_t,
        65460 => return TK_PAD4 as libc::c_int as uint8_t,
        65461 => return TK_PAD5 as libc::c_int as uint8_t,
        65462 => return TK_PAD6 as libc::c_int as uint8_t,
        65463 => return TK_PAD7 as libc::c_int as uint8_t,
        65464 => return TK_PAD8 as libc::c_int as uint8_t,
        65465 => return TK_PAD9 as libc::c_int as uint8_t,
        65450 => return TK_PADMUL as libc::c_int as uint8_t,
        65455 => return TK_PADDIV as libc::c_int as uint8_t,
        65451 => return TK_PADADD as libc::c_int as uint8_t,
        65453 => return TK_PADSUB as libc::c_int as uint8_t,
        65454 => return TK_PADDOT as libc::c_int as uint8_t,
        65421 => return TK_PADENTER as libc::c_int as uint8_t,
        65470 => return TK_F1 as libc::c_int as uint8_t,
        65471 => return TK_F2 as libc::c_int as uint8_t,
        65472 => return TK_F3 as libc::c_int as uint8_t,
        65473 => return TK_F4 as libc::c_int as uint8_t,
        65474 => return TK_F5 as libc::c_int as uint8_t,
        65475 => return TK_F6 as libc::c_int as uint8_t,
        65476 => return TK_F7 as libc::c_int as uint8_t,
        65477 => return TK_F8 as libc::c_int as uint8_t,
        65478 => return TK_F9 as libc::c_int as uint8_t,
        65479 => return TK_F10 as libc::c_int as uint8_t,
        65480 => return TK_F11 as libc::c_int as uint8_t,
        65481 => return TK_F12 as libc::c_int as uint8_t,
        65288 => return TK_BACKSPACE as libc::c_int as uint8_t,
        65289 => return TK_TAB as libc::c_int as uint8_t,
        65293 => return TK_RETURN as libc::c_int as uint8_t,
        65299 => return TK_PAUSE as libc::c_int as uint8_t,
        65509 => return TK_CAPSLOCK as libc::c_int as uint8_t,
        65307 => return TK_ESCAPE as libc::c_int as uint8_t,
        32 => return TK_SPACE as libc::c_int as uint8_t,
        65365 => return TK_PAGEUP as libc::c_int as uint8_t,
        65366 => return TK_PAGEDN as libc::c_int as uint8_t,
        65367 => return TK_END as libc::c_int as uint8_t,
        65360 => return TK_HOME as libc::c_int as uint8_t,
        65361 => return TK_LEFT as libc::c_int as uint8_t,
        65362 => return TK_UP as libc::c_int as uint8_t,
        65363 => return TK_RIGHT as libc::c_int as uint8_t,
        65364 => return TK_DOWN as libc::c_int as uint8_t,
        65379 => return TK_INSERT as libc::c_int as uint8_t,
        65535 => return TK_DELETE as libc::c_int as uint8_t,
        65511 => return TK_LWIN as libc::c_int as uint8_t,
        65512 => return TK_RWIN as libc::c_int as uint8_t,
        65407 => return TK_NUMLOCK as libc::c_int as uint8_t,
        65300 => return TK_SCROLL as libc::c_int as uint8_t,
        65505 => return TK_LSHIFT as libc::c_int as uint8_t,
        65506 => return TK_RSHIFT as libc::c_int as uint8_t,
        65507 => return TK_LCONTROL as libc::c_int as uint8_t,
        65508 => return TK_RCONTROL as libc::c_int as uint8_t,
        65513 => return TK_LALT as libc::c_int as uint8_t,
        65514 => return TK_RALT as libc::c_int as uint8_t,
        59 => return TK_SEMICOLON as libc::c_int as uint8_t,
        61 => return TK_EQUALS as libc::c_int as uint8_t,
        44 => return TK_COMMA as libc::c_int as uint8_t,
        45 => return TK_MINUS as libc::c_int as uint8_t,
        46 => return TK_DOT as libc::c_int as uint8_t,
        47 => return TK_SLASH as libc::c_int as uint8_t,
        96 => return TK_BACKTICK as libc::c_int as uint8_t,
        91 => return TK_LSQUARE as libc::c_int as uint8_t,
        92 => return TK_BACKSLASH as libc::c_int as uint8_t,
        93 => return TK_RSQUARE as libc::c_int as uint8_t,
        39 => return TK_TICK as libc::c_int as uint8_t,
        _ => {}
    }
    return 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn tigrUpdateModifiers(mut win: *mut TigrInternal) {
    (*win)
        .keys[TK_SHIFT as libc::c_int
        as usize] = ((*win).keys[TK_LSHIFT as libc::c_int as usize] as libc::c_int != 0
        || (*win).keys[TK_RSHIFT as libc::c_int as usize] as libc::c_int != 0)
        as libc::c_int as libc::c_char;
    (*win)
        .keys[TK_CONTROL as libc::c_int
        as usize] = ((*win).keys[TK_LCONTROL as libc::c_int as usize] as libc::c_int != 0
        || (*win).keys[TK_RCONTROL as libc::c_int as usize] as libc::c_int != 0)
        as libc::c_int as libc::c_char;
    (*win)
        .keys[TK_ALT as libc::c_int
        as usize] = ((*win).keys[TK_LALT as libc::c_int as usize] as libc::c_int != 0
        || (*win).keys[TK_RALT as libc::c_int as usize] as libc::c_int != 0)
        as libc::c_int as libc::c_char;
}
unsafe extern "C" fn tigrInterpretChar(
    mut win: *mut TigrInternal,
    mut root_0: Window,
    mut keycode: libc::c_uint,
    mut mask: libc::c_uint,
) {
    let mut event: XKeyEvent = XKeyEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: 0 as *mut Display,
        window: 0,
        root: 0,
        subwindow: 0,
        time: 0,
        x: 0,
        y: 0,
        x_root: 0,
        y_root: 0,
        state: 0,
        keycode: 0,
        same_screen: 0,
    };
    memset(
        &mut event as *mut XKeyEvent as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<XKeyEvent>() as libc::c_ulong,
    );
    event.type_0 = 2 as libc::c_int;
    event.display = (*win).dpy;
    event.root = root_0;
    event.window = (*win).win;
    event.state = mask;
    event.keycode = keycode;
    let mut inputTextUTF8: [libc::c_char; 10] = [0; 10];
    let mut status: libc::c_int = 0 as libc::c_int;
    Xutf8LookupString(
        (*win).ic,
        &mut event,
        inputTextUTF8.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as libc::c_int,
        0 as *mut KeySym,
        &mut status,
    );
    if status == 2 as libc::c_int {
        tigrDecodeUTF8(inputTextUTF8.as_mut_ptr(), &mut (*win).lastChar);
    }
}
unsafe extern "C" fn tigrProcessInput(
    mut win: *mut TigrInternal,
    mut winWidth: libc::c_int,
    mut winHeight: libc::c_int,
) {
    let mut focused: Window = 0;
    let mut revertTo: libc::c_int = 0;
    XGetInputFocus((*win).dpy, &mut focused, &mut revertTo);
    if (*win).win != focused {
        return;
    }
    let mut root_0: Window = 0;
    let mut child: Window = 0;
    let mut rootX: libc::c_int = 0;
    let mut rootY: libc::c_int = 0;
    let mut winX: libc::c_int = 0;
    let mut winY: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    if XQueryPointer(
        (*win).dpy,
        (*win).win,
        &mut root_0,
        &mut child,
        &mut rootX,
        &mut rootY,
        &mut winX,
        &mut winY,
        &mut mask,
    ) != 0
    {
        static mut prevButtons: libc::c_uint = 0;
        let mut buttons: libc::c_uint = mask
            & ((1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint;
        (*win).mouseX = (winX - (*win).pos[0 as libc::c_int as usize]) / (*win).scale;
        (*win).mouseY = (winY - (*win).pos[1 as libc::c_int as usize]) / (*win).scale;
        if buttons != prevButtons && (winX > 0 as libc::c_int && winX < winWidth)
            && (winY > 0 as libc::c_int && winY < winHeight)
        {
            (*win)
                .mouseButtons = if buttons
                & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint != 0
            {
                1 as libc::c_int
            } else if 0 as libc::c_int as libc::c_uint
                    | buttons & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint
                    != 0
                {
                2 as libc::c_int
            } else if 0 as libc::c_int as libc::c_uint
                    | buttons & ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint
                    != 0
                {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
        prevButtons = buttons;
    }
    static mut prevKeys: [libc::c_char; 32] = [0; 32];
    let mut keys: [libc::c_char; 32] = [0; 32];
    XQueryKeymap((*win).dpy, keys.as_mut_ptr());
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut thisBlock: libc::c_char = keys[i as usize];
        let mut prevBlock: libc::c_char = prevKeys[i as usize];
        if thisBlock as libc::c_int != prevBlock as libc::c_int {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                let mut thisBit: libc::c_int = thisBlock as libc::c_int
                    & 1 as libc::c_int;
                let mut prevBit: libc::c_int = prevBlock as libc::c_int
                    & 1 as libc::c_int;
                thisBlock = (thisBlock as libc::c_int >> 1 as libc::c_int)
                    as libc::c_char;
                prevBlock = (prevBlock as libc::c_int >> 1 as libc::c_int)
                    as libc::c_char;
                if thisBit != prevBit {
                    let mut keyCode: libc::c_int = 8 as libc::c_int * i + j;
                    let mut keySym: KeySym = XkbKeycodeToKeysym(
                        (*win).dpy,
                        keyCode as KeyCode,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    if keySym != 0 as libc::c_long as libc::c_ulong {
                        let mut key: libc::c_int = tigrKeyFromX11(keySym) as libc::c_int;
                        (*win).keys[key as usize] = thisBit as libc::c_char;
                        tigrUpdateModifiers(win);
                        if thisBit != 0 {
                            tigrInterpretChar(
                                win,
                                root_0,
                                keyCode as libc::c_uint,
                                mask,
                            );
                        }
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    memcpy(
        prevKeys.as_mut_ptr() as *mut libc::c_void,
        keys.as_mut_ptr() as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
    let mut event: XEvent = _XEvent { type_0: 0 };
    while XCheckTypedWindowEvent((*win).dpy, (*win).win, 33 as libc::c_int, &mut event)
        != 0
    {
        if event.xclient.data.l[0 as libc::c_int as usize] as libc::c_ulong
            == wmDeleteMessage
        {
            glXMakeCurrent(
                (*win).dpy,
                0 as libc::c_long as GLXDrawable,
                0 as GLXContext,
            );
            glXDestroyContext((*win).dpy, (*win).glc);
            XDestroyWindow((*win).dpy, (*win).win);
            (*win).win = 0 as libc::c_int as Window;
        }
    }
    XFlush((*win).dpy);
}
#[no_mangle]
pub unsafe extern "C" fn tigrUpdate(mut bmp: *mut Tigr) {
    let mut gwa: XWindowAttributes = XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: 0 as *mut Visual,
        root: 0,
        class: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        colormap: 0,
        map_installed: 0,
        map_state: 0,
        all_event_masks: 0,
        your_event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        screen: 0 as *mut Screen,
    };
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    memcpy(
        ((*win).prev).as_mut_ptr() as *mut libc::c_void,
        ((*win).keys).as_mut_ptr() as *const libc::c_void,
        256 as libc::c_int as libc::c_ulong,
    );
    XGetWindowAttributes((*win).dpy, (*win).win, &mut gwa);
    if (*win).flags & 1 as libc::c_int != 0 {
        tigrResize(bmp, gwa.width / (*win).scale, gwa.height / (*win).scale);
    } else {
        (*win)
            .scale = tigrEnforceScale(
            tigrCalcScale((*bmp).w, (*bmp).h, gwa.width, gwa.height),
            (*win).flags,
        );
    }
    tigrPosition(bmp, (*win).scale, gwa.width, gwa.height, ((*win).pos).as_mut_ptr());
    glXMakeCurrent((*win).dpy, (*win).win, (*win).glc);
    tigrGAPIPresent(bmp, gwa.width, gwa.height);
    glXSwapBuffers((*win).dpy, (*win).win);
    tigrProcessInput(win, gwa.width, gwa.height);
}
#[no_mangle]
pub unsafe extern "C" fn tigrFree(mut bmp: *mut Tigr) {
    if !((*bmp).handle).is_null() {
        let mut win: *mut TigrInternal = tigrInternal(bmp);
        if (*win).win != 0 {
            glXMakeCurrent(
                (*win).dpy,
                0 as libc::c_long as GLXDrawable,
                0 as GLXContext,
            );
            glXDestroyContext((*win).dpy, (*win).glc);
            XDestroyWindow((*win).dpy, (*win).win);
            (*win).win = 0 as libc::c_int as Window;
        }
    }
    free((*bmp).pix as *mut libc::c_void);
    free(bmp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tigrError(
    mut _bmp: *mut Tigr,
    mut message: *const libc::c_char,
    mut args: ...
) {
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vsnprintf(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        message,
        args_0.as_va_list(),
    );
    tmp[(::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    printf(
        b"tigr fatal error: %s\n\0" as *const u8 as *const libc::c_char,
        tmp.as_mut_ptr(),
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tigrTime() -> libc::c_float {
    static mut lastTime: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    let mut now: libc::c_double = tv.tv_sec as libc::c_double
        + tv.tv_usec as libc::c_double / 1000000.0f64;
    let mut elapsed: libc::c_double = if lastTime == 0 as libc::c_int as libc::c_double {
        0 as libc::c_int as libc::c_double
    } else {
        now - lastTime
    };
    lastTime = now;
    return elapsed as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn tigrMouse(
    mut bmp: *mut Tigr,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut buttons: *mut libc::c_int,
) {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    if !x.is_null() {
        *x = (*win).mouseX;
    }
    if !y.is_null() {
        *y = (*win).mouseY;
    }
    if !buttons.is_null() {
        *buttons = (*win).mouseButtons;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrTouch(
    mut bmp: *mut Tigr,
    mut points: *mut TigrTouchPoint,
    mut maxPoints: libc::c_int,
) -> libc::c_int {
    let mut buttons: libc::c_int = 0 as libc::c_int;
    if maxPoints > 0 as libc::c_int {
        tigrMouse(bmp, &mut (*points).x, &mut (*points).y, &mut buttons);
    }
    return if buttons != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn tigrCheckGLError(mut state: *const libc::c_char) {
    let mut err: GLenum = glGetError();
    if err != 0 as libc::c_int as libc::c_uint {
        tigrError(
            0 as *mut Tigr,
            b"got GL error %x when doing %s\n\0" as *const u8 as *const libc::c_char,
            err,
            state,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrCheckShaderErrors(mut object: GLuint) {
    let mut success: GLint = 0;
    let mut info: [GLchar; 2048] = [0; 2048];
    glGetShaderiv(object, 0x8b81 as libc::c_int as GLenum, &mut success);
    if success == 0 {
        glGetShaderInfoLog(
            object,
            ::core::mem::size_of::<[GLchar; 2048]>() as libc::c_ulong as GLsizei,
            0 as *mut GLsizei,
            info.as_mut_ptr(),
        );
        tigrError(
            0 as *mut Tigr,
            b"shader compile error : %s\n\0" as *const u8 as *const libc::c_char,
            info.as_mut_ptr(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrCheckProgramErrors(mut object: GLuint) {
    let mut success: GLint = 0;
    let mut info: [GLchar; 2048] = [0; 2048];
    glGetProgramiv(object, 0x8b82 as libc::c_int as GLenum, &mut success);
    if success == 0 {
        glGetProgramInfoLog(
            object,
            ::core::mem::size_of::<[GLchar; 2048]>() as libc::c_ulong as GLsizei,
            0 as *mut GLsizei,
            info.as_mut_ptr(),
        );
        tigrError(
            0 as *mut Tigr,
            b"shader link error : %s\n\0" as *const u8 as *const libc::c_char,
            info.as_mut_ptr(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrCreateShaderProgram(
    mut gl: *mut GLStuff,
    mut fxSource: *const libc::c_char,
    mut fxSize: libc::c_int,
) {
    if (*gl).program != 0 as libc::c_int as libc::c_uint {
        glDeleteProgram((*gl).program);
        (*gl).program = 0 as libc::c_int as GLuint;
    }
    let mut vs: GLuint = glCreateShader(0x8b31 as libc::c_int as GLenum);
    let mut vs_source: *const libc::c_char = &tigr_upscale_gl_vs
        as *const [libc::c_char; 235] as *const libc::c_char;
    glShaderSource(
        vs,
        1 as libc::c_int,
        &mut vs_source as *mut *const libc::c_char as *const *const GLchar,
        &tigr_upscale_gl_vs_size,
    );
    glCompileShader(vs);
    tigrCheckShaderErrors(vs);
    let mut fs: GLuint = glCreateShader(0x8b30 as libc::c_int as GLenum);
    let mut fs_sources: [*const libc::c_char; 2] = [
        tigr_upscale_gl_fs.as_ptr(),
        fxSource,
    ];
    let fs_lengths: [libc::c_int; 2] = [tigr_upscale_gl_fs_size, fxSize];
    glShaderSource(
        fs,
        2 as libc::c_int,
        fs_sources.as_mut_ptr() as *const *const GLchar,
        fs_lengths.as_ptr(),
    );
    glCompileShader(fs);
    tigrCheckShaderErrors(fs);
    (*gl).program = glCreateProgram();
    glAttachShader((*gl).program, vs);
    glAttachShader((*gl).program, fs);
    glLinkProgram((*gl).program);
    tigrCheckProgramErrors((*gl).program);
    glDeleteShader(vs);
    glDeleteShader(fs);
    (*gl)
        .uniform_projection = glGetUniformLocation(
        (*gl).program,
        b"projection\0" as *const u8 as *const libc::c_char,
    ) as GLuint;
    (*gl)
        .uniform_model = glGetUniformLocation(
        (*gl).program,
        b"model\0" as *const u8 as *const libc::c_char,
    ) as GLuint;
    (*gl)
        .uniform_parameters = glGetUniformLocation(
        (*gl).program,
        b"parameters\0" as *const u8 as *const libc::c_char,
    ) as GLuint;
}
#[no_mangle]
pub unsafe extern "C" fn tigrGAPICreate(mut bmp: *mut Tigr) {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    let mut gl: *mut GLStuff = &mut (*win).gl;
    let mut VBO: GLuint = 0;
    let mut vertices: [GLfloat; 24] = [
        0.0f32,
        1.0f32,
        0.0f32,
        1.0f32,
        1.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
        1.0f32,
        1.0f32,
        1.0f32,
        1.0f32,
        1.0f32,
        1.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
    ];
    if (*gl).gl_legacy == 0 {
        glGenVertexArrays(1 as libc::c_int, &mut (*gl).vao);
        glGenBuffers(1 as libc::c_int, &mut VBO);
        glBindBuffer(0x8892 as libc::c_int as GLenum, VBO);
        glBufferData(
            0x8892 as libc::c_int as GLenum,
            ::core::mem::size_of::<[GLfloat; 24]>() as libc::c_ulong as GLsizeiptr,
            vertices.as_mut_ptr() as *const libc::c_void,
            0x88e4 as libc::c_int as GLenum,
        );
        glBindVertexArray((*gl).vao);
        glEnableVertexAttribArray(0 as libc::c_int as GLuint);
        glEnableVertexAttribArray(1 as libc::c_int as GLuint);
        glVertexAttribPointer(
            0 as libc::c_int as GLuint,
            2 as libc::c_int,
            0x1406 as libc::c_int as GLenum,
            0 as libc::c_int as GLboolean,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<GLfloat>() as libc::c_ulong)
                as GLsizei,
            0 as *const libc::c_void,
        );
        glVertexAttribPointer(
            1 as libc::c_int as GLuint,
            2 as libc::c_int,
            0x1406 as libc::c_int as GLenum,
            0 as libc::c_int as GLboolean,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<GLfloat>() as libc::c_ulong)
                as GLsizei,
            0 as *const libc::c_void,
        );
        tigrCreateShaderProgram(
            gl,
            tigr_default_fx_gl_fs.as_ptr(),
            tigr_default_fx_gl_fs_size,
        );
    }
    if (*gl).gl_legacy != 0 {
        glEnable(0xde1 as libc::c_int as GLenum);
    }
    glGenTextures(2 as libc::c_int, ((*gl).tex).as_mut_ptr());
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        glBindTexture(0xde1 as libc::c_int as GLenum, (*gl).tex[i as usize]);
        glTexParameteri(
            0xde1 as libc::c_int as GLenum,
            0x2801 as libc::c_int as GLenum,
            if (*gl).gl_legacy != 0 {
                0x2600 as libc::c_int
            } else {
                0x2601 as libc::c_int
            },
        );
        glTexParameteri(
            0xde1 as libc::c_int as GLenum,
            0x2800 as libc::c_int as GLenum,
            if (*gl).gl_legacy != 0 {
                0x2600 as libc::c_int
            } else {
                0x2601 as libc::c_int
            },
        );
        glPixelStorei(0xcf2 as libc::c_int as GLenum, 0 as libc::c_int);
        glPixelStorei(0xcf5 as libc::c_int as GLenum, 1 as libc::c_int);
        i += 1;
    }
    tigrCheckGLError(b"initialization\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn tigrGAPIDestroy(mut bmp: *mut Tigr) {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    let mut gl: *mut GLStuff = &mut (*win).gl;
    if tigrGAPIBegin(bmp) < 0 as libc::c_int {
        tigrError(
            bmp,
            b"Cannot activate OpenGL context.\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*gl).gl_legacy == 0 {
        glDeleteTextures(2 as libc::c_int, ((*gl).tex).as_mut_ptr());
        glDeleteProgram((*gl).program);
    }
    tigrCheckGLError(b"destroy\0" as *const u8 as *const libc::c_char);
    if tigrGAPIEnd(bmp) < 0 as libc::c_int {
        tigrError(
            bmp,
            b"Cannot deactivate OpenGL context.\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tigrGAPIDraw(
    mut legacy: libc::c_int,
    mut uniform_model: GLuint,
    mut tex: GLuint,
    mut bmp: *mut Tigr,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
) {
    glBindTexture(0xde1 as libc::c_int as GLenum, tex);
    glTexImage2D(
        0xde1 as libc::c_int as GLenum,
        0 as libc::c_int,
        0x8058 as libc::c_int,
        (*bmp).w,
        (*bmp).h,
        0 as libc::c_int,
        0x1908 as libc::c_int as GLenum,
        0x1401 as libc::c_int as GLenum,
        (*bmp).pix as *const libc::c_void,
    );
    if legacy == 0 {
        let mut sx: libc::c_float = (x2 - x1) as libc::c_float;
        let mut sy: libc::c_float = (y2 - y1) as libc::c_float;
        let mut tx: libc::c_float = x1 as libc::c_float;
        let mut ty: libc::c_float = y1 as libc::c_float;
        let mut model: [libc::c_float; 16] = [
            sx,
            0.0f32,
            0.0f32,
            0.0f32,
            0.0f32,
            sy,
            0.0f32,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
            0.0f32,
            tx,
            ty,
            0.0f32,
            1.0f32,
        ];
        glUniformMatrix4fv(
            uniform_model as GLint,
            1 as libc::c_int,
            0 as libc::c_int as GLboolean,
            model.as_mut_ptr(),
        );
        glDrawArrays(0x4 as libc::c_int as GLenum, 0 as libc::c_int, 6 as libc::c_int);
    } else {
        glBegin(0x7 as libc::c_int as GLenum);
        glTexCoord2f(1.0f32, 0.0f32);
        glVertex2i(x2, y1);
        glTexCoord2f(0.0f32, 0.0f32);
        glVertex2i(x1, y1);
        glTexCoord2f(0.0f32, 1.0f32);
        glVertex2i(x1, y2);
        glTexCoord2f(1.0f32, 1.0f32);
        glVertex2i(x2, y2);
        glEnd();
    };
}
#[no_mangle]
pub unsafe extern "C" fn tigrGAPIPresent(
    mut bmp: *mut Tigr,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    let mut gl: *mut GLStuff = &mut (*win).gl;
    glViewport(0 as libc::c_int, 0 as libc::c_int, w, h);
    if (*gl).gl_user_opengl_rendering == 0 {
        glClearColor(
            0 as libc::c_int as GLclampf,
            0 as libc::c_int as GLclampf,
            0 as libc::c_int as GLclampf,
            1 as libc::c_int as GLclampf,
        );
        glClear(0x4000 as libc::c_int as GLbitfield);
    }
    if (*gl).gl_legacy == 0 {
        let mut projection: [libc::c_float; 16] = [
            2.0f32 / w as libc::c_float,
            0.0f32,
            0.0f32,
            0.0f32,
            0.0f32,
            -2.0f32 / h as libc::c_float,
            0.0f32,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
            0.0f32,
            -1.0f32,
            1.0f32,
            0.0f32,
            1.0f32,
        ];
        glActiveTexture(0x84c0 as libc::c_int as GLenum);
        glBindVertexArray((*gl).vao);
        glUseProgram((*gl).program);
        glUniformMatrix4fv(
            (*gl).uniform_projection as GLint,
            1 as libc::c_int,
            0 as libc::c_int as GLboolean,
            projection.as_mut_ptr(),
        );
        glUniform4f(
            (*gl).uniform_parameters as GLint,
            (*win).p1,
            (*win).p2,
            (*win).p3,
            (*win).p4,
        );
    } else {
        glMatrixMode(0x1701 as libc::c_int as GLenum);
        glLoadIdentity();
        glOrtho(
            0 as libc::c_int as GLdouble,
            w as GLdouble,
            h as GLdouble,
            0 as libc::c_int as GLdouble,
            -1.0f32 as GLdouble,
            1.0f32 as GLdouble,
        );
        glEnable(0xde1 as libc::c_int as GLenum);
    }
    if (*gl).gl_user_opengl_rendering != 0 {
        glEnable(0xbe2 as libc::c_int as GLenum);
        glBlendFunc(0x302 as libc::c_int as GLenum, 0x303 as libc::c_int as GLenum);
    } else {
        glDisable(0xbe2 as libc::c_int as GLenum);
    }
    tigrGAPIDraw(
        (*gl).gl_legacy,
        (*gl).uniform_model,
        (*gl).tex[0 as libc::c_int as usize],
        bmp,
        (*win).pos[0 as libc::c_int as usize],
        (*win).pos[1 as libc::c_int as usize],
        (*win).pos[2 as libc::c_int as usize],
        (*win).pos[3 as libc::c_int as usize],
    );
    if (*win).widgetsScale > 0 as libc::c_int as libc::c_float {
        glEnable(0xbe2 as libc::c_int as GLenum);
        glBlendFunc(0x302 as libc::c_int as GLenum, 0x303 as libc::c_int as GLenum);
        tigrGAPIDraw(
            (*gl).gl_legacy,
            (*gl).uniform_model,
            (*gl).tex[1 as libc::c_int as usize],
            (*win).widgets,
            (w as libc::c_float
                - (*(*win).widgets).w as libc::c_float * (*win).widgetsScale)
                as libc::c_int,
            0 as libc::c_int,
            w,
            ((*(*win).widgets).h as libc::c_float * (*win).widgetsScale) as libc::c_int,
        );
    }
    tigrCheckGLError(b"present\0" as *const u8 as *const libc::c_char);
    (*gl).gl_user_opengl_rendering = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tigrReadFile(
    mut fileName: *const libc::c_char,
    mut length: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if !length.is_null() {
        *length = 0 as libc::c_int;
    }
    file = fopen(fileName, b"rb\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return 0 as *mut libc::c_void;
    }
    fseek(file, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    len = ftell(file) as size_t;
    fseek(file, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    data = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if data.is_null() {
        fclose(file);
        return 0 as *mut libc::c_void;
    }
    if fread(data as *mut libc::c_void, 1 as libc::c_int as libc::c_ulong, len, file)
        != len
    {
        free(data as *mut libc::c_void);
        fclose(file);
        return 0 as *mut libc::c_void;
    }
    *data.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    fclose(file);
    if !length.is_null() {
        *length = len as libc::c_int;
    }
    return data as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn tigrDecodeUTF8(
    mut text: *const libc::c_char,
    mut cp: *mut libc::c_int,
) -> *const libc::c_char {
    let fresh39 = text;
    text = text.offset(1);
    let mut c: libc::c_uchar = *fresh39 as libc::c_uchar;
    let mut extra: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0 as libc::c_int;
    *cp = 0 as libc::c_int;
    if c as libc::c_int >= 0xf0 as libc::c_int {
        *cp = c as libc::c_int & 0x7 as libc::c_int;
        extra = 3 as libc::c_int;
        min = 0x10000 as libc::c_int;
    } else if c as libc::c_int >= 0xe0 as libc::c_int {
        *cp = c as libc::c_int & 0xf as libc::c_int;
        extra = 2 as libc::c_int;
        min = 0x800 as libc::c_int;
    } else if c as libc::c_int >= 0xc0 as libc::c_int {
        *cp = c as libc::c_int & 0x1f as libc::c_int;
        extra = 1 as libc::c_int;
        min = 0x80 as libc::c_int;
    } else if c as libc::c_int >= 0x80 as libc::c_int {
        *cp = 0xfffd as libc::c_int;
    } else {
        *cp = c as libc::c_int;
    }
    loop {
        let fresh40 = extra;
        extra = extra - 1;
        if !(fresh40 != 0) {
            break;
        }
        let fresh41 = text;
        text = text.offset(1);
        c = *fresh41 as libc::c_uchar;
        if c as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int {
            *cp = 0xfffd as libc::c_int;
            break;
        } else {
            *cp = *cp << 6 as libc::c_int | c as libc::c_int & 0x3f as libc::c_int;
        }
    }
    if *cp < min {
        *cp = 0xfffd as libc::c_int;
    }
    return text;
}
#[no_mangle]
pub unsafe extern "C" fn tigrEncodeUTF8(
    mut text: *mut libc::c_char,
    mut cp: libc::c_int,
) -> *mut libc::c_char {
    if cp < 0 as libc::c_int || cp > 0x10ffff as libc::c_int {
        cp = 0xfffd as libc::c_int;
    }
    if cp < 0x80 as libc::c_int {
        let fresh42 = text;
        text = text.offset(1);
        *fresh42 = (0 as libc::c_int | cp >> 0 as libc::c_int & 0x7f as libc::c_int)
            as libc::c_char;
    } else if cp < 0x800 as libc::c_int {
        let fresh43 = text;
        text = text.offset(1);
        *fresh43 = (0xc0 as libc::c_int | cp >> 6 as libc::c_int & 0x1f as libc::c_int)
            as libc::c_char;
        let fresh44 = text;
        text = text.offset(1);
        *fresh44 = (0x80 as libc::c_int | cp >> 0 as libc::c_int & 0x3f as libc::c_int)
            as libc::c_char;
    } else if cp < 0x10000 as libc::c_int {
        let fresh45 = text;
        text = text.offset(1);
        *fresh45 = (0xe0 as libc::c_int | cp >> 12 as libc::c_int & 0xf as libc::c_int)
            as libc::c_char;
        let fresh46 = text;
        text = text.offset(1);
        *fresh46 = (0x80 as libc::c_int | cp >> 6 as libc::c_int & 0x3f as libc::c_int)
            as libc::c_char;
        let fresh47 = text;
        text = text.offset(1);
        *fresh47 = (0x80 as libc::c_int | cp >> 0 as libc::c_int & 0x3f as libc::c_int)
            as libc::c_char;
    } else {
        let fresh48 = text;
        text = text.offset(1);
        *fresh48 = (0xf0 as libc::c_int | cp >> 18 as libc::c_int & 0x7 as libc::c_int)
            as libc::c_char;
        let fresh49 = text;
        text = text.offset(1);
        *fresh49 = (0x80 as libc::c_int | cp >> 12 as libc::c_int & 0x3f as libc::c_int)
            as libc::c_char;
        let fresh50 = text;
        text = text.offset(1);
        *fresh50 = (0x80 as libc::c_int | cp >> 6 as libc::c_int & 0x3f as libc::c_int)
            as libc::c_char;
        let fresh51 = text;
        text = text.offset(1);
        *fresh51 = (0x80 as libc::c_int | cp >> 0 as libc::c_int & 0x3f as libc::c_int)
            as libc::c_char;
    }
    return text;
}
#[no_mangle]
pub unsafe extern "C" fn tigrBeginOpenGL(mut bmp: *mut Tigr) -> libc::c_int {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    (*win).gl.gl_user_opengl_rendering = 1 as libc::c_int;
    return (tigrGAPIBegin(bmp) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tigrSetPostShader(
    mut bmp: *mut Tigr,
    mut code: *const libc::c_char,
    mut size: libc::c_int,
) {
    tigrGAPIBegin(bmp);
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    let mut gl: *mut GLStuff = &mut (*win).gl;
    tigrCreateShaderProgram(gl, code, size);
    tigrGAPIEnd(bmp);
}
#[no_mangle]
pub unsafe extern "C" fn tigrSetPostFX(
    mut bmp: *mut Tigr,
    mut p1: libc::c_float,
    mut p2: libc::c_float,
    mut p3: libc::c_float,
    mut p4: libc::c_float,
) {
    let mut win: *mut TigrInternal = tigrInternal(bmp);
    (*win).p1 = p1;
    (*win).p2 = p2;
    (*win).p3 = p3;
    (*win).p4 = p4;
}
