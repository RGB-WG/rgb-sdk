%module rgb
%{
#include "../../librgb/librgb.h"
%}

%typemap(out) CResult %{
    switch ($1.result) {
        case CResultValue::Ok:
            $result = SWIG_NewPointerObj((new COpaqueStruct(static_cast< const COpaqueStruct& >($1.inner))), SWIGTYPE_p_COpaqueStruct, SWIG_POINTER_OWN |  0 );
            break;
        case CResultValue::Err:
            SWIG_V8_Raise((const char*) $1.inner.ptr);
            break;
    }
%}

%typemap(out) CResultString %{
    switch ($1.result) {
        case CResultValue::Ok:
            $result = v8::String::NewFromUtf8(args.GetIsolate(), (const char*) $1.inner);
            break;
        case CResultValue::Err:
            SWIG_V8_Raise((const char*) $1.inner);
            break;
    }
%}

%inline %{
    uint8_t *uint_array(int size) {
        return (uint8_t *) malloc(size*sizeof(uint8_t));
    }
    void uint_array_set(uint8_t *a, int i, int val) {
        a[i] = val;
    }
    // for debugging purposes
    int uint_array_get(uint8_t *a, int i) {
        return a[i];
    }
%}

%include "../../librgb/librgb.h"
