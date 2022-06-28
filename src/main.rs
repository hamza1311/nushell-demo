use nu_cli::{evaluate_file, get_init_cwd};
use nu_command::create_default_context;
use nu_engine::{get_full_help, CallExt};
use nu_protocol::{PipelineData, Span, engine::Stack, Value};

fn main() {
    let pwd = std::env::var("PWD").unwrap();
    let init_cwd = get_init_cwd();
    let mut engine_state = create_default_context(&init_cwd);
    let delta = {
        let mut working_set = nu_protocol::engine::StateWorkingSet::new(&engine_state);
//        working_set.add_decl(Box::new(nu_cli::NuHighlight));
        working_set.add_decl(Box::new(nu_cli::Print));

        working_set.render()
    };
    let _ = engine_state.merge_delta(delta, None, &init_cwd);

    let mut stack = Stack::new();

    stack.add_env_var("PWD".to_string(), Value::string(pwd, Span::new(0, 0)));

    let script_name = "somefile.nu".to_string();
    let args_to_script = [];
    let input = PipelineData::new(Span::new(0, 0));

    let ret_val = evaluate_file(
        script_name,
        &args_to_script,
        &mut engine_state,
        &mut stack,
        input,
        false,
    );

    // let _ = dbg!(ret_val);
}
