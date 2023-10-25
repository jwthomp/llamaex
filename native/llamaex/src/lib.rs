use rustler::{Env, Error, ResourceArc, Term};
use llm::{InferenceSession, Model};
use std::sync::Mutex;
use std::{convert::Infallible, io::Write};

// Structs
//#[derive(NifStruct)]
//#[module = "Llamaex.Native.Context"]
struct Context {
    pub session: Mutex<InferenceSession>,
    pub model: llm::models::Llama
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(Context, env);
    true
}

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn llama_init_from_file(filename: String) -> Result<ResourceArc<Context>, Error> {
    let default_parameters = String::from("Some parameters");
    let (session, model) = native_llama_init_from_file(filename, default_parameters);

    let ctx = ResourceArc::new(Context {
        session: Mutex::new(session),
        model: model
    });
    Ok(ctx)
}

#[rustler::nif]
fn llama_tokenize(ctx: ResourceArc<Context>, prompt: String) -> Result<String, Error> {

    let result = native_llama_tokenize(ctx, prompt);
    Ok(result)
}

#[rustler::nif]
fn llama_free(ctx: ResourceArc<Context>) {
    native_llama_free(ctx)
}


// Private Methods that will get replaced with real code
fn native_llama_init_from_file(filename: String, _default_parameters: String) -> (InferenceSession, llm::models::Llama) {
    // load a GGML model from disk
    let llama = llm::load::<llm::models::Llama>(
        // path to GGML file
        std::path::Path::new(&*filename),
        // llm::ModelParameters
        Default::default(),
        // load progress callback
        llm::load_progress_callback_stdout
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

    let session = llama.start_session(Default::default());
    (session, llama)
}

fn native_llama_tokenize(ctx: ResourceArc<Context>, prompt: String) -> String {
    let mut session = ctx.session.try_lock().unwrap();
    let model = &ctx.model;

    let mut rng = rand::thread_rng();

    let _res = session.infer::<Infallible>(
        model,
        &mut rng,
        &llm::InferenceRequest {
            prompt: &*prompt,
            ..Default::default()

        },
        &mut Default::default(),
        |t| {
            print!("{t}");
            std::io::stdout().flush().unwrap();

            Ok(())
        },
    );

    String::from("You got a response")
}



fn native_llama_free(_ctx: ResourceArc<Context>) {
    

    // Just pretending...
    return
}


rustler::init!(
    "Elixir.Llamaex.Native", 
    [add, llama_init_from_file, llama_tokenize, llama_free],
    load = load
);
