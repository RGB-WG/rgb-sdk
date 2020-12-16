%module rgb
%{
#define SWIG_FILE_WITH_INIT
#include "../../librgb/librgb.h"
%}

%typemap(out) CResult %{
    switch ($1.result) {
        case CResultValue::Ok:
            $result = SWIG_NewPointerObj((new COpaqueStruct(static_cast< const COpaqueStruct& >($1.inner))), SWIGTYPE_p_COpaqueStruct, SWIG_POINTER_OWN |  0 );
            break;
        case CResultValue::Err:
            SWIG_exception_fail(SWIG_RuntimeError, (const char*) $1.inner.ptr);
            break;
    }
%}

%typemap(out) CResultString %{
    switch ($1.result) {
        case CResultValue::Ok:
            $result = PyString_FromString((const char*) $1.inner);
            break;
        case CResultValue::Err:
            SWIG_exception_fail(SWIG_RuntimeError, (const char*) $1.inner);
            break;
    }
%}

%include "../../librgb/librgb.h"
