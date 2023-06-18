use rustler::{NifStruct};

// Structs
#[derive(NifStruct)]
#[module = "Llamaex.Native.Context"]
struct Context {
    pub data: String
}

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn llama_init_from_file(filename: String) -> Context {
    let default_parameters = String::from("Some parameters");
    let ctx: Context = native_llama_init_from_file(filename, default_parameters);
    ctx
}

#[rustler::nif]
fn llama_tokenize(ctx: Context, prompt: String) -> String {
    native_llama_tokenize(ctx, prompt)
}

#[rustler::nif]
fn llama_free(ctx: Context) {
    native_llama_free(ctx)
}


// Private Methods that will get replaced with real code
fn native_llama_init_from_file(_filename: String, _default_parameters: String) -> Context {
    Context { data: String::from("This is a Context") }
}

fn native_llama_tokenize(_ctx: Context, _prompt: String) -> String {
    String::from("You just got a response")
}

fn native_llama_free(_ctx: Context) {
    // Just pretending...
    return
}


rustler::init!("Elixir.Llamaex.Native", [add, llama_init_from_file, llama_tokenize, llama_free]);
