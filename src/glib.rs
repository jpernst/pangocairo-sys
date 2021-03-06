

pub type gboolean = ::libc::c_int;
pub type gint = ::libc::c_int;
pub type guint = ::libc::c_uint;
pub type gsize = ::libc::size_t;
pub type gssize = ::libc::ptrdiff_t;
pub type guint8 = u8;
pub type guint16 = u16;
pub type gint32 = i32;
pub type guint32 = u32;
pub type gchar = ::libc::c_char;
pub type guchar = ::libc::c_uchar;
pub type gunichar = ::libc::c_uint;

pub type gpointer = *mut ::libc::c_void;
pub type gconstpointer = *const ::libc::c_void;

pub type GType = ::libc::size_t;
pub type GQuark = u32;

pub type GFunc = extern "C" fn (data : gpointer, userdata : gpointer);
pub type GCopyFunc = extern "C" fn (src : gconstpointer, data : gpointer) -> gpointer;
pub type GCompareFunc = extern "C" fn (a : gconstpointer, b : gconstpointer) -> gint;
pub type GCompareDataFunc = extern "C" fn (a : gconstpointer, b : gconstpointer, user_data : gpointer) -> gint;
pub type GDestroyNotify = extern "C" fn (data : gpointer);


extern "C"
{
    pub fn g_object_unref (object : gpointer);
}


/* automatically generated by rust-bindgen */

// gerror.h
pub type GError = Struct__GError;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
impl ::std::clone::Clone for Struct__GError {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__GError {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn g_error_new(domain: GQuark, code: gint, format: *const gchar, ...)
     -> *mut GError;
    pub fn g_error_new_literal(domain: GQuark, code: gint,
                               message: *const gchar) -> *mut GError;
    //pub fn g_error_new_valist(domain: GQuark, code: gint,
                              //format: *const gchar, args: va_list)
     //-> *mut GError;
    pub fn g_error_free(error: *mut GError) ;
    pub fn g_error_copy(error: *const GError) -> *mut GError;
    pub fn g_error_matches(error: *const GError, domain: GQuark, code: gint)
     -> gboolean;
    pub fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint,
                       format: *const gchar, ...) ;
    pub fn g_set_error_literal(err: *mut *mut GError, domain: GQuark,
                               code: gint, message: *const gchar) ;
    pub fn g_propagate_error(dest: *mut *mut GError, src: *mut GError) ;
    pub fn g_clear_error(err: *mut *mut GError) ;
    pub fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...)
     ;
    pub fn g_propagate_prefixed_error(dest: *mut *mut GError,
                                      src: *mut GError,
                                      format: *const gchar, ...) ;
}

// glist.h
pub type GList = Struct__GList;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
impl ::std::clone::Clone for Struct__GList {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__GList {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn g_list_alloc() -> *mut GList;
    pub fn g_list_free(list: *mut GList) ;
    pub fn g_list_free_1(list: *mut GList) ;
    pub fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify)
     ;
    pub fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    pub fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    pub fn g_list_insert(list: *mut GList, data: gpointer, position: gint)
     -> *mut GList;
    pub fn g_list_insert_sorted(list: *mut GList, data: gpointer,
                                func: GCompareFunc) -> *mut GList;
    pub fn g_list_insert_sorted_with_data(list: *mut GList, data: gpointer,
                                          func: GCompareDataFunc,
                                          user_data: gpointer) -> *mut GList;
    pub fn g_list_insert_before(list: *mut GList, sibling: *mut GList,
                                data: gpointer) -> *mut GList;
    pub fn g_list_concat(list1: *mut GList, list2: *mut GList) -> *mut GList;
    pub fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    pub fn g_list_remove_all(list: *mut GList, data: gconstpointer)
     -> *mut GList;
    pub fn g_list_remove_link(list: *mut GList, llink: *mut GList)
     -> *mut GList;
    pub fn g_list_delete_link(list: *mut GList, link_: *mut GList)
     -> *mut GList;
    pub fn g_list_reverse(list: *mut GList) -> *mut GList;
    pub fn g_list_copy(list: *mut GList) -> *mut GList;
    pub fn g_list_copy_deep(list: *mut GList, func: GCopyFunc,
                            user_data: gpointer) -> *mut GList;
    pub fn g_list_nth(list: *mut GList, n: guint) -> *mut GList;
    pub fn g_list_nth_prev(list: *mut GList, n: guint) -> *mut GList;
    pub fn g_list_find(list: *mut GList, data: gconstpointer) -> *mut GList;
    pub fn g_list_find_custom(list: *mut GList, data: gconstpointer,
                              func: GCompareFunc) -> *mut GList;
    pub fn g_list_position(list: *mut GList, llink: *mut GList) -> gint;
    pub fn g_list_index(list: *mut GList, data: gconstpointer) -> gint;
    pub fn g_list_last(list: *mut GList) -> *mut GList;
    pub fn g_list_first(list: *mut GList) -> *mut GList;
    pub fn g_list_length(list: *mut GList) -> guint;
    pub fn g_list_foreach(list: *mut GList, func: GFunc, user_data: gpointer)
     ;
    pub fn g_list_sort(list: *mut GList, compare_func: GCompareFunc)
     -> *mut GList;
    pub fn g_list_sort_with_data(list: *mut GList,
                                 compare_func: GCompareDataFunc,
                                 user_data: gpointer) -> *mut GList;
    pub fn g_list_nth_data(list: *mut GList, n: guint) -> gpointer;
}

// gmarkup.h
pub type Enum_Unnamed1 = ::libc::c_uint;
pub const G_MARKUP_ERROR_BAD_UTF8: ::libc::c_uint = 0;
pub const G_MARKUP_ERROR_EMPTY: ::libc::c_uint = 1;
pub const G_MARKUP_ERROR_PARSE: ::libc::c_uint = 2;
pub const G_MARKUP_ERROR_UNKNOWN_ELEMENT: ::libc::c_uint = 3;
pub const G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE: ::libc::c_uint = 4;
pub const G_MARKUP_ERROR_INVALID_CONTENT: ::libc::c_uint = 5;
pub const G_MARKUP_ERROR_MISSING_ATTRIBUTE: ::libc::c_uint = 6;
pub type GMarkupError = Enum_Unnamed1;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG: ::libc::c_uint = 1;
pub const G_MARKUP_TREAT_CDATA_AS_TEXT: ::libc::c_uint = 2;
pub const G_MARKUP_PREFIX_ERROR_POSITION: ::libc::c_uint = 4;
pub const G_MARKUP_IGNORE_QUALIFIED: ::libc::c_uint = 8;
pub type GMarkupParseFlags = Enum_Unnamed2;
pub enum Struct__GMarkupParseContext { }
pub type GMarkupParseContext = Struct__GMarkupParseContext;
pub type GMarkupParser = Struct__GMarkupParser;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__GMarkupParser {
    pub start_element: ::std::option::Option<extern "C" fn(context:
                                                               *mut GMarkupParseContext,
                                                           element_name:
                                                               *const gchar,
                                                           attribute_names:
                                                               *mut *const gchar,
                                                           attribute_values:
                                                               *mut *const gchar,
                                                           user_data:
                                                               gpointer,
                                                           error:
                                                               *mut *mut GError)
                                                 >,
    pub end_element: ::std::option::Option<extern "C" fn(context:
                                                             *mut GMarkupParseContext,
                                                         element_name:
                                                             *const gchar,
                                                         user_data: gpointer,
                                                         error:
                                                             *mut *mut GError)
                                               >,
    pub text: ::std::option::Option<extern "C" fn(context:
                                                      *mut GMarkupParseContext,
                                                  text: *const gchar,
                                                  text_len: gsize,
                                                  user_data: gpointer,
                                                  error: *mut *mut GError)
                                        >,
    pub passthrough: ::std::option::Option<extern "C" fn(context:
                                                             *mut GMarkupParseContext,
                                                         passthrough_text:
                                                             *const gchar,
                                                         text_len: gsize,
                                                         user_data: gpointer,
                                                         error:
                                                             *mut *mut GError)
                                               >,
    pub error: ::std::option::Option<extern "C" fn(context:
                                                       *mut GMarkupParseContext,
                                                   error: *mut GError,
                                                   user_data: gpointer)
                                         >,
}
impl ::std::clone::Clone for Struct__GMarkupParser {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__GMarkupParser {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const G_MARKUP_COLLECT_INVALID: ::libc::c_uint = 0;
pub const G_MARKUP_COLLECT_STRING: ::libc::c_uint = 1;
pub const G_MARKUP_COLLECT_STRDUP: ::libc::c_uint = 2;
pub const G_MARKUP_COLLECT_BOOLEAN: ::libc::c_uint = 3;
pub const G_MARKUP_COLLECT_TRISTATE: ::libc::c_uint = 4;
pub const G_MARKUP_COLLECT_OPTIONAL: ::libc::c_uint = 65536;
pub type GMarkupCollectType = Enum_Unnamed3;
extern "C" {
    pub fn g_markup_error_quark() -> GQuark;
    pub fn g_markup_parse_context_new(parser: *const GMarkupParser,
                                      flags: GMarkupParseFlags,
                                      user_data: gpointer,
                                      user_data_dnotify: GDestroyNotify)
     -> *mut GMarkupParseContext;
    pub fn g_markup_parse_context_ref(context: *mut GMarkupParseContext)
     -> *mut GMarkupParseContext;
    pub fn g_markup_parse_context_unref(context: *mut GMarkupParseContext)
     ;
    pub fn g_markup_parse_context_free(context: *mut GMarkupParseContext)
     ;
    pub fn g_markup_parse_context_parse(context: *mut GMarkupParseContext,
                                        text: *const gchar, text_len: gssize,
                                        error: *mut *mut GError) -> gboolean;
    pub fn g_markup_parse_context_push(context: *mut GMarkupParseContext,
                                       parser: *const GMarkupParser,
                                       user_data: gpointer) ;
    pub fn g_markup_parse_context_pop(context: *mut GMarkupParseContext)
     -> gpointer;
    pub fn g_markup_parse_context_end_parse(context: *mut GMarkupParseContext,
                                            error: *mut *mut GError)
     -> gboolean;
    pub fn g_markup_parse_context_get_element(context:
                                                  *mut GMarkupParseContext)
     -> *const gchar;
    pub fn g_markup_parse_context_get_element_stack(context:
                                                        *mut GMarkupParseContext)
     -> *const GSList;
    pub fn g_markup_parse_context_get_position(context:
                                                   *mut GMarkupParseContext,
                                               line_number: *mut gint,
                                               char_number: *mut gint) ;
    pub fn g_markup_parse_context_get_user_data(context:
                                                    *mut GMarkupParseContext)
     -> gpointer;
    pub fn g_markup_escape_text(text: *const gchar, length: gssize)
     -> *mut gchar;
    pub fn g_markup_printf_escaped(format: *const ::libc::c_char, ...)
     -> *mut gchar;
    //pub fn g_markup_vprintf_escaped(format: *const ::libc::c_char,
                                    //args: va_list) -> *mut gchar;
    pub fn g_markup_collect_attributes(element_name: *const gchar,
                                       attribute_names: *mut *const gchar,
                                       attribute_values: *mut *const gchar,
                                       error: *mut *mut GError,
                                       first_type: GMarkupCollectType,
                                       first_attr: *const gchar, ...)
     -> gboolean;
}

// gslist.h
pub type GSList = Struct__GSList;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
impl ::std::clone::Clone for Struct__GSList {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__GSList {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn g_slist_alloc() -> *mut GSList;
    pub fn g_slist_free(list: *mut GSList) ;
    pub fn g_slist_free_1(list: *mut GSList) ;
    pub fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify)
     ;
    pub fn g_slist_append(list: *mut GSList, data: gpointer) -> *mut GSList;
    pub fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    pub fn g_slist_insert(list: *mut GSList, data: gpointer, position: gint)
     -> *mut GSList;
    pub fn g_slist_insert_sorted(list: *mut GSList, data: gpointer,
                                 func: GCompareFunc) -> *mut GSList;
    pub fn g_slist_insert_sorted_with_data(list: *mut GSList, data: gpointer,
                                           func: GCompareDataFunc,
                                           user_data: gpointer)
     -> *mut GSList;
    pub fn g_slist_insert_before(slist: *mut GSList, sibling: *mut GSList,
                                 data: gpointer) -> *mut GSList;
    pub fn g_slist_concat(list1: *mut GSList, list2: *mut GSList)
     -> *mut GSList;
    pub fn g_slist_remove(list: *mut GSList, data: gconstpointer)
     -> *mut GSList;
    pub fn g_slist_remove_all(list: *mut GSList, data: gconstpointer)
     -> *mut GSList;
    pub fn g_slist_remove_link(list: *mut GSList, link_: *mut GSList)
     -> *mut GSList;
    pub fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList)
     -> *mut GSList;
    pub fn g_slist_reverse(list: *mut GSList) -> *mut GSList;
    pub fn g_slist_copy(list: *mut GSList) -> *mut GSList;
    pub fn g_slist_copy_deep(list: *mut GSList, func: GCopyFunc,
                             user_data: gpointer) -> *mut GSList;
    pub fn g_slist_nth(list: *mut GSList, n: guint) -> *mut GSList;
    pub fn g_slist_find(list: *mut GSList, data: gconstpointer)
     -> *mut GSList;
    pub fn g_slist_find_custom(list: *mut GSList, data: gconstpointer,
                               func: GCompareFunc) -> *mut GSList;
    pub fn g_slist_position(list: *mut GSList, llink: *mut GSList) -> gint;
    pub fn g_slist_index(list: *mut GSList, data: gconstpointer) -> gint;
    pub fn g_slist_last(list: *mut GSList) -> *mut GSList;
    pub fn g_slist_length(list: *mut GSList) -> guint;
    pub fn g_slist_foreach(list: *mut GSList, func: GFunc,
                           user_data: gpointer) ;
    pub fn g_slist_sort(list: *mut GSList, compare_func: GCompareFunc)
     -> *mut GSList;
    pub fn g_slist_sort_with_data(list: *mut GSList,
                                  compare_func: GCompareDataFunc,
                                  user_data: gpointer) -> *mut GSList;
    pub fn g_slist_nth_data(list: *mut GSList, n: guint) -> gpointer;
}

// gstring.h
pub type GString = Struct__GString;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__GString {
    pub _str: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
impl ::std::clone::Clone for Struct__GString {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__GString {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn g_string_new(init: *const gchar) -> *mut GString;
    pub fn g_string_new_len(init: *const gchar, len: gssize) -> *mut GString;
    pub fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    pub fn g_string_free(string: *mut GString, free_segment: gboolean)
     -> *mut gchar;
    //pub fn g_string_free_to_bytes(string: *mut GString) -> *mut GBytes;
    pub fn g_string_equal(v: *const GString, v2: *const GString) -> gboolean;
    pub fn g_string_hash(str: *const GString) -> guint;
    pub fn g_string_assign(string: *mut GString, rval: *const gchar)
     -> *mut GString;
    pub fn g_string_truncate(string: *mut GString, len: gsize)
     -> *mut GString;
    pub fn g_string_set_size(string: *mut GString, len: gsize)
     -> *mut GString;
    pub fn g_string_insert_len(string: *mut GString, pos: gssize,
                               val: *const gchar, len: gssize)
     -> *mut GString;
    pub fn g_string_append(string: *mut GString, val: *const gchar)
     -> *mut GString;
    pub fn g_string_append_len(string: *mut GString, val: *const gchar,
                               len: gssize) -> *mut GString;
    pub fn g_string_append_c(string: *mut GString, c: gchar) -> *mut GString;
    pub fn g_string_append_unichar(string: *mut GString, wc: gunichar)
     -> *mut GString;
    pub fn g_string_prepend(string: *mut GString, val: *const gchar)
     -> *mut GString;
    pub fn g_string_prepend_c(string: *mut GString, c: gchar) -> *mut GString;
    pub fn g_string_prepend_unichar(string: *mut GString, wc: gunichar)
     -> *mut GString;
    pub fn g_string_prepend_len(string: *mut GString, val: *const gchar,
                                len: gssize) -> *mut GString;
    pub fn g_string_insert(string: *mut GString, pos: gssize,
                           val: *const gchar) -> *mut GString;
    pub fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar)
     -> *mut GString;
    pub fn g_string_insert_unichar(string: *mut GString, pos: gssize,
                                   wc: gunichar) -> *mut GString;
    pub fn g_string_overwrite(string: *mut GString, pos: gsize,
                              val: *const gchar) -> *mut GString;
    pub fn g_string_overwrite_len(string: *mut GString, pos: gsize,
                                  val: *const gchar, len: gssize)
     -> *mut GString;
    pub fn g_string_erase(string: *mut GString, pos: gssize, len: gssize)
     -> *mut GString;
    pub fn g_string_ascii_down(string: *mut GString) -> *mut GString;
    pub fn g_string_ascii_up(string: *mut GString) -> *mut GString;
    //pub fn g_string_vprintf(string: *mut GString, format: *const gchar,
                            //args: va_list) ;
    pub fn g_string_printf(string: *mut GString, format: *const gchar, ...)
     ;
    //pub fn g_string_append_vprintf(string: *mut GString, format: *const gchar,
                                   //args: va_list) ;
    pub fn g_string_append_printf(string: *mut GString,
                                  format: *const gchar, ...) ;
    pub fn g_string_append_uri_escaped(string: *mut GString,
                                       unescaped: *const gchar,
                                       reserved_chars_allowed: *const gchar,
                                       allow_utf8: gboolean) -> *mut GString;
    pub fn g_string_down(string: *mut GString) -> *mut GString;
    pub fn g_string_up(string: *mut GString) -> *mut GString;
}


