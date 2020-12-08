%module rgb_node
%{
#include "../../rust-lib/rgb_node.h"
%}

%typemap(out) CResult (v8::Local<v8::Promise::Resolver> resolver) %{
    resolver = v8::Promise::Resolver::New(args.GetIsolate());
    switch ($1.result) {
        case CResultValue::Ok:
            resolver->Resolve(SWIG_NewPointerObj((new COpaqueStruct(static_cast< const COpaqueStruct& >($1.inner))), SWIGTYPE_p_COpaqueStruct, SWIG_POINTER_OWN |  0 ));
            break;
        case CResultValue::Err:
            resolver->Reject(v8::String::NewFromUtf8(args.GetIsolate(), (const char*) $1.inner.ptr));
            break;
    }
    $result = resolver->GetPromise();
%}

%typemap(out) CResultString (v8::Local<v8::Promise::Resolver> resolver) %{
    resolver = v8::Promise::Resolver::New(args.GetIsolate());
    switch ($1.result) {
        case CResultValue::Ok:
            resolver->Resolve(v8::String::NewFromUtf8(args.GetIsolate(), (const char*) $1.inner));
            break;
        case CResultValue::Err:
            resolver->Reject(v8::String::NewFromUtf8(args.GetIsolate(), (const char*) $1.inner));
            break;
    }
    $result = resolver->GetPromise();
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

%include "../../rust-lib/rgb_node.h"
