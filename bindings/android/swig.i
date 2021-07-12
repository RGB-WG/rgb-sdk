%module rgb
%{
#include "../../librgb/librgb.h"
%}

%typemap(jni) unsigned char "jbyte"
%typemap(jtype) unsigned char "byte"
%typemap(jstype) unsigned char "byte"

%typemap(jstype) CResult "COpaqueStruct"
%typemap(javaout) CResult {
    return new COpaqueStruct($jnicall, $owner);
}
%typemap(out) CResult %{
    switch ($1.result) {
        case CResultValue::Ok:
            *(COpaqueStruct **)&$result = new COpaqueStruct((const COpaqueStruct &) $1.inner);
            break;
        case CResultValue::Err:
            SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, (const char*) $1.inner.ptr);
            break;
    }
%}

%typemap(jni) CResultString "jstring"
%typemap(jtype) CResultString "String"
%typemap(jstype) CResultString "String"
%typemap(javaout) CResultString {
    return $jnicall;
}
%typemap(out) CResultString %{
    jstring j_string;
    switch ($1.result) {
        case CResultValue::Ok:
            $result = jenv->NewStringUTF((const char*) $1.inner);
            break;
        case CResultValue::Err:
            SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, (const char*) $1.inner);
            break;
    }
%}

%include "../../librgb/librgb.h"
